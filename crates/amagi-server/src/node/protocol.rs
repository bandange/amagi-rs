use std::collections::BTreeMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::node::NodeRole;
use amagi_core::AppError;
use amagi_core::Platform;

/// Maximum allowed clock skew for the minimum auth handshake.
pub(crate) const AUTH_MAX_SKEW_MS: u64 = 5 * 60 * 1000;
/// Internal HTTP header used to propagate proxy hop count across local task execution.
pub(crate) const PROXY_HOP_HEADER: &str = "x-amagi-proxy-hop";
/// Internal HTTP header used to preserve one node trace identifier across relay hops.
pub(crate) const NODE_TRACE_ID_HEADER: &str = "x-amagi-node-trace-id";
/// Internal HTTP header used to preserve one absolute task deadline across relay hops.
pub(crate) const NODE_DEADLINE_MS_HEADER: &str = "x-amagi-node-deadline-ms";
/// Internal HTTP header used to preserve one node request identifier across relay hops.
pub(crate) const NODE_REQUEST_ID_HEADER: &str = "x-amagi-node-request-id";
/// Internal HTTP header used to preserve one original task ingress timestamp across relay hops.
pub(crate) const NODE_REQUESTED_AT_MS_HEADER: &str = "x-amagi-node-requested-at-ms";
/// Internal HTTP header used to preserve one optional caller hint across relay hops.
pub(crate) const NODE_CALLER_HEADER: &str = "x-amagi-node-caller";
/// WebSocket handshake header used to declare the connecting node id before the first frame.
pub(crate) const NODE_HANDSHAKE_NODE_ID_HEADER: &str = "x-amagi-node-id";

/// Shared node-envelope kind.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub(crate) enum NodeEnvelopeKind {
    Request,
    Response,
    Event,
}

/// Structured node-level error.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeError {
    pub code: String,
    pub message: String,
}

/// Minimal JSON-RPC-like envelope used on the WSS node channel.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub(crate) struct NodeEnvelope {
    pub kind: NodeEnvelopeKind,
    pub id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub trace_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hop_count: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub deadline_ms: Option<u64>,
    pub method: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub params: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub result: Option<Value>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<NodeError>,
}

/// Parameters for the first-frame auth request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeAuthParams {
    pub node_id: String,
    pub token: String,
    pub timestamp_ms: u64,
    pub nonce: String,
}

/// Parameters for the hello request sent after auth.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeHelloParams {
    pub node_id: String,
    pub role: NodeRole,
    pub version: String,
    pub capabilities: Vec<String>,
    pub platforms: Vec<String>,
}

/// Result payload returned after a successful hello exchange.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeHelloAck {
    pub state: String,
    pub session_id: String,
    pub node_id: String,
    pub role: NodeRole,
    pub version: String,
    pub capabilities: Vec<String>,
    pub platforms: Vec<String>,
}

/// Parameters for the heartbeat event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeHeartbeatParams {
    pub timestamp_ms: u64,
}

/// Parameters for one node capability/platform advertisement update.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeAdvertiseParams {
    pub capabilities: Vec<String>,
    pub platforms: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_tasks: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub active_tasks: Option<u32>,
}

/// Parameters for one node drain request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeDrainParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Parameters for one node ready request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeReadyParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Parameters for one node isolation request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeIsolateParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Parameters for one node capacity update request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeCapacityParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_concurrent_tasks: Option<u32>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Parameters for one parent-to-child shutdown notice event.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeShutdownNoticeParams {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reconnect_after_ms: Option<u64>,
}

/// One platform-to-node route update entry.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeRouteUpdateEntry {
    pub platform: Platform,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub route_node: Option<String>,
}

/// Parameters for one runtime route update request.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeRouteUpdateParams {
    pub updates: Vec<NodeRouteUpdateEntry>,
}

/// Parameters for one node-routed HTTP task dispatch.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeTaskDispatchParams {
    pub platform: Platform,
    pub http_method: String,
    pub path: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub query: Vec<(String, String)>,
    pub path_and_query: String,
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub caller: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub requested_at_ms: Option<u64>,
}

/// Progress event payload for one node-routed task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeTaskProgressParams {
    pub request_id: String,
    pub stage: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percent: Option<u8>,
}

/// Cancellation request payload for one node-routed task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeTaskCancelParams {
    pub request_id: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Result payload for one node-routed HTTP task.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub(crate) struct NodeTaskResult {
    pub status: u16,
    pub content_type: Option<String>,
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub headers: BTreeMap<String, String>,
    pub body: Vec<u8>,
}

impl NodeEnvelope {
    pub(crate) fn request(method: impl Into<String>, params: Value) -> Self {
        Self {
            kind: NodeEnvelopeKind::Request,
            id: new_message_id("req"),
            trace_id: None,
            session_id: None,
            from: None,
            to: None,
            hop_count: None,
            deadline_ms: None,
            method: method.into(),
            params: Some(params),
            result: None,
            error: None,
        }
    }

    pub(crate) fn response_ok(
        request_id: impl Into<String>,
        method: impl Into<String>,
        from: Option<String>,
        to: Option<String>,
        session_id: Option<String>,
        result: Value,
    ) -> Self {
        Self {
            kind: NodeEnvelopeKind::Response,
            id: request_id.into(),
            trace_id: None,
            session_id,
            from,
            to,
            hop_count: None,
            deadline_ms: None,
            method: method.into(),
            params: None,
            result: Some(result),
            error: None,
        }
    }

    pub(crate) fn response_error(
        request_id: impl Into<String>,
        method: impl Into<String>,
        from: Option<String>,
        to: Option<String>,
        session_id: Option<String>,
        code: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            kind: NodeEnvelopeKind::Response,
            id: request_id.into(),
            trace_id: None,
            session_id,
            from,
            to,
            hop_count: None,
            deadline_ms: None,
            method: method.into(),
            params: None,
            result: None,
            error: Some(NodeError {
                code: code.into(),
                message: message.into(),
            }),
        }
    }

    pub(crate) fn event(
        method: impl Into<String>,
        from: Option<String>,
        to: Option<String>,
        session_id: Option<String>,
        params: Value,
    ) -> Self {
        Self {
            kind: NodeEnvelopeKind::Event,
            id: new_message_id("evt"),
            trace_id: None,
            session_id,
            from,
            to,
            hop_count: None,
            deadline_ms: None,
            method: method.into(),
            params: Some(params),
            result: None,
            error: None,
        }
    }

    pub(crate) fn parse_params<T>(&self) -> Result<T, AppError>
    where
        T: for<'de> Deserialize<'de>,
    {
        let Some(params) = &self.params else {
            return Err(AppError::InvalidRequestConfig(format!(
                "node method `{}` is missing params",
                self.method
            )));
        };

        serde_json::from_value(params.clone()).map_err(AppError::from)
    }
}

pub(crate) fn new_message_id(prefix: &str) -> String {
    static NEXT_ID: AtomicU64 = AtomicU64::new(1);
    let counter = NEXT_ID.fetch_add(1, Ordering::Relaxed);
    format!("{prefix}_{:x}_{counter:x}", now_ms())
}

pub(crate) fn now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}
