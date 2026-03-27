use crate::error::AppError;

use super::super::ordered_json::OrderedJson;

pub(crate) fn extract_api_path(uri_with_data: &str) -> &str {
    let brace_pos = uri_with_data.find('{');
    let question_pos = uri_with_data.find('?');

    match (brace_pos, question_pos) {
        (Some(left), Some(right)) => &uri_with_data[..left.min(right)],
        (Some(left), None) => &uri_with_data[..left],
        (None, Some(right)) => &uri_with_data[..right],
        (None, None) => uri_with_data,
    }
}

pub(crate) fn build_url(base_url: &str, params: Option<&OrderedJson>) -> Result<String, AppError> {
    if base_url.trim().is_empty() {
        return Err(AppError::InvalidRequestConfig(
            "baseUrl must be a non-empty string".to_owned(),
        ));
    }

    let Some(OrderedJson::Object(entries)) = params else {
        return Ok(base_url.to_owned());
    };

    if entries.is_empty() {
        return Ok(base_url.to_owned());
    }

    let mut parts = Vec::with_capacity(entries.len());
    for (key, value) in entries {
        let formatted = value.to_query_value()?.replace('=', "%3D");
        parts.push(format!("{key}={formatted}"));
    }

    let query_string = parts.join("&");
    let separator = if !base_url.contains('?') {
        "?"
    } else if base_url.ends_with('?') || base_url.ends_with('&') {
        ""
    } else {
        "&"
    };
    Ok(format!("{base_url}{separator}{query_string}"))
}
