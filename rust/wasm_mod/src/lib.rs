use wasm_bindgen::prelude::*;

pub mod md;

/// Logs output into browser console.
macro_rules! info {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}
pub(crate) use info;

/// A demo function for getting WASM working for the first time
#[wasm_bindgen(start)]
fn init() {
    info!("WASM initializing");
    console_error_panic_hook::set_once();
}

/// A demo function for getting WASM working for the first time
#[wasm_bindgen]
pub async fn hello_world() {
    info!("Hello world!");
}

/// Converts a markdown string to HTML. Available in WASM only.
#[wasm_bindgen]
pub async fn md_to_html(md: &str) -> String {
    info!("Converting MD ({}) to HTML", md.len());
    md::md_to_html(md).await
}
