pub use question_impl::Question;
use serde::{Deserialize, Serialize};
use std::fmt::Display;
use std::str::FromStr;

mod question_impl;

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

/// Stats about the user answers, correct, incorrect, skipped.
/// The counters are DDB fields.
/// The struct values are set during DDB reads.
#[derive(Deserialize, Serialize, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    /// The total number of correct answers.
    pub correct: u32,
    /// The total number of incorrect answers.
    pub incorrect: u32,
    /// The total number the use chose to skipped the question.
    pub skipped: u32,
}

#[derive(Deserialize, Serialize, Debug, PartialEq, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ContributorProfile {
    /// The name of the contributor as it was entered with the questions.
    pub name: Option<String>,
    /// A URL to the contributor's profile, website or project as it was entered with the question.
    pub url: Option<String>,
    /// A URL for the the contributor's logo or avatar as it was entered with the question.
    pub img_url: Option<String>,
    /// A free text description, blurb or a quote about the contributor
    pub about: Option<String>,
}

/// Formats the contributor profile as `name / url / img_url / about` skipping empty values,
/// after trimming white space.
impl Display for ContributorProfile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = self.name.as_ref().map(|v| v.trim());
        let url = self.url.as_ref().map(|v| v.trim());
        let img_url = self.img_url.as_ref().map(|v| v.trim());
        let about = self.about.as_ref().map(|v| v.trim());

        let contents = [name, url, img_url, about]
            .iter()
            .filter_map(|v| v.to_owned())
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|v| v.trim())
            .collect::<Vec<&str>>()
            .join(" / ");

        write!(f, "{}", contents)
    }
}

/// Controls visibility of the question.
/// - Draft - visible to the author and mods
/// - Published - visible to everyone
#[derive(Deserialize, Serialize, Debug, PartialEq, Clone, Default)]
#[serde(rename_all = "camelCase")]
pub enum PublishStage {
    #[default]
    Draft,
    Published,
}

impl FromStr for PublishStage {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "draft" => Ok(PublishStage::Draft),
            "published" => Ok(PublishStage::Published),
            _ => Err(format!("Invalid publish stage: {}", s)),
        }
    }
}

impl Display for PublishStage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PublishStage::Draft => write!(f, "draft"),
            PublishStage::Published => write!(f, "published"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_publish_stage_from_str() {
        let x = PublishStage::Draft;
        assert_eq!(PublishStage::from_str(&x.to_string()).unwrap(), PublishStage::Draft);
        let x = PublishStage::Published;
        assert_eq!(PublishStage::from_str(&x.to_string()).unwrap(), PublishStage::Published);
        assert!(PublishStage::from_str("invalid").is_err());
    }
}
