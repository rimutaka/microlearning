use anyhow::{Error, Result};
use chrono::{DateTime, SecondsFormat, Utc};
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use tracing::error;

/// How the question was answered and when.
/// The timestamp is for the last attempt.
/// DDB value example: c2024-01-01T00:00:00Z
/// Defaults to Unanswered.
#[derive(Deserialize, Serialize, Debug, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum AnswerState {
    /// All options were selected correctly
    /// DDB value: c2024-01-01T00:00:00Z
    Correct(DateTime<Utc>),
    /// Some options were selected incorrectly
    /// DDB value: c2024-01-01T00:00:00Z
    Incorrect(DateTime<Utc>),
    /// The question was skipped
    /// DDB value: c2024-01-01T00:00:00Z
    Skipped(DateTime<Utc>),
    /// Asked, the user may have looked at it, but not answered.
    /// DDB value: u or ""
    #[default]
    Unanswered,
    // remember to update the tests if the enum changes
    // see https://users.rust-lang.org/t/ensure-exhaustiveness-of-list-of-enum-variants/99891/3
}

/// User interaction with the question.
#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AskedQuestion {
    /// Question's topic, PK of questions table.
    topic: String,
    /// Question's ID, SK of questions table.
    qid: String,
    /// Timestamp when the question was asked.
    asked: DateTime<Utc>,
    /// Timestamp when the question was answered with the result.
    #[serde(default)]
    answered: AnswerState,
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
}

// Convert it into c2024-01-01T00:00:00Z format
impl Display for AnswerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AnswerState::Correct(ts) => write!(f, "c{}", ts.to_rfc3339_opts(SecondsFormat::Secs, true)),
            AnswerState::Incorrect(ts) => write!(f, "i{}", ts.to_rfc3339_opts(SecondsFormat::Secs, true)),
            AnswerState::Skipped(ts) => write!(f, "s{}", ts.to_rfc3339_opts(SecondsFormat::Secs, true)),
            AnswerState::Unanswered => write!(f, "u"),
        }
    }
}

impl Display for AskedQuestion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}/{}/{}/{}",
            self.topic,
            self.qid,
            self.asked.to_rfc3339_opts(SecondsFormat::Secs, true),
            self.answered
        )
    }
}

impl FromStr for AnswerState {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        // possible values: u, c2024-01-01T00:00:00Z, i2024-01-01T00:00:00Z, s2024-01-01T00:00:00Z or blank
        match s {
            "u" | "" => Ok(AnswerState::Unanswered),
            _ if s.len() == 21 => {
                let ts = match DateTime::parse_from_rfc3339(&s[1..]) {
                    Ok(v) => v.to_utc(),
                    Err(e) => {
                        error!("Invalid AnswerState: {s},{:?}", e);
                        return Err(Error::msg("Invalid AnswerState"));
                    }
                };

                match &s[0..1] {
                    "c" => Ok(AnswerState::Correct(ts)),
                    "i" => Ok(AnswerState::Incorrect(ts)),
                    "s" => Ok(AnswerState::Skipped(ts)),
                    _ => {
                        error!("Invalid AnswerState: {}", s);
                        Err(Error::msg("Invalid AnswerState"))
                    }
                }
            }
            _ => {
                error!("Invalid AnswerState: {}", s);
                Err(Error::msg("Invalid AnswerState"))
            }
        }
    }
}

impl FromStr for AskedQuestion {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self> {
        let parts: Vec<&str> = s.split('/').collect();
        if parts.len() != 4 {
            error!("Expected 4 parts in AskedQuestion: {}", s);
            return Err(Error::msg("Invalid AskedQuestion"));
        }

        let topic = parts[0].to_string();
        let qid = parts[1].to_string();
        let asked = match DateTime::parse_from_rfc3339(parts[2]) {
            Ok(v) => v.to_utc(),
            Err(e) => {
                error!("Invalid asked in AskedQuestion: {s},{:?}", e);
                return Err(Error::msg("Invalid AskedQuestion"));
            }
        };

        let answered = AnswerState::from_str(parts[3])?;

        // this structure should be safe enough for further processing
        Ok(AskedQuestion {
            topic,
            qid,
            asked,
            answered,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_from_answer_state() {
        let ts = DateTime::parse_from_rfc3339(&Utc::now().to_rfc3339_opts(SecondsFormat::Secs, true))
            .unwrap()
            .to_utc();
        let state1 = AnswerState::Correct(ts);
        let state2 = AnswerState::from_str(&state1.to_string()).unwrap();
        assert_eq!(state1, state2);

        let state1 = AnswerState::Incorrect(ts);
        let state2 = AnswerState::from_str(&state1.to_string()).unwrap();
        assert_eq!(state1, state2);

        let state1 = AnswerState::Skipped(ts);
        let state2 = AnswerState::from_str(&state1.to_string()).unwrap();
        assert_eq!(state1, state2);

        let state1 = AnswerState::Unanswered;
        let state2 = AnswerState::from_str(&state1.to_string()).unwrap();
        assert_eq!(state1, state2);

        let state2 = AnswerState::from_str("").unwrap();
        assert_eq!(state1, state2);
    }

    #[test]
    fn bad_answer_state() {
        assert!(AnswerState::from_str("c").is_err());
        assert!(AnswerState::from_str("x2024-01-01T00:00:00Z").is_err());
        assert!(AnswerState::from_str("u2024-01-01T00:00:00Z").is_err());
        assert!(AnswerState::from_str("/").is_err());
        assert!(AnswerState::from_str(" ").is_err());
        assert!(AnswerState::from_str("cello").is_err());
    }
}
