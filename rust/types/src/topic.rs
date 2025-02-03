use tracing::info;

pub struct Topic {}

impl Topic {
    /// The list of valid topics. Must be synchronized with the front-end manually
    pub const TOPICS: [&'static str; 5] = ["aws", "css", "general", "js-ts", "rust"];
    /// A user-friendly list of topic names. Must match the TOPICS list
    /// TODO: change this to a more robust struct or enum
    pub const TOPIC_NAMES: [&'static str; Self::TOPICS.len()] = ["AWS", "CSS", "Programming", "JS/TS", "Rust"];

    /// Returns only valid topics from the given list.
    pub fn filter_valid_topics(topics: Vec<String>) -> Vec<String> {
        // return an empty list if empty
        if topics.is_empty() {
            info!("No topics provided");
            return topics;
        }

        // filter out invalid values
        topics
            .into_iter()
            .filter_map(|v| {
                if Self::TOPICS.contains(&v.as_str()) {
                    Some(v)
                } else {
                    info!("Invalid topic: {v}");
                    None
                }
                //
            })
            .collect::<Vec<String>>()
    }

    /// Converts a topic ID into its readable name.
    /// E.g. `js-ts` -> `JS/TS`
    pub fn into_name(topic: &str) -> &str {
        match Self::TOPICS.iter().position(|&v| v == topic) {
            Some(i) => Self::TOPIC_NAMES[i],
            None => {
                info!("Invalid topic: {topic}");
                "Invalid topic"
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_both_arrays_have_same_length() {
        assert_eq!(Topic::TOPICS.len(), Topic::TOPIC_NAMES.len());
    }

    #[test]
    fn test_filter_valid_topics() {
        let topics = vec!["aws".to_string(), "css".to_string(), "invalid".to_string()];
        let result = Topic::filter_valid_topics(topics);
        assert_eq!(result, vec!["aws".to_string(), "css".to_string()]);
    }

    #[test]
    fn test_into_name() {
        assert_eq!(Topic::into_name("aws"), "AWS");
        assert_eq!(Topic::into_name("invalid"), "Invalid topic");
    }
}
