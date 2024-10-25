use aws_lambda_events::{
    http::header::HeaderMap,
    http::method::Method,
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
use bitie_types::{
    ddb::fields,
    jwt,
    lambda::text_response,
    // question::{Question, QuestionFormat},
    topic::Topic,
};
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

// mod photo;
mod user;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .without_time()
        .with_max_level(LevelFilter::INFO)
        .with_ansi(false)
        .init();

    let func = service_fn(my_handler);
    let runtime = Runtime::new(func);
    #[cfg(not(debug_assertions))]
    let runtime = runtime.layer(lambda_runtime::layers::TracingLayer::new());
    runtime.run().await?;
    Ok(())
}

pub(crate) async fn my_handler(
    event: LambdaEvent<LambdaFunctionUrlRequest>,
) -> Result<LambdaFunctionUrlResponse, Error> {
    // info!("Received event: {:?}", event);
    let path = event.payload.raw_path.clone().unwrap_or("".to_string());
    info!("Path: {}", path);

    // convert the method to the enum
    let method = match event.payload.request_context.http.method {
        Some(v) => {
            if let Ok(method) = Method::from_bytes(v.as_bytes()) {
                method
            } else {
                info!("Invalid HTTP method: {v}");
                return text_response(Some("Invalid HTTP method".to_string()), 400);
            }
        }
        None => {
            info!("Missing HTTP method");
            return text_response(Some("Missing HTTP method. It's a bug.".to_string()), 400);
        }
    };
    info!("Method: {}", method);

    let email = match get_email_from_token(&event.payload.headers) {
        Some(v) => v,
        None => {
            info!("Returning Unauthorized");
            return text_response(Some("Unauthorized".to_string()), 401);
        }
    };

    // topics param is required for get queries
    let topics = match event.payload.query_string_parameters.get(fields::TOPICS) {
        Some(v) if !v.trim().is_empty() => v
            .trim()
            .to_lowercase()
            .split('.')
            .filter_map(|t| {
                let t = t.trim();
                if t.is_empty() {
                    None
                } else {
                    Some(t.to_string())
                }
            })
            .collect::<Vec<String>>(),

        _ => {
            info!("No topic found in the query string");
            return text_response(Some("No topic found in the query string".to_string()), 400);
        }
    };
    let topics = Topic::filter_valid_topics(topics);

    //decide on the action depending on the HTTP method
    match method {
        Method::GET => {
            if topics.is_empty() {
                text_response(Some("No valid topics found".to_string()), 400)
            } else {
                match user::update_subscription(email, topics).await {
                    Ok(_) => text_response(None, 204),
                    Err(e) => text_response(Some(e.to_string()), 400),
                }
            }
        }

        // unsupported method
        _ => text_response(Some("Unsupported HTTP method".to_string()), 400),
    }
}

/// Returns an email from the token if the token is valid, normalized to lower case.
/// Returns None in any other case.
fn get_email_from_token(headers: &HeaderMap) -> Option<String> {
    const TOKEN_HEADER_NAME: &str = "x-bitie-token";

    // get the token from the headers
    let jwt = match headers.get(TOKEN_HEADER_NAME) {
        Some(v) => match v.to_str() {
            Ok(v) => v,
            Err(e) => {
                info!("Error converting token to string: {:?}", e);
                return None;
            }
        },
        None => {
            info!("Missing {TOKEN_HEADER_NAME} header");
            return None;
        }
    };

    jwt::get_email_from_token(jwt)
}
