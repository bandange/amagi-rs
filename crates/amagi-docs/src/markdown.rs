use crate::docs_index::{docs_for, route_path_for};
use crate::models::Language;
use pulldown_cmark::{CowStr, Event, HeadingLevel, Options, Parser, Tag, TagEnd, html};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct HeadingAnchor {
    pub(crate) level: u8,
    pub(crate) id: String,
    pub(crate) title: String,
}

pub(crate) fn render_markdown(markdown: &str, current_path: &str) -> String {
    let mut heading_ids = markdown_heading_anchors(markdown)
        .into_iter()
        .map(|heading| heading.id)
        .collect::<Vec<_>>()
        .into_iter();

    let parser = Parser::new_ext(markdown, markdown_options())
        .map(|event| rewrite_markdown_event(event, current_path, &mut heading_ids));
    let mut output = String::new();
    html::push_html(&mut output, parser);
    output
}

pub(crate) fn page_headings(markdown: &str) -> Vec<HeadingAnchor> {
    markdown_heading_anchors(markdown)
        .into_iter()
        .filter(|heading| matches!(heading.level, 2 | 3))
        .collect()
}

fn markdown_options() -> Options {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_TABLES);
    options.insert(Options::ENABLE_FOOTNOTES);
    options.insert(Options::ENABLE_STRIKETHROUGH);
    options.insert(Options::ENABLE_TASKLISTS);
    options.insert(Options::ENABLE_HEADING_ATTRIBUTES);
    options
}

fn rewrite_markdown_event<'a>(
    event: Event<'a>,
    current_path: &str,
    heading_ids: &mut impl Iterator<Item = String>,
) -> Event<'a> {
    match event {
        Event::Start(Tag::Heading {
            level,
            id,
            classes,
            attrs,
        }) => {
            let id = id.or_else(|| heading_ids.next().map(CowStr::from));

            Event::Start(Tag::Heading {
                level,
                id,
                classes,
                attrs,
            })
        }
        Event::Start(Tag::Link {
            link_type,
            dest_url,
            title,
            id,
        }) => {
            let dest_url = rewrite_doc_link(current_path, dest_url.as_ref())
                .map(CowStr::from)
                .unwrap_or(dest_url);

            Event::Start(Tag::Link {
                link_type,
                dest_url,
                title,
                id,
            })
        }
        _ => event,
    }
}

fn markdown_heading_anchors(markdown: &str) -> Vec<HeadingAnchor> {
    let mut headings = Vec::new();
    let mut current_heading: Option<HeadingBuilder> = None;
    let mut seen_ids = Vec::new();

    for event in Parser::new_ext(markdown, markdown_options()) {
        match event {
            Event::Start(Tag::Heading { level, id, .. }) => {
                current_heading = Some(HeadingBuilder {
                    level,
                    explicit_id: id.map(|id| id.into_string()),
                    title: String::new(),
                });
            }
            Event::Text(text) | Event::Code(text) => {
                if let Some(heading) = &mut current_heading {
                    heading.title.push_str(text.as_ref());
                }
            }
            Event::End(TagEnd::Heading(_)) => {
                if let Some(heading) = current_heading.take() {
                    let title = heading
                        .title
                        .split_whitespace()
                        .collect::<Vec<_>>()
                        .join(" ");
                    if title.is_empty() {
                        continue;
                    }

                    let id = match heading.explicit_id {
                        Some(id) => id,
                        None => unique_heading_id(&title, &mut seen_ids),
                    };
                    if !seen_ids.iter().any(|seen_id| seen_id == &id) {
                        seen_ids.push(id.clone());
                    }

                    headings.push(HeadingAnchor {
                        level: heading_level_number(heading.level),
                        id,
                        title,
                    });
                }
            }
            _ => {}
        }
    }

    headings
}

struct HeadingBuilder {
    level: HeadingLevel,
    explicit_id: Option<String>,
    title: String,
}

fn unique_heading_id(title: &str, seen_ids: &mut Vec<String>) -> String {
    let base_id = slugify_heading(title);
    let mut id = base_id.clone();
    let mut suffix = 2;

    while seen_ids.iter().any(|seen_id| seen_id == &id) {
        id = format!("{base_id}-{suffix}");
        suffix += 1;
    }

    seen_ids.push(id.clone());
    id
}

fn slugify_heading(title: &str) -> String {
    let mut slug = String::new();
    let mut last_was_separator = false;

    for character in title.chars().flat_map(char::to_lowercase) {
        if character.is_alphanumeric() {
            slug.push(character);
            last_was_separator = false;
        } else if !last_was_separator && !slug.is_empty() {
            slug.push('-');
            last_was_separator = true;
        }
    }

    let slug = slug.trim_matches('-');
    if slug.is_empty() {
        "section".to_string()
    } else {
        slug.to_string()
    }
}

fn heading_level_number(level: HeadingLevel) -> u8 {
    match level {
        HeadingLevel::H1 => 1,
        HeadingLevel::H2 => 2,
        HeadingLevel::H3 => 3,
        HeadingLevel::H4 => 4,
        HeadingLevel::H5 => 5,
        HeadingLevel::H6 => 6,
    }
}

pub(crate) fn rewrite_doc_link(current_path: &str, dest_url: &str) -> Option<String> {
    let dest_url = dest_url.trim();

    if dest_url.is_empty()
        || dest_url.starts_with('#')
        || dest_url.starts_with("//")
        || has_url_scheme(dest_url)
    {
        return None;
    }

    let (path, suffix) = split_link_suffix(dest_url);
    if !path.ends_with(".md") {
        return None;
    }

    let target_path = if path.starts_with('/') {
        normalize_doc_path(path.trim_start_matches('/'))
    } else {
        let base_dir = current_path.rsplit_once('/').map_or("", |(dir, _)| dir);
        normalize_doc_path(&format!("{base_dir}/{path}"))
    };

    let current_route = doc_route_for_path(current_path).unwrap_or_else(|| "/".to_string());

    doc_route_for_path(&target_path)
        .map(|route| format!("{}{}", relative_route_href(&current_route, &route), suffix))
}

fn split_link_suffix(dest_url: &str) -> (&str, &str) {
    match dest_url.find(['#', '?']) {
        Some(index) => (&dest_url[..index], &dest_url[index..]),
        None => (dest_url, ""),
    }
}

fn has_url_scheme(dest_url: &str) -> bool {
    let Some(colon_index) = dest_url.find(':') else {
        return false;
    };
    let slash_index = dest_url.find('/').unwrap_or(usize::MAX);

    colon_index < slash_index
        && dest_url[..colon_index].chars().all(|character| {
            character.is_ascii_alphanumeric() || matches!(character, '+' | '-' | '.')
        })
}

fn normalize_doc_path(path: &str) -> String {
    let mut parts = Vec::new();

    for part in path.split('/') {
        match part {
            "" | "." => {}
            ".." => {
                parts.pop();
            }
            _ => parts.push(part),
        }
    }

    parts.join("/")
}

fn doc_route_for_path(path: &str) -> Option<String> {
    [Language::Chinese, Language::English]
        .into_iter()
        .flat_map(|language| docs_for(language).iter().map(move |page| (language, page)))
        .find_map(|(language, page)| {
            if page.path == path {
                Some(route_path_for(language, page.slug))
            } else {
                None
            }
        })
}

pub(crate) fn relative_route_href(current_route: &str, target_route: &str) -> String {
    let mut base_segments = route_segments(current_route);
    base_segments.pop();
    let target_segments = route_segments(target_route);

    let common_len = base_segments
        .iter()
        .zip(target_segments.iter())
        .take_while(|(left, right)| left == right)
        .count();

    let mut parts = Vec::new();
    parts.extend(std::iter::repeat_n(
        "..",
        base_segments.len().saturating_sub(common_len),
    ));
    parts.extend(target_segments[common_len..].iter().copied());

    if parts.is_empty() {
        ".".to_string()
    } else {
        parts.join("/")
    }
}

fn route_segments(route: &str) -> Vec<&str> {
    route
        .trim_matches('/')
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rewrites_same_language_markdown_links_to_routes() {
        assert_eq!(
            rewrite_doc_link("README.zh-CN.md", "docs/中文/参考/命令行参考.md").as_deref(),
            Some("zh/cli")
        );
        assert_eq!(
            rewrite_doc_link("README.md", "docs/en/installation.md").as_deref(),
            Some("en/installation")
        );
    }

    #[test]
    fn rewrites_root_readme_language_links_to_routes() {
        assert_eq!(
            rewrite_doc_link("README.md", "README.zh-CN.md").as_deref(),
            Some("zh")
        );
        assert_eq!(
            rewrite_doc_link("README.zh-CN.md", "README.md").as_deref(),
            Some("en")
        );
    }

    #[test]
    fn rewrites_cross_language_markdown_links_to_routes() {
        assert_eq!(
            rewrite_doc_link("docs/中文/参考/命令行参考.md", "../../en/reference/cli.md")
                .as_deref(),
            Some("../en/cli")
        );
        assert_eq!(
            rewrite_doc_link("docs/en/testing.md", "../中文/测试分层.md#fixtures").as_deref(),
            Some("../zh/testing#fixtures")
        );
    }

    #[test]
    fn rewrites_root_disclaimer_links_to_routes() {
        assert_eq!(
            rewrite_doc_link("DISCLAIMER.md", "DISCLAIMER.zh-CN.md").as_deref(),
            Some("../zh/disclaimer")
        );
        assert_eq!(
            rewrite_doc_link("DISCLAIMER.zh-CN.md", "DISCLAIMER.md").as_deref(),
            Some("../en/disclaimer")
        );
    }

    #[test]
    fn rewrites_markdown_links_as_project_path_safe_relative_routes() {
        assert_eq!(
            rewrite_doc_link(
                "docs/中文/参考/Rust API 参考.md",
                "../../en/reference/rust-api.md",
            )
            .as_deref(),
            Some("../en/rust-api")
        );
        assert_eq!(
            relative_route_href("/zh/rust-api", "/en/rust-api"),
            "../en/rust-api"
        );
        assert_eq!(relative_route_href("/zh", "/en"), "en");
    }

    #[test]
    fn leaves_external_and_anchor_links_unchanged() {
        assert_eq!(
            rewrite_doc_link("README.md", "https://example.com/readme.md"),
            None
        );
        assert_eq!(rewrite_doc_link("README.md", "#overview"), None);
    }

    #[test]
    fn extracts_page_headings_with_stable_ids() {
        let headings = page_headings(
            r#"
# Title

## Install CLI

### Windows `PowerShell`

## Install CLI

## 自定义 设置
"#,
        );

        assert_eq!(
            headings,
            vec![
                HeadingAnchor {
                    level: 2,
                    id: "install-cli".to_string(),
                    title: "Install CLI".to_string(),
                },
                HeadingAnchor {
                    level: 3,
                    id: "windows-powershell".to_string(),
                    title: "Windows PowerShell".to_string(),
                },
                HeadingAnchor {
                    level: 2,
                    id: "install-cli-2".to_string(),
                    title: "Install CLI".to_string(),
                },
                HeadingAnchor {
                    level: 2,
                    id: "自定义-设置".to_string(),
                    title: "自定义 设置".to_string(),
                },
            ]
        );
    }

    #[test]
    fn render_markdown_adds_generated_heading_ids() {
        let html = render_markdown("## Install CLI\n\n### Windows PowerShell", "README.md");

        assert!(html.contains(r#"<h2 id="install-cli">Install CLI</h2>"#));
        assert!(html.contains(r#"<h3 id="windows-powershell">Windows PowerShell</h3>"#));
    }
}
