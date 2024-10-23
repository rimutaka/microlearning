/// The endpoint for question-related requests.
export const QUESTION_HANDLER_URL = "https://bitesized.info/q?";

/// E.g. .../q?topic=foo&qid=bar
export const URL_PARAM_TOPIC = "topic"

/// E.g. .../q?topic=foo&qid=bar
export const URL_PARAM_QID = "qid"

/// A temporary measure to limit who can save data in DDB
export const TOKEN_HEADER_NAME = "x-bitie-token";

/// The key name for the last authentication timestamp in the localStorage.
/// The user is asked to auth if the key is present.
export const LAST_AUTH_TIMESTAMP = "auth";

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

/// Returns the topic title by its ID
export function findTopicById (id: string): string | undefined { 
  return TOPICS.find((topic) => topic.id === id)?.t;
}

/// A mirror of the Rust's type
export interface Answer {
  /// Answer option
  a: string,
  /// Detailed explanation
  e: string,
  /// Set to true if correct
  c: boolean,
  /// Set to true if this is the user selection
  sel: boolean,
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

// /// The header name for the question format.
// /// The value should be one of the `QuestionFormat` enum values (see Rust code).
// /// CloudFront has to be configured to vary the cache depending on the header contents.
// export const QUESTION_FORMAT_HEADER_NAME = "x-bitie-question-format";
// /// `QUESTION_FORMAT_HEADER_NAME` value to return the full question with explanation in HTML.
// export const QUESTION_FORMAT_HEADER_HTML_FULL = "html_full";
// /// `QUESTION_FORMAT_HEADER_NAME` value to return the full question with explanation in Markdown.
// export const QUESTION_FORMAT_HEADER_MARKDOWN_FULL = "markdown_full";
