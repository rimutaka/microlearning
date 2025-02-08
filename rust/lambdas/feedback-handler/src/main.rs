use aws_lambda_events::{
    http::method::Method,
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
use bitie_types::{ddb::fields, question::Question, topic::Topic};
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use tracing::{info, warn};
use tracing_subscriber::filter::LevelFilter;

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
                return lambda_utils::text_response(Some("Invalid HTTP method".to_string()), 400);
            }
        }
        None => {
            info!("Missing HTTP method");
            return lambda_utils::text_response(Some("Missing HTTP method. It's a bug.".to_string()), 400);
        }
    };
    info!("Method: {}", method);
    if method != Method::POST {
        return lambda_utils::text_response(Some("Only POST method is accepted".to_string()), 400);
    }

    // the user may be authenticated with an email inside the token
    let user_email = match lambda_utils::get_email_from_token(&event.payload.headers) {
        Some(v) => v.email,
        None => "".to_string(),
    };

    let user_ip = match event.payload.request_context.http.source_ip {
        Some(v) => v,
        None => {
            warn!("Missing source IP");
            "".to_string()
        }
    };

    info!("Submitter: {user_email} / {user_ip}");

    let (topic_id, topic_name) = match event.payload.query_string_parameters.get(fields::TOPIC) {
        Some(v) => (v.clone(), Topic::into_name(v).to_string()),
        None => {
            warn!("Missing topic");
            return lambda_utils::text_response(Some("Missing topic param".to_string()), 400);
        }
    };

    if topic_name.is_empty() {
        warn!("Invalid topic: {topic_id}");
        return lambda_utils::text_response(Some("Invalid topic".to_string()), 400);
    }

    let qid = match event.payload.query_string_parameters.get(fields::QID) {
        Some(v) => v.trim().to_string(),
        None => {
            warn!("Missing qid");
            return lambda_utils::text_response(Some("Missing qid param".to_string()), 400);
        }
    };

    if !Question::validate_qid(&qid) {
        warn!("Invalid qid: {qid}");
        return lambda_utils::text_response(Some("Invalid qid".to_string()), 400);
    }

    let feedback_text = match event.payload.body {
        Some(v) => {
            let v = v.trim();
            if v.chars().count() < 10 {
                return lambda_utils::text_response(Some("Feedback text too short".to_string()), 400);
            }
            if v.chars().count() > 2_000 {
                return lambda_utils::text_response(Some("Feedback text too long".to_string()), 400);
            }
            v.to_string()
        }

        None => {
            warn!("Missing feedback text");
            return lambda_utils::text_response(Some("Missing feedback text".to_string()), 400);
        }
    };

    let subject = format!("Feedback for {topic_name}/{qid}");
    let question_url = format!("https://bitesized.info/question?topic={topic_id}&qid={qid}");
    let body = format!("{question_url}\n\nSubmitter: {user_email} / {user_ip}\n\n\n{feedback_text}");

    lambda_utils::email::send_text_email("max@onebro.me", &subject, &body).await;
    lambda_utils::text_response(None, 204)
}
