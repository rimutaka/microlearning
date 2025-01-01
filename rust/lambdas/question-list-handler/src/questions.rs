use anyhow::Error;
use aws_sdk_dynamodb::{types::AttributeValue, Client as DdbClient};
use bitie_types::{ddb::fields, ddb::tables, question::Question, topic::Topic};
use chrono::{DateTime, Utc};
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
        .index_name(tables::QUESTIONS_IDX_TOPIC_QID)
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

                        // get question title from an included attribute
                        let title = match item.get(fields::TITLE) {
                            Some(AttributeValue::S(v)) => Some(v.clone()),
                            _ => {
                                warn!("invalid `title` attribute for {topic} / {item_qid}");
                                Some(Question::DEFAULT_TITLE.to_string())
                            }
                        };

                        // get updated field from an included attribute
                        let updated = match item.get(fields::UPDATED) {
                            Some(AttributeValue::S(v)) => match DateTime::parse_from_rfc3339(v) {
                                Ok(v) => Some(v.with_timezone(&Utc)),
                                Err(e) => {
                                    warn!("invalid `updated` attribute for {topic} / {item_qid}: {:?}", e);
                                    Some(DateTime::<Utc>::MIN_UTC)
                                }
                            },
                            _ => {
                                warn!("invalid `updated` attribute for {topic} / {item_qid}");
                                Some(DateTime::<Utc>::MIN_UTC)
                            }
                        };

                        let question = Question {
                            topic: topic.to_string(),
                            qid: item_qid,
                            title,
                            updated,
                            answers: Vec::new(),
                            question: "".to_string(),
                            correct: 0,
                            author: None,
                            contributor: None,
                            stats: None,
                        };

                        fetched_questions.push(question);
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
