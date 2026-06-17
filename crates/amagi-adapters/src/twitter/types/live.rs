use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Twitter/X live room information discovered from a user account.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterLiveRoomInfo {
    /// Stable numeric Twitter/X user id used for the lookup.
    pub user_id: String,
    /// Public screen name when it was resolved or returned by upstream.
    pub screen_name: Option<String>,
    /// Whether the account currently has a running video broadcast.
    pub is_live: bool,
    /// Suggested refresh delay returned by X, in seconds.
    pub refresh_delay_secs: Option<u64>,
    /// Current live video broadcast payload when upstream exposes one.
    pub live_video: Option<TwitterLiveVideoBroadcast>,
    /// Full upstream avatar-content response with remaining fields preserved.
    pub upstream_payload: Value,
}

/// Twitter/X live video broadcast metadata.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterLiveVideoBroadcast {
    /// Stable X Broadcast id.
    pub broadcast_id: String,
    /// Broadcast state such as `RUNNING`, `ENDED`, or `TIMED_OUT`.
    pub state: Option<String>,
    /// Human-readable broadcast title/status when present.
    pub title: Option<String>,
    /// Media key used by `/live_video_stream/status`.
    pub media_key: Option<String>,
    /// Numeric media id when present.
    pub media_id: Option<String>,
    /// Tweet id associated with this broadcast when present.
    pub tweet_id: Option<String>,
    /// Periscope/proxsee user id backing the broadcast.
    pub periscope_user_id: Option<String>,
    /// Numeric Twitter/X user id of the broadcaster.
    pub twitter_user_id: Option<String>,
    /// Twitter/X username of the broadcaster.
    pub twitter_username: Option<String>,
    /// Display name of the broadcaster.
    pub display_name: Option<String>,
    /// Whether replay is available for this broadcast.
    pub available_for_replay: bool,
    /// Broadcast start time as returned by upstream.
    pub started_at: Option<String>,
    /// Last upstream ping time as returned by upstream.
    pub ping_at: Option<String>,
    /// Video width in pixels.
    pub width: Option<u64>,
    /// Video height in pixels.
    pub height: Option<u64>,
    /// Current watcher count when present.
    pub total_watching: Option<u64>,
    /// Total watched count when present.
    pub total_watched: Option<u64>,
    /// Upstream broadcast source, such as `producer`.
    pub broadcast_source: Option<String>,
    /// Whether upstream marks the broadcast as high latency.
    pub is_high_latency: Option<bool>,
    /// Wrapper-stripped copy of the upstream broadcast payload with remaining fields preserved.
    pub upstream_payload: Value,
}
