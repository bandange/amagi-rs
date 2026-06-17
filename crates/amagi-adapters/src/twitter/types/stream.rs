use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Twitter/X live-room playback stream resolved from a broadcast, media key, or tweet.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterLiveRoomStream {
    /// Stable X Broadcast id when it was known or discoverable.
    pub broadcast_id: Option<String>,
    /// Broadcast media key used by `/live_video_stream/status`.
    pub media_key: String,
    /// Tweet id associated with the broadcast when it was known or discoverable.
    pub tweet_id: Option<String>,
    /// Master HLS URL returned by X/Periscope.
    pub hls_url: String,
    /// Alternate no-redirect playback URL when upstream returns one.
    pub no_redirect_playback_url: Option<String>,
    /// Upstream source status such as `LIVE_PUBLIC`.
    pub source_status: Option<String>,
    /// Upstream stream type such as `HLS`.
    pub stream_type: Option<String>,
    /// Whether the resolved stream is a replay rather than a currently running live stream.
    pub is_replay: bool,
    /// Upstream session id returned with the playback status.
    pub session_id: Option<String>,
    /// X share URL for the broadcast when upstream exposes it.
    pub share_url: Option<String>,
    /// Broadcast metadata when the lookup used or discovered a broadcast id.
    pub broadcast: Option<TwitterLiveBroadcastDetail>,
    /// Redacted upstream payloads used to resolve the stream.
    pub upstream_payload: Value,
}

/// Twitter/X broadcast metadata used when resolving a live-room playback stream.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterLiveBroadcastDetail {
    /// Stable X Broadcast id.
    pub broadcast_id: String,
    /// Broadcast media key used by `/live_video_stream/status`.
    pub media_key: Option<String>,
    /// Numeric media id when present.
    pub media_id: Option<String>,
    /// Broadcast state such as `RUNNING`, `ENDED`, or `Ended`.
    pub state: Option<String>,
    /// Human-readable broadcast title/status when present.
    pub title: Option<String>,
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
    pub available_for_replay: Option<bool>,
    /// Broadcast start time as returned by upstream.
    pub started_at: Option<String>,
    /// Broadcast end time as returned by upstream.
    pub ended_at: Option<String>,
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
    /// Redacted copy of the upstream broadcast payload with remaining fields preserved.
    pub upstream_payload: Value,
}
