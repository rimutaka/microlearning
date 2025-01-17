use bitie_types::{info, markdown};
use wasm_bindgen::prelude::*;

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
    markdown::md_to_html(md, true).html
}

/// Extracts a list of markdown links. Available in WASM only.
/// Only links that are converted into <a> tags are returned.
#[wasm_bindgen]
pub async fn extract_links_from_md(md: &str) -> Vec<String> {
    info!("Extracting links from MD ({})", md.len());
    markdown::md_to_html(md, false).links
}
