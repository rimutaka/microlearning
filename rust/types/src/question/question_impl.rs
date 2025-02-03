use super::{Answer, ContributorProfile, PublishStage, QuestionFormat, Stats};
use crate::markdown::{self, md_to_html, ValidatedMarkdown};
use crate::topic::Topic;
use anyhow::{Error, Result};
use chrono::{DateTime, Timelike, Utc};
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;
use tracing::error;

/// A question with multiple answers.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
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
    /// The date when the question was last modified.
    /// This value only changes when the contents are updated.
    pub updated: Option<DateTime<Utc>>,
    /// A one line summary of the question to display in the list of questions.
    /// This was an afterthought and is not present in the existing questions.
    /// Ideally, it needs to be generated from the question and answers.
    pub title: String,
    /// Controls visibility of the question: draft, review, published.
    /// Maintained in the struct and as a DDB attribute for indexing.
    /// The DDB attribute is the source of truth and is copied to the struct on DDB read.
    #[serde(default)]
    pub stage: PublishStage,
    /// Counters for correct, incorrect and skipped user interactions with the question
    /// to provide the data to the front-end.
    /// The values are set during DDB reads.
    /// It is removed during writes and is never deserialized.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub stats: Option<Stats>,
    /// Details of the person or business who contributed the question
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub contributor: Option<ContributorProfile>,
    /// A sorted list of links extracted from the Markdown of the question,
    /// answers and explanations.
    /// This data is not persisted in the DB.
    /// The links are built on the fly by the server.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub refresher_links: Option<Vec<String>>,
}

impl Question {
    /// The maximum size of a serialized question in bytes
    /// .from_str() returns an error if the size is exceeded.
    pub const MAX_QUESTION_LEN: usize = 12_000;

    /// The maximum size of a deserialized title in bytes
    /// The excess should be truncated.
    pub const MAX_TITLE_LEN: usize = 120;

    /// The value to use when no title is present and it cannot be generated from the question.
    pub const DEFAULT_TITLE: &str = "Untitled";

    /// Converts markdown members (question, answers) to HTML.
    /// Extracts `refresher_links` links from the markdown.
    /// Supports CommonMark only.
    /// See https://crates.io/crates/pulldown-cmark for more information.
    fn into_html(self, learner_answers: Option<Vec<usize>>) -> Self {
        // containers for collecting links extracted from the markdown
        let mut q_links = Vec::with_capacity(10); // question links
        let mut c_links = Vec::with_capacity(10); // correct answers links
        let mut i_links = Vec::with_capacity(10); // incorrect answers links

        // convert the question to HTML and extract links in one step
        let question_as_html = {
            let ValidatedMarkdown { html, links, .. } = md_to_html(&self.question, true);
            q_links.extend(links);
            html
        };

        // convert answers to HTML
        let answers_as_html = self
            .answers
            .into_iter()
            .map(|answer| {
                let ValidatedMarkdown { html, links, .. } = md_to_html(&answer.a, true);
                let a = {
                    if answer.c.unwrap_or_default() {
                        c_links.extend(links);
                    } else {
                        i_links.extend(links);
                    }
                    html
                };

                let e = answer.e.map(|e| {
                    let ValidatedMarkdown { html, links, .. } = md_to_html(&e, true);
                    if answer.c.unwrap_or_default() {
                        c_links.extend(links);
                    } else {
                        i_links.extend(links);
                    }
                    html
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

        let refresher_links = markdown::sort_links(q_links, c_links, i_links);
        let refresher_links = if refresher_links.is_empty() {
            None
        } else {
            Some(refresher_links)
        };

        Question {
            question: question_as_html,
            answers: answers_as_html,
            refresher_links,
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
            // TODO: add a flag to extract links, but do not convert certain parts of the question into HTML
            QuestionFormat::HtmlShort => self.into_html(None).without_detailed_explanations(),
        }
    }

    /// Generates a random question ID as UUID4 in Base58 encoding.,
    /// e.g. 1D759ksnnlogULbRPng3noG, 2gS2XiBnscLX5dQFDP3kiJo, 3SPUtNR96QCIsdu1je8Duki
    fn generate_random_qid() -> String {
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

    /// Sets `updated`` field to the current UTC with no fractional seconds.
    pub fn with_updated(self) -> Self {
        Question {
            updated: Some(
                Utc::now()
                    .with_nanosecond(0)
                    .expect("Invalid nanoseconds. This is an impossible bug"),
            ),
            ..self
        }
    }

    /// Sets the stats counters for correct, incorrect, and skipped answers.
    /// Uses zeros for missing or incorrect values.
    pub fn with_stats(self, correct: Option<&str>, incorrect: Option<&str>, skipped: Option<&str>) -> Self {
        let correct = correct.unwrap_or_default().parse::<u32>().unwrap_or_default();
        let incorrect = incorrect.unwrap_or_default().parse::<u32>().unwrap_or_default();
        let skipped = skipped.unwrap_or_default().parse::<u32>().unwrap_or_default();

        Question {
            stats: Some(Stats {
                correct,
                incorrect,
                skipped,
            }),
            ..self
        }
    }

    /// Returns Self with updated `stage` field.
    pub fn with_stage(self, stage: PublishStage) -> Self {
        Question { stage, ..self }
    }

    /// Removes everything except stats, IDs and the title
    pub fn strip_for_list_display(self) -> Self {
        Question {
            qid: self.qid,
            topic: self.topic,
            title: self.title,
            stats: self.stats,
            updated: self.updated,
            stage: self.stage,
            answers: vec![],
            question: "".to_string(),
            correct: 0,
            author: None,
            contributor: None,
            refresher_links: None,
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

    /// Returns True if the question has all the required parts.
    pub fn is_complete(&self) -> bool {
        !self.topic.is_empty()
            && self.question.len() > 10
            && self.answers.len() >= 2
            && self
                .answers
                .iter()
                .all(|a| !a.a.is_empty() && a.e.as_ref().unwrap_or(&"".to_string()).len() > 10)
            && self.correct > 0
            && self.title.len() > 10
    }
}

/// Converts a JSON string to a Question struct with validation:
/// - qid is a valid UUID4 in Base58 encoding or a new random one is generated
/// - topic is present in the TOPICS list
/// - correct is recalculated from the answers
/// - answering stats are set to None
impl FromStr for Question {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // limit the size to something reasonable
        if s.len() > Self::MAX_QUESTION_LEN {
            error!("Question is too large: {}", s.len());
            return Err(Error::msg(format!(
                "Question is too large. {} bytes allowed",
                Self::MAX_QUESTION_LEN
            )));
        }

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

        // a helper function needed at the next step
        let title_from_question = || {
            if q.question.len() > 10 {
                let v = &q.question.trim().replace(['\n', '\r'], " ").replace("  ", " ");
                v[..v.len().min(Self::MAX_TITLE_LEN)].to_string()
            } else {
                Self::DEFAULT_TITLE.to_string()
            }
        };

        // the title should be trimmed and truncated, if present
        // if not, we should make something up
        // it is needed in the front-end to display the list of questions
        let title = {
            let v = q.title.trim();
            if v.is_empty() {
                title_from_question()
            } else {
                v[..v.len().min(Self::MAX_TITLE_LEN)].to_string()
            }
        };

        // this structure should be safe enough for further processing
        Ok(Question {
            qid,
            topic,
            title,
            correct,
            stats: None,
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
            updated: Some(Utc::now()),
            stats: None,
            contributor: None,
            title: "".to_string(),
            stage: PublishStage::Draft,
            refresher_links: None,
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
            updated: Some(Utc::now()),
            stats: None,
            contributor: None,
            title: "".to_string(),
            stage: PublishStage::Draft,
            refresher_links: None,
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
            updated: Some(Utc::now()),
            stats: None,
            contributor: Some(ContributorProfile {
                name: Some("John Doe".to_string()),
                url: Some("https://example.com".to_string()),
                img_url: Some("https://example.com/img.jpg".to_string()),
                about: Some("A great developer".to_string()),
            }),
            title: "Simple Rust question".to_string(),
            stage: PublishStage::Draft,
            refresher_links: None,
        };

        let s = q.to_string();
        println!("{}", s);
        let q2 = Question::from_str(&s).unwrap();

        assert_eq!(q, q2);
    }

    #[test]
    fn test_question_to_from_str_with_stats() {
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
            updated: Some(Utc::now()),
            stats: Some(Stats {
                correct: 1,
                incorrect: 2,
                skipped: 3,
            }),
            contributor: Some(ContributorProfile {
                name: Some("John Doe".to_string()),
                url: Some("https://example.com".to_string()),
                img_url: Some("https://example.com/img.jpg".to_string()),
                about: Some("A great developer".to_string()),
            }),
            title: "Simple Rust question".to_string(),
            stage: PublishStage::Draft,
            refresher_links: None,
        };

        let s = q.to_string();
        println!("{}", s);
        let q2 = Question::from_str(&s).unwrap();

        assert_ne!(q, q2);
    }

    #[test]
    fn test_question_to_from_str_no_title() {
        let mut q = Question {
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
            ],
            correct: 1,
            author: Some("you@me.us".to_string()),
            updated: Some(Utc::now()),
            stats: None,
            contributor: Some(ContributorProfile {
                name: Some("John Doe".to_string()),
                url: Some("https://example.com".to_string()),
                img_url: Some("https://example.com/img.jpg".to_string()),
                about: Some("A great developer".to_string()),
            }),
            stage: PublishStage::Published, // it was Draft in other tests, vary the test here
            title: "".to_string(),
            refresher_links: None,
        };

        // blank title, question copied to title as-is
        assert_eq!(
            Question::from_str(&q.to_string()).unwrap().title,
            q.question,
            "blank title should be == question"
        );

        // test for question copied to title as-is
        q.title = "  ".to_string();
        assert_eq!(
            Question::from_str(&q.to_string()).unwrap().title,
            q.question,
            "whitespace title should be == question"
        );

        // test for a single line truncation
        q.question = "\nThe syntax and capabilities of closures make them very convenient for on the fly usage.\r \
        Calling a closure is exactly like calling a function. However,\
         both input and return types can be inferred and input variable names must be specified."
            .to_string();
        assert_eq!(
            Question::from_str(&q.to_string()).unwrap().title,
            "The syntax and capabilities of closures make them very convenient for on the fly usage. Calling a closure is exactly lik".to_string(),
            "expected question truncated to MAX_TITLE_LEN with line breaks removed"
        );
    }

    #[test]
    fn test_question_with_stage() {
        let q = Question {
            qid: "89yZBXJBa9t2LB6xfj46Rm".to_string(),
            topic: "aws".to_string(),
            question: "What is 1+1?".to_string(),
            answers: vec![],
            correct: 1,
            author: None,
            updated: None,
            stats: None,
            contributor: None,
            title: "".to_string(),
            stage: PublishStage::Published,
            refresher_links: None,
        };

        let q = q.with_stage(PublishStage::Draft);
        assert_eq!(q.stage, PublishStage::Draft);
    }

    // tests if the refresher links were extract correctly
    #[test]
    fn test_question_into_html() {
        // test link extraction and ordering
        let mut q = Question {
            qid: "".to_string(),
            topic: "".to_string(),
            question: "What is 1+1? [link](https://a.com)".to_string(),
            answers: vec![
                Answer {
                    a: "<https://b.com>".to_string(),
                    e: Some("<https://b.com/c>".to_string()),
                    c: Some(false),
                    sel: None,
                },
                Answer {
                    a: "<https://c.com>".to_string(),
                    e: Some("<https://c.com#c>".to_string()),
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
            author: None,
            updated: None,
            stats: None,
            contributor: None,
            title: "".to_string(),
            stage: PublishStage::Published,
            refresher_links: None,
        };

        assert_eq!(
            q.clone().into_html(None).refresher_links,
            Some(vec![
                "https://a.com".to_string(),
                "https://c.com".to_string(),
                "https://b.com".to_string(),
                "https://b.com/c".to_string(),
            ])
        );

        // test without links in the question or answers
        q.question = "What is 1+1?".to_string();
        q.answers[0].a = "1".to_string();
        q.answers[0].e = None;
        q.answers[1].a = "2".to_string();
        q.answers[1].e = Some("3".to_string());

        assert_eq!(q.into_html(None).refresher_links, None);
    }

    // test if the question complete
    #[test]
    fn test_question_is_complete() {
        // happy path
        let q = Question {
            qid: "".to_string(),
            topic: "aws".to_string(),
            question: "What is 1+1? [link](https://a.com)".to_string(),
            answers: vec![
                Answer {
                    a: "<https://b.com>".to_string(),
                    e: Some("<https://b.com/c>".to_string()),
                    c: Some(false),
                    sel: None,
                },
                Answer {
                    a: "<https://c.com>".to_string(),
                    e: Some("<https://c.com#c>".to_string()),
                    c: Some(true),
                    sel: None,
                },
                Answer {
                    a: "3 is an invalid answer".to_string(),
                    e: Some("3 is an invalid explanation".to_string()),
                    c: Some(false),
                    sel: None,
                },
            ],
            correct: 1,
            author: None,
            updated: None,
            stats: None,
            contributor: None,
            title: "A test questions".to_string(),
            stage: PublishStage::Draft,
            refresher_links: None,
        };

        assert!(q.is_complete());

        // test unhappy paths with missing parts, one by one

        assert!(
            !Question {
                question: "123".to_string(),
                ..q.clone()
            }
            .is_complete(),
            "invalid question"
        );

        assert!(
            !Question {
                topic: "".to_string(),
                ..q.clone()
            }
            .is_complete(),
            "invalid topic"
        );

        assert!(
            !Question {
                title: "".to_string(),
                ..q.clone()
            }
            .is_complete(),
            "invalid title"
        );

        assert!(
            !Question {
                correct: 0,
                ..q.clone()
            }
            .is_complete(),
            "invalid correct"
        );

        assert!(
            !Question {
                answers: vec![
                    Answer {
                        a: "<https".to_string(),
                        e: Some("".to_string()),
                        c: Some(false),
                        sel: None,
                    },
                    Answer {
                        a: "<https://c.com>".to_string(),
                        e: Some("<https://c.com#c>".to_string()),
                        c: Some(true),
                        sel: None,
                    },
                ],
                ..q.clone()
            }
            .is_complete(),
            "invalid answer.a"
        );

        assert!(
            !Question {
                answers: vec![
                    Answer {
                        a: "<https".to_string(),
                        e: Some("<https".to_string()),
                        c: Some(false),
                        sel: None,
                    },
                    Answer {
                        a: "<https://c.com>".to_string(),
                        e: Some("<https://c.com#c>".to_string()),
                        c: Some(true),
                        sel: None,
                    },
                ],
                ..q.clone()
            }
            .is_complete(),
            "invalid answer.e"
        );
    }
}
