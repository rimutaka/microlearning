use anyhow::Error;
use aws_sdk_dynamodb::{types::AttributeValue, Client};
use bitie_types::{
    ddb::fields,
    ddb::tables,
    question::{Answer, Question},
};
// use chrono::{DateTime, Utc};
use tracing::{info, warn};

/// Returns a single question for the given topic.
/// Returns a error if no questions found.
pub(crate) async fn get_random(topic: &str) -> Result<Question, Error> {
    let client = Client::new(&aws_config::load_from_env().await);

    // generate a random question ID to choose the one before that
    // e.g. 1D759ksnnlogULbRPng3noG, 2gS2XiBnscLX5dQFDP3kiJo, 3SPUtNR96QCIsdu1je8Duki
    let random_qid = uuid::Uuid::new_v4();
    let random_qid = bs58::encode(random_qid.as_bytes()).into_string();

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

                    let answers = match item.get(fields::ANSWERS) {
                        Some(AttributeValue::S(v)) => match serde_json::from_str::<Vec<Answer>>(v) {
                            Ok(v) => v,
                            Err(_) => {
                                warn!("Invalid question {topic} / {item_qid}: cannot deserialize answers attribute");
                                return Err(Error::msg("Invalid question in DDB".to_string()));
                            }
                        },
                        _ => {
                            warn!("Invalid question {topic} / {item_qid}: missing answers attribute");
                            return Err(Error::msg("Invalid question in DDB".to_string()));
                        }
                    };

                    let question = match item.get(fields::QUESTION) {
                        Some(AttributeValue::S(v)) => v.clone(),
                        _ => {
                            warn!("Invalid question {topic} / {item_qid}: missing question attribute");
                            return Err(Error::msg("Invalid question in DDB".to_string()));
                        }
                    };

                    let correct = match item.get(fields::CORRECT) {
                        Some(AttributeValue::N(v)) => match v.parse::<u8>() {
                            Ok(v) => v,
                            Err(_) => {
                                warn!("Invalid question {topic} / {item_qid}: invalid correct attribute");
                                return Err(Error::msg("Invalid question in DDB".to_string()));
                            }
                        },
                        _ => {
                            warn!("Invalid question {topic} / {item_qid}: missing correct attribute");
                            return Err(Error::msg("Invalid question in DDB".to_string()));
                        }
                    };

                    info!("Returning {topic} / {item_qid}");
                    return Ok(Some(Question {
                        qid: item_qid,
                        topic: topic.to_string(),
                        question,
                        answers,
                        correct,
                    }));
                }
                //
                warn!("No items in query response for {topic} / {query_qid} / {comparison_op}");
                Ok(None)
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
