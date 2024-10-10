use serde::{Deserialize, Serialize};

/// A question with multiple answers.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Answer { 
  /// The short answer that in Markdown format that appears as an option
  /// when the question is asked.
  a: String,
  /// A detailed explanation why this answer is correct or incorrect
  /// in Markdown format.
  e: Option<String>,
  /// A flag to indicate if this answer is correct.
  /// Only present if true.
  #[serde(skip_serializing_if = "std::ops::Not::not", default)] 
  c: bool,
}

/// A question with multiple answers.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Question {
    /// Base-58 encoded UUID4
    pub qid: String,
    /// The single topic the question belongs to.
    /// Can only contain lower-case characters, digits, and underscores.
    pub topic: String,
    /// The question in Markdown format.
    pub question: String,
    /// The list of answers in Markdown format. Max length is 10.
    /// The order of the answers must be preserved to match the correct answers list.
    pub answers: Vec<Answer>,
    /// How many answers are correct.
    /// It is needed to tell the front-end how many answers to expect
    /// without revealing the correct answers.
    pub correct: u8,
}
