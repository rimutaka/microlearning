use crate::jwt;
use aws_lambda_events::{
    http::{HeaderMap, HeaderValue},
    lambda_function_urls::LambdaFunctionUrlResponse,
};
use lambda_runtime::Error;
use serde::Serialize;
use tracing::info;

// /// The header name for the question format.
// /// The value should be one of the `QuestionFormat` enum values.
// /// CloudFront has to be configured to vary the cache depending on the header contents.
// pub const QUESTION_FORMAT_HEADER_NAME: &str = "x-bitie-question-format";

/// A shortcut for returning the lambda response in the required format.
/// Always returns OK.
/// Adds `Content-Type=text/html` header.
pub fn text_response(body: Option<String>, status: i64) -> Result<LambdaFunctionUrlResponse, Error> {
    // a collector for all headers added along the way
    let mut headers = HeaderMap::new();
    headers.append("Content-Type", HeaderValue::from_static("text/html; charset=utf-8"));

    Ok(LambdaFunctionUrlResponse {
        status_code: status,
        headers,
        cookies: Default::default(),
        body,
        is_base64_encoded: false,
    })
}

/// A shortcut for returning the lambda response in the required format.
/// May return an error if the serialization of the body object fails.
/// Adds `Content-Type=application/json` header.
pub fn json_response<T: Serialize>(body: Option<&T>, status: i64) -> Result<LambdaFunctionUrlResponse, Error> {
    let body = match body {
        Some(v) => match serde_json::to_string(v) {
            Ok(v) => Some(v),
            Err(e) => {
                info!("Failed to serialize the response: {:?}", e);
                return text_response(Some(e.to_string()), 400);
            }
        },
        None => None,
    };

    // a collector for all headers added along the way
    let mut headers = HeaderMap::new();
    headers.append(
        "Content-Type",
        HeaderValue::from_static("application/json; charset=utf-8"),
    );

    Ok(LambdaFunctionUrlResponse {
        status_code: status,
        headers,
        cookies: Default::default(),
        body,
        is_base64_encoded: false,
    })
}

/// Returns user details, if the token is valid :
/// Otherwise returns None.
/// All errors are logged inside the function.
pub fn get_email_from_token(headers: &HeaderMap) -> Option<jwt::JwtUser> {
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

/// Converts a .-separated list of values in a URL to a Vec<String>.
/// E.g. for "?topics=a.b.c" it is "a.b.c" -> ["a", "b", "c"]
pub fn url_list_to_vec(url_list: Option<&String>) -> Option<Vec<String>> {
    match url_list {
        Some(v) if v.trim().is_empty() => {
            // info!("URL list is empty");
            Some(Vec::new())
        }
        Some(v) => {
            let values = v
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
                .collect::<Vec<String>>();
            // convert the list of answers to a Vec<u8>
            // info!("URL list len: {}", values.len());
            Some(values)
        }

        None => {
            // info!("No answers param in the query string");
            None
        }
    }
}

#[test]
fn test_url_list_to_vec() {
    assert_eq!(url_list_to_vec(None), None);
    assert_eq!(url_list_to_vec(Some(&"".to_string())), Some(Vec::new()));
    assert_eq!(
        url_list_to_vec(Some(&"a".to_string())),
        Some(vec!["a"].into_iter().map(|v| v.to_string()).collect())
    );
    assert_eq!(
        url_list_to_vec(Some(&"a.b.c".to_string())),
        Some(vec!["a", "b", "c"].into_iter().map(|v| v.to_string()).collect())
    );
    assert_eq!(
        url_list_to_vec(Some(&"a.b.c...".to_string())),
        Some(vec!["a", "b", "c"].into_iter().map(|v| v.to_string()).collect())
    );
    assert_eq!(
        url_list_to_vec(Some(&"a.b.c..d".to_string())),
        Some(vec!["a", "b", "c", "d"].into_iter().map(|v| v.to_string()).collect())
    );
}
