use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::DouyinExtraFields;

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinLogPb {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub impr_id: Option<String>,
    #[serde(flatten)]
    pub extra_fields: DouyinExtraFields,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinResponseMeta {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_code: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status_msg: Option<String>,
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub extra: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub log_pb: Option<DouyinLogPb>,
}

#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct DouyinRawPayload {
    #[serde(flatten)]
    pub meta: DouyinResponseMeta,
    #[serde(flatten)]
    pub payload: DouyinExtraFields,
}

impl DouyinRawPayload {
    pub fn get(&self, key: &str) -> Option<&Value> {
        self.payload.get(key)
    }

    pub fn into_payload(self) -> DouyinExtraFields {
        self.payload
    }
}
