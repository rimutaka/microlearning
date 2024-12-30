use anyhow::{Error, Result};
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::cmp::PartialOrd;
use std::collections::HashMap;
use std::{fmt::Display, str::FromStr};
use tracing::error;

/// How the question was answered and when.  
/// DDB value example: 2024-01-01T00:00:00Zc  
/// Implements FromStr to convert from a string stored in DDB.  
/// Implements Display to convert to a string for DDB.
#[derive(Deserialize, Serialize, Debug, PartialEq, Eq, Clone)]
#[serde(rename_all = "camelCase")]
pub enum AnswerStatus {
    /// The question was emailed or shown to the user
    /// DDB value: 2024-01-01T00:00:00Za
    Asked(DateTime<Utc>),
    /// All options were selected correctly
    /// DDB value: 2024-01-01T00:00:00Zc
    Correct(DateTime<Utc>),
    /// Some options were selected incorrectly
    /// DDB value: 2024-01-01T00:00:00Zi
    Incorrect(DateTime<Utc>),
    /// The question was skipped
    /// DDB value: 2024-01-01T00:00:00Zs
    Skipped(DateTime<Utc>),
    // remember to update the tests if the enum changes
    // see https://users.rust-lang.org/t/ensure-exhaustiveness-of-list-of-enum-variants/99891/3
}

/// User interaction with the question.  
/// Implements FromStr to convert from a string stored in DDB.  
/// Implements Display to convert to a string for DDB.
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AskedQuestion {
    /// Question's topic, PK of questions table.
    pub topic: String,
    /// Question's ID, SK of questions table.
    pub qid: String,
    /// Timestamp when the question was emailed, displayed, or answered with the result.
    pub status: AnswerStatus,
}

/// Full user details.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct User {
    /// User's email address
    pub email: String,
    /// Hash of the email address in hex format, e.g. 0e3bf888c95b085a7172b2e819692bb5b46c26ad067f9405c8ba1dd950732b65
    /// It is used an the anonymous user ID that can be recalculated from the email address without a DB request
    pub email_hash: String,
    /// The list of subscribed topics
    pub topics: Vec<String>,
    /// The list of asked questions
    #[serde(default = "Vec::new")]
    pub questions: Vec<AskedQuestion>,
    /// A unique string to use an unsubscribe key
    /// A shortened base58 encoded UUID
    pub unsubscribe: String,
    /// When the subscription was last updated
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub updated: Option<DateTime<Utc>>,
}

/// Convert it into 2024-01-01T00:00:00Za format,
/// where the last character is the status.
/// Putting the status at the end helps with sorting them by date in chronological order.
impl Display for AnswerStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnswerStatus::Asked(ts) => write!(f, "{}a", ts.to_rfc3339_opts(SecondsFormat::Secs, true)),
            AnswerStatus::Correct(ts) => write!(f, "{}c", ts.to_rfc3339_opts(SecondsFormat::Secs, true)),
            AnswerStatus::Incorrect(ts) => write!(f, "{}i", ts.to_rfc3339_opts(SecondsFormat::Secs, true)),
            AnswerStatus::Skipped(ts) => write!(f, "{}s", ts.to_rfc3339_opts(SecondsFormat::Secs, true)),
        }
    }
}

/// Convert it into topic/qid/2024-01-01T00:00:00Za format,
/// where the last character is the status.
/// E.g. `general/3RuWxwkgBgpWk6ZUARaZx6//2024-10-31T08:39:17Zc``
impl Display for AskedQuestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}/{}", self.topic, self.qid, self.status)
    }
}

impl FromStr for AnswerStatus {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // possible values: 2024-01-01T00:00:00Zc, 2024-01-01T00:00:00Zi, ...
        if s.len() != 21 {
            error!("Invalid AnswerStatus: {}", s);
            return Err(Error::msg(format!("Invalid AnswerStatus (len): {}", s)));
        }
        let ts = match DateTime::parse_from_rfc3339(&s[..s.len() - 1]) {
            Ok(v) => v.to_utc(),
            Err(e) => {
                error!("Invalid AnswerStatus: {s},{:?}", e);
                return Err(Error::msg(format!("Invalid AnswerStatus: {s},{:?}", e)));
            }
        };

        match s.as_bytes()[s.len() - 1] as char {
            'a' => Ok(AnswerStatus::Asked(ts)),
            'c' => Ok(AnswerStatus::Correct(ts)),
            'i' => Ok(AnswerStatus::Incorrect(ts)),
            's' => Ok(AnswerStatus::Skipped(ts)),
            _ => {
                error!("Invalid AnswerStatus: {}", s);
                Err(Error::msg(format!("Invalid AnswerStatus (match): {}", s)))
            }
        }
    }
}

impl FromStr for AskedQuestion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 3 {
            error!("Expected 3 parts in AskedQuestion: {}", s);
            return Err(Error::msg("Invalid AskedQuestion (part count)"));
        }

        let topic = parts[0].to_string();
        let qid = parts[1].to_string();
        let status = AnswerStatus::from_str(parts[2])?;

        // this structure should be safe enough for further processing
        Ok(AskedQuestion { topic, qid, status })
    }
}

impl PartialOrd for AnswerStatus {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for AnswerStatus {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = match self {
            AnswerStatus::Asked(v) => v,
            AnswerStatus::Skipped(v) => v,
            AnswerStatus::Correct(v) => v,
            AnswerStatus::Incorrect(v) => v,
        };
        let b = match other {
            AnswerStatus::Asked(v) => v,
            AnswerStatus::Skipped(v) => v,
            AnswerStatus::Correct(v) => v,
            AnswerStatus::Incorrect(v) => v,
        };
        a.cmp(b)
    }
}

impl AskedQuestion {
    /// Return a unique list of questions with the latest answer status.
    /// If the question was answered, it returns the latest answer.
    /// If the question was never answered, it returns the latest viewing.
    pub fn latest_answer_list(mut questions: Vec<AskedQuestion>) -> Vec<AskedQuestion> {
        let mut viewed = HashMap::<String, AskedQuestion>::with_capacity(questions.len());
        let mut answered = HashMap::<String, AskedQuestion>::with_capacity(questions.len());

        // this list should already be sorted in the ascending order by DDB as it appends to the end of the array
        // but who knows if that is enforced, so sorting it again just in case
        // a sort with no moves is cheap and linear
        questions.sort_by(|a, b| a.status.cmp(&b.status));

        // sort them into answered and viewed buckets
        for q in questions.into_iter().rev() {
            match q.status {
                AnswerStatus::Asked(_) | AnswerStatus::Skipped(_) => {
                    if !viewed.contains_key(&q.qid) {
                        viewed.insert(q.qid.clone(), q);
                    }
                }
                AnswerStatus::Correct(_) | AnswerStatus::Incorrect(_) => {
                    if !answered.contains_key(&q.qid) {
                        answered.insert(q.qid.clone(), q);
                    }
                }
            }
        }

        // replace the viewed questions with the answered ones
        for (k, v) in answered.into_iter() {
            viewed.insert(k, v);
        }

        // convert the hashmap into a vector for sorting
        let mut unique_questions = viewed.into_values().collect::<Vec<_>>();

        // sort them by the timestamp in descending order
        unique_questions.sort_by(|a, b| b.status.cmp(&a.status));

        unique_questions
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Timelike;

    #[test]
    fn to_from_answer_status() {
        let ts: [_; 2] = [
            DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap().to_utc(),
            Utc::now().with_nanosecond(0).unwrap(), // have to reset the nanoseconds to match the shorter &str format
        ];

        let state1 = AnswerStatus::Correct(ts[0]);
        let state2 = AnswerStatus::from_str("2024-01-01T00:00:00Zc").unwrap();
        assert_eq!(state1, state2);

        for &ts in ts.iter() {
            println!("TS: {}", ts);
            let state1 = AnswerStatus::Correct(ts);
            let state2 = AnswerStatus::from_str(&state1.to_string()).unwrap();
            assert_eq!(state1, state2);

            let state1 = AnswerStatus::Incorrect(ts);
            let state2 = AnswerStatus::from_str(&state1.to_string()).unwrap();
            assert_eq!(state1, state2);

            let state1 = AnswerStatus::Skipped(ts);
            let state2 = AnswerStatus::from_str(&state1.to_string()).unwrap();
            assert_eq!(state1, state2);

            let state1 = AnswerStatus::Asked(ts);
            let state2 = AnswerStatus::from_str(&state1.to_string()).unwrap();
            assert_eq!(state1, state2);
        }
    }

    #[test]
    fn bad_answer_status() {
        let test_values: [&str; 7] = [
            "c",
            "2024-01-01T00:00:00Zx",
            "2024-01-01T00:00:00.000Za",
            "/",
            " ",
            "",
            "abcd-01-01T00:00:00Zx",
        ];
        for s in test_values.iter() {
            assert!(AnswerStatus::from_str(s).is_err(), "Test value: {}", s);
        }
    }

    #[test]
    fn asked_question_to_from_str() {
        let q1 = AskedQuestion {
            topic: "aws".to_string(),
            qid: "3RuWxwkgBgpWk6ZUARaZx6".to_string(),
            status: AnswerStatus::Correct(Utc::now().with_nanosecond(0).unwrap()),
        };
        println!("Q1: {}", q1);
        let q2 = AskedQuestion::from_str(&q1.to_string()).unwrap();
        assert_eq!(q1, q2);

        let q1 = AskedQuestion {
            topic: "aws".to_string(),
            qid: "3RuWxwkgBgpWk6ZUARaZx6".to_string(),
            status: AnswerStatus::Correct(DateTime::parse_from_rfc3339("2024-10-31T08:39:17Z").unwrap().to_utc()),
        };

        let q2 = AskedQuestion::from_str("aws/3RuWxwkgBgpWk6ZUARaZx6/2024-10-31T08:39:17Zc").unwrap();
        assert_eq!(q1, q2);
    }

    #[test]
    fn asked_question_from_str_valid_but_make_no_sense() {
        let test_values: [&str; 2] = [
            "/3RuWxwkgBgpWk6ZUARaZx6/2024-10-31T08:39:17Zc",
            "aws//2024-10-31T08:39:17Zc",
        ];
        for s in test_values.iter() {
            assert!(AskedQuestion::from_str(s).is_ok(), "Test value: {}", s);
        }
    }

    #[test]
    fn asked_question_from_str_invalid() {
        let test_values: [&str; 7] = [
            "aws/3RuWxwkgBgpWk6ZUARaZx6//2024-10-31T08:39:17Zc",
            "3RuWxwkgBgpWk6ZUARaZx6/2024-10-31T08:39:17Zc",
            "aws/3RuWxwkgBgpWk6ZUARaZx6/",
            "///",
            "",
            "hello",
            "aws/3RuWxwkgBgpWk6ZUARaZx6//",
        ];
        for s in test_values.iter() {
            assert!(AskedQuestion::from_str(s).is_err(), "Test value: {}", s);
        }
    }

    #[test]
    /// Test the sorting of the questions by the latest answer status.
    fn answer_status_sort() {
        let ts: [_; 4] = [
            DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap().to_utc(),
            DateTime::parse_from_rfc3339("2024-01-01T00:00:01Z").unwrap().to_utc(),
            DateTime::parse_from_rfc3339("2024-01-01T00:00:02Z").unwrap().to_utc(),
            DateTime::parse_from_rfc3339("2024-01-01T00:00:03Z").unwrap().to_utc(),
        ];

        let s0 = AnswerStatus::Correct(ts[0]);
        let s1 = AnswerStatus::Asked(ts[1]);
        let s2 = AnswerStatus::Incorrect(ts[2]);
        let s3 = AnswerStatus::Skipped(ts[3]);

        let qid = "3RuWxwkgBgpWk6ZUARaZx6".to_string();
        let topic = "aws".to_string();

        let q0 = AskedQuestion {
            topic: topic.clone(),
            qid: qid.clone(),
            status: s0.clone(),
        };
        let q1 = AskedQuestion {
            topic: topic.clone(),
            qid: qid.clone(),
            status: s1,
        };
        let q2 = AskedQuestion {
            topic: topic.clone(),
            qid: qid.clone(),
            status: s2,
        };
        let q3 = AskedQuestion {
            topic: topic.clone(),
            qid: qid.clone(),
            status: s3.clone(),
        };

        let mut questions = vec![q0, q1, q2, q3];

        // sort ascending
        questions.sort_by(|a, b| a.status.cmp(&b.status));
        assert_eq!(questions[0].status, s0, "Ascending, expected: {:?}", s0);

        // sort descending
        questions.sort_by(|a, b| b.status.cmp(&a.status));
        assert_eq!(questions[0].status, s3, "Descending, expected: {:?}", s3);
    }

    #[test]
    /// Test the sorting of the questions by the latest answer status.
    fn latest_answer_list() {
        let qid = "3RuWxwkgBgpWk6ZUARaZx6".to_string();
        let topic = "aws".to_string();

        let ts1 = DateTime::parse_from_rfc3339("2024-01-01T00:00:00Z").unwrap().to_utc();
        let ts2 = DateTime::parse_from_rfc3339("2024-01-01T00:00:01Z").unwrap().to_utc();
        let ts3 = DateTime::parse_from_rfc3339("2024-01-01T00:00:02Z").unwrap().to_utc();
        let ts4 = DateTime::parse_from_rfc3339("2024-01-01T00:00:03Z").unwrap().to_utc();
        let ts5 = DateTime::parse_from_rfc3339("2024-01-01T00:00:04Z").unwrap().to_utc();

        let q = AskedQuestion {
            topic: topic.clone(),
            qid: qid.clone(),
            status: AnswerStatus::Asked(ts1),
        };

        let qa1 = AskedQuestion {
            status: AnswerStatus::Asked(ts1),
            ..q.clone()
        };

        let qa2 = AskedQuestion {
            status: AnswerStatus::Asked(ts2),
            ..q.clone()
        };

        let qc3 = AskedQuestion {
            status: AnswerStatus::Correct(ts3),
            ..q.clone()
        };

        let qi4 = AskedQuestion {
            status: AnswerStatus::Incorrect(ts4),
            ..q.clone()
        };

        let qc5 = AskedQuestion {
            status: AnswerStatus::Correct(ts5),
            ..q.clone()
        };

        // correct ordering
        let questions = vec![qa1.clone(), qa2.clone(), qc3.clone(), qi4.clone(), qc5.clone()];
        let sorted = AskedQuestion::latest_answer_list(questions);

        assert_eq!(sorted.len(), 1, "Expected 1 question in the list");
        assert_eq!(&sorted[0].status, &qc5.status, "Correct ordering");

        // incorrect ordering
        let questions = vec![qi4.clone(), qc3.clone(), qc5.clone(), qa1.clone(), qa2.clone()];
        let sorted = AskedQuestion::latest_answer_list(questions);

        assert_eq!(sorted.len(), 1, "Expected 1 question in the list");
        assert_eq!(&sorted[0].status, &qc5.status, "Incorrect ordering");

        // skipped after answering
        let qa1 = AskedQuestion {
            status: AnswerStatus::Asked(ts1),
            ..q.clone()
        };

        let qi2 = AskedQuestion {
            status: AnswerStatus::Incorrect(ts2),
            ..q.clone()
        };

        let qc3 = AskedQuestion {
            status: AnswerStatus::Correct(ts3),
            ..q.clone()
        };

        let qs4 = AskedQuestion {
            status: AnswerStatus::Skipped(ts4),
            ..q.clone()
        };

        let qa5 = AskedQuestion {
            status: AnswerStatus::Asked(ts5),
            ..q.clone()
        };

        let questions = vec![qa1.clone(), qi2.clone(), qc3.clone(), qs4.clone(), qa5.clone()];
        let sorted = AskedQuestion::latest_answer_list(questions);

        assert_eq!(sorted.len(), 1, "Expected 1 question in the list");
        assert_eq!(&sorted[0].status, &qc3.status, "Skipped after answering");

        // no answers in the list
        let questions = vec![qa1.clone(), qa2.clone(), qs4.clone(), qa5.clone()];
        let sorted = AskedQuestion::latest_answer_list(questions);

        assert_eq!(sorted.len(), 1, "Expected 1 question in the list");
        assert_eq!(&sorted[0].status, &qa5.status, "No answers in the list");
    }
}
