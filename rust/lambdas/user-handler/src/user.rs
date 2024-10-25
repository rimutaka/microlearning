use anyhow::Error;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use bitie_types::{
    ddb::fields,
    ddb::tables,
    // user::{AnswerState, AskedQuestion, User},
};
use chrono::Utc;
use tracing::{error, info};

/// Save a user in the main user table.
/// Replaces existing records unconditionally.
pub(crate) async fn update_subscription(email: String, topics: Vec<String>) -> Result<(), Error> {
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
        .send()
        .await
    {
        Ok(_) => {
            info!("User subs saved in DDB");
            Ok(())
        }
        Err(e) => {
            error!("Failed to save user subs {}: {:?}", email, e);
            Err(Error::msg("Failed to save question".to_string()))
        }
    }
}
