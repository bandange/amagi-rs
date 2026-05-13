use serde::Serialize;

use crate::catalog::{ApiMethodSpec, Platform};
use crate::node::session::NodeSessionState;
use crate::node::{NodeRole, registry::NodeAvailability};
use crate::server::runtime::PlatformServeMode;

/// Summary of one published platform.
#[derive(Debug, Serialize)]
pub struct PlatformSummary {
    /// Stable platform identifier.
    pub platform: Platform,
    /// Shared API base path for the platform.
    pub api_base_path: &'static str,
    /// Number of published methods for the platform.
    pub method_count: usize,
    /// Whether the current client has a bound cookie for the platform.
    pub has_cookie: bool,
    /// Serving mode resolved for the platform on this node.
    pub mode: PlatformServeMode,
    /// Whether the platform is published by the current node.
    pub published: bool,
    /// Effective node route target when the platform is WSS-routed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_node: Option<String>,
    /// Source of the effective node route when present.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub route_source: Option<PlatformRouteSource>,
}

/// Root metadata payload for the HTTP server.
#[derive(Debug, Serialize)]
pub struct RootResponse {
    /// Application name.
    pub name: &'static str,
    /// Application version.
    pub version: &'static str,
    /// Service mode.
    pub mode: &'static str,
    /// Service status.
    pub status: &'static str,
    /// Effective bind address.
    pub bind: String,
    /// Effective base URL.
    pub base_url: String,
    /// Published metadata endpoints.
    pub endpoints: Vec<&'static str>,
    /// Published platform summaries.
    pub platforms: Vec<PlatformSummary>,
    /// Runtime node summary when node transport is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<NodeSummary>,
    /// Runtime platform route overrides learned after startup.
    pub runtime_routes: Vec<RuntimeRouteSummary>,
    /// Currently connected downstream nodes.
    pub downstream_nodes: Vec<DownstreamNodeSummary>,
}

/// Summary of the current process as a node participant.
#[derive(Debug, Serialize)]
pub struct NodeSummary {
    /// Stable node identifier for this process.
    pub node_id: Option<String>,
    /// Runtime node role when configured.
    pub role: Option<NodeRole>,
    /// Current local node availability.
    pub availability: NodeAvailability,
    /// Declared runtime node capabilities.
    pub capabilities: Vec<String>,
    /// Local maximum number of concurrent node tasks.
    pub max_concurrent_tasks: Option<u32>,
    /// Current upstream-session summary when this node connects upward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream: Option<UpstreamConnectionSummary>,
    /// Number of downstream nodes that are currently authenticating.
    pub downstream_authenticating: usize,
    /// Number of downstream nodes that are currently ready.
    pub downstream_ready: usize,
    /// Number of downstream nodes whose sessions are connected but degraded.
    pub downstream_degraded: usize,
    /// Number of downstream nodes that are currently draining.
    pub downstream_draining: usize,
    /// Number of downstream nodes that are currently isolated.
    pub downstream_isolated: usize,
}

/// Summary of one connected downstream node.
#[derive(Debug, Serialize)]
pub struct DownstreamNodeSummary {
    /// Downstream session identifier.
    pub session_id: String,
    /// Stable downstream node identifier.
    pub node_id: String,
    /// Downstream node role.
    pub role: NodeRole,
    /// Downstream node version when hello has completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Current downstream session lifecycle state.
    pub session_state: NodeSessionState,
    /// Current availability state.
    pub availability: NodeAvailability,
    /// Advertised capability identifiers.
    pub capabilities: Vec<String>,
    /// Advertised published platforms.
    pub platforms: Vec<String>,
    /// Advertised maximum number of concurrent node tasks.
    pub max_concurrent_tasks: Option<u32>,
    /// Current number of active node tasks observed for the node.
    pub active_tasks: u32,
    /// Timestamp when the downstream session became ready.
    pub connected_at_ms: u64,
    /// Timestamp of the last heartbeat or advertisement observed from the node.
    pub last_seen_ms: u64,
}

/// Summary of one runtime platform route override.
#[derive(Debug, Serialize)]
pub struct RuntimeRouteSummary {
    /// Platform affected by the override.
    pub platform: Platform,
    /// Current target node id.
    pub route_node: String,
}

/// Source of one effective node route.
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PlatformRouteSource {
    Configured,
    Runtime,
}

/// Summary of the current upstream-session state.
#[derive(Debug, Serialize)]
pub struct UpstreamConnectionSummary {
    /// Whether one upstream WSS session is currently ready.
    pub connected: bool,
    /// Current upstream WSS session lifecycle state.
    pub state: NodeSessionState,
    /// Current or most recent upstream session id when known.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_id: Option<String>,
    /// Stable upstream node identifier when hello has completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node_id: Option<String>,
    /// Upstream node role when hello has completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub role: Option<NodeRole>,
    /// Upstream node version when hello has completed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    /// Upstream-advertised capability identifiers.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub capabilities: Vec<String>,
    /// Upstream-advertised published platforms.
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub platforms: Vec<String>,
    /// Timestamp when the current or most recent upstream session became ready.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connected_at_ms: Option<u64>,
    /// Timestamp when the current upstream session was last observed alive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_seen_ms: Option<u64>,
    /// Timestamp when the most recent upstream session was disconnected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_disconnect_ms: Option<u64>,
    /// Most recent connection or handshake failure observed locally.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
}

/// Health check payload.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Health status.
    pub status: &'static str,
    /// Service name.
    pub service: &'static str,
    /// Service version.
    pub version: &'static str,
    /// Node-transport health summary when node mode is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub node: Option<NodeHealthSummary>,
}

/// Health-oriented summary of the current process as a node participant.
#[derive(Debug, Serialize)]
pub struct NodeHealthSummary {
    /// Stable node identifier for this process.
    pub node_id: Option<String>,
    /// Runtime node role when configured.
    pub role: Option<NodeRole>,
    /// Current local node availability.
    pub availability: NodeAvailability,
    /// Whether the configured node transport is currently ready for traffic.
    pub ready: bool,
    /// Current upstream-session summary when this node connects upward.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub upstream: Option<UpstreamConnectionSummary>,
    /// Number of currently connected downstream nodes.
    pub downstream_total: usize,
    /// Number of downstream nodes that are currently authenticating.
    pub downstream_authenticating: usize,
    /// Number of downstream nodes that are currently ready.
    pub downstream_ready: usize,
    /// Number of downstream nodes whose sessions are connected but degraded.
    pub downstream_degraded: usize,
    /// Number of downstream nodes that are currently draining.
    pub downstream_draining: usize,
    /// Number of downstream nodes that are currently isolated.
    pub downstream_isolated: usize,
}

/// Catalog payload for every platform.
#[derive(Debug, Serialize)]
pub struct ApiCatalogResponse {
    /// Service version used to build the catalog.
    pub version: &'static str,
    /// Every published platform catalog.
    pub platforms: Vec<PlatformCatalogResponse>,
}

/// Catalog payload for one platform.
#[derive(Debug, Serialize)]
pub struct PlatformCatalogResponse {
    /// Platform this catalog belongs to.
    pub platform: Platform,
    /// Shared API base path for the platform.
    pub api_base_path: &'static str,
    /// Number of published methods for the platform.
    pub method_count: usize,
    /// Every published method for the platform.
    pub methods: Vec<ApiMethodSpec>,
}

/// Error payload returned for invalid catalog requests.
#[derive(Debug, Serialize)]
pub struct CatalogErrorResponse {
    /// Human-readable error message.
    pub error: &'static str,
    /// Raw platform segment from the request path.
    pub platform: String,
}

/// Error payload returned for fetch failures.
#[derive(Debug, Serialize)]
pub struct FetchErrorResponse {
    /// Human-readable error category.
    pub error: &'static str,
    /// Detailed error message.
    pub detail: String,
}

/// Error payload returned by internal control endpoints.
#[derive(Debug, Serialize)]
pub struct ControlErrorResponse {
    /// Stable control-plane error identifier.
    pub error: &'static str,
    /// Detailed error message.
    pub detail: String,
}

/// Generic acknowledgement payload returned by internal control endpoints.
#[derive(Debug, Serialize)]
pub struct ControlActionResponse {
    /// Whether the action was accepted locally.
    pub ok: bool,
    /// Stable action identifier.
    pub action: &'static str,
    /// Optional logical target such as `upstream` or one downstream node id.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Optional human-readable detail.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
}

/// Result payload returned after mutating runtime platform routes locally.
#[derive(Debug, Serialize)]
pub struct RuntimeRouteMutationResponse {
    /// Whether the mutation batch was accepted locally.
    pub ok: bool,
    /// Every runtime route currently installed after the mutation.
    pub runtime_routes: Vec<RuntimeRouteSummary>,
    /// Platforms whose runtime override was cleared by this mutation batch.
    pub cleared_platforms: Vec<Platform>,
}
