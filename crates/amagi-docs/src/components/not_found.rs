use crate::app::Route;
use crate::models::Language;
use dioxus::prelude::*;

#[component]
pub(crate) fn NotFoundView(requested: String) -> Element {
    rsx! {
        div { class: "not-found-shell",
            div { class: "not-found-panel",
                span { class: "brand-mark", "A" }
                p { class: "sidebar-kicker", "404" }
                h1 { "Document not found" }
                p { "No Amagi-rs document is registered for {requested}." }
                div { class: "not-found-actions",
                    Link {
                        class: "source-button primary",
                        to: Route::LanguageHome {
                            lang: Language::Chinese.code().to_string(),
                        },
                        "中文文档"
                    }
                    Link {
                        class: "source-button",
                        to: Route::LanguageHome {
                            lang: Language::English.code().to_string(),
                        },
                        "English Docs"
                    }
                }
            }
        }
    }
}
