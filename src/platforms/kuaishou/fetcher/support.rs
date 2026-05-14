use reqwest::Url;
use serde_json::{Map, Value};

pub(super) fn empty_object() -> Value {
    Value::Object(Map::new())
}

pub(super) fn is_populated_object(value: Option<&Value>) -> bool {
    matches!(value, Some(Value::Object(map)) if !map.is_empty())
}

pub(super) fn as_object(value: Option<&Value>) -> Option<&Map<String, Value>> {
    value?.as_object()
}

pub(super) fn value_field<'a>(value: &'a Value, key: &str) -> Option<&'a Value> {
    value.get(key)
}

pub(super) fn value_from_object<'a>(
    value: Option<&'a Map<String, Value>>,
    key: &str,
) -> Option<&'a Value> {
    value?.get(key)
}

pub(super) fn string_value(value: Option<&Value>) -> Option<String> {
    match value {
        Some(Value::String(value)) => Some(value.clone()),
        Some(Value::Number(value)) => Some(value.to_string()),
        _ => None,
    }
}

pub(super) fn bool_value(value: Option<&Value>) -> Option<bool> {
    value.and_then(Value::as_bool)
}

pub(super) fn number_value(value: Option<&Value>) -> Option<u64> {
    match value {
        Some(Value::Number(number)) => number
            .as_u64()
            .or_else(|| number.as_i64().map(|value| value as u64)),
        Some(Value::String(value)) => value.parse::<u64>().ok(),
        _ => None,
    }
}

pub(super) fn i64_value(value: Option<&Value>) -> Option<i64> {
    match value {
        Some(Value::Number(number)) => number
            .as_i64()
            .or_else(|| number.as_u64().map(|value| value as i64)),
        Some(Value::String(value)) => value.parse::<i64>().ok(),
        _ => None,
    }
}

pub(super) fn array_value(value: Option<&Value>) -> Vec<Value> {
    value.and_then(Value::as_array).cloned().unwrap_or_default()
}

pub(super) fn object_or_empty(value: Option<&Value>) -> Value {
    value
        .and_then(Value::as_object)
        .cloned()
        .map(Value::Object)
        .unwrap_or_else(empty_object)
}

pub(super) fn pick_first_non_empty_string(values: &[Option<String>]) -> String {
    values
        .iter()
        .flatten()
        .find(|value| !value.trim().is_empty())
        .cloned()
        .unwrap_or_default()
}

pub(super) fn normalize_kuaishou_hls_play_url(url: &str) -> String {
    let Ok(mut parsed) = Url::parse(url) else {
        return url.to_owned();
    };

    let original_pairs = parsed
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect::<Vec<_>>();
    let has_tsc_param = original_pairs.iter().any(|(key, _)| key == "tsc");
    let mut normalized_pairs = Vec::with_capacity(original_pairs.len() + 1);
    let mut changed = false;

    for (key, value) in original_pairs {
        if key == "sidc" {
            if let Some((sidc_value, suffix_value)) = value.split_once("tsc=") {
                let normalized_sidc = sidc_value.trim_end_matches(['&', '?']);
                normalized_pairs.push((key, normalized_sidc.to_owned()));

                if !has_tsc_param && !suffix_value.trim().is_empty() {
                    normalized_pairs.push(("tsc".to_owned(), suffix_value.to_owned()));
                }

                changed = true;
                continue;
            }
        }

        normalized_pairs.push((key, value));
    }

    if !changed {
        return url.to_owned();
    }

    parsed.set_query(None);
    {
        let mut query_pairs = parsed.query_pairs_mut();
        for (key, value) in normalized_pairs {
            query_pairs.append_pair(&key, &value);
        }
    }

    parsed.to_string()
}
