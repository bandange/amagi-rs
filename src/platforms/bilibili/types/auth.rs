use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::BilibiliJsonResponse;

/// Typed response for the Bilibili `loginStatus` fetcher.
pub type BilibiliLoginStatus = BilibiliJsonResponse;

/// Typed response for the Bilibili `loginQrcode` fetcher.
pub type BilibiliLoginQrcode = BilibiliJsonResponse;

/// Wrapped QR-code poll payload preserving upstream headers.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BilibiliQrcodeStatus {
    /// Upstream status code returned by Bilibili.
    pub code: i64,
    /// Human-readable upstream message.
    pub message: String,
    /// QR-code status body plus selected response headers.
    pub data: BilibiliQrcodeStatusData,
    /// Complete upstream payload snapshot with the common envelope removed.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
}

/// Response body for a QR-code status poll.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BilibiliQrcodeStatusData {
    /// Original upstream `data` payload.
    pub data: Value,
    /// Flattened response headers returned by the upstream poll endpoint.
    pub headers: BTreeMap<String, String>,
}
