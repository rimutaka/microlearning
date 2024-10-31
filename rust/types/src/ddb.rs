/// SK is not used at the moment and is set to a constant value.
pub const DEFAULT_USER_TABLE_SK_VALUE: &str = "sub";

/// The list of table names in the DynamoDB.
pub mod tables {
    /// Contains the full text of the questions along with the metadata.
    pub const QUESTIONS: &str = "questions_20241009_1118";
    /// List of users, their subscriptions and answered questions.
    pub const USERS: &str = "users_20241023_0712";
}

/// The list of field names across all DDB tables.
pub mod fields {
    /// An ID of the topic for a group of questions.
    /// May contains lower-case characters, digits, and underscores.
    pub const TOPIC: &str = "topic";
    /// A list of topics as a DDB String Set.
    pub const TOPICS: &str = "topics";
    /// Question ID - a base58 encoded UUID7.
    pub const QID: &str = "qid";
    /// A timestamp for when the record was last updated.
    pub const UPDATED: &str = "updated";
    /// A markdown text for all the answers together.
    pub const ANSWERS: &str = "answers";
    /// A markdown text for the question.
    pub const QUESTION: &str = "question";
    /// A list of questions sent to the user or the user interacted with.
    pub const QUESTIONS: &str = "questions";
    /// A Number Set of correct answers.
    pub const CORRECT: &str = "correct";
    /// User ID - a base58 encoded UUID7.
    pub const UID: &str = "uid";
    /// The generic sort key
    pub const SORT_KEY: &str = "sk";
    /// A generic details field with a JSON structure.
    pub const DETAILS: &str = "details";
    /// User's email address
    pub const EMAIL: &str = "email";
    /// A unique unsubscribe token in lower-case
    pub const UNSUBSCRIBE: &str = "unsubscribe";
}
