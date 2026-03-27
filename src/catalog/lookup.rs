use super::catalog::platform_spec;
use super::{ApiMethodSpec, Platform};

/// Return every published method for a platform.
pub fn method_specs(platform: Platform) -> &'static [ApiMethodSpec] {
    platform_spec(platform).methods
}

/// Find a method by its stable method key.
pub fn find_method(platform: Platform, method_key: &str) -> Option<&'static ApiMethodSpec> {
    method_specs(platform)
        .iter()
        .find(|spec| spec.method_key == method_key)
}

/// Return the English fetcher name for a Chinese method label.
pub fn get_english_method_name(platform: Platform, chinese_method: &str) -> Option<&'static str> {
    method_specs(platform)
        .iter()
        .find(|spec| spec.chinese_name == chinese_method)
        .map(|spec| spec.fetcher_name)
}

/// Return the Chinese method label for an English fetcher name.
pub fn get_chinese_method_name(platform: Platform, english_method: &str) -> Option<&'static str> {
    method_specs(platform)
        .iter()
        .find(|spec| spec.fetcher_name == english_method)
        .map(|spec| spec.chinese_name)
}

/// Return the published API route for a stable method key.
pub fn get_api_route(platform: Platform, method_key: &str) -> Option<&'static str> {
    find_method(platform, method_key).map(|spec| spec.route)
}
