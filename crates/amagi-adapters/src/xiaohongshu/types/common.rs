use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

/// Shared Xiaohongshu JSON response envelope.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuJsonResponse<T> {
    /// Upstream status code.
    pub code: i64,
    /// Endpoint-specific payload body.
    pub data: T,
    /// Complete upstream payload snapshot with the common envelope removed.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
    /// Human-readable upstream message.
    pub msg: String,
    /// Optional success marker returned by some endpoints.
    #[serde(default)]
    pub success: Option<bool>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Nested Xiaohongshu result marker used by profile and emoji payloads.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuStatusResult {
    /// Nested status code.
    pub code: i64,
    /// Nested message.
    pub message: String,
    /// Nested success marker.
    pub success: bool,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Shared Xiaohongshu image variant.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuImageInfo {
    /// Upstream image scene label.
    pub image_scene: String,
    /// Resolved image URL.
    pub url: String,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Shared Xiaohongshu image asset.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuImageAsset {
    /// Optional file id.
    #[serde(default)]
    pub file_id: Option<String>,
    /// Image height.
    pub height: u64,
    /// Image variants.
    #[serde(default)]
    pub info_list: Vec<XiaohongshuImageInfo>,
    /// Default resolved URL.
    #[serde(default)]
    pub url: Option<String>,
    /// Alternate default URL.
    #[serde(default)]
    pub url_default: Option<String>,
    /// Preload URL.
    #[serde(default)]
    pub url_pre: Option<String>,
    /// Image width.
    pub width: u64,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Shared Xiaohongshu interaction stats.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct XiaohongshuInteractInfo {
    /// Whether the current viewer has liked the note.
    #[serde(default)]
    pub liked: Option<bool>,
    /// Like count as returned by the upstream API.
    #[serde(default)]
    pub liked_count: Option<String>,
    /// Whether the current viewer has collected the note.
    #[serde(default)]
    pub collected: Option<bool>,
    /// Collection count.
    #[serde(default)]
    pub collected_count: Option<String>,
    /// Comment count.
    #[serde(default)]
    pub comment_count: Option<String>,
    /// Share count.
    #[serde(default)]
    pub share_count: Option<String>,
    /// Follow relationship marker.
    #[serde(default)]
    pub followed: Option<bool>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Shared Xiaohongshu user summary embedded in notes and search results.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct XiaohongshuUserSummary {
    /// Avatar URL.
    #[serde(default)]
    pub avatar: Option<String>,
    /// Preferred nickname field.
    #[serde(default)]
    pub nickname: Option<String>,
    /// User id.
    pub user_id: String,
    /// Optional `xsec_token`.
    #[serde(default)]
    pub xsec_token: Option<String>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize)]
struct XiaohongshuInteractInfoRaw {
    #[serde(default)]
    liked: Option<bool>,
    #[serde(default)]
    liked_count: Option<String>,
    #[serde(default)]
    collected: Option<bool>,
    #[serde(default)]
    collected_count: Option<String>,
    #[serde(default)]
    comment_count: Option<String>,
    #[serde(default)]
    share_count: Option<String>,
    #[serde(default)]
    shared_count: Option<String>,
    #[serde(default)]
    followed: Option<bool>,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

#[derive(Debug, Deserialize)]
struct XiaohongshuUserSummaryRaw {
    #[serde(default)]
    avatar: Option<String>,
    #[serde(default)]
    image: Option<String>,
    #[serde(default)]
    nickname: Option<String>,
    #[serde(default, rename = "nick_name")]
    nick_name: Option<String>,
    user_id: String,
    #[serde(default)]
    xsec_token: Option<String>,
    #[serde(flatten)]
    extra: BTreeMap<String, Value>,
}

impl<'de> Deserialize<'de> for XiaohongshuInteractInfo {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = XiaohongshuInteractInfoRaw::deserialize(deserializer)?;
        Ok(Self {
            liked: raw.liked,
            liked_count: raw.liked_count,
            collected: raw.collected,
            collected_count: raw.collected_count,
            comment_count: raw.comment_count,
            share_count: first_non_empty(raw.share_count, raw.shared_count),
            followed: raw.followed,
            extra: raw.extra,
        })
    }
}

impl<'de> Deserialize<'de> for XiaohongshuUserSummary {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = XiaohongshuUserSummaryRaw::deserialize(deserializer)?;
        Ok(Self {
            avatar: first_non_empty(raw.avatar, raw.image),
            nickname: first_non_empty(raw.nickname, raw.nick_name),
            user_id: raw.user_id,
            xsec_token: raw.xsec_token,
            extra: raw.extra,
        })
    }
}

fn first_non_empty(primary: Option<String>, fallback: Option<String>) -> Option<String> {
    primary
        .filter(|value| !value.is_empty())
        .or_else(|| fallback.filter(|value| !value.is_empty()))
}
