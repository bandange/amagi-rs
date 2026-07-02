use crate::app::route_for;
use crate::build_info::{PROJECT_LICENSE_URL, docs_source_href, docs_source_link_label};
use crate::models::Language;
use dioxus::prelude::*;

#[component]
pub(crate) fn SiteFooter(language: Language) -> Element {
    let source_href = docs_source_href();
    let source_label = docs_source_link_label(language);

    rsx! {
        footer { class: "site-footer",
            div { class: "site-footer-main",
                strong { "Amagi-rs" }
                span { "{language.version_label()} {env!(\"CARGO_PKG_VERSION\")}" }
                span { "{language.footer_note()}" }
            }
            nav { class: "site-footer-links", "aria-label": "Footer",
                Link {
                    to: route_for(language, "disclaimer"),
                    "{language.disclaimer_label()}"
                }
                a {
                    href: PROJECT_LICENSE_URL,
                    target: "_blank",
                    rel: "noreferrer",
                    "{language.license_label()}"
                }
                a {
                    href: "{source_href}",
                    target: "_blank",
                    rel: "noreferrer",
                    "{source_label}"
                }
            }
        }
    }
}
