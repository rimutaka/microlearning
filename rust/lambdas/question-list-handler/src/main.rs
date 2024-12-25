use aws_lambda_events::{
    http::method::Method,
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
use aws_sdk_dynamodb::Client;
use bitie_types::{ddb::fields, lambda, relations::QuestionWithHistory, user::AskedQuestion};
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use std::collections::HashMap;
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

mod questions;
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

    // get the topic from the query string
    let topic = match event.payload.query_string_parameters.get(fields::TOPIC) {
        Some(v) => {
            let v = v.trim().to_lowercase();
            if !questions::validate_topic(&v) {
                info!("Invalid topic: {v}");
                return lambda::text_response(Some("Invalid topic".to_string()), 400);
            }
            v
        }
        None => {
            info!("Missing topic param");
            return lambda::text_response(Some("Missing topic param.".to_string()), 400);
        }
    };

    // get user details from the JWT token
    let jwt_user = lambda::get_email_from_token(&event.payload.headers);

    let client = Client::new(&aws_config::load_from_env().await);

    //decide on the action depending on the HTTP method
    match method {
        Method::GET => {
            // get the question from the DB
            let questions = match questions::get_all_questions_by_topic(&client, &topic).await {
                Ok(v) => v,
                Err(e) => return lambda::text_response(Some(e.to_string()), 500),
            };

            if let Some(jwt_user) = jwt_user {
                // get the user's question history
                let mut user_question_history =
                    match user::get_user_question_history(&client, Some(&topic), &jwt_user.email).await {
                        Some(v) => {
                            // reduce the list to one entry per question to see if the question is worth looking at or has been answered before
                            // and convert it into a hashmap for quicker search
                            AskedQuestion::latest_answer_list(v)
                                .into_iter()
                                .map(|v| (v.qid.clone(), v))
                                .collect::<HashMap<String, AskedQuestion>>()
                        }
                        None => HashMap::new(),
                    };

                info!(
                    "Reduced history to one status per question: {}",
                    user_question_history.len()
                );

                // combine the questions with the user's history
                let questions_with_history = questions
                    .into_iter()
                    .map(|v| {
                        let history = user_question_history.remove(&v.qid).map(|v| vec![v]);
                        QuestionWithHistory { question: v, history }
                    })
                    .collect::<Vec<QuestionWithHistory>>();

                return lambda::json_response(Some(&questions_with_history), 200);
            }

            // convert questions into questions with history, but without the history
            let questions_with_history = questions
                .into_iter()
                .map(|v| QuestionWithHistory {
                    question: v,
                    history: None,
                })
                .collect::<Vec<QuestionWithHistory>>();

            lambda::json_response(Some(&questions_with_history), 200)
        }

        // unsupported method
        _ => lambda::text_response(Some("Unsupported HTTP method".to_string()), 400),
    }
}
