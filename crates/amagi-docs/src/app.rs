use crate::components::{DocsLayout, NotFoundView};
use crate::docs_index::find_page;
use crate::models::Language;
use crate::storage::read_stored_theme_preference;
use crate::theme::docs_theme_css;
use dioxus::prelude::*;

static MAIN_CSS: Asset = asset!("/assets/main.css");
static HIGHLIGHT_JS: Asset = asset!("/assets/highlight.min.js");
static SYNTAX_HIGHLIGHT_JS: Asset = asset!("/assets/syntax-highlight.js");

#[derive(Clone, Debug, PartialEq, Routable)]
pub(crate) enum Route {
    #[route("/")]
    Home {},
    #[route("/:lang")]
    LanguageHome { lang: String },
    #[route("/:lang/:slug")]
    Doc { lang: String, slug: String },
    #[route("/:..route")]
    NotFound { route: Vec<String> },
}

#[component]
pub(crate) fn App() -> Element {
    let theme_css = docs_theme_css();
    let theme_preference = use_signal(read_stored_theme_preference);
    use_context_provider(|| theme_preference);
    let root_class = theme_preference().root_class();
    let highlight_js = HIGHLIGHT_JS.to_string();
    let syntax_highlight_js = SYNTAX_HIGHLIGHT_JS.to_string();

    rsx! {
        document::Stylesheet { href: MAIN_CSS }
        document::Script {
            r#"
            (function () {{
              function loadScript(src, next) {{
                var script = document.createElement("script");
                script.src = src;
                script.onload = next;
                script.onerror = next;
                document.head.appendChild(script);
              }}

              loadScript("{highlight_js}", function () {{
                loadScript("{syntax_highlight_js}");
              }});
            }})();
            "#
        }
        style { "{theme_css}" }
        div { class: "{root_class}",
            Router::<Route> {}
        }
    }
}

#[component]
fn Home() -> Element {
    rsx! {
        DocsLayout {
            language: Language::Chinese,
            slug: "home".to_string(),
        }
    }
}

#[component]
fn LanguageHome(lang: String) -> Element {
    match Language::from_code(&lang) {
        Some(language) => rsx! {
            DocsLayout {
                language,
                slug: "home".to_string(),
            }
        },
        None => rsx! {
            NotFoundView {
                requested: format!("/{lang}"),
            }
        },
    }
}

#[component]
fn Doc(lang: String, slug: String) -> Element {
    match Language::from_code(&lang) {
        Some(language) if find_page(language, &slug).is_some() => rsx! {
            DocsLayout {
                language,
                slug,
            }
        },
        _ => rsx! {
            NotFoundView {
                requested: format!("/{lang}/{slug}"),
            }
        },
    }
}

#[component]
fn NotFound(route: Vec<String>) -> Element {
    rsx! {
        NotFoundView {
            requested: format!("/{}", route.join("/")),
        }
    }
}

pub(crate) fn route_for(language: Language, slug: &str) -> Route {
    if slug == "home" {
        Route::LanguageHome {
            lang: language.code().to_string(),
        }
    } else if find_page(language, slug).is_some() {
        Route::Doc {
            lang: language.code().to_string(),
            slug: slug.to_string(),
        }
    } else {
        Route::LanguageHome {
            lang: language.code().to_string(),
        }
    }
}
