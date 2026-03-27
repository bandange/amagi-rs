use serde::Deserialize;

use crate::platforms::douyin::DouyinSearchType;

/// Optional pagination query for Douyin comment lists.
#[derive(Debug, Default, Deserialize)]
pub struct DouyinCommentQuery {
    /// Optional page size.
    pub number: Option<u32>,
    /// Optional pagination cursor.
    pub cursor: Option<u64>,
}

/// Optional pagination query for Douyin user lists.
#[derive(Debug, Default, Deserialize)]
pub struct DouyinUserListQuery {
    /// Optional page size.
    pub number: Option<u32>,
    /// Optional pagination cursor.
    pub max_cursor: Option<String>,
}

/// Search query parameters for Douyin content search.
#[derive(Debug, Deserialize)]
pub struct DouyinSearchQuery {
    /// Search keyword.
    pub query: String,
    /// Optional search type.
    #[serde(rename = "type")]
    pub search_type: Option<DouyinSearchType>,
    /// Optional page size.
    pub number: Option<u32>,
    /// Optional search cursor id.
    pub search_id: Option<String>,
}

/// Query parameters for Douyin live-room requests.
#[derive(Debug, Deserialize)]
pub struct DouyinLiveRoomQuery {
    /// Required `web_rid` of the target room.
    pub web_rid: String,
}

/// Query parameters for Douyin login QR code requests.
#[derive(Debug, Default, Deserialize)]
pub struct DouyinLoginQrcodeQuery {
    /// Optional verify_fp override.
    pub verify_fp: Option<String>,
}

/// Query parameters for Douyin danmaku requests.
#[derive(Debug, Deserialize)]
pub struct DouyinDanmakuQuery {
    /// Required work duration.
    pub duration: u64,
    /// Optional segment start.
    pub start_time: Option<u64>,
    /// Optional segment end.
    pub end_time: Option<u64>,
}
