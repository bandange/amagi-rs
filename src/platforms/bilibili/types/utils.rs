use serde_json::Value;

use serde::{Deserialize, Serialize};

/// Typed response for the Bilibili `avToBv` fetcher.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BilibiliAvToBv {
    /// Stable success code emitted by the local converter.
    pub code: i64,
    /// Human-readable result message.
    pub message: String,
    /// Converted identifier payload.
    pub data: BilibiliAvToBvData,
    /// Complete converted payload snapshot with the synthetic envelope removed.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
}

/// Conversion payload for one `av -> BV` operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BilibiliAvToBvData {
    /// Converted BV identifier.
    pub bvid: String,
}

/// Typed response for the Bilibili `bvToAv` fetcher.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BilibiliBvToAv {
    /// Stable success code emitted by the local converter.
    pub code: i64,
    /// Human-readable result message.
    pub message: String,
    /// Converted identifier payload.
    pub data: BilibiliBvToAvData,
    /// Complete converted payload snapshot with the synthetic envelope removed.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
}

/// Conversion payload for one `BV -> av` operation.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct BilibiliBvToAvData {
    /// Converted AV identifier with the `av` prefix retained for compatibility.
    pub aid: String,
}
