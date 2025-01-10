use wasm_bindgen::prelude::*;

/// Logs output into browser console.
macro_rules!  log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

/// The main entry point for the UI thread to request book data.
/// Multiple responses are sent back via `progress.js` to the UI thread.
/// See `fn report_progress()` for more details.
#[wasm_bindgen]
pub async fn hello_world() {
    log!("Hello world!");
}
