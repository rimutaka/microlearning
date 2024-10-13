/// The endpoint for question-related requests.
export const QUESTION_HANDLER_URL = "https://bitesized.info/q?";

/// E.g. .../q?topic=foo&qid=bar
export const URL_PARAM_TOPIC = "topic"

/// E.g. .../q?topic=foo&qid=bar
export const URL_PARAM_QID = "qid"

/// Keypair for the topic title and DDB topic ID, e.g. "AWS"/"aws".
export interface TopicTitleId {
  t: string,
  id: string,
};

/// The exclusive list of topics that can be used in the app
/// It is sync'd with the DDB and the lambdas manually
export const TOPICS = <Array<TopicTitleId>>[
  { t: "AWS", id: "aws" },
  { t: "CSS", id: "css" },
  { t: "General", id: "general" },
  { t: "JS / TS", id: "js-ts" },
  { t: "Rust", id: "rust" }
];

/// A mirror of the Rust's type
export interface Answer {
  /// Answer option
  a: string,
  /// Detailed explanation
  e: string,
  /// Set to true if correct
  c: boolean,
}

/// A mirror of the Rust's type
export interface Question {
  /// Leave blank for new questions
  qid: string,
  /// Required - use the TOPICS.id
  topic: string,
  question: string,
  answers: Array<Answer>,
  /// It is recalculated on the server on submission
  correct: number,
}