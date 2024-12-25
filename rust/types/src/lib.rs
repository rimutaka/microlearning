pub mod ddb;
pub mod jwt;
pub mod lambda;
pub mod payments;
pub mod question;
pub mod topic;
pub mod user;
pub mod relations;

/// Timestamp for 1 Jan 2024.
/// Timestamps used for IDs can be made shorter by subtracting this constant.
pub const TIMESTAMP_BASE: u64 = 1_704_067_200;

/// A not-so-secret salt used for hashing.
/// Do not change it ever without converting the existing hash-based IDs.
pub const PUBLIC_SALT: &str = "bite-sized";
