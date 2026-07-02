use crate::models::{DocPage, Language, SearchResult};

pub(crate) fn search_pages(pages: &'static [DocPage], query: &str) -> Vec<SearchResult> {
    pages
        .iter()
        .filter_map(|page| search_page(page, query))
        .collect()
}

pub(crate) fn search_result_count_label(language: Language, count: usize) -> String {
    match language {
        Language::English if count == 1 => "1 result".to_string(),
        Language::English => format!("{count} results"),
        Language::Chinese => format!("{count} 个结果"),
    }
}

fn search_page(page: &'static DocPage, query: &str) -> Option<SearchResult> {
    if query.is_empty() {
        return Some(SearchResult {
            page,
            snippet: None,
        });
    }

    let title = page.title.to_lowercase();
    let description = page.description.to_lowercase();
    let path = page.path.to_lowercase();
    let markdown = compact_search_text(page.markdown);
    let normalized_markdown = markdown.to_lowercase();
    let metadata_matches = title.contains(query)
        || description.contains(query)
        || page.slug.contains(query)
        || path.contains(query);
    let snippet = search_snippet(&markdown, &normalized_markdown, query);

    if metadata_matches || snippet.is_some() {
        Some(SearchResult { page, snippet })
    } else {
        None
    }
}

pub(crate) fn grouped_search_results(
    results: &[SearchResult],
) -> Vec<(&'static str, Vec<&SearchResult>)> {
    let mut groups: Vec<(&'static str, Vec<&SearchResult>)> = Vec::new();

    for result in results {
        match groups
            .iter_mut()
            .find(|(section, _)| *section == result.page.section)
        {
            Some((_, section_results)) => section_results.push(result),
            None => groups.push((result.page.section, vec![result])),
        }
    }

    groups
}

fn compact_search_text(markdown: &str) -> String {
    markdown.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn search_snippet(text: &str, normalized_text: &str, query: &str) -> Option<String> {
    let match_start = normalized_text.find(query)?;
    let match_end = match_start + query.len();
    let start = snippet_boundary_before(text, match_start, 42);
    let end = snippet_boundary_after(text, match_end, 94);
    let prefix = if start > 0 { "..." } else { "" };
    let suffix = if end < text.len() { "..." } else { "" };

    Some(format!("{prefix}{}{suffix}", text[start..end].trim()))
}

fn snippet_boundary_before(text: &str, index: usize, max_chars: usize) -> usize {
    let boundary = nearest_char_boundary_before(text, index);

    text[..boundary]
        .char_indices()
        .rev()
        .nth(max_chars.saturating_sub(1))
        .map(|(position, _)| position)
        .unwrap_or(0)
}

fn snippet_boundary_after(text: &str, index: usize, max_chars: usize) -> usize {
    let boundary = nearest_char_boundary_after(text, index);

    text[boundary..]
        .char_indices()
        .nth(max_chars)
        .map(|(position, _)| boundary + position)
        .unwrap_or(text.len())
}

fn nearest_char_boundary_before(text: &str, mut index: usize) -> usize {
    index = index.min(text.len());
    while !text.is_char_boundary(index) {
        index -= 1;
    }
    index
}

fn nearest_char_boundary_after(text: &str, mut index: usize) -> usize {
    index = index.min(text.len());
    while !text.is_char_boundary(index) {
        index += 1;
    }
    index
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::docs_index::{CHINESE_DOCS, ENGLISH_DOCS};

    #[test]
    fn empty_search_returns_every_page_without_snippets() {
        let results = search_pages(CHINESE_DOCS, "");

        assert_eq!(results.len(), CHINESE_DOCS.len());
        assert!(results.iter().all(|result| result.snippet.is_none()));
    }

    #[test]
    fn search_matches_markdown_body_content() {
        let results = search_pages(ENGLISH_DOCS, "x-amagi-cookie");

        assert!(
            results
                .iter()
                .any(|result| result.page.slug == "web-api" && result.snippet.is_some())
        );
    }

    #[test]
    fn search_matches_chinese_markdown_body_content() {
        let results = search_pages(CHINESE_DOCS, "游客模式");

        assert!(
            results
                .iter()
                .any(|result| result.page.slug == "web-api" && result.snippet.is_some())
        );
    }
}
