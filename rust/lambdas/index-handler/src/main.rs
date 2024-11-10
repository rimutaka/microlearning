use aws_lambda_events::lambda_function_urls::{LambdaFunctionUrlRequest, LambdaFunctionUrlResponse};
use bitie_types::{ddb::fields, lambda, topic::Topic};
use index::get_index_from_s3;
use lambda_runtime::{service_fn, Error, LambdaEvent, Runtime};
use tracing::{error, info};
use tracing_subscriber::filter::LevelFilter;

mod index;

#[tokio::main]
async fn main() -> Result<(), Error> {
    // required to enable CloudWatch error logging by the runtime
    tracing_subscriber::fmt()
        .with_ansi(false)
        .without_time()
        .with_max_level(LevelFilter::INFO)
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
    let path = event.payload.raw_path.clone().unwrap_or("".to_string());
    info!("Path: {}", path);

    // get index.html from S3
    let index_html = get_index_from_s3().await?;

    let topic = match event.payload.query_string_parameters.get(fields::TOPIC) {
        Some(v) => v.trim().to_ascii_lowercase(),
        _ => {
            // log the error, return the original HTML and let the frontend handle it
            info!("Missing topic in the query string");
            return lambda::text_response(Some(index_html), 200);
        }
    };
    info!("Topic: {:?}", topic);

    // return index.html with OG tags updated
    lambda::text_response(Some(replace_with_regex(index_html, &topic)), 200)
}

/// Replaces Title and ogImage in the HTML, if the new values are not empty strings.
/// Otherwise keeps the existing values.
fn replace_with_regex(index_html: String, topic: &str) -> String {
    // get the user-friendly topic name
    let topic_name = match Topic::TOPICS.iter().position(|v| v == &topic) {
        Some(v) => Topic::TOPIC_NAMES[v],
        None => {
            // log the error, return the original HTML and let the frontend handle it
            info!("Invalid topic: {:?}", topic);
            return index_html;
        }
    };

    // description should not be empty
    // we use a generic blurb if the book data has none
    // trip it to 500 characters
    // let description = ["Can you answer this question about ", topic_name, "?"].concat();

    // // <meta name="description" content="A pocket assistant ...">
    // let index_html = match regex::Regex::new(r#"("description"[^>]+content=")([^"]+)"#) {
    //     Ok(v) => v.replace(&index_html, ["${1}", &description].concat()),
    //     Err(e) => {
    //         error!("Invalid description regex. It's a bug. {:?}", e);
    //         std::borrow::Cow::Borrowed(index_html.as_str())
    //     }
    // };

    // // <meta property="og:description" content="A pocket assistant for keen readers...">
    // let index_html = match regex::Regex::new(r#"("og:description"[^>]+content=")([^"]+)"#) {
    //     Ok(v) => v.replace(&index_html, ["${1}", &description].concat()),
    //     Err(e) => {
    //         error!("Invalid og:description regex. It's a bug. {:?}", e);
    //         index_html
    //     }
    // };

    // // <meta name="twitter:description" content="A pocket assistant for keen readers..." />
    // let index_html = match regex::Regex::new(r#"("twitter:description"[^>]+content=")([^"]+)"#) {
    //     Ok(v) => v.replace(&index_html, ["${1}", &description].concat()),
    //     Err(e) => {
    //         error!("Invalid twitter:description regex. It's a bug. {:?}", e);
    //         index_html
    //     }
    // };

    let title = [topic_name, ": something I learned today"].concat();

    // replace the title in multiple places if the value is not empty
    // <title>bla-bla</title>
    let index_html = match regex::Regex::new(r"(<title>)([^<]+)") {
        Ok(v) => v.replace(&index_html, ["${1}", &title].concat()),
        Err(e) => {
            error!("Invalid title regex. It's a bug. {:?}", e);
            std::borrow::Cow::Borrowed(index_html.as_str())
        }
    };

    // <meta property="og:title" content="bla...bla" />
    let index_html = match regex::Regex::new(r#"("og:title"[^>]+content=")([^"]+)"#) {
        Ok(v) => v.replace(&index_html, ["${1}", &title].concat()),
        Err(e) => {
            error!("Invalid og:title regex. It's a bug. {:?}", e);
            index_html
        }
    };

    // <meta name="twitter:title" content="bla...bla..." />
    let index_html = match regex::Regex::new(r#"("twitter:title"[^>]+content=")([^"]+)"#) {
        Ok(v) => v.replace(&index_html, ["${1}", &title].concat()),
        Err(e) => {
            error!("Invalid twitter:title regex. It's a bug. {:?}", e);
            index_html
        }
    };

    // replace images with the topic-specific image
    // e.g. <meta property="og:image" itemprop="image" content="https://bitesized.info/og-image.png" />
    // the new name should be og-aws.png, og-rust.png, etc.
    let og_image = ["/og-images/og-", topic, ".png"].concat();
    index_html.replace("/og-images/default.png", &og_image)
}
