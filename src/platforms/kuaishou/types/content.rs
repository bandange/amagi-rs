use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Typed response for the Kuaishou `emojiList` fetcher.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouEmojiList {
    /// GraphQL response payload returned by the platform.
    pub data: KuaishouEmojiListData,
    /// Complete upstream payload snapshot with the GraphQL envelope removed.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
}

/// Data section returned by the Kuaishou emoji GraphQL query.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KuaishouEmojiListData {
    /// Base emoji catalog published by Kuaishou.
    #[serde(rename = "visionBaseEmoticons")]
    pub vision_base_emoticons: KuaishouEmojiCatalog,
}

/// Emoji icon catalog keyed by the platform's emoji token.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KuaishouEmojiCatalog {
    /// GraphQL typename returned by Kuaishou.
    #[serde(rename = "__typename")]
    pub type_name: String,
    /// Mapping from the emoji token such as `[smile]` to the icon URL.
    #[serde(rename = "iconUrls")]
    pub icon_urls: BTreeMap<String, String>,
}

/// Typed response for the Kuaishou `videoWork` fetcher.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouVideoWork {
    /// GraphQL response payload returned by the platform.
    pub data: KuaishouVideoWorkData,
    /// Complete upstream payload snapshot with the GraphQL envelope removed.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
}

/// Data section returned by the Kuaishou video-work GraphQL query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouVideoWorkData {
    /// Normalized work detail block.
    #[serde(rename = "visionVideoDetail")]
    pub vision_video_detail: Value,
}

/// Typed response for the Kuaishou `comments` fetcher.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouWorkComments {
    /// GraphQL response payload returned by the platform.
    pub data: KuaishouWorkCommentsData,
    /// Complete upstream payload snapshot with the GraphQL envelope removed.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
}

/// Data section returned by the Kuaishou comments GraphQL query.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouWorkCommentsData {
    /// Normalized comment-list block.
    #[serde(rename = "visionCommentList")]
    pub vision_comment_list: Value,
}

/// Paginated user work-list result derived from Kuaishou `profile/public`.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouUserWorkList {
    /// Requested profile principal id.
    #[serde(rename = "principalId")]
    pub principal_id: String,
    /// Work items for the current page.
    pub list: Vec<Value>,
    /// Pagination cursor.
    pub pcursor: String,
    /// Whether more pages are available.
    #[serde(rename = "hasMore")]
    pub has_more: bool,
    /// Upstream result code.
    pub result: i64,
    /// Complete upstream payload snapshot with the live API envelope removed.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
}
