use anyhow::{Error, Result};
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use tracing::error;

/// How the question was answered and when.  
/// DDB value example: 2024-01-01T00:00:00Zc  
/// Implements FromStr to convert from a string stored in DDB.  
/// Implements Display to convert to a string for DDB.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
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
#[derive(Deserialize, Serialize, Debug, PartialEq)]
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
}
