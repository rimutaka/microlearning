use crate::topic::Topic;
use anyhow::{Error, Result};
use pulldown_cmark::{html::push_html, Parser};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
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
    HtmlFull(Option<Vec<usize>>),
    /// Return the short question in HTML format for the user to answer.
    HtmlShort,
}

/// A question with multiple answers.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
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
#[derive(Deserialize, Serialize, Debug, PartialEq)]
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
    /// A hash of the email of the user who created the question.
    /// The value is set at the server side.
    /// User-submitted data is ignored.
    /// 
    /// The values are hex-encoded to make it easier to generate a matching value in JS.
    /// E.g. 0e3bf888c95b085a7172b2e819692bb5b46c26ad067f9405c8ba1dd950732b65
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub author: Option<String>,
}

impl Question {
    /// Converts markdown members (question, answers) to HTML.
    /// Supports CommonMark only.
    /// See https://crates.io/crates/pulldown-cmark for more information.
    fn into_html(self, learner_answers: Option<Vec<usize>>) -> Self {
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
                    if v.contains(&idx) {
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

    /// Sets the author field.
    /// e.g. 0e3bf888c95b085a7172b2e819692bb5b46c26ad067f9405c8ba1dd950732b65
    pub fn with_author(self, email_hash: &str) -> Self {
        Question {
            author: Some(email_hash.to_string()),
            ..self
        }
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

    /// Returns True if the the list of answers matches the correct answers in the question.
    /// Returns False if the list cannot be converted into numbers or there is a mismatch.
    pub fn is_correct(&self, answers: &[usize]) -> bool {
        // is it the right number of answers?
        if self.correct as usize != answers.len() {
            return false;
        }

        // loop thru the answers and check if they are correct
        for (idx, a) in self.answers.iter().enumerate() {
            if a.c.unwrap_or_default() && !answers.contains(&idx) {
                return false;
            }
        }

        // the number of answers matches the number of correct answers
        // and all correct answers are in the list, so it must be correct
        true
    }
}

/// Converts a JSON string to a Question struct with validation:
/// - qid is a valid UUID4 in Base58 encoding or a new random one is generated
/// - topic is present in the TOPICS list
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
        if !Topic::TOPICS.contains(&topic.as_str()) {
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

impl Display for Question {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let contents = match serde_json::to_string(self) {
            Ok(v) => v,
            Err(e) => {
                error!("Cannot serialize question: {:?}, error: {:?}", self, e);
                return write!(f, "Cannot serialize question");
            }
        };

        write!(f, "{}", contents)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_question_is_correct_1() {
        let q = Question {
            qid: "1".to_string(),
            topic: "math".to_string(),
            question: "What is 1+1?".to_string(),
            answers: vec![
                Answer {
                    a: "1".to_string(),
                    e: None,
                    c: Some(false),
                    sel: None,
                },
                Answer {
                    a: "2".to_string(),
                    e: None,
                    c: Some(true),
                    sel: None,
                },
                Answer {
                    a: "3".to_string(),
                    e: None,
                    c: Some(false),
                    sel: None,
                },
            ],
            correct: 1,
            author: Some("you@me.us".to_string()),
        };

        assert!(q.is_correct(&[1]), "correct");
        assert!(!q.is_correct(&[0]), "incorrect");
        assert!(!q.is_correct(&[1, 2]), "too many");
        assert!(!q.is_correct(&[1, 2, 3, 4, 5]), "way too many");
        assert!(!q.is_correct(&[]), "empty");
    }

    #[test]
    fn test_question_is_correct_2() {
        let q = Question {
            qid: "1".to_string(),
            topic: "math".to_string(),
            question: "What is red?".to_string(),
            answers: vec![
                Answer {
                    a: "tomato".to_string(),
                    e: None,
                    c: Some(true),
                    sel: None,
                },
                Answer {
                    a: "2".to_string(),
                    e: None,
                    c: Some(false),
                    sel: None,
                },
                Answer {
                    a: "cherry".to_string(),
                    e: None,
                    c: Some(true),
                    sel: None,
                },
            ],
            correct: 2,
            author: Some("you@me.us".to_string()),
        };

        assert!(q.is_correct(&[0, 2]), "correct");
        assert!(!q.is_correct(&[1, 2]), "incorrect");
        assert!(!q.is_correct(&[0]), "not enough");
        assert!(!q.is_correct(&[1, 5]), "out of bound");
        assert!(!q.is_correct(&[1, 2, 3]), "too many");
        assert!(!q.is_correct(&[1, 2, 3, 4, 5]), "way too many");
        assert!(!q.is_correct(&[]), "empty");
    }

    #[test]
    fn test_question_to_from_str() {
        let q = Question {
            qid: "89yZBXJBa9t2LB6xfj46Rm".to_string(),
            topic: "aws".to_string(),
            question: "What is 1+1?".to_string(),
            answers: vec![
                Answer {
                    a: "1".to_string(),
                    e: None,
                    c: Some(false),
                    sel: None,
                },
                Answer {
                    a: "2".to_string(),
                    e: None,
                    c: Some(true),
                    sel: None,
                },
                Answer {
                    a: "3".to_string(),
                    e: None,
                    c: Some(false),
                    sel: None,
                },
            ],
            correct: 1,
            author: Some("you@me.us".to_string()),
        };

        let s = q.to_string();
        println!("{}", s);
        let q2 = Question::from_str(&s).unwrap();

        assert_eq!(q, q2);
    }
}
