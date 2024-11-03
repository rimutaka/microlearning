use anyhow::Error;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use bitie_types::{ddb::fields, ddb::tables, question::Question};
use std::str::FromStr;
// use chrono::{DateTime, Utc};
use tracing::{error, info, warn};

/// Returns a single question for the given topic.
/// Returns a error if no questions found.
pub(crate) async fn get_random(topic: &str) -> Result<Question, Error> {
    let client = Client::new(&aws_config::load_from_env().await);

    // generate a random question ID to choose the one before that
    let random_qid = Question::generate_random_qid();

    match get_question(&client, topic, &random_qid, "<").await {
        Ok(Some(v)) => Ok(v),
        Ok(None) => {
            // if no question found, try to find the one after the random ID
            match get_question(&client, topic, &random_qid, ">").await {
                Ok(Some(v)) => Ok(v),
                Ok(None) => {
                    warn!("No questions found for {topic}");
                    Err(Error::msg("No questions found".to_string()))
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => Err(e),
    }
}

/// Returns a single question for the given topic / qid.
/// Returns a error if this exact question was not found.
pub(crate) async fn get_exact(topic: &str, qid: &str) -> Result<Question, Error> {
    let client = Client::new(&aws_config::load_from_env().await);

    match get_question(&client, topic, qid, "=").await {
        Ok(Some(v)) => Ok(v),
        Ok(None) => {
            warn!("No question found for {topic} / {qid}");
            Err(Error::msg("Question not found".to_string()))
        }
        Err(e) => Err(e),
    }
}

/// Returns one matching question depending on the comparison operator, e.g. `<`, `>`, `=`.
/// Returns None if no records found.
/// Returns an error if the query fails.
async fn get_question(
    client: &Client,
    topic: &str,
    query_qid: &str,
    comparison_op: &str,
) -> Result<Option<Question>, Error> {
    info!("Query for {topic} / {query_qid} / {comparison_op}");

    // search for IDs in descending order for < because DDB picks the first encountered record
    // e.g. 12345 < 6 returns 1 because it is the first record encountered unless we search in descending order
    let ascending = comparison_op != "<";

    match client
        .query()
        .table_name(tables::QUESTIONS)
        .key_condition_expression(["#topic = :topic AND #qid ", comparison_op, " :qid"].concat())
        .expression_attribute_names("#topic", fields::TOPIC)
        .expression_attribute_values(":topic", AttributeValue::S(topic.to_owned()))
        .expression_attribute_names("#qid", fields::QID)
        .expression_attribute_values(":qid", AttributeValue::S(query_qid.to_owned()))
        .limit(1)
        .scan_index_forward(ascending)
        .send()
        .await
    {
        Ok(v) => match v.items {
            // extract a single item from the response - there should be only one
            Some(items) => {
                // check how many items there are
                if items.len() > 1 {
                    // should not happen, but carry on anyway
                    warn!("Found multiple records for {topic} / {query_qid} / {comparison_op}. Returning one only.");
                }

                // process a single item
                if let Some(item) = items.into_iter().next() {
                    let item_qid = match item.get(fields::QID) {
                        Some(AttributeValue::S(v)) => v.clone(),
                        _ => {
                            warn!("Invalid question {topic} / {query_qid} / {comparison_op}: missing qid attribute");
                            return Err(Error::msg("Invalid question in DDB".to_string()));
                        }
                    };

                    match item.get(fields::DETAILS) {
                        Some(AttributeValue::S(v)) => match Question::from_str(v) {
                            Ok(v) => {
                                info!("Returning {topic} / {item_qid}");
                                Ok(Some(v))
                            }
                            Err(_) => {
                                warn!("Cannot deser details attribute: {topic} / {item_qid}: ");
                                Err(Error::msg("Invalid question in DDB".to_string()))
                            }
                        },
                        _ => {
                            warn!("Invalid question {topic} / {item_qid}: missing details attribute");
                            Err(Error::msg("Invalid question in DDB".to_string()))
                        }
                    }
                } else {
                    warn!("No items in query response for {topic} / {query_qid} / {comparison_op}");
                    Ok(None)
                }
            }
            None => {
                warn!("No query response for {topic} / {query_qid} / {comparison_op}");
                Ok(None)
            }
        },
        Err(e) => {
            info!("Query for {topic} / {query_qid} / {comparison_op} failed: {:?}", e);
            Err(Error::msg("DDB error".to_string()))
        }
    }
}

/// Save a question in the main questions table.
/// Replaces existing records unconditionally.
pub(crate) async fn save(q: &Question) -> Result<(), Error> {
    info!("Saving question {}/{}", q.topic, q.qid);

    info!("{:?}", q);

    let client = Client::new(&aws_config::load_from_env().await);

    // this field is optional, but must be present for the question to be saved
    let author = match &q.author {
        Some(v) => v.clone(),
        None => {
            error!("Missing author field. It's a bug.");
            return Err(Error::msg("Failed to save question".to_string()));
        }
    };

    // this has to be an update to prevent overwriting photo IDs
    const UPDATE_EXPRESSION: &str =
        "SET #author = if_not_exists(#author, :author), #updated = :updated, #details = :details";

    match client
        .update_item()
        .table_name(tables::QUESTIONS)
        .update_expression(UPDATE_EXPRESSION)
        .key(fields::TOPIC, AttributeValue::S(q.topic.clone()))
        .key(fields::QID, AttributeValue::S(q.qid.clone()))
        .expression_attribute_names("#author", fields::AUTHOR)
        .expression_attribute_values(":author", AttributeValue::S(author))
        .expression_attribute_names("#updated", fields::UPDATED)
        .expression_attribute_values(":updated", AttributeValue::S(chrono::Utc::now().to_rfc3339()))
        .expression_attribute_names("#details", fields::DETAILS)
        .expression_attribute_values(":details", AttributeValue::S(q.to_string()))
        .condition_expression("#author = :author OR attribute_not_exists(#author)") // makes the query fail with an error if the author is different
        .send()
        .await
    {
        Ok(_) => {
            info!("Question saved in DDB");
            Ok(())
        }
        Err(e) => {
            error!("Failed to save question {}/{}: {:?}", q.topic, q.qid, e);
            Err(Error::msg("Failed to save question".to_string()))
        }
    }
}
