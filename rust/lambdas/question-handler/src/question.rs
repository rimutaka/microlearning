use anyhow::Error;
use aws_sdk_dynamodb::{types::AttributeValue, Client as DdbClient};
use bitie_types::{ddb::fields, ddb::tables, jwt::JwtUser, lambda::url_list_to_vec, question::Question, topic::Topic};
use chrono::Utc;
use rand::prelude::*;
use std::str::FromStr;
use tracing::{error, info, warn};

/// Returns a single question for the given topic.
/// Returns a error if no questions found.
pub(crate) async fn get_random(
    client: &DdbClient,
    topic: &String,
    recent_questions: &Option<String>,
) -> Result<Option<Question>, Error> {
    // convert a single line ?topic=... value to a list of allowed topics
    let mut allowed_topics =
        Topic::filter_valid_topics(url_list_to_vec(Some(topic)).unwrap_or_else(|| vec![Topic::ANY_TOPIC.to_owned()]));
    allowed_topics.shuffle(&mut rand::thread_rng());

    for topic in allowed_topics {
        for attempt in 0..2 {
            info!("Get random attempt {attempt} for {topic}");
            // generate a random question ID to choose the one before that
            let random_qid = Question::generate_random_qid();
            let comparison_op = if Utc::now().timestamp() % 2 == 0 { "<" } else { ">" };

            // ignore recent questions on the last attempt
            let recent_questions = if attempt == 1 { &None } else { recent_questions };

            // try to get the questions from DDB
            match get_question(client, &topic, &random_qid, comparison_op, 10, recent_questions).await {
                Ok(Some(v)) => {
                    return Ok(Some(v));
                }
                Ok(None) => {
                    // if no questions found, try to find the one in the different sorting direction
                    let comparison_op = if comparison_op == "<" { ">" } else { "<" };
                    match get_question(client, &topic, &random_qid, comparison_op, 10, recent_questions).await {
                        Ok(Some(v)) => {
                            return Ok(Some(v));
                        }
                        Ok(None) => continue,
                        Err(e) => return Err(e),
                    }
                }
                Err(e) => {
                    return Err(e);
                }
            }
        }
    }

    Ok(None)
}

/// Returns a single question for the given topic / qid.
/// Returns a error if this exact question was not found.
pub(crate) async fn get_exact(client: &DdbClient, topic: &String, qid: &str) -> Result<Option<Question>, Error> {
    match get_question(client, topic, qid, "=", 1, &None).await {
        Ok(Some(v)) => Ok(Some(v)),
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
    client: &DdbClient,
    topic: &String,
    query_qid: &str,
    comparison_op: &str,
    limit: i32,
    recent_questions: &Option<String>,
) -> Result<Option<Question>, Error> {
    info!("Query for {topic} / {query_qid} / {comparison_op}");

    // search for IDs in descending order for < because DDB picks the first encountered record
    // e.g. 12345 < 6 returns 1 because it is the first record encountered unless we search in descending order
    let ascending = comparison_op != "<";

    // convert the list of recent questions to a Vec
    let recent_questions = match recent_questions {
        Some(v) => v.split(',').map(|v| v.to_string()).collect(),
        None => vec![],
    };
    info!("Recent questions: {}", recent_questions.len());

    match client
        .query()
        .table_name(tables::QUESTIONS)
        .key_condition_expression(["#topic = :topic AND #qid ", comparison_op, " :qid"].concat())
        .expression_attribute_names("#topic", fields::TOPIC)
        .expression_attribute_values(":topic", AttributeValue::S(topic.to_owned()))
        .expression_attribute_names("#qid", fields::QID)
        .expression_attribute_values(":qid", AttributeValue::S(query_qid.to_owned()))
        .limit(limit)
        .scan_index_forward(ascending)
        .send()
        .await
    {
        Ok(v) => {
            match v.items {
                // extract a single item from the response
                Some(mut items) => {
                    items.shuffle(&mut rand::thread_rng());

                    // select a single item
                    for item in items.into_iter() {
                        let item_qid = match item.get(fields::QID) {
                            Some(AttributeValue::S(v)) => v.clone(),
                            _ => {
                                warn!(
                                    "Invalid question {topic} / {query_qid} / {comparison_op}: missing qid attribute"
                                );
                                return Err(Error::msg("Invalid question in DDB".to_string()));
                            }
                        };

                        if recent_questions.contains(&item_qid) {
                            info!("Skipping recent question {topic} / {item_qid}");
                            continue;
                        }

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
                                    return Ok(Some(v.with_stats(correct, incorrect, skipped)));
                                }
                                Err(_) => {
                                    warn!("Cannot deser details attribute: {topic} / {item_qid}: ");
                                    return Err(Error::msg("Invalid question in DDB".to_string()));
                                }
                            },
                            _ => {
                                warn!("Invalid question {topic} / {item_qid}: missing details attribute");
                                return Err(Error::msg("Invalid question in DDB".to_string()));
                            }
                        }
                    }

                    // the loop above did not find any matches
                    warn!("No items in query response for {topic} / {query_qid} / {comparison_op}");
                    Ok(None)
                }
                None => {
                    warn!("No query response for {topic} / {query_qid} / {comparison_op}");
                    Ok(None)
                }
            }
        }
        Err(e) => {
            info!("Query for {topic} / {query_qid} / {comparison_op} failed: {:?}", e);
            Err(Error::msg("DDB error".to_string()))
        }
    }
}

/// Save a question in the main questions table.
/// Replaces existing records unconditionally.
pub(crate) async fn save(client: &DdbClient, q: &Question) -> Result<(), Error> {
    info!("Saving question {}/{}", q.topic, q.qid);
    info!("{:?}", q);

    // this field is optional, but must be present for the question to be saved
    let author = match &q.author {
        Some(v) => v.clone(),
        None => {
            error!("Missing author field. It's a bug.");
            return Err(Error::msg("Failed to save question".to_string()));
        }
    };

    // this field may be needed for sorting later, remove fractional seconds
    let updated = match q.updated {
        Some(v) => v.to_rfc3339_opts(chrono::SecondsFormat::Secs, true),
        None => {
            error!("Missing updated field. It's a bug.");
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
        .expression_attribute_values(":updated", AttributeValue::S(updated))
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
