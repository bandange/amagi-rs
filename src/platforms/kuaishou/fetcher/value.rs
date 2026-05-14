use serde_json::{Map, Value};

use crate::platforms::kuaishou::{KuaishouJsonObject, KuaishouJsonValue, KuaishouUserWorkList};

use super::support::normalize_kuaishou_hls_urls_in_value;

pub(super) fn json_value_to_value(value: &KuaishouJsonValue) -> Value {
    match value {
        KuaishouJsonValue::Null => Value::Null,
        KuaishouJsonValue::Bool(value) => Value::Bool(*value),
        KuaishouJsonValue::Number(value) => value
            .parse::<serde_json::Number>()
            .map(Value::Number)
            .unwrap_or_else(|_| Value::String(value.clone())),
        KuaishouJsonValue::String(value) => Value::String(value.clone()),
        KuaishouJsonValue::Array(values) => {
            Value::Array(values.iter().map(json_value_to_value).collect())
        }
        KuaishouJsonValue::Object(value) => json_object_to_value(value),
    }
}

pub(super) fn json_object_to_value(value: &KuaishouJsonObject) -> Value {
    let mut object = Map::new();

    for (key, inner) in value.entries() {
        object.insert(key.clone(), json_value_to_value(inner));
    }

    Value::Object(object)
}

pub(super) fn resolve_kuaishou_user_work_list(
    principal_id: &str,
    payload: &Value,
) -> KuaishouUserWorkList {
    let data = payload.get("data").and_then(Value::as_object);
    let list = data
        .and_then(|value| value.get("list"))
        .or_else(|| payload.get("feeds"))
        .and_then(Value::as_array)
        .cloned()
        .unwrap_or_default();
    let pcursor = data
        .and_then(|value| value.get("pcursor"))
        .or_else(|| payload.get("pcursor"))
        .and_then(Value::as_str)
        .unwrap_or_default()
        .to_owned();
    let has_more = data
        .and_then(|value| value.get("hasMore"))
        .or_else(|| payload.get("hasMore"))
        .and_then(Value::as_bool)
        .unwrap_or_else(|| !pcursor.is_empty() && pcursor != "no_more");
    let result = data
        .and_then(|value| value.get("result"))
        .or_else(|| payload.get("result"))
        .and_then(Value::as_i64)
        .unwrap_or(0);

    KuaishouUserWorkList {
        principal_id: principal_id.to_owned(),
        list,
        pcursor,
        has_more,
        result,
        upstream_payload: unwrap_data_payload(payload),
    }
}

pub(super) fn unwrap_data_payload(payload: &Value) -> Value {
    let mut value = payload
        .get("data")
        .cloned()
        .unwrap_or_else(|| payload.clone());
    normalize_kuaishou_hls_urls_in_value(&mut value);
    value
}
