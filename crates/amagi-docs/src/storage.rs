use crate::models::ThemePreference;

const THEME_STORAGE_KEY: &str = "amagi-docs-theme";

pub(crate) fn read_stored_theme_preference() -> ThemePreference {
    storage_get(THEME_STORAGE_KEY)
        .as_deref()
        .and_then(ThemePreference::from_storage)
        .unwrap_or(ThemePreference::System)
}

pub(crate) fn store_theme_preference(preference: ThemePreference) {
    storage_set(THEME_STORAGE_KEY, preference.storage_value());
}

#[cfg(target_arch = "wasm32")]
fn storage_get(key: &str) -> Option<String> {
    web_sys::window()?
        .local_storage()
        .ok()
        .flatten()?
        .get_item(key)
        .ok()
        .flatten()
}

#[cfg(not(target_arch = "wasm32"))]
fn storage_get(_key: &str) -> Option<String> {
    None
}

#[cfg(target_arch = "wasm32")]
fn storage_set(key: &str, value: &str) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let _ = storage.set_item(key, value);
    }
}

#[cfg(not(target_arch = "wasm32"))]
fn storage_set(_key: &str, _value: &str) {}
