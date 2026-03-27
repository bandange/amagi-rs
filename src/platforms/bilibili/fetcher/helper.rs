use std::collections::BTreeMap;

use reqwest::header::HeaderMap;
use serde_json::Value;

use crate::error::AppError;

pub(super) fn ensure_bilibili_success(url: &str, value: &Value) -> Result<(), AppError> {
    let code = value
        .get("code")
        .and_then(Value::as_i64)
        .unwrap_or_default();

    if code == 0 {
        return Ok(());
    }

    let message = value
        .get("message")
        .or_else(|| value.get("msg"))
        .and_then(Value::as_str)
        .unwrap_or("unknown bilibili upstream error");

    Err(AppError::UpstreamResponse {
        status: None,
        message: format!("bilibili api returned code {code} for {url}: {message}"),
    })
}

pub(super) fn dedupe_comment_replies(items: Vec<Value>) -> Vec<Value> {
    let mut seen = std::collections::BTreeSet::new();
    let mut deduped = Vec::new();

    for item in items {
        let key = item
            .get("rpid")
            .and_then(|value| match value {
                Value::String(value) => Some(value.clone()),
                Value::Number(value) => Some(value.to_string()),
                _ => None,
            })
            .unwrap_or_else(|| serde_json::to_string(&item).unwrap_or_default());

        if seen.insert(key) {
            deduped.push(item);
        }
    }

    deduped
}

pub(super) fn set_json_array_field(
    root: &mut Value,
    path: &[&str],
    values: Vec<Value>,
    limit: Option<usize>,
) {
    let Some((last, parents)) = path.split_last() else {
        return;
    };
    let mut current = root;

    for segment in parents {
        let Some(next) = current.get_mut(*segment) else {
            return;
        };
        current = next;
    }

    if let Some(object) = current.as_object_mut() {
        let values = match limit {
            Some(limit) => values.into_iter().take(limit).collect(),
            None => values,
        };
        object.insert((*last).to_owned(), Value::Array(values));
    }
}

pub(super) fn flatten_headers(headers: &HeaderMap) -> BTreeMap<String, String> {
    let mut flattened = BTreeMap::new();

    for name in headers.keys() {
        let values = headers
            .get_all(name)
            .iter()
            .filter_map(|value| value.to_str().ok())
            .collect::<Vec<_>>();

        if !values.is_empty() {
            flattened.insert(name.as_str().to_owned(), values.join(", "));
        }
    }

    flattened
}

pub(super) fn append_query_segments(base_url: &str, segments: &[String]) -> String {
    let suffix = segments
        .iter()
        .filter(|segment| !segment.is_empty())
        .map(String::as_str)
        .collect::<Vec<_>>()
        .join("&");

    if suffix.is_empty() {
        return base_url.to_owned();
    }

    let separator = if base_url.contains('?') { '&' } else { '?' };
    format!("{base_url}{separator}{suffix}")
}

pub(super) fn extract_signature_suffix_from_query(query: &str) -> String {
    query
        .split('&')
        .filter(|segment| segment.starts_with("wts=") || segment.starts_with("w_rid="))
        .collect::<Vec<_>>()
        .join("&")
}
