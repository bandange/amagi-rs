use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::profile::KuaishouUserProfileUserInfo;

/// Aggregated live-room payload for a Kuaishou author page.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouLiveRoomInfo {
    /// Requested principal id.
    #[serde(rename = "principalId")]
    pub principal_id: String,
    /// Index of the currently selected room in `playList`.
    #[serde(rename = "activeIndex")]
    pub active_index: u32,
    /// Current live room item.
    pub current: Option<KuaishouLiveRoomPlayItem>,
    /// Recommended play list including the current room.
    #[serde(rename = "playList")]
    pub play_list: Vec<KuaishouLiveRoomPlayItem>,
    /// WebSocket URLs used by the live room page.
    #[serde(rename = "websocketUrls")]
    pub websocket_urls: Vec<String>,
    /// WebSocket token when available.
    pub token: String,
    /// Notice list emitted by the live room.
    #[serde(rename = "noticeList")]
    pub notice_list: Vec<Value>,
    /// Loading state mirrored from the original page store.
    pub loading: bool,
    /// Emoji and gift metadata for the room.
    pub emoji: KuaishouLiveRoomEmojiState,
    /// Complete upstream payload snapshot grouped by contributing sources.
    #[serde(default, skip_serializing_if = "Value::is_null")]
    pub upstream_payload: Value,
}

/// Emoji and gift state embedded in a Kuaishou live room.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouLiveRoomEmojiState {
    /// Standard emoji icon catalog.
    #[serde(rename = "iconUrls")]
    pub icon_urls: BTreeMap<String, String>,
    /// Live gift list.
    #[serde(rename = "giftList")]
    pub gift_list: Vec<Value>,
    /// Gift panel list.
    #[serde(rename = "giftPanelList")]
    pub gift_panel_list: Vec<Value>,
    /// Gift token.
    pub token: String,
    /// Gift panel token.
    #[serde(rename = "panelToken")]
    pub panel_token: String,
    /// Long-send gift type when present.
    #[serde(rename = "longSendGiftType")]
    pub long_send_gift_type: Option<Value>,
}

/// One live-room item from the Kuaishou play list.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouLiveRoomPlayItem {
    /// Primary live stream info.
    #[serde(rename = "liveStream")]
    pub live_stream: KuaishouLiveStreamInfo,
    /// Normalized author info.
    pub author: KuaishouUserProfileUserInfo,
    /// Game info block.
    #[serde(rename = "gameInfo")]
    pub game_info: KuaishouLiveRoomGameInfo,
    /// Whether the room is currently live.
    #[serde(rename = "isLiving")]
    pub is_living: bool,
    /// Auth token for the room when present.
    #[serde(rename = "authToken")]
    pub auth_token: Option<String>,
    /// Dynamic room config object.
    pub config: Value,
    /// Dynamic WebSocket info object.
    #[serde(rename = "websocketInfo")]
    pub websocket_info: Value,
    /// Dynamic room status object.
    pub status: Value,
}

/// Primary stream info for a Kuaishou live room.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct KuaishouLiveStreamInfo {
    /// Live stream id.
    pub id: String,
    /// Poster URL.
    pub poster: String,
    /// Raw play-url payload.
    #[serde(rename = "playUrls")]
    pub play_urls: Value,
    /// Raw stream URL.
    pub url: String,
    /// HLS play URL.
    #[serde(rename = "hlsPlayUrl")]
    pub hls_play_url: String,
    /// Optional location string.
    pub location: Option<String>,
    /// Stream type.
    #[serde(rename = "type")]
    pub stream_type: String,
    /// Whether the room is guessed live content.
    #[serde(rename = "liveGuess")]
    pub live_guess: bool,
    /// Experience tag.
    #[serde(rename = "expTag")]
    pub exp_tag: String,
    /// Whether the room is private.
    #[serde(rename = "privateLive")]
    pub private_live: bool,
}

/// Game-info block for a Kuaishou live room.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct KuaishouLiveRoomGameInfo {
    /// Game id.
    pub id: String,
    /// Game name.
    pub name: String,
    /// Poster URL.
    pub poster: String,
    /// Description.
    pub description: String,
    /// Category abbreviation.
    #[serde(rename = "categoryAbbr")]
    pub category_abbr: String,
    /// Category name.
    #[serde(rename = "categoryName")]
    pub category_name: String,
    /// Watching-count string.
    #[serde(rename = "watchingCount")]
    pub watching_count: String,
    /// Room-count string.
    #[serde(rename = "roomCount")]
    pub room_count: String,
}
