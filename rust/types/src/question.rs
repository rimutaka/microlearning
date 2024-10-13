use anyhow::{Error, Result};
use pulldown_cmark::{html::push_html, Parser};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::error;

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
    #[serde(default = "Question::generate_random_qid")]
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
    /// Calculated from the `answers` attribute inside `from_str()`.
    #[serde(default)]
    pub correct: u8,
}

impl Question {
    /// The list supported topics. It must be synchronized with the same list in the front-end manually.
    pub const TOPICS: [&'static str; 5] = ["aws", "css", "general", "js-ts", "rust"];

    /// Converts markdown members (question, answers) to HTML.
    /// Supports CommonMark only.
    /// See https://crates.io/crates/pulldown-cmark for more information.
    pub fn to_html(self) -> Self {
        // the parser can have Options for extended MD support, but they don't seem to be needed

        // convert the question to HTML
        let parser = pulldown_cmark::Parser::new(&self.question);
        let mut question = String::new();
        push_html(&mut question, parser);

        // convert answers to HTML
        let answers = self
            .answers
            .into_iter()
            .map(|answer| {
                let parser = Parser::new(&answer.a);
                let mut a = String::new();
                push_html(&mut a, parser);

                let e = answer.e.map(|e| {
                    let parser = Parser::new(&e);
                    let mut e = String::new();
                    push_html(&mut e, parser);
                    e
                });

                Answer { a, e, c: answer.c }
            })
            .collect();

        Question {
            question,
            answers,
            ..self
        }
    }

    /// Generates a random question ID as UUID4 in Base58 encoding.,
    /// e.g. 1D759ksnnlogULbRPng3noG, 2gS2XiBnscLX5dQFDP3kiJo, 3SPUtNR96QCIsdu1je8Duki
    pub fn generate_random_qid() -> String {
        bs58::encode(uuid::Uuid::new_v4().as_bytes()).into_string()
    }

    /// Serializes `answers` attribute to a JSON string.
    pub fn serialize_answers(&self) -> Result<String> {
        match serde_json::to_string(&self.answers) {
            Ok(v) => Ok(v),
            Err(e) => {
                error!(
                    "Invalid question: cannot serialize answers for {} / {}: {:?}",
                    self.topic, self.qid, e
                );
                Err(Error::msg("Invalid question: cannot serialize answers attribute"))
            }
        }
    }
}

impl FromStr for Question {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let q = match serde_json::from_str::<Self>(s) {
            Ok(v) => v,
            Err(e) => {
                error!("Cannot deserialize question: {:?} from {s}", e);
                return Err(Error::msg("Cannot deserialize question"));
            }
        };

        // Checks how many answers have `correct` flag set to true
        // and updates `correct` attribute.
        let correct = q.answers.iter().filter(|a| a.c).count() as u8;

        // qid is missing for new questions
        // it should be a valid UUID4 if present, but check it just in case
        let qid = match bs58::decode(&q.qid).into_vec() {
            Ok(v) if v.len() == 16 => q.qid.clone(),
            _ => Question::generate_random_qid(),
        };

        // only supported topics are allowed
        let topic = q.topic.trim().to_lowercase();
        if !Question::TOPICS.contains(&topic.as_str()) {
            error!("Invalid topic {topic}");
            return Err(Error::msg("Invalid topic"));
        }

        // this structure should be safe enough for further processing
        Ok(Question {
            qid,
            topic,
            correct,
            ..q
        })
    }
}
