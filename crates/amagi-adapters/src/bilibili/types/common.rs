use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Shared JSON response envelope returned by many Bilibili web APIs.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BilibiliJsonResponse {
    /// Upstream status code returned by Bilibili.
    pub code: i64,
    /// Human-readable upstream message.
    pub message: String,
    /// Optional TTL field returned by some endpoints.
    pub ttl: Option<i64>,
    /// Endpoint-specific payload body.
    pub data: Value,
    /// Complete upstream payload snapshot with the common envelope removed.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
}
