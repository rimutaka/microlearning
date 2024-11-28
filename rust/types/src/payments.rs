use serde::{Deserialize, Serialize};

pub const STRIPE_SECRETS_ENV_VAR: &str = "stripe_secret_arn";

/// Describes a single donation for one or more questions
#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct QuestionDonation {
    /// Email of the person who made the transaction
    pub contact_email: String,

    /// The number of questions to be paid for
    pub qty: u32,

    /// A fully-qualified cancel URL for the payment processor,
    pub cancel_url: String,
    
    /// A fully-qualified success URL for the payment processor,
    pub success_url: String,

    /// Optional contributor details
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub contributor: Option<crate::question::ContributorProfile>,

    /// What topics should be covered, if there is a preference
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub topics: Option<String>,
}

/// A pair of keys from the payment processor.
/// This struct is saved in AWS Secrets Manager as a JSON object.
/// E.g. `{"pub_key":"pk_live_51Ok...U","secret":"sk_live_51Ok...x6"}`
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct PaymentProcessorSecrets {
    /// Key ID
    pub pub_key: String,
    /// Key secret
    pub secret: String,
}
