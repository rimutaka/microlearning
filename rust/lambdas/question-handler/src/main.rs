use aws_lambda_events::{
    http::method::Method,
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
use aws_sdk_dynamodb::Client;
use bitie_types::{
    ddb::fields,
    lambda,
    question::{PublishStage, Question, QuestionFormat},
};
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use std::str::FromStr;
use tracing::{info, warn};
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

    let topic = event
        .payload
        .query_string_parameters
        .get(fields::TOPIC)
        .map(|v| v.trim().to_ascii_lowercase()) // trim whitespace, convert to lowercase
        .filter(|v| !v.is_empty()); // convert "" to None
    let qid = event
        .payload
        .query_string_parameters
        .get(fields::QID)
        .map(|v| v.trim().to_string()) // trim whitespace, qid is case sensitive
        .filter(|v| !v.is_empty()); // convert "" to None

    let client = Client::new(&aws_config::load_from_env().await);

    //decide on the action depending on the HTTP method
    match method {
        Method::GET => {
            // topic param is required for get queries
            let (topic, qid) = match (topic, qid) {
                (Some(topic), Some(qid)) => (topic, qid),
                _ => {
                    info!("Missing topic/qid in the query string");
                    return lambda::text_response(Some("No topic/qid found in the query string".to_string()), 400);
                }
            };

            // get the question from the DB
            let question = question::get(&client, &topic, &qid).await;

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
            // add / edit a question, get as markdown

            // must be an authenticated user
            let jwt_user = match jwt_user {
                Some(v) => v,
                None => {
                    info!("Unauthorized");
                    return lambda::text_response(Some("Unauthorized".to_string()), 401);
                }
            };

            match event.payload.body {
                // save the question in the DB if there is a body
                Some(body) => {
                    // info!("Received question: {body}");
                    let q = match Question::from_str(&body) {
                        // add the email hash of the current user and update the timestamp
                        Ok(v) => v
                            .with_author(&jwt_user.email_hash) // defaults to the current user
                            .with_updated()
                            .with_stage(PublishStage::Draft), // always reset it to Draft in save, other stages are set elsewhere
                        Err(_) => return lambda::text_response(Some("Invalid question".to_string()), 400),
                    };

                    // DDB returns an error if the author does not match
                    match question::save(&client, &q).await {
                        Ok(_) => lambda::json_response(Some(&q.format(QuestionFormat::HtmlShort)), 200),
                        Err(e) => lambda::text_response(Some(e.to_string()), 400),
                    }
                }
                // return the question in markdown format if there is no body
                None => {
                    let (topic, qid) = match (&topic, &qid) {
                        (Some(topic), Some(qid)) => (topic, qid),
                        _ => {
                            info!("Missing topic or qid in the query string: {:?} / {:?}", topic, qid);
                            return lambda::text_response(
                                Some("Missing topic or qid in the query string".to_string()),
                                400,
                            );
                        }
                    };
                    // return the question in markdown format if the author matches
                    let question = match question::get(&client, topic, qid).await {
                        Ok(Some(v)) => v,
                        Ok(None) => return lambda::text_response(Some("No question found".to_string()), 404), // this would be a bug
                        Err(e) => return lambda::text_response(Some(e.to_string()), 400),
                    };

                    if question.author.as_ref() == Some(&jwt_user.email_hash) {
                        lambda::json_response(Some(&question.format(QuestionFormat::MarkdownFull)), 200)
                    } else {
                        warn!("Unauthorized (email hash mismatch): {:?}", jwt_user);
                        lambda::text_response(Some("Unauthorized".to_string()), 401)
                    }
                }
            }
        }

        // unsupported method
        _ => lambda::text_response(Some("Unsupported HTTP method".to_string()), 400),
    }
}
