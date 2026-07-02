use crate::app::{Route, route_for};
use crate::components::footer::SiteFooter;
use crate::components::language::LanguageDropdown;
use crate::components::theme_switcher::ThemeSwitcher;
use crate::docs_index::{docs_for, find_page, route_path_for};
use crate::markdown::{HeadingAnchor, page_headings, render_markdown};
use crate::models::Language;
use crate::search::{grouped_search_results, search_pages, search_result_count_label};
use dioxus::prelude::*;

#[component]
pub(crate) fn DocsLayout(language: Language, slug: String) -> Element {
    let mut filter = use_signal(String::new);
    let pages = docs_for(language);
    let current_page = find_page(language, &slug).unwrap_or(&pages[0]);
    let rendered_markdown = render_markdown(current_page.markdown, current_page.path);
    let headings = page_headings(current_page.markdown);
    let shell_class = if headings.is_empty() {
        "docs-shell"
    } else {
        "docs-shell with-page-toc"
    };
    let current_slug = current_page.slug;
    let normalized_filter = filter().trim().to_lowercase();
    let navigation_results = search_pages(pages, "");
    let search_results = if normalized_filter.is_empty() {
        Vec::new()
    } else {
        search_pages(pages, &normalized_filter)
    };
    let show_search_results = !normalized_filter.is_empty();
    let search_result_count = search_result_count_label(language, search_results.len());

    rsx! {
        div { class: "{shell_class}",
            header { class: "topbar",
                Link {
                    class: "brand-link",
                    to: Route::LanguageHome {
                        lang: language.code().to_string(),
                    },
                    span { class: "brand-mark", "A" }
                    span { class: "brand-text",
                        strong { "Amagi-rs" }
                        small { "{language.docs_label()}" }
                    }
                }
                Link {
                    class: "route-pill read-before-link",
                    to: route_for(language, "disclaimer"),
                    "{language.read_before_use_label()}"
                }

                div { class: "topbar-actions",
                    label { class: "search-box topbar-search",
                        span { "{language.search_label()}" }
                        input {
                            value: "{filter}",
                            placeholder: "{language.search_placeholder()}",
                            oninput: move |event| filter.set(event.value()),
                        }
                    }
                    ThemeSwitcher {}
                    LanguageDropdown { language, current_slug }
                    a {
                        class: "source-button",
                        href: format!(
                            "https://github.com/bandange/amagi-rs/blob/main/{}",
                            current_page.path,
                        ),
                        target: "_blank",
                        rel: "noreferrer",
                        "{language.source_label()}"
                    }
                }
            }

            if show_search_results {
                section { class: "search-results-panel", "aria-label": "{language.search_results_label()}",
                    div { class: "search-results-inner",
                        div { class: "search-results-head",
                            p { class: "search-results-kicker", "{language.search_results_label()}" }
                            p { class: "search-results-count", "{search_result_count}" }
                        }
                        div { class: "search-results-list",
                            for result in &search_results {
                                Link {
                                    key: "{result.page.slug}",
                                    class: "search-result-link",
                                    to: route_for(language, result.page.slug),
                                    onclick: move |_| filter.set(String::new()),
                                    span { class: "search-result-title", "{result.page.title}" }
                                    span { class: "search-result-path", "{route_path_for(language, result.page.slug)}" }
                                    span { class: "search-result-description", "{result.page.description}" }
                                    if let Some(snippet) = &result.snippet {
                                        span { class: "search-result-snippet", "{snippet}" }
                                    }
                                }
                            }
                            if search_results.is_empty() {
                                p { class: "empty-state search-empty", "{language.empty_search_label()}" }
                            }
                        }
                    }
                }
            }

            aside { class: "sidebar",
                div { class: "sidebar-head",
                    p { class: "sidebar-kicker", "{language.sidebar_kicker()}" }
                    h1 { "{language.sidebar_title()}" }
                    p { "{language.sidebar_description()}" }
                }

                nav { class: "doc-nav", "aria-label": "Documents",
                    for (section, section_results) in grouped_search_results(&navigation_results) {
                        div { class: "nav-section", key: "{section}",
                            p { class: "nav-section-title", "{section}" }
                            for result in section_results {
                                Link {
                                    key: "{result.page.slug}",
                                    class: nav_link_class(result.page.slug == current_page.slug),
                                    to: route_for(language, result.page.slug),
                                    span { class: "nav-title", "{result.page.title}" }
                                    span { class: "nav-description", "{result.page.description}" }
                                }
                            }
                        }
                    }
                }
            }

            if !headings.is_empty() {
                PageToc { language, headings }
            }

            main { class: "content",
                div { class: "content-inner",
                    header { class: "doc-header",
                        div { class: "doc-meta-row",
                            span { class: "doc-section", "{current_page.section}" }
                            span { class: "doc-language", "{language.label()}" }
                        }
                        h2 { "{current_page.title}" }
                        p { "{current_page.description}" }
                        code { class: "doc-path", "{current_page.path}" }
                    }

                    article {
                        class: "markdown-body",
                        dangerous_inner_html: "{rendered_markdown}",
                    }

                    SiteFooter { language }
                }
            }
        }
    }
}

#[component]
fn PageToc(language: Language, headings: Vec<HeadingAnchor>) -> Element {
    rsx! {
        aside { class: "page-toc", "aria-label": "{language.page_nav_label()}",
            p { class: "page-toc-title", "{language.page_nav_label()}" }
            nav { class: "page-toc-nav",
                for heading in headings {
                    a {
                        key: "{heading.id}",
                        class: toc_link_class(heading.level),
                        href: format!("#{}", heading.id),
                        "{heading.title}"
                    }
                }
            }
        }
    }
}

fn toc_link_class(level: u8) -> &'static str {
    match level {
        3 => "page-toc-link depth-3",
        _ => "page-toc-link",
    }
}

fn nav_link_class(selected: bool) -> &'static str {
    if selected {
        "nav-link selected"
    } else {
        "nav-link"
    }
}
