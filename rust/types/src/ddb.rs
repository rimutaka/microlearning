pub mod tables {
    /// Contains the full text of the questions along with the metadata.
    pub const QUESTIONS: &str = "questions_20241009_1118";
}

/// The list of field names in `USER_BOOKS_TABLE_NAME` table.
pub mod fields {
    /// An ID of the topic for a group of questions.
    /// May contains lower-case characters, digits, and underscores.
    pub const TOPIC: &str = "topic";
    /// Question ID - a base58 encoded UUID7.
    pub const QID: &str = "qid";
    /// A timestamp for when the record was last updated.
    pub const UPDATED: &str = "updated";
    /// A markdown text for all the answers together.
    pub const ANSWERS: &str = "answers";
    /// A markdown text for the question.
    pub const QUESTION: &str = "question";
    /// A Number Set of correct answers.
    pub const CORRECT: &str = "correct";
}
