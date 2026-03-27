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
