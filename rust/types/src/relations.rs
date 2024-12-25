use crate::question::Question;
use crate::user::AskedQuestion;
use serde::{Deserialize, Serialize};

/// Combines a question with its history.
/// The history may be for the user or the question itself.
/// E.g. how a particular user interacted with the question, or
/// how the question was interacted by multiple users.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct QuestionWithHistory {
    pub question: Question,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub history: Option<Vec<AskedQuestion>>,
}
