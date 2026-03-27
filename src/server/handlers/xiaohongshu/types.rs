use serde::Deserialize;

use crate::platforms::xiaohongshu::{XiaohongshuSearchNoteType, XiaohongshuSearchSortType};

/// Optional query parameters for Xiaohongshu home-feed requests.
#[derive(Debug, Default, Deserialize)]
pub struct XiaohongshuHomeFeedQuery {
    /// Optional cursor score.
    pub cursor_score: Option<String>,
    /// Optional page size.
    pub num: Option<u32>,
    /// Optional refresh type.
    pub refresh_type: Option<u32>,
    /// Optional note index.
    pub note_index: Option<u32>,
    /// Optional feed category.
    pub category: Option<String>,
    /// Optional feed search key.
    pub search_key: Option<String>,
}

/// Query parameters for Xiaohongshu note-detail requests.
#[derive(Debug, Deserialize)]
pub struct XiaohongshuNoteQuery {
    /// Required xsec token.
    pub xsec_token: String,
}

/// Query parameters for Xiaohongshu note-comments requests.
#[derive(Debug, Deserialize)]
pub struct XiaohongshuNoteCommentsQuery {
    /// Required xsec token.
    pub xsec_token: String,
    /// Optional pagination cursor.
    pub cursor: Option<String>,
}

/// Query parameters for Xiaohongshu user-profile requests.
#[derive(Debug, Deserialize)]
pub struct XiaohongshuUserProfileQuery {
    /// Required xsec token.
    pub xsec_token: String,
    /// Optional xsec source.
    pub xsec_source: Option<String>,
}

/// Query parameters for Xiaohongshu user-note-list requests.
#[derive(Debug, Default, Deserialize)]
pub struct XiaohongshuUserNoteListQuery {
    /// Required xsec token.
    pub xsec_token: String,
    /// Optional xsec source.
    pub xsec_source: Option<String>,
    /// Optional pagination cursor.
    pub cursor: Option<String>,
    /// Optional page size.
    pub num: Option<u32>,
}

/// Query parameters for Xiaohongshu note-search requests.
#[derive(Debug, Deserialize)]
pub struct XiaohongshuSearchQuery {
    /// Search keyword.
    pub keyword: String,
    /// Optional page number.
    pub page: Option<u32>,
    /// Optional page size.
    pub page_size: Option<u32>,
    /// Optional sort order.
    pub sort: Option<XiaohongshuSearchSortType>,
    /// Optional note-type filter.
    pub note_type: Option<XiaohongshuSearchNoteType>,
}
