use aws_lambda_events::{
    http::method::Method,
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
use bitie_types::{
    ddb::fields,
    lambda,
    // question::{Question, QuestionFormat},
    topic::Topic,
};
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;

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
                return lambda::text_response(Some("Invalid HTTP method".to_string()), 400);
            }
        }
        None => {
            info!("Missing HTTP method");
            return lambda::text_response(Some("Missing HTTP method. It's a bug.".to_string()), 400);
        }
    };
    info!("Method: {}", method);

    // can only proceed if the user is authenticated with an email
    let jwt_user = match lambda::get_email_from_token(&event.payload.headers) {
        Some(v) => v,
        None => {
            info!("Returning Unauthorized");
            return lambda::text_response(Some("Unauthorized".to_string()), 401);
        }
    };

    // topics param is optional
    let topics = lambda::url_list_to_vec(event.payload.query_string_parameters.get(fields::TOPICS))
        .map(Topic::filter_valid_topics);

    //decide on the action depending on the HTTP method
    match method {
        Method::GET => {
            // get the user or update the user subscription
            let user = match topics {
                Some(v) => user::update_subscription(&jwt_user.email, v).await,
                None => user::get_user(&jwt_user.email).await,
            };

            // create a new user if it's the first time login
            let user = match user {
                Ok(Some(v)) => Ok(Some(v)),
                Ok(None) => user::create_new(&jwt_user.email, &jwt_user.email_hash).await,
                _ => user,
            };

            // return the right response
            match user {
                Ok(Some(v)) => lambda::json_response(Some(&v), 200),
                Ok(None) => {
                    error!("User not found after it was created");
                    lambda::text_response(Some("Failed to created a new user".to_owned()), 500)
                }
                Err(e) => lambda::text_response(Some(e.to_string()), 400),
            }
        }

        // unsupported method
        _ => lambda::text_response(Some("Unsupported HTTP method".to_string()), 400),
    }
}
