pub mod ddb;
pub mod question;
pub mod lambda;

/// Timestamp for 1 Jan 2024.
/// Timestamps used for IDs can be made shorter by subtracting this constant.
pub const TIMESTAMP_BASE: u64 = 1_704_067_200;
