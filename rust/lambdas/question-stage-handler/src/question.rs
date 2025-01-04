use anyhow::Error;
use aws_sdk_dynamodb::{types::AttributeValue, Client as DdbClient};
use bitie_types::{
    ddb::fields,
    ddb::tables,
    question::{PublishStage, Question},
};
use std::str::FromStr;
use tracing::{error, info, warn};

/// Changes the publish stage inside question details and in DDB attributes.
/// Returns an error if the query fails.
pub(crate) async fn change_publish_stage(
    client: &DdbClient,
    topic: &str,
    qid: &str,
    stage: PublishStage,
) -> Result<(), Error> {
    info!("Changing publish stage for {topic} / {qid} to {stage}");

    let question = match client
        .query()
        .table_name(tables::QUESTIONS)
        .key_condition_expression(["#topic = :topic AND #qid = :qid"].concat())
        .expression_attribute_names("#topic", fields::TOPIC)
        .expression_attribute_values(":topic", AttributeValue::S(topic.to_owned()))
        .expression_attribute_names("#qid", fields::QID)
        .expression_attribute_values(":qid", AttributeValue::S(qid.to_owned()))
        .send()
        .await
    {
        Ok(v) => {
            match v.items {
                // extract a single item from the response
                Some(items) => {
                    // select a single item
                    if let Some(item) = items.into_iter().next() {
                        match item.get(fields::DETAILS) {
                            Some(AttributeValue::S(v)) => match Question::from_str(v) {
                                Ok(v) => v.with_stage(stage),
                                Err(e) => {
                                    warn!("Cannot deser details attribute: {:?}", e);
                                    return Err(Error::msg("Invalid question in DDB".to_string()));
                                }
                            },
                            _ => {
                                warn!("Missing details attribute");
                                return Err(Error::msg("Invalid question in DDB".to_string()));
                            }
                        }
                    } else {
                        // the loop above did not find any matches
                        warn!("No items in query response for {topic} / {qid}");
                        return Err(Error::msg("No question found".to_string()));
                    }
                }
                None => {
                    warn!("No query response for {topic} / {qid}");
                    return Err(Error::msg("No response from DDB".to_string()));
                }
            }
        }
        Err(e) => {
            info!("Query for {topic} / {qid} failed: {:?}", e);
            return Err(Error::msg("DDB error".to_string()));
        }
    };

    info!("Saving question {}/{}", question.topic, question.qid);
    // info!("{:?}", question);

    // this has to be an update to prevent overwriting photo IDs
    const UPDATE_EXPRESSION: &str = "SET #details = :details, #stage = :stage, #updated = :updated";

    match client
        .update_item()
        .table_name(tables::QUESTIONS)
        .update_expression(UPDATE_EXPRESSION)
        .key(fields::TOPIC, AttributeValue::S(question.topic.clone()))
        .key(fields::QID, AttributeValue::S(question.qid.clone()))
        .expression_attribute_names("#details", fields::DETAILS)
        .expression_attribute_values(":details", AttributeValue::S(question.to_string()))
        .expression_attribute_names("#stage", fields::STAGE)
        .expression_attribute_values(":stage", AttributeValue::S(question.stage.to_string()))
        .expression_attribute_names("#updated", fields::UPDATED)
        .expression_attribute_values(
            ":updated",
            AttributeValue::S(chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Secs, true)),
        )
        .send()
        .await
    {
        Ok(_) => {
            info!("Question updated in DDB");
            Ok(())
        }
        Err(e) => {
            error!("Failed to save question {}/{}: {:?}", question.topic, question.qid, e);
            Err(Error::msg("Failed to save question".to_string()))
        }
    }
}
