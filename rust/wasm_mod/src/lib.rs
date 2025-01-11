use pulldown_cmark::{html::push_html, Parser};
use wasm_bindgen::prelude::*;

/// Logs output into browser console.
macro_rules!  log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into())
    }
}

/// A demo function for getting WASM working for the first time
#[wasm_bindgen(start)]
fn init() {
    log!("WASM initializing");
    console_error_panic_hook::set_once();
}

/// A demo function for getting WASM working for the first time
#[wasm_bindgen]
pub async fn hello_world() {
    log!("Hello world!");
}

/// Converts a markdown string to HTML
#[wasm_bindgen]
pub async fn md_to_html(md: &str) -> String {
    log!("Converting MD ({}) to HTML", md.len());

    if md.is_empty() {
        return String::new();
    }

    // convert the question to HTML
    let parser = Parser::new(md);
    // log!("Parser created");
    let mut html = String::new();
    // log!("HTML created");
    push_html(&mut html, parser);
    // log!("HTML pushed");
    // let end_time = std::time::Instant::now();
    // let duration = (end_time - start_time).as_millis();
    log!(
        "MD: {}, HTML cap: {}, HTML len: {}",
        md.len(),
        html.capacity(),
        html.len(),
    );
    html
}
