use anyhow::Error;
use aws_sdk_dynamodb::{types::AttributeValue, Client as DdbClient};
use bitie_types::{ddb::fields, ddb::tables, jwt::JwtUser, question::Question};
use std::str::FromStr;
use tracing::{error, info, warn};

/// Returns one matching question depending on the comparison operator, e.g. `<`, `>`, `=`.
/// Returns None if no records found.
/// Returns an error if the query fails.
pub(crate) async fn get(client: &DdbClient, topic: &str, qid: &str) -> Result<Option<Question>, Error> {
    info!("Query for {topic} / {qid}");

    match client
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
                        let item_qid = match item.get(fields::QID) {
                            Some(AttributeValue::S(v)) => v.clone(),
                            _ => {
                                warn!("Invalid question {topic} / {qid}: missing qid attribute");
                                return Err(Error::msg("Invalid question in DDB".to_string()));
                            }
                        };

                        let correct = match item.get(fields::QUESTION_STATS_CORRECT) {
                            Some(AttributeValue::N(v)) => Some(v.as_str()),
                            _ => None,
                        };
                        let incorrect = match item.get(fields::QUESTION_STATS_INCORRECT) {
                            Some(AttributeValue::N(v)) => Some(v.as_str()),
                            _ => None,
                        };
                        let skipped = match item.get(fields::QUESTION_STATS_SKIPPED) {
                            Some(AttributeValue::N(v)) => Some(v.as_str()),
                            _ => None,
                        };

                        match item.get(fields::DETAILS) {
                            Some(AttributeValue::S(v)) => match Question::from_str(v) {
                                Ok(v) => {
                                    info!("Returning {topic} / {item_qid}");
                                    Ok(Some(v.with_stats(correct, incorrect, skipped)))
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
                        // the loop above did not find any matches
                        warn!("No items in query response for {topic} / {qid}");
                        Ok(None)
                    }
                }
                None => {
                    warn!("No query response for {topic} / {qid}");
                    Ok(None)
                }
            }
        }
        Err(e) => {
            info!("Query for {topic} / {qid} failed: {:?}", e);
            Err(Error::msg("DDB error".to_string()))
        }
    }
}

/// Save a question in the main questions table.
/// Replaces existing records unconditionally.
pub(crate) async fn save(client: &DdbClient, question: &Question) -> Result<(), Error> {
    info!("Saving question {}/{}", question.topic, question.qid);
    info!("{:?}", question);

    // this field is optional, but must be present for the question to be saved
    let author = match &question.author {
        Some(v) => v.clone(),
        None => {
            error!("Missing author field. It's a bug.");
            return Err(Error::msg("Failed to save question".to_string()));
        }
    };

    // this field may be needed for sorting later, remove fractional seconds
    let updated = match question.updated {
        Some(v) => v.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        None => {
            error!("Missing updated field. It's a bug.");
            return Err(Error::msg("Failed to save question".to_string()));
        }
    };

    // Title is used in indexes and must be present at the record level
    let title = match &question.title {
        Some(v) => v.clone(),
        None => {
            error!("Missing Title field. it's a bug.");
            "Untitled".to_string()
        }
    };

    // this has to be an update to prevent overwriting photo IDs
    const UPDATE_EXPRESSION: &str =
        "SET #author = if_not_exists(#author, :author), #updated = :updated, #details = :details, #title = :title";

    match client
        .update_item()
        .table_name(tables::QUESTIONS)
        .update_expression(UPDATE_EXPRESSION)
        .key(fields::TOPIC, AttributeValue::S(question.topic.clone()))
        .key(fields::QID, AttributeValue::S(question.qid.clone()))
        .expression_attribute_names("#author", fields::AUTHOR)
        .expression_attribute_values(":author", AttributeValue::S(author))
        .expression_attribute_names("#updated", fields::UPDATED)
        .expression_attribute_values(":updated", AttributeValue::S(updated))
        .expression_attribute_names("#details", fields::DETAILS)
        .expression_attribute_values(":details", AttributeValue::S(question.to_string()))
        .expression_attribute_names("#title", fields::TITLE)
        .expression_attribute_values(":title", AttributeValue::S(title))
        .condition_expression("#author = :author OR attribute_not_exists(#author)") // makes the query fail with an error if the author is different
        .send()
        .await
    {
        Ok(_) => {
            info!("Question saved in DDB");
            Ok(())
        }
        Err(e) => {
            error!("Failed to save question {}/{}: {:?}", question.topic, question.qid, e);
            Err(Error::msg("Failed to save question".to_string()))
        }
    }
}

/// Increments the stats counters for the given question.
/// Ignores the request if the user is the author or answers == None
pub(crate) async fn update_answer_stats(
    client: &DdbClient,
    jwt_user: &Option<JwtUser>,
    question: &Question,
    answers: &Option<Vec<usize>>,
) {
    // do not update answers if the user is the author
    if let Some(jwt_user) = jwt_user {
        if Some(&jwt_user.email_hash) == question.author.as_ref() {
            info!("User is the author - NOT updating user answers");
            return;
        }
    }

    let update_expression = match answers {
        None => return,
        Some(v) => {
            if v.is_empty() {
                fields::QUESTION_STATS_SKIPPED
            } else if question.is_correct(v) {
                fields::QUESTION_STATS_CORRECT
            } else {
                fields::QUESTION_STATS_INCORRECT
            }
        }
    };

    match client
        .update_item()
        .table_name(tables::QUESTIONS)
        .update_expression(["ADD ", update_expression, " :v"].concat())
        .key(fields::TOPIC, AttributeValue::S(question.topic.to_string()))
        .key(fields::QID, AttributeValue::S(question.qid.to_string()))
        .expression_attribute_values(":v", AttributeValue::N("1".to_string()))
        .send()
        .await
    {
        Ok(_) => {
            info!("Question stats updated");
        }
        Err(e) => {
            error!("Failed to update question stats: {:?}", e);
        }
    }
}
