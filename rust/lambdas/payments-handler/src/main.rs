use aws_lambda_events::{
    http::method::Method,
    lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse},
};
use bitie_types::{
    lambda,
    payments::{PaymentProcessorSecrets, QuestionDonation, STRIPE_SECRETS_ENV_VAR},
};
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use tracing::info;
use tracing_subscriber::filter::LevelFilter;

mod checkout;

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
    // get the name of the secret from the environment
    let secrets = match get_secrets().await {
        Some(v) => v,
        None => {
            info!("Missing payment processor secrets");
            return lambda::text_response(Some("Server misconfiguration.".to_string()), 500);
        }
    };

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

    if method != Method::POST {
        info!("Unsupported HTTP method - only POSTs are allowed");
        return lambda::text_response(Some("Unsupported HTTP method".to_string()), 400);
    }

    // this request must have a body
    let order_details = match &event.payload.body {
        Some(v) => match serde_json::from_str::<QuestionDonation>(v) {
            Ok(v) => v,
            Err(e) => {
                info!("Failed to parse the body: {:?}", e);
                return lambda::text_response(Some("Failed to parse the body".to_string()), 400);
            }
        },
        None => {
            info!("Missing HTTP body");
            return lambda::text_response(Some("Missing HTTP body. It's a bug.".to_string()), 400);
        }
    };

    info!("Order details: {:?}", order_details);

    // attempt to get the checkout URL from the payment provider and return it as text
    match checkout::get_checkout_url(order_details, secrets).await {
        Some(v) => lambda::text_response(Some(v), 200),
        None => {
            info!("Failed to get the checkout URL");
            lambda::text_response(Some("Failed to get the checkout URL".to_string()), 500)
        }
    }
}

/// Returns Stripe Key and Stripe Secret from AWS Secrets Manager or None if the secrets cannot be retrieved.
/// Errors are logged inside the function.
async fn get_secrets() -> Option<PaymentProcessorSecrets> {
    // It is logical to return Result, but the errors are handled inside the function, so Option is easier.

    // get the name of the secret from the environment
    let secret_arn = match std::env::var(STRIPE_SECRETS_ENV_VAR) {
        Ok(v) => v.trim().to_string(),
        Err(e) => {
            info!("Missing `{STRIPE_SECRETS_ENV_VAR}` env var with the ARN of the secret containing Stripe keys: {e}");
            return None;
        }
    };

    let asm = aws_sdk_secretsmanager::Client::new(&aws_config::load_from_env().await);

    let response = match asm.get_secret_value().secret_id(secret_arn).send().await {
        Ok(v) => v,
        Err(e) => {
            info!("Failed to get Stripe keys: {:?}", e);
            return None;
        }
    };

    let secrets = match response.secret_string() {
        Some(v) => match serde_json::from_str::<PaymentProcessorSecrets>(v) {
            Ok(v) => v,
            Err(e) => {
                info!("Failed to parse the secret: {:?}", e);
                return None;
            }
        },

        None => {
            info!("Secret is not a string");
            return None;
        }
    };

    Some(secrets)
}

#[cfg(test)]
mod tests {
    use super::*;
    use bitie_types::question::ContributorProfile;
    use test_log::test;

    #[test(tokio::test)]
    async fn test_get_secrets() {
        let secrets = get_secrets().await;
        assert!(secrets.is_some());
    }

    #[test(tokio::test)]
    async fn test_get_checkout_url() {
        let secrets = get_secrets().await.unwrap();

        let full_order_details = QuestionDonation {
            contributor: Some(ContributorProfile {
                name: Some("Consulting Solutions".to_string()),
                url: Some("https://example.com/consul-sol".to_string()),
                img_url: Some("https://example.com/consul-sol.png".to_string()),
                about: Some("We provide consulting solutions".to_string()),
            }),
            qty: 1,
            cancel_url: "https://example.com/retry".to_string(),
            success_url: "https://example.com/thankyou".to_string(),
            topics: Some("AWS Rust".to_string()),
        };

        let order_details = full_order_details.clone();

        let url = checkout::get_checkout_url(order_details, secrets.clone()).await;
        assert!(url.is_some(), "Full input URL");
        println!("Full input URL: {}", url.unwrap());

        let order_details = QuestionDonation {
            contributor: None,
            ..full_order_details.clone()
        };

        let url = checkout::get_checkout_url(order_details, secrets.clone()).await;
        assert!(url.is_some(), "No contrib URL");
        println!("No contrib URL: {}", url.unwrap());

        let order_details = QuestionDonation {
            contributor: None,
            topics: None,
            ..full_order_details.clone()
        };

        let url = checkout::get_checkout_url(order_details, secrets.clone()).await;
        assert!(url.is_some(), "No contrib, no topics URL");
        println!("No contrib, no topics URL: {}", url.unwrap());

        // invalid input tests
        assert!(
            checkout::get_checkout_url(
                QuestionDonation {
                    qty: 21,
                    ..full_order_details.clone()
                },
                secrets.clone(),
            )
            .await
            .is_none(),
            "Qty == 21"
        );
    }
}
