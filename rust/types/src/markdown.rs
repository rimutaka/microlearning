use pulldown_cmark::{html::push_html, Event, Parser, Tag};
use serde::Serialize;
use wasm_bindgen::prelude::*;

#[cfg(target_family = "wasm")]
use crate::info;
#[cfg(not(target_family = "wasm"))]
use tracing::info;

/// Contains the result of the streaming parser that returns HTML
/// and what was filtered out in a single pass.
#[wasm_bindgen(getter_with_clone)]
#[derive(Debug, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ValidatedMarkdown {
    /// The HTML representation of the markdown
    /// with disallowed elements removed.
    #[serde(skip_serializing_if = "String::is_empty")]
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
pub fn md_to_html(md: &str, include_html: bool) -> ValidatedMarkdown {
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

    // convert the tokens to HTML, if requested
    let html = if include_html {
        // pre-allocate the HTML string for 1.5 times the size of the markdown
        let mut html = String::with_capacity(md.len() * 3 / 2);
        push_html(&mut html, parser);
        html
    } else {
        let _ = parser.collect::<Vec<_>>();
        // parser.for_each(|_| ());
        String::new()
    };

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

/// Combines all the links in the logical order:
/// - question links
/// - correct answer links
/// - incorrect answer links
///
/// All links are sorted alphabetically within their logical group
pub fn sort_links(
    mut question_links: Vec<String>,
    mut correct_answer_links: Vec<String>,
    mut incorrect_answer_links: Vec<String>,
) -> Vec<String> {
    // remove #-part of the links
    for link in &mut question_links
        .iter_mut()
        .chain(correct_answer_links.iter_mut())
        .chain(incorrect_answer_links.iter_mut())
    {
        if let Some(pos) = link.find('#') {
            // info!("Removing #-part from link: {}", link);
            link.truncate(pos);
        }
    }

    // sort all the links alphabetically
    question_links.sort();
    correct_answer_links.sort();
    incorrect_answer_links.sort();

    // combine all the links into a single list
    let mut all_links =
        Vec::with_capacity(question_links.len() + correct_answer_links.len() + incorrect_answer_links.len());
    all_links.append(&mut question_links);
    all_links.append(&mut correct_answer_links);
    all_links.append(&mut incorrect_answer_links);

    // remove duplicates
    all_links.dedup();

    info!("Returning {} links", all_links.len());

    all_links
}

#[cfg(test)]
mod tests {
    use super::*;
    // use pulldown_cmark::{Event, Parser, Tag, TagEnd};

    // the tests have no WASM support and are meant to run in server environment only

    #[test]
    fn test_sort_links() {
        let _ = tracing_subscriber::fmt().try_init();

        let question_links = vec!["https://a.com".to_string()];
        let correct_answer_links = vec![
            "https://b.com#foo".to_string(),
            "https://b.com#bar".to_string(),
            "https://b.com/baz".to_string(),
        ];
        let incorrect_answer_links = vec![
            "https://c.com".to_string(),
            "https://c.com".to_string(),
            "https://c.com#foo".to_string(),
        ];

        assert_eq!(
            sort_links(question_links, correct_answer_links, incorrect_answer_links),
            vec![
                "https://a.com".to_string(),
                "https://b.com".to_string(),
                "https://b.com/baz".to_string(),
                "https://c.com".to_string()
            ]
        );
    }

    #[test]
    fn test_md_to_html() {
        let _ = tracing_subscriber::fmt().try_init();

        assert_eq!(
            md_to_html("Hello World!!!", false),
            ValidatedMarkdown {
                html: String::new(),
                ignored: Vec::new(),
                links: Vec::new(),
                images: Vec::new(),
            }
        );

        assert_eq!(
            md_to_html("Hello World!!!", false),
            ValidatedMarkdown {
                html: String::new(),
                ignored: Vec::new(),
                links: Vec::new(),
                images: Vec::new(),
            }
        );

        assert_eq!(
            md_to_html("", true),
            ValidatedMarkdown {
                html: String::new(),
                ignored: Vec::new(),
                links: Vec::new(),
                images: Vec::new(),
            }
        );

        assert_eq!(
            md_to_html("Hello\nWorld", true),
            ValidatedMarkdown {
                html: "<p>Hello\nWorld</p>\n".to_string(),
                ignored: Vec::new(),
                links: Vec::new(),
                images: Vec::new(),
            }
        );

        let long_md = "Hello\nWorld<p>boo</p><https://example.com> [br](/br) ![img](/img) <script>alert('hi')</script>";
        assert_eq!(
            md_to_html(long_md, true),
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

        assert_eq!(
            md_to_html(long_md, false),
            ValidatedMarkdown {
                html: "".to_string(),
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

    #[test]
    fn playground() {
        let _ = tracing_subscriber::fmt().try_init();

        let md = r#"This is an example of a _string literal_ with an escaped Unicode NULL character (`\x00` or `\0` or `\u{0}`).

The NULL character may not appear as a visible symbol in the output, but it is present in the actual value.

#### Example:
```rust
let x = "foo\x00barbaz";

println!("{x}");
println!("{:?}", x);

assert_eq!(x, "foobarbaz"); <-- fail
```
- _println!("{x}");_ output appears to be correct: `foobarbaz`
- _println!("{:?}", x);_ output has the Unicode NULL character: `"foo\0barbaz"`
- _assert_eq!_ output:
```
assertion `left == right` failed
  left: "foo\0barbaz"
 right: "foobarbaz"
```

More info: <https://doc.rust-lang.org/reference/tokens.html#character-escapes>"#;

        // let parser = Parser::new(md).filter(|event| {
        //     matches!(
        //         event,
        //         Event::Start(Tag::Link { dest_url, .. } ) if !dest_url.is_empty()
        //     )
        // });

        // for evt in parser {
        //     info!("{:?}", evt);
        // }

        let html = md_to_html(md, false);
        print!("{:#?}", html.links);
    }

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
