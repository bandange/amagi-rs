use std::collections::BTreeMap;

use reqwest::Url;

use crate::error::AppError;

use super::super::types::{KuaishouHxfalconPayload, KuaishouJsonObject, KuaishouJsonValue};

const SIGN_INPUT_SKIP_KEYWORD: &str = "__NS";

fn normalize_pathname(url_or_path: &str) -> String {
    Url::parse(url_or_path)
        .map(|url| url.path().to_owned())
        .unwrap_or_else(|_| {
            url_or_path
                .split('?')
                .next()
                .unwrap_or(url_or_path)
                .to_owned()
        })
}

fn sort_search_params(parsed_url: &Url) -> BTreeMap<String, String> {
    let mut entries = parsed_url
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect::<Vec<_>>();

    entries.sort_by(|(left, _), (right, _)| left.cmp(right));

    let mut result = BTreeMap::new();

    for (key, value) in entries {
        result.insert(key, value);
    }

    result
}

fn escape_json_string(value: &str) -> String {
    let mut result = String::with_capacity(value.len() + 2);

    for character in value.chars() {
        match character {
            '"' => result.push_str("\\\""),
            '\\' => result.push_str("\\\\"),
            '\u{08}' => result.push_str("\\b"),
            '\u{0C}' => result.push_str("\\f"),
            '\n' => result.push_str("\\n"),
            '\r' => result.push_str("\\r"),
            '\t' => result.push_str("\\t"),
            '\u{2028}' => result.push_str("\\u2028"),
            '\u{2029}' => result.push_str("\\u2029"),
            character if character <= '\u{1F}' => {
                result.push_str(&format!("\\u{:04x}", character as u32));
            }
            character => result.push(character),
        }
    }

    result
}

fn serialize_kuaishou_json_value(value: &KuaishouJsonValue) -> String {
    match value {
        KuaishouJsonValue::Null => "null".to_owned(),
        KuaishouJsonValue::Bool(value) => value.to_string(),
        KuaishouJsonValue::Number(value) => value.clone(),
        KuaishouJsonValue::String(value) => format!("\"{}\"", escape_json_string(value)),
        KuaishouJsonValue::Array(values) => {
            let serialized = values
                .iter()
                .map(serialize_kuaishou_json_value)
                .collect::<Vec<_>>()
                .join(",");
            format!("[{serialized}]")
        }
        KuaishouJsonValue::Object(object) => serialize_kuaishou_json_object(object),
    }
}

fn serialize_kuaishou_json_object(value: &KuaishouJsonObject) -> String {
    let serialized = value
        .entries()
        .iter()
        .map(|(key, inner)| {
            format!(
                "\"{}\":{}",
                escape_json_string(key),
                serialize_kuaishou_json_value(inner)
            )
        })
        .collect::<Vec<_>>()
        .join(",");

    format!("{{{serialized}}}")
}

/// Resolve the canonical sign path used by Kuaishou `__NS_hxfalcon`.
pub fn resolve_kuaishou_hxfalcon_sign_path(
    url_or_path: &str,
    sign_path: Option<&str>,
) -> Result<String, AppError> {
    let pathname = normalize_pathname(sign_path.unwrap_or(url_or_path));

    if pathname.starts_with('/') {
        Ok(pathname)
    } else {
        Err(AppError::InvalidRequestConfig(format!(
            "Invalid Kuaishou signing path: {pathname}"
        )))
    }
}

/// Build a normalized Kuaishou hxfalcon payload from a request URL.
pub fn build_kuaishou_hxfalcon_payload(
    url: &str,
    sign_path: Option<&str>,
) -> Result<KuaishouHxfalconPayload, AppError> {
    let parsed_url = Url::parse(url).map_err(|error| {
        AppError::InvalidRequestConfig(format!("Invalid Kuaishou signing URL {url}: {error}"))
    })?;
    let real_path = resolve_kuaishou_hxfalcon_sign_path(parsed_url.path(), sign_path)?;
    let query = sort_search_params(&parsed_url);

    if !query.contains_key("caver") {
        return Err(AppError::InvalidRequestConfig(format!(
            "Missing caver query parameter for Kuaishou signing: {url}"
        )));
    }

    Ok(KuaishouHxfalconPayload {
        url: real_path,
        query,
        form: BTreeMap::new(),
        request_body: KuaishouJsonObject::new(),
    })
}

/// Build the Kuaishou sign input string from a normalized payload.
pub fn build_kuaishou_hxfalcon_sign_input(payload: &KuaishouHxfalconPayload) -> String {
    let mut combined_params = payload.query.clone();
    combined_params.extend(payload.form.clone());

    let mut serialized_params = combined_params
        .into_iter()
        .filter(|(key, _)| !key.contains(SIGN_INPUT_SKIP_KEYWORD))
        .map(|(key, value)| format!("{key}={value}"))
        .collect::<Vec<_>>();

    serialized_params.sort();

    let request_body = if payload.request_body.is_empty() {
        String::new()
    } else {
        serialize_kuaishou_json_object(&payload.request_body)
    };

    format!(
        "{}{}{}",
        normalize_pathname(&payload.url),
        serialized_params.join(""),
        request_body
    )
}
