use serde::{Deserialize, Serialize};

use super::super::{
    OrderedJson,
    types::{XiaohongshuSearchNoteType, XiaohongshuSearchSortType},
};

/// Public Xiaohongshu request descriptor.
#[derive(Debug, Clone, PartialEq)]
pub struct XiaohongshuRequestSpec {
    /// API path used by the Xiaohongshu signer.
    pub api_path: String,
    /// Fully qualified request URL.
    pub url: String,
    /// Ordered query parameters used both for signing and URL generation.
    pub params: Option<OrderedJson>,
    /// Ordered JSON body used both for signing and POST requests.
    pub body: Option<OrderedJson>,
}

/// Options for Xiaohongshu home-feed requests.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct XiaohongshuHomeFeedOptions {
    /// Cursor score used for pagination.
    pub cursor_score: Option<String>,
    /// Number of items to request.
    pub num: Option<u32>,
    /// Refresh type used by the upstream feed API.
    pub refresh_type: Option<u32>,
    /// Index of the last visible note.
    pub note_index: Option<u32>,
    /// Feed category. Defaults to `homefeed_recommend`.
    pub category: Option<String>,
    /// Optional search keyword carried by the feed request.
    pub search_key: Option<String>,
}

/// Options for Xiaohongshu note-detail requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XiaohongshuNoteDetailOptions {
    /// Target note id.
    pub note_id: String,
    /// Anti-bot token extracted from the note page.
    pub xsec_token: String,
}

/// Options for Xiaohongshu note-comment requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XiaohongshuCommentsOptions {
    /// Target note id.
    pub note_id: String,
    /// Optional pagination cursor.
    pub cursor: Option<String>,
    /// Anti-bot token extracted from the note page.
    pub xsec_token: String,
}

/// Options for Xiaohongshu user-profile requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XiaohongshuUserProfileOptions {
    /// Target user id.
    pub user_id: String,
    /// Anti-bot token extracted from the user page.
    pub xsec_token: String,
    /// Upstream xsec source. Defaults to `pc_feed` when omitted.
    pub xsec_source: Option<String>,
}

/// Options for Xiaohongshu user-note-list requests.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct XiaohongshuUserNotesOptions {
    /// Target user id.
    pub user_id: String,
    /// Anti-bot token extracted from the user page.
    pub xsec_token: String,
    /// Upstream xsec source. Defaults to `pc_feed` when omitted.
    pub xsec_source: Option<String>,
    /// Optional pagination cursor.
    pub cursor: Option<String>,
    /// Number of notes to request.
    pub num: Option<u32>,
}

/// Options for Xiaohongshu note-search requests.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct XiaohongshuSearchNotesOptions {
    /// Search keyword.
    pub keyword: String,
    /// Result page index, starting at `1`.
    pub page: Option<u32>,
    /// Number of results per page.
    pub page_size: Option<u32>,
    /// Optional upstream sort order.
    pub sort: Option<XiaohongshuSearchSortType>,
    /// Optional note-type filter.
    pub note_type: Option<XiaohongshuSearchNoteType>,
}
