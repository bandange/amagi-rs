use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{
    XiaohongshuImageAsset, XiaohongshuInteractInfo, XiaohongshuJsonResponse, XiaohongshuUserSummary,
};

/// Xiaohongshu home-feed response.
pub type XiaohongshuHomeFeed = XiaohongshuJsonResponse<XiaohongshuHomeFeedData>;

/// Payload body for Xiaohongshu home-feed responses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuHomeFeedData {
    /// Upstream cursor score for pagination.
    pub cursor_score: String,
    /// Feed items returned by the current request.
    #[serde(default)]
    pub items: Vec<XiaohongshuFeedItem>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// One Xiaohongshu feed item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuFeedItem {
    /// Feed item id.
    pub id: String,
    /// Whether the item should be ignored by the client.
    #[serde(default)]
    pub ignore: Option<bool>,
    /// Upstream model type.
    #[serde(default)]
    pub model_type: Option<String>,
    /// Embedded note card.
    #[serde(default)]
    pub note_card: Option<XiaohongshuFeedNoteCard>,
    /// Track id emitted by the upstream feed API.
    #[serde(default)]
    pub track_id: Option<String>,
    /// Optional xsec token carried with the feed item.
    #[serde(default)]
    pub xsec_token: Option<String>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Embedded note card returned inside feed and search payloads.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuFeedNoteCard {
    /// Optional cover asset.
    #[serde(default)]
    pub cover: Option<XiaohongshuImageAsset>,
    /// Display title shown in feed cards.
    #[serde(default)]
    pub display_title: Option<String>,
    /// Interaction stats.
    #[serde(default)]
    pub interact_info: Option<XiaohongshuInteractInfo>,
    /// Upstream note type marker.
    #[serde(default)]
    pub r#type: Option<String>,
    /// Embedded author summary.
    #[serde(default)]
    pub user: Option<XiaohongshuUserSummary>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
