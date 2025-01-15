use pulldown_cmark::{html::push_html, Event, Parser, Tag};
use serde::Serialize;

#[cfg(target_family = "wasm")]
use crate::info;
#[cfg(not(target_family = "wasm"))]
use tracing::info;

/// Contains the result of the streaming parser that returns HTML
/// and what was filtered out in a single pass.
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatedMarkdown {
    /// The HTML representation of the markdown
    /// with disallowed elements removed.
    pub html: String,
    /// The list of disallowed markdown elements that were ignored during the HTML conversion.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub ignored: Vec<String>,
    /// The list of link URLs found in the markdown.
    /// The are what the parser considers a link, which may be absolute or relative.
    /// The URLs are not validated and appear in the order they were encountered.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub links: Vec<String>,
    /// The list of image URLs found in the markdown.
    /// The URLs are not validated and appear in the order they were encountered.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub images: Vec<String>,
}

/// Converts markdown to HTML.
/// Disallowed elements are ignored and not included in the HTML.
/// Links and images are collected and returned in addition to the HTML.
///
/// Disallowed elements: html tags, images.
pub fn md_to_html(md: &str) -> ValidatedMarkdown {
    if md.is_empty() {
        return ValidatedMarkdown {
            html: String::new(),
            ignored: Vec::new(),
            links: Vec::new(),
            images: Vec::new(),
        };
    }

    // convert the markdown to tokens
    let parser = Parser::new(md);
    info!("Input parsed into tokens");

    // containers for specific elements with pre-allocated capacity
    // 10 is an arbitrary number for a single field
    let mut ignored = Vec::with_capacity(10);
    let mut links = Vec::with_capacity(10);
    let mut images = Vec::with_capacity(10);

    // filter out disallowed elements and collect links and images
    let parser = parser.filter(|event| match event {
        Event::InlineHtml(v) => {
            ignored.push(v.to_string());
            false
        }
        Event::Html(v) => {
            ignored.push(v.to_string());
            false
        }
        Event::Start(Tag::Image { dest_url, .. }) => {
            ignored.push(["image (", dest_url, ")"].concat());
            images.push(dest_url.to_string());
            false
        }
        Event::Start(Tag::Link { dest_url, .. }) if !dest_url.is_empty() => {
            links.push(dest_url.to_string());
            true
        }
        _ => true,
    });

    // pre-allocate the HTML string for 1.5 times the size of the markdown
    let mut html = String::with_capacity(md.len() * 3 / 2);

    // info!("HTML created");
    push_html(&mut html, parser);
    info!(
        "MD: {}, HTML cap: {}, HTML len: {}, HTML/MD: {}, overalloc: {}, ignored: {}, links: {}, images: {}",
        md.len(),
        html.capacity(),
        html.len(),
        html.len() as f64 / md.len() as f64,
        html.capacity() - html.len(),
        ignored.len(),
        links.len(),
        images.len()
    );

    ValidatedMarkdown {
        html,
        ignored,
        links,
        images,
    }
}

// the tests have no WASM support and are meant to run in server environment only
#[cfg(test)]
mod tests {
    use super::*;
    // use pulldown_cmark::{Event, Parser, Tag, TagEnd};

    #[test]
    fn test_md_to_html() {
        let _ = tracing_subscriber::fmt().try_init();

        assert_eq!(
            md_to_html(""),
            ValidatedMarkdown {
                html: String::new(),
                ignored: Vec::new(),
                links: Vec::new(),
                images: Vec::new(),
            }
        );

        assert_eq!(
            md_to_html("Hello\nWorld"),
            ValidatedMarkdown {
                html: "<p>Hello\nWorld</p>\n".to_string(),
                ignored: Vec::new(),
                links: Vec::new(),
                images: Vec::new(),
            }
        );

        assert_eq!(
            md_to_html(
                "Hello\nWorld<p>boo</p><https://example.com> [br](/br) ![img](/img) <script>alert('hi')</script>"
            ),
            ValidatedMarkdown {
                html: "<p>Hello\nWorldboo<a href=\"https://example.com\">https://example.com</a> <a href=\"/br\">br</a> img alert('hi')</p>\n".to_string(),
                ignored: vec!(
                    "<p>".to_string(),
                    "</p>".to_string(),
                    "image (/img)".to_string(),
                    "<script>".to_string(),
                    "</script>".to_string()
                ),
                links: vec!["https://example.com".to_string(), "/br".to_string()],
                images: vec!["/img".to_string()],
            }
        );
    }

    /*
        #[test]
        fn playground() {
            let _ = tracing_subscriber::fmt().try_init();

            let md = r#"What is the proper name for a group of _cosmic squirrels_?

    Review: <https://doc.rust-lang.org/rust-by-example/scope/lifetime/fn.html>

    <b>BOLD</b>"#;

            let parser = Parser::new(md);

            // let parser = Parser::new(md).filter(|event| {
            //     !matches!(
            //         event,
            //         Event::InlineHtml(_)
            //                 | Event::Html(_)
            //                 // | Event::Start(Tag::HtmlBlock)
            //                 // | Event::End(TagEnd::HtmlBlock)
            //                 | Event::Start(Tag::Image { .. })
            //                 | Event::End(TagEnd::Image)
            //     )
            // });

            // for evt in parser {
            //     info!("{:?}", evt);
            // }

            let html = md_to_html(md);
            print!("{:#?}", html);


        }
    */

    // #[test]
    // fn links_only() {
    //     let _ = tracing_subscriber::fmt().try_init();

    //     let md = "Hello\nWorld<p>boo</p><https://example.com> [br](/br)";
    //     let mut is_inside_link = false;
    //     let parser = Parser::new(md).filter(|event| match event {
    //         Event::Start(Tag::Link { .. }) => {
    //             is_inside_link = true;
    //             true
    //         }
    //         Event::End(TagEnd::Link) => {
    //             is_inside_link = false;
    //             true
    //         }
    //         _ => is_inside_link,
    //     });

    //     for evt in parser {
    //         info!("{:?}", evt);
    //     }
    // }
}
