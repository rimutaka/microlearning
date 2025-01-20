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

/// Contains URLs extracted from different parts of the question.
/// The URL origin is important to arrange the links in the correct order.
#[wasm_bindgen(getter_with_clone)]
#[derive(Default, Debug, Clone)]
pub struct ExtractedLinks {
    pub question_links: Vec<String>,
    pub correct_answer_links: Vec<String>,
    pub incorrect_answer_links: Vec<String>,
}

#[wasm_bindgen]
impl ExtractedLinks {
    #[wasm_bindgen(constructor)]
    pub fn new() -> ExtractedLinks {
        ExtractedLinks::default()
    }
}

/// Combines all the links in the logical order:
/// - question links
/// - correct answer links
/// - incorrect answer links
///
/// All links are sorted alphabetically within their logical group
#[wasm_bindgen]
pub fn sort_links(links: &ExtractedLinks) -> Vec<String> {
    info!("Sorting the links in the right order");

    markdown::sort_links(
        links.question_links.clone(),
        links.correct_answer_links.clone(),
        links.incorrect_answer_links.clone(),
    )
}
