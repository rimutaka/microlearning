pub mod ddb;
pub mod jwt;
pub mod markdown;
pub mod payments;
pub mod question;
pub mod relations;
pub mod topic;
pub mod user;

/// Logs output into browser console.
#[macro_export]
macro_rules! info {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into())
  }
}

/// Timestamp for 1 Jan 2024.
/// Timestamps used for IDs can be made shorter by subtracting this constant.
pub const TIMESTAMP_BASE: u64 = 1_704_067_200;

/// A not-so-secret salt used for hashing.
/// Do not change it ever without converting the existing hash-based IDs.
pub const PUBLIC_SALT: &str = "bite-sized";

/// An HTTP header for the JWT token.
pub const X_BITIE_TOKEN_HEADER: &str = "x-bitie-token";
