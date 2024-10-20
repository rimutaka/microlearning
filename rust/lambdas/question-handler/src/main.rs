use aws_lambda_events::{
    http::header::HeaderMap,
    http::method::Method,
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
use bitie_types::{
    ddb::fields,
    lambda::{json_response, text_response},
    question::{Question, QuestionFormat},
};
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use std::str::FromStr;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

// mod photo;
mod question;

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

    //decide on the action depending on the HTTP method
    match method {
        Method::GET => {
            // topic param is required for get queries
            let topic = match event.payload.query_string_parameters.get(fields::TOPIC) {
                Some(v) if !v.trim().is_empty() => v.trim().to_lowercase(),
                _ => {
                    info!("No topic found in the query string");
                    return text_response(Some("No topic found in the query string".to_string()), 400);
                }
            };

            // get the question from the DB
            match event.payload.query_string_parameters.get(fields::QID) {
                Some(qid) if !qid.is_empty() => match question::get_exact(&topic, qid).await {
                    Ok(v) => {
                        let response_format = if is_valid_token(&event.payload.headers) {
                            // the caller will include the token only if markdown is requested
                            info!("Token found");
                            QuestionFormat::MarkdownFull
                        } else {
                            QuestionFormat::HtmlShort
                        };

                        json_response(Some(&v.format(response_format)), 200)
                    }
                    Err(e) => text_response(Some(e.to_string()), 400),
                },
                _ => match question::get_random(&topic).await {
                    Ok(v) => json_response(Some(&v.format(QuestionFormat::HtmlShort)), 200),
                    Err(e) => text_response(Some(e.to_string()), 400),
                },
            }
        }

        Method::PUT => {
            // all put events must be authorized
            if !is_valid_token(&event.payload.headers) {
                return text_response(Some("Unauthorized".to_string()), 401);
            }

            let body = match event.payload.body {
                Some(v) => v,
                None => {
                    info!("Missing body");
                    return text_response(Some("Missing body".to_string()), 400);
                }
            };

            let q = match Question::from_str(&body) {
                Ok(v) => v,
                Err(_) => return text_response(Some("Invalid question".to_string()), 400),
            };

            match question::save(&q).await {
                Ok(_) => json_response(Some(&q.format(QuestionFormat::MarkdownFull)), 200),
                Err(e) => text_response(Some(e.to_string()), 400),
            }
        }

        Method::POST => {
            // topic param is required for get queries
            let topic = match event.payload.query_string_parameters.get(fields::TOPIC) {
                Some(v) if !v.trim().is_empty() => v.trim().to_lowercase(),
                _ => {
                    info!("No topic found in the query string");
                    return text_response(Some("No topic found in the query string".to_string()), 400);
                }
            };

            let qid = match event.payload.query_string_parameters.get(fields::QID) {
                Some(v) if !v.is_empty() => v,
                _ => {
                    info!("No qid found in the query string");
                    return text_response(Some("No qid found in the query string".to_string()), 400);
                }
            };

            // get the list of answers from the body
            let body = match event.payload.body {
                Some(v) => v,
                None => {
                    info!("Missing body");
                    return text_response(Some("Missing body".to_string()), 400);
                }
            };

            // parse them, but we have nowhere to save them yet
            let answers = match serde_json::from_str::<Vec<u8>>(&body) {
                Ok(v) if !v.is_empty() => v,
                _ => {
                    info!("Invalid list of answers: {body}");
                    return text_response(Some("Invalid list of answers".to_string()), 400);
                }
            };
            info!("Answers: {:?}", answers);

            // get the question from the DB and return as HTML with explanations
            match question::get_exact(&topic, qid).await {
                Ok(v) => json_response(Some(&v.format(QuestionFormat::HtmlFull(Some(answers)))), 200),
                Err(e) => text_response(Some(e.to_string()), 400),
            }
        }

        // unsupported method
        _ => text_response(Some("Unsupported HTTP method".to_string()), 400),
    }
}

/// Returns true if the token in the headers matches the token in the environment var.
/// It is a temporary solution for authenticating DDB update requests.
/// Logs an error if the token env var is missing.
fn is_valid_token(headers: &HeaderMap) -> bool {
    const TOKEN_ENV_VAR: &str = "x_bitie_token";
    const TOKEN_HEADER_NAME: &str = "x-bitie-token";

    // get the token from the environment
    let token_env = match std::env::var(TOKEN_ENV_VAR) {
        Ok(v) => v.trim().to_string(),
        Err(e) => {
            info!("Missing {TOKEN_ENV_VAR} with a token: {e}");
            return false;
        }
    };

    // make sure the token env var is not empty
    if token_env.is_empty() {
        info!("Empty {TOKEN_ENV_VAR}");
        return false;
    }

    // get the token from the headers and compare
    match headers.get(TOKEN_HEADER_NAME) {
        Some(v) => v.to_str().unwrap_or_default() == token_env,
        None => {
            info!("Missing {TOKEN_HEADER_NAME} header");
            false
        }
    }
}
