use anyhow::{Error, Result};
use pulldown_cmark::{html::push_html, Parser};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use tracing::error;

/// The possible formats for the question response.
/// The value is taken from the `QUESTION_FORMAT_HEADER_NAME` header.
/// Use the `FromStr` trait to convert the string to the enum.
pub enum QuestionFormat {
    /// Return the full question in Markdown format for editing.
    MarkdownFull,
    /// Return the full question in HTML format for rendering with explanations.
    /// Learner answers are enclosed in the Vec<u8>.
    /// This is only valid in the context of a learner answering the question.
    HtmlFull(Option<Vec<u8>>),
    /// Return the short question in HTML format for the user to answer.
    HtmlShort,
}

// impl FromStr for QuestionFormat {
//     type Err = ();

//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         match s {
//             "markdown_full" => Ok(QuestionFormat::MarkdownFull),
//             "html_full" => Ok(QuestionFormat::HtmlFull),
//             _ => Ok(QuestionFormat::HtmlShort),
//         }
//     }
// }

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
    #[serde(skip_serializing_if = "Option::is_none", default)]
    c: Option<bool>,
    /// Learner's choice. It is set to true if the learner selected this answer.
    /// Present in JSON only if true.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    sel: Option<bool>,
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
    fn into_html(self, learner_answers: Option<Vec<u8>>) -> Self {
        // the parser can have Options for extended MD support, but they don't seem to be needed

        // convert the question to HTML
        let parser = pulldown_cmark::Parser::new(&self.question);
        let mut question_as_html = String::new();
        push_html(&mut question_as_html, parser);

        // convert answers to HTML
        let answers_as_html = self
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

                Answer {
                    a,
                    e,
                    c: answer.c,
                    sel: None,
                }
            })
            .collect::<Vec<Answer>>();

        // sort the answers so that the answered questions are at the top
        let answers_as_html = match learner_answers {
            Some(v) => {
                // sort them into two buckets, then append unanswered to answered
                // the original order in the buckets is preserved
                let mut answered = Vec::with_capacity(self.correct as usize);
                let mut unanswered = Vec::with_capacity(answers_as_html.len() - self.correct as usize);
                for (idx, answer) in answers_as_html.into_iter().enumerate() {
                    if v.contains(&(idx as u8)) {
                        answered.push(Answer {
                            sel: Some(true),
                            ..answer
                        });
                    } else {
                        unanswered.push(answer);
                    }
                }
                answered.append(&mut unanswered);
                answered
            }
            None => answers_as_html,
        };

        Question {
            question: question_as_html,
            answers: answers_as_html,
            ..self
        }
    }

    /// Removes detailed explanations from the answers
    /// to display the question for answering.
    fn without_detailed_explanations(self) -> Self {
        let answers = self
            .answers
            .into_iter()
            .map(|answer| Answer {
                e: None,
                c: None,
                ..answer
            })
            .collect();

        Question { answers, ..self }
    }

    /// Formats the question to provide the the required format.
    pub fn format(self, format: QuestionFormat) -> Self {
        match format {
            QuestionFormat::MarkdownFull => self,
            QuestionFormat::HtmlFull(v) => self.into_html(v),
            QuestionFormat::HtmlShort => self.without_detailed_explanations().into_html(None),
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
        let correct = q.answers.iter().filter(|a| a.c.unwrap_or_default()).count() as u8;

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
