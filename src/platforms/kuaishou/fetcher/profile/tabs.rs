use serde_json::Value;

use super::super::{
    super::types::{KuaishouUserProfilePublicTabData, KuaishouUserProfileTabData},
    support::{array_value, i64_value, string_value, value_field},
};

pub(super) fn resolve_tab_data(
    payload: Option<&Value>,
    fallback: &KuaishouUserProfileTabData,
) -> KuaishouUserProfileTabData {
    let Some(data) = payload
        .and_then(|value| value_field(value, "data"))
        .and_then(Value::as_object)
    else {
        return fallback.clone();
    };

    if i64_value(data.get("result")).unwrap_or_default() != 1
        || !matches!(data.get("list"), Some(Value::Array(_)))
    {
        return fallback.clone();
    }

    KuaishouUserProfileTabData {
        list: array_value(data.get("list")),
        pcursor: string_value(data.get("pcursor")).unwrap_or_else(|| fallback.pcursor.clone()),
    }
}

pub(super) fn resolve_public_tab_data(
    payload: Option<&Value>,
    fallback: &KuaishouUserProfilePublicTabData,
) -> KuaishouUserProfilePublicTabData {
    let Some(data) = payload
        .and_then(|value| value_field(value, "data"))
        .and_then(Value::as_object)
    else {
        return fallback.clone();
    };

    if i64_value(data.get("result")).unwrap_or_default() != 1
        || !matches!(data.get("list"), Some(Value::Array(_)))
    {
        return fallback.clone();
    }

    KuaishouUserProfilePublicTabData {
        live: data.get("live").cloned().or_else(|| fallback.live.clone()),
        list: array_value(data.get("list")),
        pcursor: string_value(data.get("pcursor")).unwrap_or_else(|| fallback.pcursor.clone()),
    }
}

pub(super) fn resolve_public_show_playback(payload: Option<&Value>) -> Option<bool> {
    payload
        .and_then(|value| value_field(value, "data"))
        .and_then(|value| value_field(value, "showPlayback"))
        .and_then(Value::as_bool)
}
