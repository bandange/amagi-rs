use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{XiaohongshuJsonResponse, XiaohongshuStatusResult};

/// Xiaohongshu emoji-list response.
pub type XiaohongshuEmojiList = XiaohongshuJsonResponse<XiaohongshuEmojiListData>;

/// Payload body for Xiaohongshu emoji-list responses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuEmojiListData {
    /// Emoji payload.
    pub emoji: XiaohongshuEmojiPayload,
    /// Nested status marker.
    pub result: XiaohongshuStatusResult,
    /// Emoji resource version.
    #[serde(default)]
    pub version: Option<u64>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Top-level emoji payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuEmojiPayload {
    /// Emoji tabs.
    #[serde(default)]
    pub tabs: Vec<XiaohongshuEmojiTab>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// One emoji tab.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuEmojiTab {
    /// Grouped emoji collections.
    #[serde(default)]
    pub collection: Vec<XiaohongshuEmojiCollection>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Named emoji collection.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuEmojiCollection {
    /// Emoji items in the collection.
    #[serde(default)]
    pub emoji: Vec<XiaohongshuEmojiItem>,
    /// Collection name.
    #[serde(default)]
    pub name: Option<String>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// One Xiaohongshu emoji item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuEmojiItem {
    /// Emoji image URL.
    #[serde(default)]
    pub image: Option<String>,
    /// Emoji image name.
    #[serde(default)]
    pub image_name: Option<String>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
