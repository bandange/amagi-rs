use serde::{Deserialize, Serialize};
use serde_json::Value;

use super::TwitterUserSummary;

/// Public Twitter/X space payload.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct TwitterSpaceDetail {
    /// Stable space id.
    pub id: String,
    /// Human-readable title when present.
    pub title: Option<String>,
    /// Space state such as `Running` or `Ended`.
    pub state: Option<String>,
    /// Space creator/host when present.
    pub host: Option<TwitterUserSummary>,
    /// Creation time as an RFC3339 or upstream timestamp string when available.
    pub created_at: Option<String>,
    /// Start time as an RFC3339 or upstream timestamp string when available.
    pub started_at: Option<String>,
    /// End time as an RFC3339 or upstream timestamp string when available.
    pub ended_at: Option<String>,
    /// Scheduled start time as an RFC3339 or upstream timestamp string when available.
    pub scheduled_start: Option<String>,
    /// Whether replay information is enabled.
    pub replay_enabled: bool,
    /// Total live listeners when available.
    pub total_live_listeners: Option<u64>,
    /// Current participant count when available.
    pub participant_count: Option<u64>,
    /// Optional media key backing the live audio stream.
    pub media_key: Option<String>,
    /// Wrapper-stripped copy of the upstream space payload with remaining fields preserved.
    pub upstream_payload: Value,
}
