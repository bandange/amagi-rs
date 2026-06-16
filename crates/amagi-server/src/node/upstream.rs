use std::fmt;
use std::sync::{Arc, RwLock};

use tokio::sync::mpsc;

use crate::node::NodeRole;
use crate::node::protocol::{NodeEnvelope, now_ms};
use crate::node::session::NodeSessionState;

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct UpstreamConnectionSnapshot {
    pub connected: bool,
    pub state: NodeSessionState,
    pub session_id: Option<String>,
    pub node_id: Option<String>,
    pub role: Option<NodeRole>,
    pub version: Option<String>,
    pub capabilities: Vec<String>,
    pub platforms: Vec<String>,
    pub connected_at_ms: Option<u64>,
    pub last_seen_ms: Option<u64>,
    pub last_disconnect_ms: Option<u64>,
    pub last_error: Option<String>,
}

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub(crate) struct UpstreamPeerInfo {
    pub session_id: Option<String>,
    pub node_id: Option<String>,
    pub role: Option<NodeRole>,
    pub version: Option<String>,
    pub capabilities: Vec<String>,
    pub platforms: Vec<String>,
}

#[derive(Default)]
struct UpstreamConnectionState {
    sender: Option<mpsc::UnboundedSender<NodeEnvelope>>,
    state: NodeSessionState,
    session_id: Option<String>,
    node_id: Option<String>,
    role: Option<NodeRole>,
    version: Option<String>,
    capabilities: Vec<String>,
    platforms: Vec<String>,
    connected_at_ms: Option<u64>,
    last_seen_ms: Option<u64>,
    last_disconnect_ms: Option<u64>,
    last_error: Option<String>,
}

#[derive(Clone, Default)]
pub(crate) struct UpstreamConnection {
    inner: Arc<RwLock<UpstreamConnectionState>>,
}

impl fmt::Debug for UpstreamConnection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let connected = self.snapshot().connected;
        f.debug_struct("UpstreamConnection")
            .field("connected", &connected)
            .finish()
    }
}

impl UpstreamConnection {
    pub(crate) fn set_state(&self, state: NodeSessionState) {
        let mut guard = self
            .inner
            .write()
            .expect("upstream connection should be writable");
        guard.state = state;
    }

    pub(crate) fn set_ready(
        &self,
        sender: mpsc::UnboundedSender<NodeEnvelope>,
        peer: UpstreamPeerInfo,
    ) {
        let mut guard = self
            .inner
            .write()
            .expect("upstream connection should be writable");
        let now = now_ms();
        guard.sender = Some(sender);
        guard.state = NodeSessionState::Ready;
        guard.session_id = peer.session_id;
        guard.node_id = peer.node_id;
        guard.role = peer.role;
        guard.version = peer.version;
        guard.capabilities = peer.capabilities;
        guard.platforms = peer.platforms;
        guard.connected_at_ms = Some(now);
        guard.last_seen_ms = Some(now);
        guard.last_error = None;
    }

    pub(crate) fn record_disconnect(
        &self,
        next_state: NodeSessionState,
        last_error: Option<String>,
    ) {
        let mut guard = self
            .inner
            .write()
            .expect("upstream connection should be writable");
        guard.last_disconnect_ms = Some(now_ms());
        guard.sender = None;
        guard.state = next_state;
        guard.last_error = last_error;
    }

    pub(crate) fn send(&self, envelope: NodeEnvelope) -> Result<(), NodeEnvelope> {
        let sender = {
            let guard = self
                .inner
                .read()
                .expect("upstream connection should be readable");
            guard.sender.clone()
        };

        match sender {
            Some(sender) => sender.send(envelope).map_err(|error| error.0),
            None => Err(envelope),
        }
    }

    pub(crate) fn touch(&self) {
        let mut guard = self
            .inner
            .write()
            .expect("upstream connection should be writable");
        if guard.sender.is_some() {
            guard.last_seen_ms = Some(now_ms());
        }
    }

    pub(crate) fn snapshot(&self) -> UpstreamConnectionSnapshot {
        let guard = self
            .inner
            .read()
            .expect("upstream connection should be readable");
        UpstreamConnectionSnapshot {
            connected: guard.sender.is_some(),
            state: guard.state,
            session_id: guard.session_id.clone(),
            node_id: guard.node_id.clone(),
            role: guard.role,
            version: guard.version.clone(),
            capabilities: guard.capabilities.clone(),
            platforms: guard.platforms.clone(),
            connected_at_ms: guard.connected_at_ms,
            last_seen_ms: guard.last_seen_ms,
            last_disconnect_ms: guard.last_disconnect_ms,
            last_error: guard.last_error.clone(),
        }
    }
}
