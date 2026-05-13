//! Shared web application state.

use std::collections::HashMap;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::{Arc, RwLock};

use crate::client::AmagiClient;
use crate::config::ServeConfig;
use crate::error::AppError;
use crate::node::NodeRole;
use crate::node::registry::{NodeAvailability, NodeRegistry};
use crate::node::session::{NodeAuthReplayCache, NodeSessionState};
use crate::node::upstream::{UpstreamConnection, UpstreamConnectionSnapshot, UpstreamPeerInfo};
use tokio::sync::mpsc;

use super::runtime::{PlatformServeMode, ServerRuntimeConfig};
use crate::catalog::Platform;
use crate::node::protocol::{
    NodeAdvertiseParams, NodeCapacityParams, NodeDrainParams, NodeEnvelope, NodeIsolateParams,
    NodeReadyParams, NodeRouteUpdateEntry, NodeRouteUpdateParams, NodeShutdownNoticeParams, now_ms,
};
use serde_json::json;

/// Shared state injected into web handlers.
#[derive(Debug, Clone)]
pub struct AppState {
    /// Crate name used in responses.
    pub app_name: &'static str,
    /// Crate version used in responses.
    pub version: &'static str,
    /// Bound web serving configuration.
    pub serve: ServeConfig,
    /// Shared SDK-style client carrying catalog and request defaults.
    pub client: AmagiClient,
    /// Node-aware runtime routing configuration.
    pub runtime: ServerRuntimeConfig,
    /// Shared HTTP client used for node-to-node proxying.
    pub proxy_client: reqwest::Client,
    /// Online downstream node registry for the WSS transport.
    pub node_registry: NodeRegistry,
    /// Runtime platform-to-node overrides learned after startup.
    pub dynamic_platform_routes: Arc<RwLock<HashMap<Platform, String>>>,
    /// Current upstream WSS session sender when connected.
    pub upstream_connection: UpstreamConnection,
    /// Replay cache for downstream node auth nonces.
    pub node_auth_replay_cache: NodeAuthReplayCache,
    /// Current local node availability used by the control plane.
    pub local_node_availability: Arc<RwLock<NodeAvailability>>,
    /// Optional runtime override for local node task capacity.
    pub local_node_capacity_override: Arc<RwLock<Option<u32>>>,
    /// Current number of active node-routed tasks executing on the local node.
    pub local_node_active_tasks: Arc<AtomicU32>,
}

impl AppState {
    /// Create a new application state container.
    ///
    /// # Errors
    ///
    /// Returns an error if the shared proxy client cannot be initialized.
    pub fn new(
        app_name: &'static str,
        version: &'static str,
        serve: ServeConfig,
        client: AmagiClient,
        runtime: ServerRuntimeConfig,
    ) -> Result<Self, AppError> {
        let proxy_client = runtime.build_proxy_client()?;

        Ok(Self {
            app_name,
            version,
            serve,
            client,
            runtime,
            proxy_client,
            node_registry: NodeRegistry::default(),
            dynamic_platform_routes: Arc::new(RwLock::new(HashMap::new())),
            upstream_connection: UpstreamConnection::default(),
            node_auth_replay_cache: NodeAuthReplayCache::default(),
            local_node_availability: Arc::new(RwLock::new(NodeAvailability::Ready)),
            local_node_capacity_override: Arc::new(RwLock::new(None)),
            local_node_active_tasks: Arc::new(AtomicU32::new(0)),
        })
    }

    /// Return the serving mode for a platform.
    pub fn platform_mode(&self, platform: Platform) -> PlatformServeMode {
        let configured_mode = self.runtime.platform_mode(platform);
        if matches!(configured_mode, PlatformServeMode::Disabled) {
            PlatformServeMode::Disabled
        } else if self.runtime_platform_route_node(platform).is_some() {
            PlatformServeMode::Upstream
        } else {
            configured_mode
        }
    }

    /// Return whether the platform is published by the current node.
    pub fn is_platform_published(&self, platform: Platform) -> bool {
        self.runtime.is_platform_published(platform)
    }

    /// Return the configured upstream base URL for a platform.
    pub fn platform_upstream(&self, platform: Platform) -> Option<&str> {
        self.runtime.platform_upstream(platform)
    }

    /// Return the configured WSS target node id for a platform when present.
    pub fn platform_route_node(&self, platform: Platform) -> Option<String> {
        if matches!(
            self.runtime.platform_mode(platform),
            PlatformServeMode::Disabled
        ) {
            None
        } else {
            self.runtime_platform_route_node(platform).or_else(|| {
                self.runtime
                    .platform_route_node(platform)
                    .map(str::to_owned)
            })
        }
    }

    /// Return whether the platform route currently comes from one runtime override.
    pub fn platform_route_is_runtime(&self, platform: Platform) -> bool {
        !matches!(
            self.runtime.platform_mode(platform),
            PlatformServeMode::Disabled
        ) && self.runtime_platform_route_node(platform).is_some()
    }

    /// Return the maximum allowed proxy hop count.
    pub const fn proxy_max_hops(&self) -> u32 {
        self.runtime.proxy_max_hops()
    }

    /// Return the configured node id when the WSS node transport is enabled.
    pub fn node_id(&self) -> Option<&str> {
        self.runtime.node_id()
    }

    /// Return the configured node role when the WSS node transport is enabled.
    pub fn node_role(&self) -> Option<NodeRole> {
        self.runtime.node_role()
    }

    /// Return whether this node accepts downstream WSS connections.
    pub fn accepts_downstream_nodes(&self) -> bool {
        self.runtime.accepts_downstream_nodes()
    }

    /// Return the configured upstream WSS URL when present.
    pub fn node_connect_upstream(&self) -> Option<&str> {
        self.runtime.node_connect_upstream()
    }

    /// Return the shared node auth token when configured.
    pub fn node_auth_token(&self) -> Option<&str> {
        self.runtime.node_auth_token()
    }

    /// Return the bearer token used by internal control APIs when configured.
    pub fn node_control_token(&self) -> Option<&str> {
        self.runtime.node_control_token()
    }

    /// Validate one incoming downstream-node auth token.
    pub fn validate_incoming_node_auth(
        &self,
        node_id: &str,
        candidate: &str,
    ) -> Result<(), AppError> {
        self.runtime
            .validate_incoming_node_token(node_id, candidate)
            .map_err(|message| AppError::InvalidRequestConfig(message.to_owned()))
    }

    /// Register one downstream-node auth nonce and reject replays.
    pub fn register_node_auth_nonce(&self, node_id: &str, nonce: &str) -> Result<(), AppError> {
        self.node_auth_replay_cache
            .register(node_id, nonce)
            .map_err(|message| AppError::InvalidRequestConfig(message.to_owned()))
    }

    /// Return the configured heartbeat interval for node sessions.
    pub fn node_heartbeat_ms(&self) -> Option<u64> {
        self.runtime.node_heartbeat_ms()
    }

    /// Return the configured request-timeout budget for one node task.
    #[allow(dead_code)]
    pub fn node_request_timeout_ms(&self) -> Option<u64> {
        self.runtime.node_request_timeout_ms()
    }

    /// Return the configured maximum WSS routing hop count.
    #[allow(dead_code)]
    pub fn node_max_hops(&self) -> Option<u32> {
        self.runtime.node_max_hops()
    }

    /// Return the configured maximum number of local concurrent node tasks.
    pub fn node_max_concurrent_tasks(&self) -> Option<u32> {
        self.local_node_capacity_override()
            .or_else(|| self.runtime.node_max_concurrent_tasks())
    }

    /// Return the runtime-local capacity override when present.
    pub fn local_node_capacity_override(&self) -> Option<u32> {
        *self
            .local_node_capacity_override
            .read()
            .expect("local node capacity override should be readable")
    }

    /// Set or clear the runtime-local capacity override.
    pub fn set_local_node_capacity_override(&self, max_concurrent_tasks: Option<u32>) {
        let mut guard = self
            .local_node_capacity_override
            .write()
            .expect("local node capacity override should be writable");
        *guard = max_concurrent_tasks;
    }

    /// Return the current number of active node-routed tasks executing locally.
    pub fn local_node_active_tasks(&self) -> u32 {
        self.local_node_active_tasks.load(Ordering::Relaxed)
    }

    /// Update the current number of active node-routed tasks executing locally.
    pub fn set_local_node_active_tasks(&self, active_tasks: u32) {
        self.local_node_active_tasks
            .store(active_tasks, Ordering::Relaxed);
    }

    /// Return the current local node availability.
    pub fn local_node_availability(&self) -> NodeAvailability {
        *self
            .local_node_availability
            .read()
            .expect("local node availability should be readable")
    }

    /// Return whether the local node is currently draining.
    pub fn is_local_node_draining(&self) -> bool {
        matches!(self.local_node_availability(), NodeAvailability::Draining)
    }

    /// Return whether the local node is currently isolated.
    pub fn is_local_node_isolated(&self) -> bool {
        matches!(self.local_node_availability(), NodeAvailability::Isolated)
    }

    /// Set the current local node availability.
    pub fn set_local_node_availability(&self, availability: NodeAvailability) {
        let mut guard = self
            .local_node_availability
            .write()
            .expect("local node availability should be writable");
        *guard = availability;
    }

    /// Return whether the local node is ready to accept new node-routed tasks.
    pub fn is_local_node_ready_for_tasks(&self) -> bool {
        matches!(self.local_node_availability(), NodeAvailability::Ready)
    }

    /// Mark the local node as draining and synchronise upstream state when configured.
    pub fn enter_local_node_drain(&self, reason: Option<&str>) -> Vec<String> {
        let already_draining = self.is_local_node_draining();
        self.set_local_node_availability(NodeAvailability::Draining);

        let mut notes = vec![if already_draining {
            "local availability already draining".to_owned()
        } else {
            "local availability set to draining".to_owned()
        }];

        if self.node_connect_upstream().is_some() {
            if let Err(envelope) = self.announce_upstream_drain(reason) {
                notes.push(format!(
                    "upstream drain skipped because no session is ready for `{}`",
                    envelope.method
                ));
            } else {
                notes.push("upstream drain announced".to_owned());
            }

            match self.announce_upstream_advertisement(self.local_node_active_tasks()) {
                Ok(()) => notes.push("upstream advertisement refreshed".to_owned()),
                Err(envelope) => notes.push(format!(
                    "upstream advertisement skipped because no session is ready for `{}`",
                    envelope.method
                )),
            }
        }

        notes
    }

    /// Mark the local node as isolated and synchronise upstream state when configured.
    pub fn enter_local_node_isolation(&self, reason: Option<&str>) -> Vec<String> {
        let already_isolated = self.is_local_node_isolated();
        self.set_local_node_availability(NodeAvailability::Isolated);

        let mut notes = vec![if already_isolated {
            "local availability already isolated".to_owned()
        } else {
            "local availability set to isolated".to_owned()
        }];

        if self.node_connect_upstream().is_some() {
            if let Err(envelope) = self.announce_upstream_isolation(reason) {
                notes.push(format!(
                    "upstream isolation skipped because no session is ready for `{}`",
                    envelope.method
                ));
            } else {
                notes.push("upstream isolation announced".to_owned());
            }

            match self.announce_upstream_advertisement(self.local_node_active_tasks()) {
                Ok(()) => notes.push("upstream advertisement refreshed".to_owned()),
                Err(envelope) => notes.push(format!(
                    "upstream advertisement skipped because no session is ready for `{}`",
                    envelope.method
                )),
            }
        }

        notes
    }

    /// Mark the local node as ready again and synchronise upstream state when configured.
    pub fn exit_local_node_drain(&self) -> Vec<String> {
        self.exit_local_node_restriction_from_upstream()
    }

    /// Mark the local node as ready again and notify the current upstream session when present.
    pub fn restore_local_node_ready(&self, reason: Option<&str>) -> Vec<String> {
        let previous = self.local_node_availability();
        self.set_local_node_availability(NodeAvailability::Ready);

        let mut notes = vec![match previous {
            NodeAvailability::Ready => "local availability already ready".to_owned(),
            NodeAvailability::Draining => {
                "local availability set to ready from draining".to_owned()
            }
            NodeAvailability::Isolated => {
                "local availability set to ready from isolated".to_owned()
            }
        }];

        if self.node_connect_upstream().is_some() {
            if let Err(envelope) = self.announce_upstream_ready(reason) {
                notes.push(format!(
                    "upstream ready skipped because no session is ready for `{}`",
                    envelope.method
                ));
            } else {
                notes.push("upstream ready announced".to_owned());
            }

            match self.announce_upstream_advertisement(self.local_node_active_tasks()) {
                Ok(()) => notes.push("upstream advertisement refreshed".to_owned()),
                Err(envelope) => notes.push(format!(
                    "upstream advertisement skipped because no session is ready for `{}`",
                    envelope.method
                )),
            }
            if self.node_auto_claim_published_routes() {
                match self.claim_published_upstream_routes() {
                    Ok(()) => notes.push("published routes claimed upstream".to_owned()),
                    Err(envelope) => notes.push(format!(
                        "upstream route claim skipped because no session is ready for `{}`",
                        envelope.method
                    )),
                }
            }
        }

        notes
    }

    /// Mark the local node as ready again without notifying upstream.
    pub fn exit_local_node_restriction_from_upstream(&self) -> Vec<String> {
        let previous = self.local_node_availability();
        self.set_local_node_availability(NodeAvailability::Ready);

        let mut notes = vec![match previous {
            NodeAvailability::Ready => "local availability already ready".to_owned(),
            NodeAvailability::Draining => {
                "local availability set to ready from draining".to_owned()
            }
            NodeAvailability::Isolated => {
                "local availability set to ready from isolated".to_owned()
            }
        }];

        if self.node_connect_upstream().is_some() {
            match self.announce_upstream_advertisement(self.local_node_active_tasks()) {
                Ok(()) => notes.push("upstream advertisement refreshed".to_owned()),
                Err(envelope) => notes.push(format!(
                    "upstream advertisement skipped because no session is ready for `{}`",
                    envelope.method
                )),
            }
            if self.node_auto_claim_published_routes() {
                match self.claim_published_upstream_routes() {
                    Ok(()) => notes.push("published routes claimed upstream".to_owned()),
                    Err(envelope) => notes.push(format!(
                        "upstream route claim skipped because no session is ready for `{}`",
                        envelope.method
                    )),
                }
            }
        }

        notes
    }

    /// Return whether the node should auto-claim its published platform routes upstream.
    pub fn node_auto_claim_published_routes(&self) -> bool {
        self.runtime
            .node_auto_claim_published_routes()
            .unwrap_or(false)
    }

    /// Return a clone of the shared node registry.
    pub fn node_registry(&self) -> NodeRegistry {
        self.node_registry.clone()
    }

    /// Install or replace one runtime node-route override for a platform.
    pub fn set_runtime_platform_route(&self, platform: Platform, node_id: impl Into<String>) {
        let mut guard = self
            .dynamic_platform_routes
            .write()
            .expect("dynamic platform routes should be writable");
        guard.insert(platform, node_id.into());
    }

    /// Remove one runtime node-route override for a platform owned by the node.
    pub fn clear_runtime_platform_route(&self, platform: Platform, node_id: &str) -> bool {
        let mut guard = self
            .dynamic_platform_routes
            .write()
            .expect("dynamic platform routes should be writable");
        if guard
            .get(&platform)
            .is_some_and(|current| current == node_id)
        {
            guard.remove(&platform);
            return true;
        }
        false
    }

    /// Remove one runtime node-route override regardless of the owning node.
    pub fn remove_runtime_platform_route(&self, platform: Platform) -> Option<String> {
        let mut guard = self
            .dynamic_platform_routes
            .write()
            .expect("dynamic platform routes should be writable");
        guard.remove(&platform)
    }

    /// Remove every runtime node-route override pointing at the node.
    pub fn clear_runtime_platform_routes_for_node(&self, node_id: &str) -> usize {
        let mut guard = self
            .dynamic_platform_routes
            .write()
            .expect("dynamic platform routes should be writable");
        let original_len = guard.len();
        guard.retain(|_, current| current != node_id);
        original_len.saturating_sub(guard.len())
    }

    /// Return a snapshot of every runtime node-route override.
    pub fn runtime_platform_routes(&self) -> Vec<(Platform, String)> {
        let guard = self
            .dynamic_platform_routes
            .read()
            .expect("dynamic platform routes should be readable");
        let mut routes = guard
            .iter()
            .map(|(platform, node_id)| (*platform, node_id.clone()))
            .collect::<Vec<_>>();
        routes.sort_by(|left, right| left.0.as_str().cmp(right.0.as_str()));
        routes
    }

    /// Mark the current upstream session as ready and remember peer metadata.
    pub fn set_upstream_connection_ready(
        &self,
        sender: mpsc::UnboundedSender<NodeEnvelope>,
        peer: UpstreamPeerInfo,
    ) {
        self.upstream_connection.set_ready(sender, peer);
    }

    /// Set the current upstream-session lifecycle state.
    pub fn set_upstream_connection_state(&self, session_state: NodeSessionState) {
        self.upstream_connection.set_state(session_state);
    }

    /// Mark the current upstream session as disconnected and remember the failure reason.
    pub fn record_upstream_disconnect(
        &self,
        next_state: NodeSessionState,
        last_error: Option<String>,
    ) {
        self.upstream_connection
            .record_disconnect(next_state, last_error);
    }

    /// Mark the upstream session as recently observed.
    pub fn touch_upstream_connection(&self) {
        self.upstream_connection.touch();
    }

    /// Return the current upstream-session connectivity snapshot.
    pub fn upstream_connection_snapshot(&self) -> UpstreamConnectionSnapshot {
        let mut snapshot = self.upstream_connection.snapshot();
        if snapshot.connected
            && snapshot.state == NodeSessionState::Ready
            && self
                .node_heartbeat_ms()
                .zip(snapshot.last_seen_ms)
                .is_some_and(|(heartbeat_ms, last_seen_ms)| {
                    now_ms().saturating_sub(last_seen_ms) > heartbeat_ms.saturating_mul(2)
                })
        {
            snapshot.state = NodeSessionState::Degraded;
        }
        snapshot
    }

    /// Send one node envelope to the current upstream session when connected.
    pub fn send_upstream_envelope(&self, envelope: NodeEnvelope) -> Result<(), NodeEnvelope> {
        self.upstream_connection.send(envelope)
    }

    /// Send one advertisement refresh to the current upstream session when connected.
    pub fn announce_upstream_advertisement(&self, active_tasks: u32) -> Result<(), NodeEnvelope> {
        let mut envelope = NodeEnvelope::request(
            "node.advertise",
            json!(self.current_node_advertisement(active_tasks)),
        );
        envelope.from = self.node_id().map(str::to_owned);
        self.send_upstream_envelope(envelope)
    }

    /// Send one route-update request to the current upstream session when connected.
    pub fn announce_upstream_route_update(
        &self,
        updates: Vec<NodeRouteUpdateEntry>,
    ) -> Result<(), NodeEnvelope> {
        let mut envelope =
            NodeEnvelope::request("route.update", json!(NodeRouteUpdateParams { updates }));
        envelope.from = self.node_id().map(str::to_owned);
        self.send_upstream_envelope(envelope)
    }

    /// Claim every currently locally executable platform on the upstream node.
    pub fn claim_published_upstream_routes(&self) -> Result<(), NodeEnvelope> {
        let Some(node_id) = self.node_id() else {
            return Ok(());
        };
        let updates = self
            .advertised_platforms()
            .into_iter()
            .map(|platform| NodeRouteUpdateEntry {
                platform,
                route_node: Some(node_id.to_owned()),
            })
            .collect::<Vec<_>>();
        if updates.is_empty() {
            return Ok(());
        }
        self.announce_upstream_route_update(updates)
    }

    /// Release every currently locally executable platform on the upstream node.
    pub fn release_published_upstream_routes(&self) -> Result<(), NodeEnvelope> {
        let updates = self
            .advertised_platforms()
            .into_iter()
            .map(|platform| NodeRouteUpdateEntry {
                platform,
                route_node: None,
            })
            .collect::<Vec<_>>();
        if updates.is_empty() {
            return Ok(());
        }
        self.announce_upstream_route_update(updates)
    }

    /// Notify the current upstream session that this node is draining.
    pub fn announce_upstream_drain(&self, reason: Option<&str>) -> Result<(), NodeEnvelope> {
        let mut envelope = NodeEnvelope::request(
            "node.drain",
            json!(NodeDrainParams {
                reason: reason.map(str::to_owned),
            }),
        );
        envelope.from = self.node_id().map(str::to_owned);
        self.send_upstream_envelope(envelope)
    }

    /// Notify the current upstream session that this node is ready again.
    pub fn announce_upstream_ready(&self, reason: Option<&str>) -> Result<(), NodeEnvelope> {
        let mut envelope = NodeEnvelope::request(
            "node.ready",
            json!(NodeReadyParams {
                reason: reason.map(str::to_owned),
            }),
        );
        envelope.from = self.node_id().map(str::to_owned);
        self.send_upstream_envelope(envelope)
    }

    /// Notify the current upstream session that this node is isolated.
    pub fn announce_upstream_isolation(&self, reason: Option<&str>) -> Result<(), NodeEnvelope> {
        let mut envelope = NodeEnvelope::request(
            "node.isolate",
            json!(NodeIsolateParams {
                reason: reason.map(str::to_owned),
            }),
        );
        envelope.from = self.node_id().map(str::to_owned);
        self.send_upstream_envelope(envelope)
    }

    /// Notify the current upstream session that this node has a new capacity override.
    pub fn announce_upstream_capacity(
        &self,
        max_concurrent_tasks: Option<u32>,
        reason: Option<&str>,
    ) -> Result<(), NodeEnvelope> {
        let mut envelope = NodeEnvelope::request(
            "node.capacity",
            json!(NodeCapacityParams {
                max_concurrent_tasks,
                reason: reason.map(str::to_owned),
            }),
        );
        envelope.from = self.node_id().map(str::to_owned);
        self.send_upstream_envelope(envelope)
    }

    /// Broadcast a shutdown notice to every connected downstream node.
    pub fn broadcast_shutdown_notice(&self, reason: Option<&str>, reconnect_after_ms: Option<u64>) {
        let _ = self.broadcast_shutdown_notice_count(reason, reconnect_after_ms);
    }

    /// Send one shutdown notice to one connected downstream node.
    pub fn send_shutdown_notice_to_node(
        &self,
        node_id: &str,
        reason: Option<&str>,
        reconnect_after_ms: Option<u64>,
    ) -> Result<(), AppError> {
        let Some((record, sender)) = self.node_registry().sender_for_node(node_id) else {
            return Err(AppError::InvalidRequestConfig(format!(
                "downstream node `{node_id}` is not connected"
            )));
        };
        let envelope = NodeEnvelope::event(
            "node.shutdown_notice",
            self.node_id().map(str::to_owned),
            Some(record.node_id.clone()),
            Some(record.session_id.clone()),
            json!(NodeShutdownNoticeParams {
                reason: reason.map(str::to_owned),
                reconnect_after_ms,
            }),
        );
        sender.send(envelope).map_err(|error| {
            AppError::InvalidRequestConfig(format!(
                "failed to send shutdown notice to downstream node `{node_id}`: {}",
                error.0.method
            ))
        })
    }

    /// Broadcast one shutdown notice to every connected downstream node and return the count.
    pub fn broadcast_shutdown_notice_count(
        &self,
        reason: Option<&str>,
        reconnect_after_ms: Option<u64>,
    ) -> usize {
        let local_node_id = self.node_id().map(str::to_owned);
        self.node_registry()
            .downstream_senders()
            .into_iter()
            .map(|(record, sender)| {
                let envelope = NodeEnvelope::event(
                    "node.shutdown_notice",
                    local_node_id.clone(),
                    Some(record.node_id.clone()),
                    Some(record.session_id.clone()),
                    json!(NodeShutdownNoticeParams {
                        reason: reason.map(str::to_owned),
                        reconnect_after_ms,
                    }),
                );
                usize::from(sender.send(envelope).is_ok())
            })
            .sum()
    }

    /// Return every platform identifier currently served by this node, locally or by proxying.
    pub fn published_platform_names(&self) -> Vec<String> {
        self.advertised_platforms()
            .into_iter()
            .map(|platform| platform.to_string())
            .collect()
    }

    /// Return the runtime node advertisement payload for the current local state.
    pub fn current_node_advertisement(&self, active_tasks: u32) -> NodeAdvertiseParams {
        let max_concurrent_tasks =
            matches!(self.local_node_availability(), NodeAvailability::Ready)
                .then(|| self.node_max_concurrent_tasks())
                .flatten();
        let platforms = self.published_platform_names();

        NodeAdvertiseParams {
            capabilities: self.node_capabilities(),
            platforms,
            max_concurrent_tasks,
            active_tasks: Some(active_tasks),
        }
    }

    /// Return the runtime node capability identifiers exposed by this process.
    pub fn node_capabilities(&self) -> Vec<String> {
        let mut capabilities = Vec::new();

        if self.accepts_downstream_nodes() {
            capabilities.push("accept_downstream".to_owned());
        }
        if self.node_connect_upstream().is_some() {
            capabilities.push("connect_upstream".to_owned());
        }
        if self.has_locally_executable_platforms() {
            capabilities.push("execute_local".to_owned());
        }
        if self.accepts_downstream_nodes() && self.node_connect_upstream().is_some() {
            capabilities.push("forward_remote".to_owned());
        }
        if self.accepts_downstream_nodes() {
            capabilities.push("expose_public_api".to_owned());
        }

        capabilities
    }

    fn has_locally_executable_platforms(&self) -> bool {
        self.locally_executable_platforms()
            .into_iter()
            .next()
            .is_some()
    }

    fn advertised_platforms(&self) -> Vec<Platform> {
        Platform::ALL
            .into_iter()
            .filter(|platform| self.is_platform_published(*platform))
            .collect()
    }

    fn runtime_platform_route_node(&self, platform: Platform) -> Option<String> {
        let guard = self
            .dynamic_platform_routes
            .read()
            .expect("dynamic platform routes should be readable");
        guard.get(&platform).cloned()
    }

    fn locally_executable_platforms(&self) -> Vec<Platform> {
        Platform::ALL
            .into_iter()
            .filter(|platform| matches!(self.platform_mode(*platform), PlatformServeMode::Local))
            .collect()
    }
}
