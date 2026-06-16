use super::platform_api_spec;
use amagi_core::{ApiOperationSpec, Platform};

/// Return every published operation for a platform adapter.
pub fn operation_specs(platform: Platform) -> &'static [ApiOperationSpec] {
    platform_api_spec(platform).methods
}

/// Find an operation by its stable operation key.
pub fn find_operation(
    platform: Platform,
    operation_key: &str,
) -> Option<&'static ApiOperationSpec> {
    operation_specs(platform)
        .iter()
        .find(|spec| spec.method_key == operation_key)
}

/// Return the English fetcher name for a Chinese operation label.
pub fn get_fetcher_name(platform: Platform, chinese_operation: &str) -> Option<&'static str> {
    operation_specs(platform)
        .iter()
        .find(|spec| spec.chinese_name == chinese_operation)
        .map(|spec| spec.fetcher_name)
}

/// Return the Chinese operation label for an English fetcher name.
pub fn get_chinese_operation_name(platform: Platform, fetcher_name: &str) -> Option<&'static str> {
    operation_specs(platform)
        .iter()
        .find(|spec| spec.fetcher_name == fetcher_name)
        .map(|spec| spec.chinese_name)
}

/// Return the published API route for a stable operation key.
pub fn get_operation_route(platform: Platform, operation_key: &str) -> Option<&'static str> {
    find_operation(platform, operation_key).map(|spec| spec.route)
}

/// Compatibility wrapper for the former method-oriented name.
pub fn method_specs(platform: Platform) -> &'static [ApiOperationSpec] {
    operation_specs(platform)
}

/// Compatibility wrapper for the former method-oriented name.
pub fn find_method(platform: Platform, method_key: &str) -> Option<&'static ApiOperationSpec> {
    find_operation(platform, method_key)
}

/// Compatibility wrapper for the former method-oriented name.
pub fn get_english_method_name(platform: Platform, chinese_method: &str) -> Option<&'static str> {
    get_fetcher_name(platform, chinese_method)
}

/// Compatibility wrapper for the former method-oriented name.
pub fn get_chinese_method_name(platform: Platform, english_method: &str) -> Option<&'static str> {
    get_chinese_operation_name(platform, english_method)
}

/// Compatibility wrapper for the former method-oriented name.
pub fn get_api_route(platform: Platform, method_key: &str) -> Option<&'static str> {
    get_operation_route(platform, method_key)
}
