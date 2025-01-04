use aws_lambda_events::{
    http::method::Method,
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
use aws_sdk_dynamodb::Client;
use bitie_types::{ddb::fields, lambda, question::PublishStage};
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use std::str::FromStr;
use tracing::{info, warn};
use tracing_subscriber::filter::LevelFilter;

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
                return lambda::text_response(Some("Invalid HTTP method".to_string()), 400);
            }
        }
        None => {
            info!("Missing HTTP method");
            return lambda::text_response(Some("Missing HTTP method. It's a bug.".to_string()), 400);
        }
    };
    info!("Method: {}", method);

    // topic, qid and stage are required - exit if any of them is missing

    let topic = match event.payload.query_string_parameters.get(fields::TOPIC) {
        Some(v) => v.trim().to_ascii_lowercase(),
        None => {
            info!("Missing topic in the query string");
            return lambda::text_response(Some("Missing topic in the query string".to_string()), 400);
        }
    };

    let qid = match event.payload.query_string_parameters.get(fields::QID) {
        Some(v) => v.trim(),
        None => {
            info!("Missing qid in the query string");
            return lambda::text_response(Some("Missing qid in the query string".to_string()), 400);
        }
    };

    let stage = match event.payload.query_string_parameters.get(fields::STAGE) {
        Some(v) => match PublishStage::from_str(v) {
            Ok(s) => s,
            Err(e) => {
                warn!("Invalid stage in the query string: {v} / {:?}", e);
                return lambda::text_response(Some("Invalid stage in the query string".to_string()), 400);
            }
        },
        None => {
            info!("Missing qid in the query string");
            return lambda::text_response(Some("Invalid stage in the query string".to_string()), 400);
        }
    };

    // this action is only allowed for mods
    // TODO: use some other way of determining if the user is a mod
    match lambda::get_email_from_token(&event.payload.headers) {
        Some(v) if v.email_hash == "0e3bf888c95b085a7172b2e819692bb5b46c26ad067f9405c8ba1dd950732b65" => {
            info!("Stage change by {}: {topic}/{qid}/{stage}", v.email)
        }
        _ => {
            info!("Missing user email in the token");
            return lambda::text_response(Some("Must provide a mod's token".to_string()), 400);
        }
    }

    let client = Client::new(&aws_config::load_from_env().await);

    //decide on the action depending on the HTTP method
    match method {
        Method::GET => match question::change_publish_stage(&client, &topic, qid, stage).await {
            Ok(_) => lambda::text_response(None, 204),
            Err(e) => lambda::text_response(Some(e.to_string()), 500),
        },

        // unsupported method
        _ => lambda::text_response(Some("Unsupported HTTP method".to_string()), 400),
    }
}
