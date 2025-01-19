//! This module contains the inner implementations of functions exposed to WASM.
//! It has to be a separate module to allow running non-WASM tests.

use crate::info;
use crate::ExtractedLinks;

/// Combines all the links in the logical order:
/// - question links
/// - correct answer links
/// - incorrect answer links
///
/// All links are sorted alphabetically within their logical group
pub(crate) fn sort_links(mut links: ExtractedLinks) -> Vec<String> {
    // remove #-part of the links
    for link in &mut links
        .question_links
        .iter_mut()
        .chain(links.correct_answer_links.iter_mut())
        .chain(links.incorrect_answer_links.iter_mut())
    {
        if let Some(pos) = link.find('#') {
            // info!("Removing #-part from link: {}", link);
            link.truncate(pos);
        }
    }

    // sort all the links alphabetically
    links.question_links.sort();
    links.correct_answer_links.sort();
    links.incorrect_answer_links.sort();

    // combine all the links into a single list
    let mut all_links = Vec::with_capacity(
        links.question_links.len() + links.correct_answer_links.len() + links.incorrect_answer_links.len(),
    );
    all_links.append(&mut links.question_links);
    all_links.append(&mut links.correct_answer_links);
    all_links.append(&mut links.incorrect_answer_links);

    // remove duplicates
    all_links.dedup();

    info!("Returning {} links", all_links.len());

    all_links
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_links() {
        let _ = tracing_subscriber::fmt().try_init();

        let mut links = ExtractedLinks::new();
        links.question_links = vec!["https://a.com".to_string()];
        links.correct_answer_links = vec![
            "https://b.com#foo".to_string(),
            "https://b.com#bar".to_string(),
            "https://b.com/baz".to_string(),
        ];
        links.incorrect_answer_links = vec![
            "https://c.com".to_string(),
            "https://c.com".to_string(),
            "https://c.com#foo".to_string(),
        ];

        assert_eq!(
            sort_links(links),
            vec![
                "https://a.com".to_string(),
                "https://b.com".to_string(),
                "https://b.com/baz".to_string(),
                "https://c.com".to_string()
            ]
        );
    }
}
