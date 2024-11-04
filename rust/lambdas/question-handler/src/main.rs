use aws_lambda_events::{
    http::method::Method,
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
use aws_sdk_dynamodb::Client;
use bitie_types::{
    ddb::fields,
    jwt::JwtUser,
    lambda,
    question::{Question, QuestionFormat},
};
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use std::str::FromStr;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

mod question;
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

    // the user may be authenticated with an email inside the token
    let jwt_user = lambda::get_email_from_token(&event.payload.headers);
    // topics param is optional
    let answers = match lambda::url_list_to_vec(event.payload.query_string_parameters.get(fields::ANSWERS)) {
        Some(v) => Some(v.iter().filter_map(|v| v.parse::<usize>().ok()).collect()),
        None => {
            info!("No answers param in the query string");
            None
        }
    };
    // get x-bitie-recent header if present
    let recent_questions = event
        .payload
        .headers
        .get(lambda::X_BITIE_RECENT)
        .map(|v| v.to_str().unwrap_or_default().to_string());

    let topic = event.payload.query_string_parameters.get(fields::TOPIC);
    let qid = event.payload.query_string_parameters.get(fields::QID);

    let client = Client::new(&aws_config::load_from_env().await);

    //decide on the action depending on the HTTP method
    match method {
        Method::GET => {
            // topic param is required for get queries
            let topic = match topic {
                Some(v) if !v.trim().is_empty() => v.trim().to_lowercase(),
                _ => {
                    info!("No topics found in the query string");
                    return lambda::text_response(Some("No topics found in the query string".to_string()), 400);
                }
            };

            // get the question from the DB
            let question = match qid {
                Some(qid) if !qid.is_empty() => question::get_exact(&client, &topic, qid).await,

                _ => question::get_random(&client, &topic, &recent_questions).await,
            };

            let question = match question {
                Ok(Some(v)) => v,
                Ok(None) => {
                    info!("No question found for topic: {topic}");
                    return lambda::text_response(Some("No question found".to_string()), 404);
                }
                Err(e) => return lambda::text_response(Some(e.to_string()), 400),
            };

            // update the user answers if the user is known
            // the logic to update or not is inside the function
            user::update_answers(&client, &jwt_user, &question, &answers).await;
            question::update_answer_stats(&client, &jwt_user, &question, &answers).await;

            // no answers means initial question display and no explanations
            let response_format = if answers.is_some() {
                QuestionFormat::HtmlFull(answers.clone())
            } else {
                QuestionFormat::HtmlShort
            };

            lambda::json_response(Some(&question.format(response_format)), 200)
        }

        Method::PUT => {
            // a temporary hack to limit who can post questions
            if !can_post(&jwt_user) {
                return lambda::text_response(Some("Unauthorized".to_string()), 401);
            }

            if let Some(jwt_user) = jwt_user {
                if let Some(body) = event.payload.body {
                    // info!("Received question: {body}");
                    let q = match Question::from_str(&body) {
                        Ok(v) => v.with_author(&jwt_user.email_hash).with_updated(),
                        Err(_) => return lambda::text_response(Some("Invalid question".to_string()), 400),
                    };

                    match question::save(&client, &q).await {
                        Ok(_) => lambda::json_response(Some(&q.format(QuestionFormat::MarkdownFull)), 200),
                        Err(e) => lambda::text_response(Some(e.to_string()), 400),
                    }
                } else {
                    let (topic, qid) = match (topic, qid) {
                        (Some(topic), Some(qid)) => (topic, qid),
                        _ => {
                            info!("Missing topic or qid in the query string: {:?} / {:?}", topic, qid);
                            return lambda::text_response(
                                Some("Missing topic or qid in the query string".to_string()),
                                400,
                            );
                        }
                    };
                    // return the question in markdown format
                    match question::get_exact(&client, topic, qid).await {
                        Ok(Some(v)) => lambda::json_response(Some(&v.format(QuestionFormat::MarkdownFull)), 200),
                        Ok(None) => lambda::text_response(Some("No question found".to_string()), 404), // this would be a bug
                        Err(e) => lambda::text_response(Some(e.to_string()), 400),
                    }
                }
            } else {
                lambda::text_response(Some("Unauthorized".to_string()), 401)
            }
        }

        // unsupported method
        _ => lambda::text_response(Some("Unsupported HTTP method".to_string()), 400),
    }
}

/// Returns true if the email from jwt_user matches the token in the environment var.
/// It is a temporary solution for authenticating DDB update requests.
/// Logs an error if the token env var is missing.
fn can_post(jwt_user: &Option<JwtUser>) -> bool {
    const TOKEN_ENV_VAR: &str = "x_bitie_token";

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
    match jwt_user {
        Some(v) => v.email == token_env,
        None => {
            info!("No email. Cannot allow posting.");
            false
        }
    }
}
