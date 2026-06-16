use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, RwLock};

use serde::Serialize;
use tokio::sync::{mpsc, oneshot};

use crate::node::NodeRole;
use crate::node::protocol::NodeEnvelope;
use crate::node::session::NodeSessionState;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum NodeAvailability {
    Ready,
    Draining,
    Isolated,
}

/// Snapshot of one online downstream node.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NodeRecord {
    pub session_id: String,
    pub node_id: String,
    pub role: NodeRole,
    pub version: Option<String>,
    pub session_state: NodeSessionState,
    pub capabilities: Vec<String>,
    pub platforms: Vec<String>,
    pub availability: NodeAvailability,
    pub max_concurrent_tasks: Option<u32>,
    pub active_tasks: u32,
    pub connected_at_ms: u64,
    pub last_seen_ms: u64,
}

#[derive(Clone)]
pub(crate) struct NodeRegistry {
    inner: Arc<RwLock<HashMap<String, RegisteredNode>>>,
    pending: Arc<RwLock<HashMap<String, PendingNodeRequest>>>,
}

#[derive(Clone)]
struct RegisteredNode {
    record: NodeRecord,
    #[allow(dead_code)]
    sender: mpsc::UnboundedSender<NodeEnvelope>,
}

struct PendingNodeRequest {
    tx: oneshot::Sender<NodeEnvelope>,
    target: NodeRecord,
    sender: mpsc::UnboundedSender<NodeEnvelope>,
}

impl Default for NodeRegistry {
    fn default() -> Self {
        Self {
            inner: Arc::new(RwLock::new(HashMap::new())),
            pending: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

impl fmt::Debug for NodeRegistry {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("NodeRegistry")
            .field("online_nodes", &self.count())
            .finish()
    }
}

impl NodeRegistry {
    pub(crate) fn upsert(&self, record: NodeRecord, sender: mpsc::UnboundedSender<NodeEnvelope>) {
        let mut guard = self
            .inner
            .write()
            .expect("node registry should be writable");
        guard.insert(record.node_id.clone(), RegisteredNode { record, sender });
    }

    pub(crate) fn remove_if_session(&self, node_id: &str, session_id: &str) -> bool {
        let mut guard = self
            .inner
            .write()
            .expect("node registry should be writable");
        if guard
            .get(node_id)
            .is_some_and(|node| node.record.session_id == session_id)
        {
            guard.remove(node_id);
            return true;
        }
        false
    }

    pub(crate) fn update_last_seen(&self, node_id: &str, timestamp_ms: u64) {
        let mut guard = self
            .inner
            .write()
            .expect("node registry should be writable");
        if let Some(node) = guard.get_mut(node_id) {
            node.record.last_seen_ms = timestamp_ms;
            node.record.session_state = NodeSessionState::Ready;
        }
    }

    pub(crate) fn update_advertisement(
        &self,
        node_id: &str,
        capabilities: Vec<String>,
        platforms: Vec<String>,
    ) -> bool {
        let mut guard = self
            .inner
            .write()
            .expect("node registry should be writable");
        let Some(node) = guard.get_mut(node_id) else {
            return false;
        };
        node.record.capabilities = capabilities;
        node.record.platforms = platforms;
        node.record.last_seen_ms = crate::node::protocol::now_ms();
        node.record.session_state = NodeSessionState::Ready;
        true
    }

    pub(crate) fn set_availability(&self, node_id: &str, availability: NodeAvailability) -> bool {
        let mut guard = self
            .inner
            .write()
            .expect("node registry should be writable");
        let Some(node) = guard.get_mut(node_id) else {
            return false;
        };
        node.record.availability = availability;
        node.record.last_seen_ms = crate::node::protocol::now_ms();
        node.record.session_state = NodeSessionState::Ready;
        true
    }

    pub(crate) fn set_active_tasks(&self, node_id: &str, active_tasks: u32) -> bool {
        let mut guard = self
            .inner
            .write()
            .expect("node registry should be writable");
        let Some(node) = guard.get_mut(node_id) else {
            return false;
        };
        node.record.active_tasks = active_tasks;
        node.record.last_seen_ms = crate::node::protocol::now_ms();
        node.record.session_state = NodeSessionState::Ready;
        true
    }

    pub(crate) fn set_capacity(&self, node_id: &str, max_concurrent_tasks: Option<u32>) -> bool {
        let mut guard = self
            .inner
            .write()
            .expect("node registry should be writable");
        let Some(node) = guard.get_mut(node_id) else {
            return false;
        };
        node.record.max_concurrent_tasks = max_concurrent_tasks;
        node.record.last_seen_ms = crate::node::protocol::now_ms();
        node.record.session_state = NodeSessionState::Ready;
        true
    }

    pub(crate) fn set_session_state(&self, node_id: &str, session_state: NodeSessionState) -> bool {
        let mut guard = self
            .inner
            .write()
            .expect("node registry should be writable");
        let Some(node) = guard.get_mut(node_id) else {
            return false;
        };
        node.record.session_state = session_state;
        node.record.last_seen_ms = crate::node::protocol::now_ms();
        true
    }

    #[allow(dead_code)]
    pub(crate) fn sender_for(&self, node_id: &str) -> Option<mpsc::UnboundedSender<NodeEnvelope>> {
        let guard = self.inner.read().expect("node registry should be readable");
        guard.get(node_id).map(|node| node.sender.clone())
    }

    #[allow(dead_code)]
    pub(crate) fn records(&self, heartbeat_ms: Option<u64>) -> Vec<NodeRecord> {
        let guard = self.inner.read().expect("node registry should be readable");
        let mut records = guard
            .values()
            .map(|node| effective_record(&node.record, heartbeat_ms))
            .collect::<Vec<_>>();
        records.sort_by(|left, right| left.node_id.cmp(&right.node_id));
        records
    }

    pub(crate) fn downstream_senders(
        &self,
    ) -> Vec<(NodeRecord, mpsc::UnboundedSender<NodeEnvelope>)> {
        let guard = self.inner.read().expect("node registry should be readable");
        guard
            .values()
            .map(|node| (node.record.clone(), node.sender.clone()))
            .collect()
    }

    pub(crate) fn count(&self) -> usize {
        let guard = self.inner.read().expect("node registry should be readable");
        guard.len()
    }

    pub(crate) fn sender_for_platform(
        &self,
        platform: &str,
        heartbeat_ms: Option<u64>,
    ) -> Option<(NodeRecord, mpsc::UnboundedSender<NodeEnvelope>)> {
        let guard = self.inner.read().expect("node registry should be readable");
        guard
            .values()
            .filter_map(|node| {
                let record = effective_record(&node.record, heartbeat_ms);
                (matches!(record.availability, NodeAvailability::Ready)
                    && matches!(record.session_state, NodeSessionState::Ready)
                    && record.platforms.iter().any(|item| item == platform))
                .then(|| (record, node.sender.clone()))
            })
            .filter(|node| {
                node.0
                    .max_concurrent_tasks
                    .is_none_or(|limit| node.0.active_tasks < limit)
            })
            .min_by(|left, right| {
                left.0
                    .active_tasks
                    .cmp(&right.0.active_tasks)
                    .then_with(|| left.0.node_id.cmp(&right.0.node_id))
            })
    }

    pub(crate) fn sender_for_node_platform(
        &self,
        node_id: &str,
        platform: &str,
        heartbeat_ms: Option<u64>,
    ) -> Option<(NodeRecord, mpsc::UnboundedSender<NodeEnvelope>)> {
        let guard = self.inner.read().expect("node registry should be readable");
        let node = guard.get(node_id)?;
        let record = effective_record(&node.record, heartbeat_ms);
        if !matches!(record.availability, NodeAvailability::Ready)
            || !matches!(record.session_state, NodeSessionState::Ready)
        {
            return None;
        }
        if record
            .max_concurrent_tasks
            .is_some_and(|limit| record.active_tasks >= limit)
        {
            return None;
        }
        record
            .platforms
            .iter()
            .any(|item| item == platform)
            .then(|| (record, node.sender.clone()))
    }

    pub(crate) fn sender_for_node(
        &self,
        node_id: &str,
    ) -> Option<(NodeRecord, mpsc::UnboundedSender<NodeEnvelope>)> {
        let guard = self.inner.read().expect("node registry should be readable");
        guard
            .get(node_id)
            .map(|node| (node.record.clone(), node.sender.clone()))
    }

    pub(crate) fn record_for_node(
        &self,
        node_id: &str,
        heartbeat_ms: Option<u64>,
    ) -> Option<NodeRecord> {
        let guard = self.inner.read().expect("node registry should be readable");
        guard
            .get(node_id)
            .map(|node| effective_record(&node.record, heartbeat_ms))
    }

    pub(crate) fn register_pending(
        &self,
        request_id: impl Into<String>,
        target: NodeRecord,
        sender: mpsc::UnboundedSender<NodeEnvelope>,
    ) -> oneshot::Receiver<NodeEnvelope> {
        let (tx, rx) = oneshot::channel();
        let mut guard = self
            .pending
            .write()
            .expect("pending node requests should be writable");
        guard.insert(request_id.into(), PendingNodeRequest { tx, target, sender });
        rx
    }

    pub(crate) fn cancel_pending(&self, request_id: &str) {
        let mut guard = self
            .pending
            .write()
            .expect("pending node requests should be writable");
        guard.remove(request_id);
    }

    pub(crate) fn fulfill_pending(&self, envelope: NodeEnvelope) -> bool {
        let pending = {
            let mut guard = self
                .pending
                .write()
                .expect("pending node requests should be writable");
            guard.remove(&envelope.id)
        };

        pending.is_some_and(|pending| pending.tx.send(envelope).is_ok())
    }

    pub(crate) fn pending_target_for(
        &self,
        request_id: &str,
    ) -> Option<(NodeRecord, mpsc::UnboundedSender<NodeEnvelope>)> {
        let guard = self
            .pending
            .read()
            .expect("pending node requests should be readable");
        guard
            .get(request_id)
            .map(|pending| (pending.target.clone(), pending.sender.clone()))
    }
}

fn effective_record(record: &NodeRecord, heartbeat_ms: Option<u64>) -> NodeRecord {
    let mut effective = record.clone();
    if effective.session_state == NodeSessionState::Ready
        && heartbeat_ms.is_some_and(|heartbeat_ms| {
            crate::node::protocol::now_ms().saturating_sub(effective.last_seen_ms)
                > heartbeat_ms.saturating_mul(2)
        })
    {
        effective.session_state = NodeSessionState::Degraded;
    }
    effective
}
