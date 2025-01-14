#[cfg(target_family = "wasm")]
use crate::info;
use pulldown_cmark::{html::push_html, Parser};
#[cfg(not(target_family = "wasm"))]
use tracing::info;

pub async fn md_to_html(md: &str) -> String {
    if md.is_empty() {
        return String::new();
    }

    // convert the question to HTML
    let parser = Parser::new(md);
    info!("Parser created");

    let mut html = String::new();

    // info!("HTML created");
    push_html(&mut html, parser);
    info!(
        "MD: {}, HTML cap: {}, HTML len: {}",
        md.len(),
        html.capacity(),
        html.len(),
    );
    html
}

// the tests have no WASM support and are meant to run in server environment only
#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_md_to_html() {
        let _ = tracing_subscriber::fmt().try_init();

        assert_eq!(md_to_html("Hello\nWorld").await, "<p>Hello\nWorld</p>\n");
        assert_eq!(md_to_html("Hello\n\nWorld").await, "<p>Hello</p>\n<p>World</p>\n");
        assert_eq!(md_to_html("Hello  \nWorld").await, "<p>Hello<br />\nWorld</p>\n");
    }
}
