use std::{borrow::Cow, collections::BTreeMap};

use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::de::DeserializeOwned;
use serde_json::{Map, Value};

use crate::error::AppError;

use super::super::types::DouyinSearchType;

pub(super) fn decode_douyin_payload<T>(value: Value) -> Result<T, AppError>
where
    T: DeserializeOwned,
{
    serde_json::from_value(inject_upstream_payload(value)).map_err(AppError::from)
}

pub(super) fn header_map_from_headers(
    headers: &BTreeMap<String, String>,
) -> Result<HeaderMap, AppError> {
    let mut header_map = HeaderMap::new();

    for (name, value) in headers {
        let header_name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| AppError::InvalidRequestConfig(format!("invalid header name `{name}`")))?;
        let header_value = HeaderValue::from_str(value).map_err(|_| {
            AppError::InvalidRequestConfig(format!("invalid header value for `{name}`"))
        })?;
        header_map.insert(header_name, header_value);
    }

    Ok(header_map)
}

pub(super) fn parse_json_payload(url: &str, body: &str) -> Result<Value, AppError> {
    let trimmed = body.trim();
    if trimmed.is_empty() {
        return Err(AppError::UpstreamResponse {
            status: None,
            message: format!("douyin request to {url} returned an empty body"),
        });
    }

    let direct_parse_error = match serde_json::from_str::<Value>(trimmed) {
        Ok(value) => {
            validate_douyin_response(url, &value)?;
            return Ok(value);
        }
        Err(error) => error,
    };

    if looks_like_html(trimmed) {
        return Err(AppError::UpstreamResponse {
            status: None,
            message: format!(
                "douyin request to {url} returned HTML instead of JSON: {}",
                html_preview(trimmed)
            ),
        });
    }

    if let Some(candidate) = extract_wrapped_json_payload(trimmed) {
        let value = serde_json::from_str::<Value>(candidate)?;
        validate_douyin_response(url, &value)?;
        return Ok(value);
    }

    Err(direct_parse_error.into())
}

pub(super) fn validate_douyin_response(url: &str, value: &Value) -> Result<(), AppError> {
    if let Some(filter_reason) = value
        .get("filter_detail")
        .and_then(|value| value.get("filter_reason"))
        .and_then(Value::as_str)
        .filter(|value| !value.is_empty())
    {
        return Err(AppError::UpstreamResponse {
            status: None,
            message: format!("douyin request to {url} was filtered: {filter_reason}"),
        });
    }

    if let Some(status_code) = value.get("status_code").and_then(Value::as_i64) {
        if status_code != 0 {
            let message = value
                .get("status_msg")
                .and_then(Value::as_str)
                .unwrap_or("unknown douyin error");
            return Err(AppError::UpstreamResponse {
                status: None,
                message: format!(
                    "douyin request to {url} failed with status_code={status_code}: {message}"
                ),
            });
        }
    }

    Ok(())
}

pub(super) fn has_more_value(value: Option<&Value>) -> bool {
    match value {
        Some(Value::Bool(value)) => *value,
        Some(Value::Number(value)) => value.as_i64().is_some_and(|value| value != 0),
        Some(Value::String(value)) => !value.is_empty() && value != "0" && value != "false",
        _ => false,
    }
}

pub(super) fn stringify_cursor(value: &Value) -> Option<String> {
    match value {
        Value::String(value) => Some(value.clone()),
        Value::Number(value) => Some(value.to_string()),
        _ => None,
    }
}

pub(super) fn extract_array_field(value: &Value, field: &str) -> Vec<Value> {
    value
        .get(field)
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default()
}

pub(super) fn set_array_field(
    value: &mut Value,
    field: &str,
    items: Vec<Value>,
    truncate: Option<usize>,
) {
    let items = truncate.map_or(items.clone(), |limit| {
        items.into_iter().take(limit).collect::<Vec<_>>()
    });
    set_field(value, field, Value::Array(items));
}

pub(super) fn set_field(value: &mut Value, field: &str, inner: Value) {
    if !value.is_object() {
        *value = Value::Object(Map::new());
    }

    if let Some(object) = value.as_object_mut() {
        object.insert(field.to_owned(), inner);
    }
}

pub(super) fn search_referer(query: &str, search_type: DouyinSearchType) -> String {
    let encoded_query = encode_search_segment(query);
    match search_type {
        DouyinSearchType::General => format!("https://www.douyin.com/root/search/{encoded_query}"),
        DouyinSearchType::User => {
            format!("https://www.douyin.com/search/{encoded_query}?type=user")
        }
        DouyinSearchType::Video => {
            format!("https://www.douyin.com/search/{encoded_query}?type=video")
        }
    }
}

pub(super) fn encode_search_segment(value: &str) -> Cow<'_, str> {
    if value
        .bytes()
        .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~'))
    {
        Cow::Borrowed(value)
    } else {
        Cow::Owned(
            value
                .bytes()
                .map(|byte| match byte {
                    b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                        (byte as char).to_string()
                    }
                    _ => format!("%{:02X}", byte),
                })
                .collect::<String>(),
        )
    }
}

pub(super) fn parse_douyin_multi_json(raw: &str) -> Vec<Value> {
    let mut blocks = Vec::new();
    let mut search_start = 0usize;

    while search_start < raw.len() {
        let rest = &raw[search_start..];
        let relative_start = rest.find(['{', '[']);
        let Some(relative_start) = relative_start else {
            break;
        };
        let block_start = search_start + relative_start;
        let Some(relative_end) = find_balanced_json_end(&raw[block_start..]) else {
            break;
        };
        let block_end = block_start + relative_end;
        let block = &raw[block_start..block_end];
        if let Ok(value) = serde_json::from_str::<Value>(block) {
            blocks.push(value);
        }
        search_start = block_end;
    }

    blocks
}

pub(super) fn filter_search_responses(values: Vec<Value>) -> Vec<Value> {
    values
        .into_iter()
        .filter(|value| {
            value.get("cursor").and_then(Value::as_i64).is_some()
                && value.get("has_more").and_then(Value::as_i64).is_some()
                && value.get("data").and_then(Value::as_array).is_some()
        })
        .collect()
}

fn looks_like_html(body: &str) -> bool {
    body.starts_with('<')
}

fn html_preview(body: &str) -> String {
    body.lines()
        .flat_map(str::split_whitespace)
        .take(12)
        .collect::<Vec<_>>()
        .join(" ")
}

fn extract_wrapped_json_payload(body: &str) -> Option<&str> {
    let start = body.find(['{', '['])?;
    let end = start + find_balanced_json_end(&body[start..])?;
    let prefix = body[..start].trim();
    let suffix = body[end..].trim();

    if is_supported_json_wrapper(prefix, suffix) {
        Some(&body[start..end])
    } else {
        None
    }
}

fn is_supported_json_wrapper(prefix: &str, suffix: &str) -> bool {
    is_supported_json_prefix(prefix) && is_supported_json_suffix(suffix)
}

fn is_supported_json_prefix(prefix: &str) -> bool {
    if prefix.is_empty() {
        return true;
    }

    if prefix.contains('<') {
        return false;
    }

    if matches!(prefix, "for(;;);" | "for(;;)" | "while(1);" | "while(1)") {
        return true;
    }

    prefix.strip_suffix('(').is_some_and(|callback| {
        let callback = callback.trim();
        !callback.is_empty()
            && callback
                .chars()
                .all(|ch| ch.is_ascii_alphanumeric() || matches!(ch, '_' | '$' | '.'))
    })
}

fn is_supported_json_suffix(suffix: &str) -> bool {
    matches!(suffix, "" | ";" | ")" | ");")
}

fn find_balanced_json_end(raw: &str) -> Option<usize> {
    let mut stack = Vec::new();
    let mut in_string = false;
    let mut escaped = false;

    for (index, ch) in raw.char_indices() {
        if in_string {
            match ch {
                '\\' if !escaped => escaped = true,
                '"' if !escaped => in_string = false,
                _ => escaped = false,
            }
            continue;
        }

        match ch {
            '"' => in_string = true,
            '{' | '[' => stack.push(ch),
            '}' => {
                if stack.pop() != Some('{') {
                    return None;
                }
                if stack.is_empty() {
                    return Some(index + ch.len_utf8());
                }
            }
            ']' => {
                if stack.pop() != Some('[') {
                    return None;
                }
                if stack.is_empty() {
                    return Some(index + ch.len_utf8());
                }
            }
            _ => {}
        }
    }

    None
}

fn inject_upstream_payload(value: Value) -> Value {
    match value {
        Value::Object(mut object) => {
            let upstream_payload = Value::Object(object.clone());
            object.insert("upstream_payload".to_owned(), upstream_payload);
            Value::Object(object)
        }
        other => other,
    }
}
