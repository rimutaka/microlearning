use tracing::info;

pub struct Topic {}

impl Topic {
    /// The list of valid topics. Must be synchronized with the front-end manually
    pub const TOPICS: [&'static str; 5] = ["aws", "css", "general", "js-ts", "rust"];
    /// A user-friendly list of topic names. Must match the TOPICS list
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
}
