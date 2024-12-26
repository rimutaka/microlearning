use anyhow::Error;
use aws_sdk_dynamodb::{types::AttributeValue, Client as DdbClient};
use bitie_types::{ddb::fields, ddb::tables, question::Question, topic::Topic};
use std::str::FromStr;
use tracing::{error, info, warn};

/// Returns True if the param a single valid topic,
/// otherwise returns false.
pub(crate) fn validate_topic(topic: &str) -> bool {
    !topic.is_empty() && Topic::filter_valid_topics(vec![topic.to_string()]).len() == 1
}

/// Returns a single question for the given topic.
/// Returns a error if no questions found.
pub(crate) async fn get_all_questions_by_topic(client: &DdbClient, topic: &String) -> Result<Vec<Question>, Error> {
    info!("Getting all questions for {topic}");
    // list of questions fetched from DDB
    let mut fetched_questions = Vec::new();

    // try to get the questions from DDB
    match client
        .query()
        .table_name(tables::QUESTIONS)
        .key_condition_expression("#topic = :topic")
        .expression_attribute_names("#topic", fields::TOPIC)
        .expression_attribute_values(":topic", AttributeValue::S(topic.to_owned()))
        .send()
        .await
    {
        Ok(v) => {
            match v.items {
                // extract a single item from the response
                Some(items) => {
                    // select a single item
                    for item in items.into_iter() {
                        let item_qid = match item.get(fields::QID) {
                            Some(AttributeValue::S(v)) => v.clone(),
                            _ => {
                                warn!("Invalid question for {topic}: missing qid attribute");
                                continue;
                            }
                        };

                        // get question stats that live outside the details attribute
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
                                    // info!("Returning {topic} / {item_qid}");
                                    fetched_questions
                                        .push(v.with_stats(correct, incorrect, skipped).strip_for_list_display());
                                }
                                Err(_) => {
                                    warn!("Cannot deser details attribute: {topic} / {item_qid}: ");
                                    continue;
                                }
                            },
                            _ => {
                                warn!("Invalid question {topic} / {item_qid}: missing details attribute");
                                continue;
                            }
                        }
                    }
                }
                None => {
                    warn!("No query response for {topic}");
                }
            }
        }
        Err(e) => {
            error!("Query for {topic} failed: {:?}", e);
            return Err(Error::msg("DDB error".to_string()));
        }
    }

    info!("Fetched questions: {}", fetched_questions.len());

    // sort the questions by updated date
    fetched_questions.sort_by(|a, b| b.updated.cmp(&a.updated));

    Ok(fetched_questions)
}
