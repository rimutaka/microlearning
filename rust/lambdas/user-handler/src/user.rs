use anyhow::Error;
use aws_sdk_dynamodb::{
    types::{AttributeValue, ReturnValue},
    Client,
};
use bitie_types::{ddb::fields, ddb::tables, user::User};
use chrono::{DateTime, Utc};
use std::collections::HashMap;
use tracing::{error, info, warn};

/// Save a user in the main user table.
/// Replaces existing records unconditionally.
pub(crate) async fn update_subscription(email: String, topics: Vec<String>) -> Result<Option<User>, Error> {
    info!("Updating user sub: {}", email);

    let client = Client::new(&aws_config::load_from_env().await);

    let unsubscribe = bs58::encode(uuid::Uuid::new_v4().as_bytes())
        .into_string()
        .to_lowercase();

    // this has to be an update to prevent overwriting photo IDs
    const UPDATE_EXPRESSION: &str = "SET #topics = :topics, #unsubscribe = :unsubscribe, #updated = :updated";

    match client
        .update_item()
        .table_name(tables::USERS)
        .update_expression(UPDATE_EXPRESSION)
        .key(fields::EMAIL, AttributeValue::S(email.clone()))
        .key(fields::SORT_KEY, AttributeValue::S("sub".to_string()))
        .expression_attribute_names("#topics", fields::TOPICS)
        .expression_attribute_values([":", fields::TOPICS].concat(), AttributeValue::Ss(topics))
        .expression_attribute_names("#unsubscribe", fields::UNSUBSCRIBE)
        .expression_attribute_values([":", fields::UNSUBSCRIBE].concat(), AttributeValue::S(unsubscribe))
        .expression_attribute_names("#updated", fields::UPDATED)
        .expression_attribute_values(
            [":", fields::UPDATED].concat(),
            AttributeValue::S(Utc::now().to_rfc3339()),
        )
        .return_values(ReturnValue::AllNew)
        .send()
        .await
    {
        Ok(v) => query_output_to_user(v.attributes, email),
        Err(e) => {
            error!("Failed to save user subs {}: {:?}", email, e);
            Err(Error::msg("Failed to save question".to_string()))
        }
    }
}

/// Get a user from the main user table.
pub(crate) async fn get_user(email: String) -> Result<Option<User>, Error> {
    info!("Getting user: {}", email);

    let client = Client::new(&aws_config::load_from_env().await);

    match client
        .query()
        .table_name(tables::USERS)
        .key_condition_expression("#email = :email")
        .expression_attribute_names("#email", fields::EMAIL)
        .expression_attribute_values(":email", AttributeValue::S(email.clone()))
        .send()
        .await
    {
        Ok(v) => match v.items {
            // extract a single item from the response - there should be only one
            Some(items) => {
                // check how many items there are
                if items.len() > 1 {
                    // should not happen, but carry on anyway
                    warn!("Found multiple records for {email}. Returning one only.");
                }
                query_output_to_user(items.into_iter().next(), email)
            }
            None => {
                warn!("No query response for {email}");
                Ok(None)
            }
        },
        Err(e) => {
            info!("Query for {email} failed: {:?}", e);
            Err(Error::msg("DDB error".to_string()))
        }
    }
}

/// A reusable part of converting DDB output into User.
fn query_output_to_user(
    query_output: Option<HashMap<String, AttributeValue>>,
    email: String,
) -> Result<Option<User>, Error> {
    /// A generic error to return back to the caller.
    const INVALID_USER: &str = "Invalid user in DDB";

    // process a single item
    if let Some(item) = query_output {
        let unsubscribe = match item.get(fields::UNSUBSCRIBE) {
            Some(AttributeValue::S(v)) => v.clone(),
            _ => {
                warn!("Invalid user {email}: missing unsubscribe attribute");
                return Err(Error::msg(INVALID_USER.to_string()));
            }
        };

        let topics = match item.get(fields::TOPICS) {
            Some(AttributeValue::Ss(v)) => v.clone(),

            _ => Vec::new(),
        };

        let updated = match item.get(fields::UPDATED) {
            Some(AttributeValue::S(v)) => match DateTime::parse_from_rfc3339(v) {
                Ok(v) => v.with_timezone(&Utc),
                Err(e) => {
                    warn!("Invalid updated field: {v},{:?}", e);
                    return Err(Error::msg(INVALID_USER.to_string()));
                }
            },
            _ => {
                warn!("Invalid user {email}: missing unsubscribe attribute");
                return Err(Error::msg(INVALID_USER.to_string()));
            }
        };

        info!("Returning user dets");
        Ok(Some(User {
            email,
            topics,
            questions: Vec::new(),
            unsubscribe,
            updated,
        }))
    } else {
        // should not happen, but carry on anyway
        warn!("No items in query response for {email}");
        Ok(None)
    }
}
