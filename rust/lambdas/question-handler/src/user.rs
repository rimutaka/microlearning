use aws_sdk_dynamodb::{types::AttributeValue, Client};
use bitie_types::{
    ddb::{fields, tables, DEFAULT_USER_TABLE_SK_VALUE},
    question::Question,
    user::{AnswerStatus, AskedQuestion},
};
use chrono::Utc;
use tracing::{error, info};

/// Adds an answer to the user's record in a serialized format, e.g. `aws/9GjFyqQMTmpDJBYgtxoaBA/2024-10-31T20:08:47Zi`
/// List present - check if correct
/// Blank list - skipped
/// No list - asked
pub(crate) async fn update_answers(email: &str, question: &Question, answers: &Option<Vec<usize>>) {
    info!("Updating user answers: {}", email);

    let client = Client::new(&aws_config::load_from_env().await);

    // prepare the answer struct and convert it to a string with /// separators
    let status = match answers {
        None => AnswerStatus::Asked(Utc::now()),
        Some(v) => {
            if v.is_empty() {
                AnswerStatus::Skipped(Utc::now())
            } else if question.is_correct(v) {
                AnswerStatus::Correct(Utc::now())
            } else {
                AnswerStatus::Incorrect(Utc::now())
            }
        }
    };

    let asked_question = AskedQuestion {
        topic: question.topic.clone(),
        qid: question.qid.clone(),
        status,
    }
    .to_string();
    // it has to be a vec to work with DDB SET type
    // sorry about the extra clone
    let asked_question = [asked_question].to_vec();

    match client
        .update_item()
        .table_name(tables::USERS)
        .update_expression("ADD #questions :questions")
        .key(fields::EMAIL, AttributeValue::S(email.to_string()))
        .key(
            fields::SORT_KEY,
            AttributeValue::S(DEFAULT_USER_TABLE_SK_VALUE.to_string()),
        )
        .expression_attribute_names("#questions", fields::QUESTIONS)
        .expression_attribute_values(":questions", AttributeValue::Ss(asked_question))
        .send()
        .await
    {
        Ok(_) => {
            info!("User answers updated");
        }
        Err(e) => {
            error!("Failed to update user answers {}: {:?}", email, e);
        }
    }
}
