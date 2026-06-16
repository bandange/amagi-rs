use serde_json::Value;

use serde::{Deserialize, Serialize};

/// One parsed Bilibili danmaku element from `DmSegMobileReply`.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BilibiliDanmakuElem {
    /// Unique danmaku identifier.
    pub id: String,
    /// Display timestamp in milliseconds.
    pub progress: i32,
    /// Rendering mode used by the player.
    pub mode: i32,
    /// Font size of the danmaku text.
    pub fontsize: i32,
    /// RGB color encoded as an integer.
    pub color: u32,
    /// Hash of the sender uid.
    #[serde(rename = "midHash")]
    pub mid_hash: String,
    /// Plain-text danmaku content.
    pub content: String,
    /// Creation timestamp returned by upstream.
    pub ctime: String,
    /// Weight used by Bilibili ranking logic.
    pub weight: i32,
    /// Optional action metadata.
    pub action: String,
    /// Danmaku pool identifier.
    pub pool: i32,
    /// String representation of the danmaku id.
    #[serde(rename = "idStr")]
    pub id_str: String,
    /// Bitflag attributes attached to the danmaku.
    pub attr: i32,
    /// Optional animation metadata.
    pub animation: String,
}

/// Parsed danmaku data payload returned by the Rust fetcher.
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct BilibiliDanmakuData {
    /// Parsed danmaku elements from one segment.
    #[serde(default)]
    pub elems: Vec<BilibiliDanmakuElem>,
}

/// Rust-native response shape for Bilibili `videoDanmaku`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BilibiliDanmakuList {
    /// Synthetic success code matching other Bilibili responses.
    pub code: i64,
    /// Synthetic status message for decoded danmaku payloads.
    pub message: String,
    /// Optional TTL value kept for response-shape compatibility.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ttl: Option<i64>,
    /// Decoded danmaku data payload.
    pub data: BilibiliDanmakuData,
    /// Complete upstream payload snapshot for the decoded danmaku segment.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
}
