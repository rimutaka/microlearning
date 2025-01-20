use bitie_types::markdown;
use wasm_bindgen::prelude::*;

// Two logging options - browser console for WASM and tracing for native
#[cfg(target_family = "wasm")]
use bitie_types::info;
#[cfg(not(target_family = "wasm"))]
use tracing::info;

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
pub async fn md_to_html(md: &str) -> markdown::ValidatedMarkdown {
    info!("Converting MD ({}) to HTML", md.len());
    markdown::md_to_html(md, true)
}

/// Combines all the links in the logical order:
/// - question links
/// - correct answer links
/// - incorrect answer links
///
/// All links are sorted alphabetically within their logical group
#[wasm_bindgen]
pub fn sort_links(
    question_links: Vec<String>,
    correct_answer_links: Vec<String>,
    incorrect_answer_links: Vec<String>,
) -> Vec<String> {
    info!("Sorting the links in the right order");

    markdown::sort_links(
        question_links.clone(),
        correct_answer_links.clone(),
        incorrect_answer_links.clone(),
    )
}
