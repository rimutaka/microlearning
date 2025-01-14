//! WASM-safe types and their implementations

pub mod markdown;

/// Logs output into browser console.
#[macro_export]
macro_rules! info {
  ( $( $t:tt )* ) => {
      web_sys::console::log_1(&format!( $( $t )* ).into())
  }
}
