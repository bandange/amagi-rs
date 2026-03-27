use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::common::{XiaohongshuJsonResponse, XiaohongshuStatusResult};

/// Xiaohongshu user-profile response.
pub type XiaohongshuUserProfile = XiaohongshuJsonResponse<XiaohongshuUserProfileData>;
/// Xiaohongshu user-note-list response.
pub type XiaohongshuUserNoteList = XiaohongshuJsonResponse<Value>;

/// Payload body for Xiaohongshu user-profile responses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuUserProfileData {
    /// Basic user info.
    #[serde(rename = "basicInfo")]
    pub basic_info: XiaohongshuUserProfileBasicInfo,
    /// Extra user info.
    #[serde(default, rename = "extraInfo")]
    pub extra_info: Option<Value>,
    /// Interaction counters shown on the profile page.
    #[serde(default)]
    pub interactions: Vec<Value>,
    /// Nested result marker.
    pub result: XiaohongshuStatusResult,
    /// Public tab state.
    #[serde(default, rename = "tabPublic")]
    pub tab_public: Option<Value>,
    /// Public tags.
    #[serde(default)]
    pub tags: Vec<Value>,
    /// Verification info.
    #[serde(default, rename = "verifyInfo")]
    pub verify_info: Option<Value>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Basic user info returned by the Xiaohongshu user-profile API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuUserProfileBasicInfo {
    /// User description.
    #[serde(default)]
    pub desc: Option<String>,
    /// Gender marker.
    #[serde(default)]
    pub gender: Option<i64>,
    /// Large avatar image.
    #[serde(default)]
    pub imageb: Option<String>,
    /// Standard avatar image.
    #[serde(default)]
    pub images: Option<String>,
    /// IP location label.
    #[serde(default, rename = "ipLocation")]
    pub ip_location: Option<String>,
    /// Profile nickname.
    pub nickname: String,
    /// Xiaohongshu red id.
    #[serde(default, rename = "redId")]
    pub red_id: Option<String>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}
