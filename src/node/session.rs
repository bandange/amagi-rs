use std::collections::HashMap;
use std::sync::{Arc, RwLock};

use serde::Serialize;

use crate::node::protocol::{AUTH_MAX_SKEW_MS, now_ms};

/// Lifecycle state of one node session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum NodeSessionState {
    Disconnected,
    Connecting,
    Authenticating,
    Ready,
    Degraded,
    Reconnecting,
}

impl Default for NodeSessionState {
    fn default() -> Self {
        Self::Disconnected
    }
}

/// In-memory replay cache for node auth nonces within the accepted auth window.
#[derive(Debug, Clone, Default)]
pub(crate) struct NodeAuthReplayCache {
    inner: Arc<RwLock<HashMap<String, u64>>>,
}

impl NodeAuthReplayCache {
    /// Remember one `(node_id, nonce)` pair and reject replays within the auth window.
    pub(crate) fn register(&self, node_id: &str, nonce: &str) -> Result<(), &'static str> {
        let nonce = nonce.trim();
        if nonce.is_empty() {
            return Err("node auth nonce must not be empty");
        }

        let now = now_ms();
        let expire_before = now.saturating_sub(AUTH_MAX_SKEW_MS.saturating_mul(2));
        let key = format!("{node_id}:{nonce}");
        let mut guard = self
            .inner
            .write()
            .expect("node auth replay cache should be writable");
        guard.retain(|_, observed_at_ms| *observed_at_ms >= expire_before);
        if guard.contains_key(&key) {
            return Err("node auth nonce replay detected");
        }
        guard.insert(key, now);
        Ok(())
    }
}
