use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::{
    common::{XiaohongshuImageAsset, XiaohongshuInteractInfo, XiaohongshuJsonResponse},
    feed::XiaohongshuFeedNoteCard,
};

/// Supported Xiaohongshu search sort orders.
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(rename_all = "snake_case")]
pub enum XiaohongshuSearchSortType {
    /// Default comprehensive ranking.
    #[default]
    General,
    /// Most recent first.
    TimeDescending,
    /// Most popular first.
    PopularityDescending,
}

impl XiaohongshuSearchSortType {
    /// Return the upstream string value expected by the Xiaohongshu API.
    pub fn as_api_str(self) -> &'static str {
        match self {
            Self::General => "general",
            Self::TimeDescending => "time_descending",
            Self::PopularityDescending => "popularity_descending",
        }
    }
}

/// Supported Xiaohongshu search note filters.
#[cfg_attr(feature = "cli", derive(clap::ValueEnum))]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
#[repr(u8)]
pub enum XiaohongshuSearchNoteType {
    /// Search all note types.
    #[default]
    All = 0,
    /// Search only videos.
    Video = 1,
    /// Search only image notes.
    Image = 2,
}

impl XiaohongshuSearchNoteType {
    /// Return the upstream integer value expected by the Xiaohongshu API.
    pub fn as_api_value(self) -> u8 {
        self as u8
    }
}

/// Xiaohongshu note-search response.
pub type XiaohongshuSearchNotes = XiaohongshuJsonResponse<XiaohongshuSearchNotesData>;

/// Payload body for Xiaohongshu note-search responses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuSearchNotesData {
    /// Whether more results are available.
    #[serde(default)]
    pub has_more: Option<bool>,
    /// Result items returned by the current page.
    #[serde(default)]
    pub items: Vec<XiaohongshuSearchItem>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// One Xiaohongshu search result item.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuSearchItem {
    /// Result id.
    pub id: String,
    /// Upstream model type.
    #[serde(default)]
    pub model_type: Option<String>,
    /// Embedded note card.
    #[serde(default)]
    pub note_card: Option<XiaohongshuSearchNoteCard>,
    /// Upstream recommendation query metadata.
    #[serde(default)]
    pub rec_query: Option<Value>,
    /// Optional xsec token carried by the result item.
    #[serde(default)]
    pub xsec_token: Option<String>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Note card embedded in search results.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuSearchNoteCard {
    /// Reuse shared feed card fields.
    #[serde(flatten)]
    pub base: XiaohongshuFeedNoteCard,
    /// Search result image list.
    #[serde(default)]
    pub image_list: Vec<XiaohongshuImageAsset>,
    /// Search result interaction stats.
    #[serde(default)]
    pub interact_info: Option<XiaohongshuInteractInfo>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
