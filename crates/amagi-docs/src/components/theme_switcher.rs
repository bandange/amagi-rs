use crate::models::ThemePreference;
use crate::storage::store_theme_preference;
use dioxus::prelude::*;

#[component]
pub(crate) fn ThemeSwitcher() -> Element {
    let mut theme_preference = use_context::<Signal<ThemePreference>>();
    let current = theme_preference();
    let next = current.next();

    rsx! {
        button {
            class: "theme-toggle",
            r#type: "button",
            title: format!("Theme: {}. Switch to {}.", current.label(), next.label()),
            "aria-label": format!("Theme: {}. Switch to {}.", current.label(), next.label()),
            onclick: move |_| {
                theme_preference.set(next);
                store_theme_preference(next);
            },
            span { class: "{current.icon_class()}" }
        }
    }
}
