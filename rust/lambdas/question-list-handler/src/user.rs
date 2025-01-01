use aws_sdk_dynamodb::{types::AttributeValue, Client as DdbClient};
use bitie_types::{
    ddb::{fields, tables, DEFAULT_USER_TABLE_SK_VALUE},
    user::AskedQuestion,
};
use std::str::FromStr;
use tracing::{error, info, warn};

/// Returns a list of questions that the user interacted with.
/// Each question appears only once with correct/incorrect status having the precedence.
pub(crate) async fn get_user_question_history(
    client: &DdbClient,
    topic: &Option<String>,
    user_email: &str,
) -> Option<Vec<AskedQuestion>> {
    info!("Getting user question history for {user_email}");

    // try to get the history from DDB
    let history = match client
        .query()
        .table_name(tables::USERS)
        .key_condition_expression("#email = :email AND #sk = :sk")
        .expression_attribute_names("#email", fields::EMAIL)
        .expression_attribute_values(":email", AttributeValue::S(user_email.to_owned()))
        .expression_attribute_names("#sk", fields::SORT_KEY)
        .expression_attribute_values(":sk", AttributeValue::S(DEFAULT_USER_TABLE_SK_VALUE.to_owned()))
        .send()
        .await
    {
        Ok(v) => {
            match v.items {
                // extract a single item from the response
                Some(items) => {
                    if items.len() > 1 {
                        warn!("Duplicate records for user: {user_email}");
                        return None;
                    }

                    // select a single item
                    if let Some(mut item) = items.into_iter().next() {
                        // get question stats that live outside the details attribute
                        match item.remove(fields::QUESTIONS) {
                            Some(AttributeValue::Ss(v)) => v,
                            _ => {
                                info!("No questions for {user_email}");
                                return None;
                            }
                        }
                    } else {
                        warn!("No record for {user_email}");
                        return None;
                    }
                }
                None => {
                    warn!("No record for {user_email}");
                    return None;
                }
            }
        }
        Err(e) => {
            error!("Query for {user_email} failed: {:?}", e);
            return None;
        }
    };
    info!("Found history records in DDB: {}", history.len());

    // convert the list of questions into AskedQuestion for the specified topic
    let history = history
        .iter()
        .filter_map(|v| match (AskedQuestion::from_str(v), topic) {
            // must match a topic if specified
            (Ok(v), Some(t)) if &v.topic == t => Some(v),
            (Ok(v), _) => Some(v), // no topic specified
            (Err(_), _) => {
                warn!("Cannot deser question: {v}");
                None
            }
        })
        .collect::<Vec<AskedQuestion>>();

    info!("Remaining after topic filter: {}", history.len());

    Some(history)
}
