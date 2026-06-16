use std::collections::BTreeMap;

use serde::{Deserialize, Deserializer, Serialize};
use serde_json::Value;

use super::common::{
    XiaohongshuImageAsset, XiaohongshuInteractInfo, XiaohongshuJsonResponse, XiaohongshuUserSummary,
};

/// Xiaohongshu note-detail response.
pub type XiaohongshuNoteDetail = XiaohongshuJsonResponse<XiaohongshuNoteDetailData>;
/// Xiaohongshu note-comments response.
pub type XiaohongshuNoteComments = XiaohongshuJsonResponse<XiaohongshuNoteCommentsData>;

/// Payload body for note-detail responses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuNoteDetailData {
    /// Server timestamp.
    #[serde(default)]
    pub current_time: Option<u64>,
    /// Cursor score emitted by the note feed endpoint.
    #[serde(default)]
    pub cursor_score: Option<String>,
    /// Wrapped note items.
    #[serde(default)]
    pub items: Vec<XiaohongshuNoteItem>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// One wrapped note item returned by the feed endpoint.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuNoteItem {
    /// Optional note id.
    #[serde(default)]
    pub id: Option<String>,
    /// Upstream model type.
    #[serde(default)]
    pub model_type: Option<String>,
    /// Embedded note card.
    #[serde(default)]
    pub note_card: Option<XiaohongshuNoteCard>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Detailed Xiaohongshu note card.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuNoteCard {
    /// Tagged users in the note body.
    #[serde(default)]
    pub at_user_list: Vec<XiaohongshuUserSummary>,
    /// Note description.
    #[serde(default)]
    pub desc: Option<String>,
    /// Image assets embedded in the note.
    #[serde(default)]
    pub image_list: Vec<XiaohongshuImageAsset>,
    /// Interaction stats.
    #[serde(default)]
    pub interact_info: Option<XiaohongshuInteractInfo>,
    /// Viewer IP location label.
    #[serde(default)]
    pub ip_location: Option<String>,
    /// Last update timestamp.
    #[serde(default)]
    pub last_update_time: Option<u64>,
    /// Note id.
    #[serde(default)]
    pub note_id: Option<String>,
    /// Attached tags.
    #[serde(default)]
    pub tag_list: Vec<XiaohongshuNoteTag>,
    /// Note publish timestamp.
    #[serde(default)]
    pub time: Option<u64>,
    /// Note title.
    #[serde(default)]
    pub title: Option<String>,
    /// Note type marker.
    #[serde(default)]
    pub r#type: Option<String>,
    /// Note author.
    #[serde(default)]
    pub user: Option<XiaohongshuUserSummary>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Xiaohongshu note tag.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuNoteTag {
    /// Tag id.
    #[serde(default)]
    pub id: Option<String>,
    /// Tag name.
    #[serde(default)]
    pub name: Option<String>,
    /// Tag type marker.
    #[serde(default)]
    pub r#type: Option<String>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Payload body for note-comments responses.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuNoteCommentsData {
    /// Comments returned by the current page.
    #[serde(default)]
    pub comments: Vec<XiaohongshuComment>,
    /// Pagination cursor.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub cursor: Option<String>,
    /// Whether more comments are available.
    #[serde(default)]
    pub has_more: Option<bool>,
    /// Upstream server timestamp.
    #[serde(default, deserialize_with = "deserialize_u64_opt")]
    pub time: Option<u64>,
    /// Target user id.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub user_id: Option<String>,
    /// Optional xsec token echoed by the API.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub xsec_token: Option<String>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Xiaohongshu root comment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuComment {
    /// Mentioned users.
    #[serde(default, deserialize_with = "deserialize_value_list")]
    pub at_users: Vec<Value>,
    /// Comment content.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub content: Option<String>,
    /// Creation timestamp.
    #[serde(default, deserialize_with = "deserialize_u64_opt")]
    pub create_time: Option<u64>,
    /// Comment id.
    #[serde(deserialize_with = "deserialize_lossy_string")]
    pub id: String,
    /// IP location label.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub ip_location: Option<String>,
    /// Like count.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub like_count: Option<String>,
    /// Whether the current viewer has liked the comment.
    #[serde(default)]
    pub liked: Option<bool>,
    /// Target note id.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub note_id: Option<String>,
    /// Attached pictures.
    #[serde(default)]
    pub pictures: Vec<XiaohongshuCommentPicture>,
    /// Attached tags.
    #[serde(default, deserialize_with = "deserialize_value_list")]
    pub show_tags: Vec<Value>,
    /// Upstream status code.
    #[serde(default)]
    pub status: Option<i64>,
    /// Reply count.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub sub_comment_count: Option<String>,
    /// Reply cursor.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub sub_comment_cursor: Option<String>,
    /// Whether more replies exist.
    #[serde(default)]
    pub sub_comment_has_more: Option<bool>,
    /// Nested reply items.
    #[serde(default)]
    pub sub_comments: Vec<XiaohongshuSubComment>,
    /// Comment author.
    #[serde(default)]
    pub user_info: Option<XiaohongshuUserSummary>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Comment picture payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuCommentPicture {
    /// Picture height.
    #[serde(default, deserialize_with = "deserialize_u64_or_default")]
    pub height: u64,
    /// Picture variants.
    #[serde(default)]
    pub info_list: Vec<super::common::XiaohongshuImageInfo>,
    /// Default URL.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub url_default: Option<String>,
    /// Preload URL.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub url_pre: Option<String>,
    /// Picture width.
    #[serde(default, deserialize_with = "deserialize_u64_or_default")]
    pub width: u64,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

/// Xiaohongshu reply comment.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct XiaohongshuSubComment {
    /// Mentioned users.
    #[serde(default, deserialize_with = "deserialize_value_list")]
    pub at_users: Vec<Value>,
    /// Comment content.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub content: Option<String>,
    /// Creation timestamp.
    #[serde(default, deserialize_with = "deserialize_u64_opt")]
    pub create_time: Option<u64>,
    /// Comment id.
    #[serde(deserialize_with = "deserialize_lossy_string")]
    pub id: String,
    /// IP location label.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub ip_location: Option<String>,
    /// Like count.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub like_count: Option<String>,
    /// Whether the current viewer has liked the comment.
    #[serde(default)]
    pub liked: Option<bool>,
    /// Target note id.
    #[serde(default, deserialize_with = "deserialize_lossy_string_opt")]
    pub note_id: Option<String>,
    /// Attached picture payloads.
    #[serde(default, deserialize_with = "deserialize_value_list")]
    pub pictures: Vec<Value>,
    /// Attached tags.
    #[serde(default, deserialize_with = "deserialize_value_list")]
    pub show_tags: Vec<Value>,
    /// Upstream status code.
    #[serde(default)]
    pub status: Option<i64>,
    /// Optional target comment.
    #[serde(default)]
    pub target_comment: Option<Value>,
    /// Reply author.
    #[serde(default)]
    pub user_info: Option<XiaohongshuUserSummary>,
    /// Unknown fields preserved for forwards compatibility.
    #[serde(flatten)]
    pub extra: BTreeMap<String, Value>,
}

fn deserialize_lossy_string<'de, D>(deserializer: D) -> Result<String, D::Error>
where
    D: Deserializer<'de>,
{
    deserialize_lossy_string_opt(deserializer)?
        .ok_or_else(|| serde::de::Error::custom("expected a non-null string-compatible value"))
}

fn deserialize_lossy_string_opt<'de, D>(deserializer: D) -> Result<Option<String>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(value.and_then(|value| value_to_lossy_string(&value)))
}

fn deserialize_u64_opt<'de, D>(deserializer: D) -> Result<Option<u64>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(value.and_then(|value| value_to_u64(&value)))
}

fn deserialize_u64_or_default<'de, D>(deserializer: D) -> Result<u64, D::Error>
where
    D: Deserializer<'de>,
{
    Ok(deserialize_u64_opt(deserializer)?.unwrap_or_default())
}

fn deserialize_value_list<'de, D>(deserializer: D) -> Result<Vec<Value>, D::Error>
where
    D: Deserializer<'de>,
{
    let value = Option::<Value>::deserialize(deserializer)?;
    Ok(match value {
        None | Some(Value::Null) => Vec::new(),
        Some(Value::Array(items)) => items,
        Some(other) => vec![other],
    })
}

fn value_to_lossy_string(value: &Value) -> Option<String> {
    match value {
        Value::Null => None,
        Value::String(text) => Some(text.clone()),
        Value::Number(number) => Some(number.to_string()),
        Value::Bool(flag) => Some(flag.to_string()),
        other => Some(other.to_string()),
    }
}

fn value_to_u64(value: &Value) -> Option<u64> {
    match value {
        Value::Number(number) => number.as_u64(),
        Value::String(text) => text.parse().ok(),
        _ => None,
    }
}
