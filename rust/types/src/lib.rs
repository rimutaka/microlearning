pub mod ddb;
pub mod jwt;
pub mod lambda;
pub mod question;
pub mod topic;
pub mod user;

/// Timestamp for 1 Jan 2024.
/// Timestamps used for IDs can be made shorter by subtracting this constant.
pub const TIMESTAMP_BASE: u64 = 1_704_067_200;
