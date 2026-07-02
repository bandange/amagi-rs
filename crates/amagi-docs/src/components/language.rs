use crate::app::route_for;
use crate::models::{LANGUAGES, Language};
use dioxus::prelude::*;

#[component]
pub(crate) fn LanguageDropdown(language: Language, current_slug: &'static str) -> Element {
    rsx! {
        details { class: "language-menu",
            summary {
                class: "language-menu-button",
                "aria-label": "{language.language_control_label()}",
                span { class: "language-menu-current", "{language.label()}" }
                span { class: "language-menu-chevron", "aria-hidden": "true" }
            }
            div { class: "language-menu-list", role: "listbox",
                for option_language in LANGUAGES {
                    Link {
                        key: "{option_language.code()}",
                        class: if *option_language == language {
                            "language-menu-option selected"
                        } else {
                            "language-menu-option"
                        },
                        to: route_for(*option_language, current_slug),
                        role: "option",
                        "aria-selected": "{*option_language == language}",
                        span { "{option_language.label()}" }
                    }
                }
            }
        }
    }
}
