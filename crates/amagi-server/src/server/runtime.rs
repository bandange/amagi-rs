//! Runtime configuration for node-aware server behavior.

use std::collections::HashMap;
use std::fmt;
use std::io;
use std::time::Duration;

use reqwest::Client;
use serde::Serialize;

use crate::node::NodeRole;
use amagi_core::AppError;
use amagi_core::Platform;
use amagi_core::{dotenv_values, env_or_dotenv};

const DEFAULT_PROXY_TIMEOUT_MS: u64 = 15_000;
const DEFAULT_PROXY_MAX_HOPS: u32 = 4;
const DEFAULT_NODE_HEARTBEAT_MS: u64 = 10_000;
const DEFAULT_NODE_REQUEST_TIMEOUT_MS: u64 = 15_000;
const DEFAULT_NODE_MAX_HOPS: u32 = 4;
const DEFAULT_NODE_MAX_CONCURRENT_TASKS: u32 = 8;

/// Serving behavior for one platform.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PlatformServeMode {
    /// Serve the platform from the current node.
    #[default]
    Local,
    /// Proxy the platform to an upstream node.
    Upstream,
    /// Keep the route shape but reject requests for the platform.
    Disabled,
}

/// Per-platform serving policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformServePolicy {
    /// Serving mode for the platform.
    pub mode: PlatformServeMode,
    /// Explicit node target used by the WSS transport when present.
    pub route_node: Option<String>,
    /// Upstream base URL used when the platform is proxied.
    pub upstream: Option<String>,
}

/// Shared proxy controls for upstream forwarding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyRuntimeConfig {
    /// Request timeout for node-to-node proxy calls.
    pub timeout_ms: u64,
    /// Maximum number of proxy hops allowed for a request.
    pub max_hops: u32,
}

/// Resolved WSS node runtime configuration for this process.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct NodeRuntimeConfig {
    pub node_id: String,
    pub role: NodeRole,
    pub accept_downstream: bool,
    pub connect_upstream: Option<String>,
    pub auth_token: String,
    pub auth_credentials: HashMap<String, String>,
    pub control_token: Option<String>,
    pub allow_insecure_ws: bool,
    pub heartbeat_ms: u64,
    pub request_timeout_ms: u64,
    pub max_hops: u32,
    pub max_concurrent_tasks: u32,
    pub auto_claim_published_routes: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum PlatformRouteDirective {
    Local,
    Disabled,
    Node(String),
}

/// Resolved server runtime configuration.
#[derive(Debug, Clone)]
pub struct ServerRuntimeConfig {
    proxy: ProxyRuntimeConfig,
    platforms: HashMap<Platform, PlatformServePolicy>,
    node: Option<NodeRuntimeConfig>,
}

impl ServerRuntimeConfig {
    /// Resolve the runtime configuration from process env and layered dotenv,
    /// with optional per-process overrides applied first.
    ///
    /// # Errors
    ///
    /// Returns an error when the configuration contains invalid values.
    pub fn from_env_with_overrides<F>(overrides: F) -> Result<Self, AppError>
    where
        F: Fn(&str) -> Option<String>,
    {
        let dotenv = dotenv_values()?;
        Self::from_value_lookup(|name| overrides(name).or_else(|| env_or_dotenv(name, &dotenv)))
    }

    fn from_value_lookup<F>(lookup: F) -> Result<Self, AppError>
    where
        F: Fn(&str) -> Option<String>,
    {
        let proxy = ProxyRuntimeConfig {
            timeout_ms: resolve_u64("AMAGI_PROXY_TIMEOUT_MS", &lookup, DEFAULT_PROXY_TIMEOUT_MS)?,
            max_hops: resolve_u32("AMAGI_PROXY_MAX_HOPS", &lookup, DEFAULT_PROXY_MAX_HOPS)?,
        };

        let platforms = Platform::ALL
            .into_iter()
            .map(|platform| {
                let mode = parse_platform_mode(lookup(platform_mode_env(platform)))?;
                let route = parse_platform_route(
                    platform_route_env(platform),
                    lookup(platform_route_env(platform)),
                )?;
                let upstream = normalize_string(lookup(platform_upstream_env(platform)));
                let (mode, route_node) = match route {
                    None => (mode, None),
                    Some(PlatformRouteDirective::Local) => (PlatformServeMode::Local, None),
                    Some(PlatformRouteDirective::Disabled) => (PlatformServeMode::Disabled, None),
                    Some(PlatformRouteDirective::Node(node_id)) => {
                        (PlatformServeMode::Upstream, Some(node_id))
                    }
                };
                Ok((
                    platform,
                    PlatformServePolicy {
                        mode,
                        route_node,
                        upstream,
                    },
                ))
            })
            .collect::<Result<HashMap<_, _>, AppError>>()?;

        let node = resolve_node_runtime_config(&lookup)?;

        let config = Self {
            proxy,
            platforms,
            node,
        };

        config.validate()?;
        Ok(config)
    }

    /// Return the maximum allowed proxy hop count.
    pub const fn proxy_max_hops(&self) -> u32 {
        self.proxy.max_hops
    }

    /// Return the resolved serving policy for a platform.
    pub fn platform_policy(&self, platform: Platform) -> &PlatformServePolicy {
        self.platforms
            .get(&platform)
            .expect("every supported platform should have a runtime policy")
    }

    /// Return the resolved mode for a platform.
    pub fn platform_mode(&self, platform: Platform) -> PlatformServeMode {
        self.platform_policy(platform).mode
    }

    /// Return whether the platform is published by the current node.
    pub fn is_platform_published(&self, platform: Platform) -> bool {
        !matches!(self.platform_mode(platform), PlatformServeMode::Disabled)
    }

    /// Return the configured upstream base URL for a platform when present.
    pub fn platform_upstream(&self, platform: Platform) -> Option<&str> {
        self.platform_policy(platform).upstream.as_deref()
    }

    /// Return the configured WSS target node id for a platform when present.
    pub fn platform_route_node(&self, platform: Platform) -> Option<&str> {
        self.platform_policy(platform).route_node.as_deref()
    }

    /// Return the resolved node transport config when WSS mode is enabled.
    pub(crate) fn node_config(&self) -> Option<&NodeRuntimeConfig> {
        self.node.as_ref()
    }

    /// Return the configured node id when present.
    pub(crate) fn node_id(&self) -> Option<&str> {
        self.node_config().map(|config| config.node_id.as_str())
    }

    /// Return the configured node role when present.
    pub(crate) fn node_role(&self) -> Option<NodeRole> {
        self.node_config().map(|config| config.role)
    }

    /// Return whether downstream node sessions should be accepted.
    pub(crate) fn accepts_downstream_nodes(&self) -> bool {
        self.node_config()
            .is_some_and(|config| config.accept_downstream)
    }

    /// Return the configured upstream WSS URL when present.
    pub(crate) fn node_connect_upstream(&self) -> Option<&str> {
        self.node_config()
            .and_then(|config| config.connect_upstream.as_deref())
    }

    /// Return the shared node auth token when present.
    pub(crate) fn node_auth_token(&self) -> Option<&str> {
        self.node_config().map(|config| config.auth_token.as_str())
    }

    /// Return the bearer token used by internal control APIs when present.
    pub(crate) fn node_control_token(&self) -> Option<&str> {
        self.node_config().map(|config| {
            config
                .control_token
                .as_deref()
                .unwrap_or(config.auth_token.as_str())
        })
    }

    /// Validate one incoming downstream-node bearer token against configured credentials.
    pub(crate) fn validate_incoming_node_token(
        &self,
        node_id: &str,
        candidate: &str,
    ) -> Result<(), &'static str> {
        let Some(config) = self.node_config() else {
            return Err("node auth is not configured on this server");
        };

        if let Some(expected) = config.auth_credentials.get(node_id) {
            return (candidate == expected)
                .then_some(())
                .ok_or("node auth token did not match the configured node credential");
        }

        (candidate == config.auth_token)
            .then_some(())
            .ok_or("node auth token did not match")
    }

    /// Return the configured heartbeat interval for node sessions.
    pub(crate) fn node_heartbeat_ms(&self) -> Option<u64> {
        self.node_config().map(|config| config.heartbeat_ms)
    }

    /// Return the configured timeout budget for one node task.
    #[allow(dead_code)]
    pub(crate) fn node_request_timeout_ms(&self) -> Option<u64> {
        self.node_config().map(|config| config.request_timeout_ms)
    }

    /// Return the configured maximum node-hop count.
    #[allow(dead_code)]
    pub(crate) fn node_max_hops(&self) -> Option<u32> {
        self.node_config().map(|config| config.max_hops)
    }

    /// Return the configured maximum number of local concurrent node tasks.
    pub(crate) fn node_max_concurrent_tasks(&self) -> Option<u32> {
        self.node_config().map(|config| config.max_concurrent_tasks)
    }

    /// Return whether the node should auto-claim its published platform routes upstream.
    pub(crate) fn node_auto_claim_published_routes(&self) -> Option<bool> {
        self.node_config()
            .map(|config| config.auto_claim_published_routes)
    }

    /// Build the shared HTTP client used for proxy requests.
    ///
    /// # Errors
    ///
    /// Returns an error when the client cannot be initialized.
    pub fn build_proxy_client(&self) -> Result<Client, AppError> {
        Client::builder()
            .timeout(Duration::from_millis(self.proxy.timeout_ms))
            .build()
            .map_err(AppError::from)
    }

    fn validate(&self) -> Result<(), AppError> {
        if self.proxy.max_hops == 0 {
            return invalid_config("AMAGI_PROXY_MAX_HOPS must be greater than 0");
        }

        for platform in Platform::ALL {
            let policy = self.platform_policy(platform);
            if policy.route_node.is_some() && self.node.is_none() {
                return invalid_config(format!(
                    "{} requires node transport configuration when set to `node:<id>`",
                    platform_route_env(platform)
                ));
            }

            if matches!(policy.mode, PlatformServeMode::Upstream)
                && policy.upstream.is_none()
                && policy.route_node.is_none()
                && self.node.is_none()
            {
                return invalid_config(format!(
                    "{} requires {} when set to `upstream` unless node transport is configured",
                    platform_mode_env(platform),
                    platform_upstream_env(platform)
                ));
            }
        }

        if let Some(node) = &self.node {
            if node.max_hops == 0 {
                return invalid_config("AMAGI_NODE_MAX_HOPS must be greater than 0");
            }

            if node.heartbeat_ms == 0 {
                return invalid_config("AMAGI_NODE_HEARTBEAT_MS must be greater than 0");
            }

            if node.request_timeout_ms == 0 {
                return invalid_config("AMAGI_NODE_REQUEST_TIMEOUT_MS must be greater than 0");
            }

            if node.max_concurrent_tasks == 0 {
                return invalid_config("AMAGI_NODE_MAX_CONCURRENT_TASKS must be greater than 0");
            }

            if matches!(node.role, NodeRole::Worker | NodeRole::Relay)
                && node.connect_upstream.is_none()
            {
                return invalid_config(
                    "AMAGI_NODE_ROLE worker/relay requires AMAGI_NODE_CONNECT_UPSTREAM",
                );
            }

            if let Some(upstream) = &node.connect_upstream {
                if !upstream.starts_with("wss://") {
                    if !(upstream.starts_with("ws://") && node.allow_insecure_ws) {
                        return invalid_config(
                            "AMAGI_NODE_CONNECT_UPSTREAM must use wss:// unless AMAGI_NODE_ALLOW_INSECURE_WS=true",
                        );
                    }
                }
            }
        }

        Ok(())
    }
}

fn resolve_node_runtime_config<F>(lookup: &F) -> Result<Option<NodeRuntimeConfig>, AppError>
where
    F: Fn(&str) -> Option<String>,
{
    let node_id = normalize_string(lookup("AMAGI_NODE_ID"));
    let node_role = normalize_string(lookup("AMAGI_NODE_ROLE"));
    let connect_upstream = normalize_string(lookup("AMAGI_NODE_CONNECT_UPSTREAM"));
    let auth_token = normalize_string(lookup("AMAGI_NODE_AUTH_TOKEN"));
    let auth_credentials = parse_node_auth_credentials(
        "AMAGI_NODE_AUTH_CREDENTIALS",
        lookup("AMAGI_NODE_AUTH_CREDENTIALS"),
    )?;
    let control_token = normalize_string(lookup("AMAGI_NODE_CONTROL_TOKEN"));
    let allow_insecure_ws = parse_optional_bool(
        "AMAGI_NODE_ALLOW_INSECURE_WS",
        lookup("AMAGI_NODE_ALLOW_INSECURE_WS"),
    )?;
    let accept_downstream_raw = parse_optional_bool(
        "AMAGI_NODE_ACCEPT_DOWNSTREAM",
        lookup("AMAGI_NODE_ACCEPT_DOWNSTREAM"),
    )?;
    let heartbeat_ms = resolve_u64("AMAGI_NODE_HEARTBEAT_MS", lookup, DEFAULT_NODE_HEARTBEAT_MS)?;
    let request_timeout_ms = resolve_u64(
        "AMAGI_NODE_REQUEST_TIMEOUT_MS",
        lookup,
        DEFAULT_NODE_REQUEST_TIMEOUT_MS,
    )?;
    let max_hops = resolve_u32("AMAGI_NODE_MAX_HOPS", lookup, DEFAULT_NODE_MAX_HOPS)?;
    let max_concurrent_tasks = resolve_u32(
        "AMAGI_NODE_MAX_CONCURRENT_TASKS",
        lookup,
        DEFAULT_NODE_MAX_CONCURRENT_TASKS,
    )?;
    let auto_claim_published_routes = parse_optional_bool(
        "AMAGI_NODE_AUTO_CLAIM_PUBLISHED_ROUTES",
        lookup("AMAGI_NODE_AUTO_CLAIM_PUBLISHED_ROUTES"),
    )?;

    let node_settings_present = node_id.is_some()
        || node_role.is_some()
        || connect_upstream.is_some()
        || auth_token.is_some()
        || !auth_credentials.is_empty()
        || control_token.is_some()
        || allow_insecure_ws.is_some()
        || accept_downstream_raw.is_some()
        || lookup("AMAGI_NODE_HEARTBEAT_MS").is_some()
        || lookup("AMAGI_NODE_REQUEST_TIMEOUT_MS").is_some()
        || lookup("AMAGI_NODE_MAX_HOPS").is_some()
        || lookup("AMAGI_NODE_MAX_CONCURRENT_TASKS").is_some()
        || auto_claim_published_routes.is_some();

    if !node_settings_present {
        return Ok(None);
    }

    let role = parse_node_role(node_role, connect_upstream.is_some(), accept_downstream_raw)?;
    let accept_downstream = accept_downstream_raw.unwrap_or(default_accept_downstream(role));

    let Some(node_id) = node_id else {
        return invalid_config("AMAGI_NODE_ID is required when node transport is configured");
    };
    let Some(auth_token) = auth_token else {
        return invalid_config(
            "AMAGI_NODE_AUTH_TOKEN is required when node transport is configured",
        );
    };

    Ok(Some(NodeRuntimeConfig {
        node_id,
        role,
        accept_downstream,
        connect_upstream,
        auth_token,
        auth_credentials,
        control_token,
        allow_insecure_ws: allow_insecure_ws.unwrap_or(cfg!(test)),
        heartbeat_ms,
        request_timeout_ms,
        max_hops,
        max_concurrent_tasks,
        auto_claim_published_routes: auto_claim_published_routes.unwrap_or(false),
    }))
}

fn platform_mode_env(platform: Platform) -> &'static str {
    match platform {
        Platform::Bilibili => "AMAGI_PLATFORM_BILIBILI_MODE",
        Platform::Douyin => "AMAGI_PLATFORM_DOUYIN_MODE",
        Platform::Kuaishou => "AMAGI_PLATFORM_KUAISHOU_MODE",
        Platform::Twitter => "AMAGI_PLATFORM_TWITTER_MODE",
        Platform::Xiaohongshu => "AMAGI_PLATFORM_XIAOHONGSHU_MODE",
    }
}

fn platform_upstream_env(platform: Platform) -> &'static str {
    match platform {
        Platform::Bilibili => "AMAGI_PLATFORM_BILIBILI_UPSTREAM",
        Platform::Douyin => "AMAGI_PLATFORM_DOUYIN_UPSTREAM",
        Platform::Kuaishou => "AMAGI_PLATFORM_KUAISHOU_UPSTREAM",
        Platform::Twitter => "AMAGI_PLATFORM_TWITTER_UPSTREAM",
        Platform::Xiaohongshu => "AMAGI_PLATFORM_XIAOHONGSHU_UPSTREAM",
    }
}

fn platform_route_env(platform: Platform) -> &'static str {
    match platform {
        Platform::Bilibili => "AMAGI_PLATFORM_BILIBILI_ROUTE",
        Platform::Douyin => "AMAGI_PLATFORM_DOUYIN_ROUTE",
        Platform::Kuaishou => "AMAGI_PLATFORM_KUAISHOU_ROUTE",
        Platform::Twitter => "AMAGI_PLATFORM_TWITTER_ROUTE",
        Platform::Xiaohongshu => "AMAGI_PLATFORM_XIAOHONGSHU_ROUTE",
    }
}

fn parse_platform_mode(value: Option<String>) -> Result<PlatformServeMode, AppError> {
    match normalize_string(value).as_deref() {
        None => Ok(PlatformServeMode::Local),
        Some("enabled") => Ok(PlatformServeMode::Local),
        Some("local") => Ok(PlatformServeMode::Local),
        Some("upstream") => Ok(PlatformServeMode::Upstream),
        Some("disabled") => Ok(PlatformServeMode::Disabled),
        Some(other) => invalid_config(format!(
            "invalid platform mode value: `{other}`; expected `enabled`, `local`, `upstream`, or `disabled`"
        )),
    }
}

fn parse_platform_route(
    env_name: &str,
    value: Option<String>,
) -> Result<Option<PlatformRouteDirective>, AppError> {
    match normalize_string(value).as_deref() {
        None => Ok(None),
        Some("local") => Ok(Some(PlatformRouteDirective::Local)),
        Some("disabled") => Ok(Some(PlatformRouteDirective::Disabled)),
        Some(route) => {
            let Some(node_id) = route.strip_prefix("node:") else {
                return invalid_config(format!(
                    "invalid value for {env_name}: `{route}`; expected `local`, `disabled`, or `node:<id>`"
                ));
            };

            if node_id.trim().is_empty() {
                return invalid_config(format!(
                    "invalid value for {env_name}: `{route}`; node id must not be empty"
                ));
            }

            Ok(Some(PlatformRouteDirective::Node(
                node_id.trim().to_owned(),
            )))
        }
    }
}

fn parse_node_role(
    value: Option<String>,
    has_connect_upstream: bool,
    accept_downstream: Option<bool>,
) -> Result<NodeRole, AppError> {
    match value.as_deref() {
        None => Ok(infer_node_role(has_connect_upstream, accept_downstream)),
        Some("root") => Ok(NodeRole::Root),
        Some("worker") => Ok(NodeRole::Worker),
        Some("relay") => Ok(NodeRole::Relay),
        Some("hybrid") => Ok(NodeRole::Hybrid),
        Some(other) => invalid_config(format!(
            "invalid value for AMAGI_NODE_ROLE: `{other}`; expected `root`, `worker`, `relay`, or `hybrid`"
        )),
    }
}

fn infer_node_role(has_connect_upstream: bool, accept_downstream: Option<bool>) -> NodeRole {
    match (accept_downstream.unwrap_or(false), has_connect_upstream) {
        (true, true) => NodeRole::Relay,
        (true, false) => NodeRole::Root,
        (false, true) => NodeRole::Worker,
        (false, false) => NodeRole::Hybrid,
    }
}

const fn default_accept_downstream(role: NodeRole) -> bool {
    matches!(role, NodeRole::Root | NodeRole::Relay)
}

fn resolve_u64<F>(env_name: &str, lookup: &F, default: u64) -> Result<u64, AppError>
where
    F: Fn(&str) -> Option<String>,
{
    resolve_number(env_name, lookup)?.map_or(Ok(default), Ok)
}

fn resolve_u32<F>(env_name: &str, lookup: &F, default: u32) -> Result<u32, AppError>
where
    F: Fn(&str) -> Option<String>,
{
    resolve_number(env_name, lookup)?.map_or(Ok(default), Ok)
}

fn resolve_number<T, F>(env_name: &str, lookup: &F) -> Result<Option<T>, AppError>
where
    T: std::str::FromStr,
    T::Err: fmt::Display,
    F: Fn(&str) -> Option<String>,
{
    match lookup(env_name) {
        Some(value) => value.parse::<T>().map(Some).map_err(|error| {
            AppError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid value for {env_name}: {error}"),
            ))
        }),
        None => Ok(None),
    }
}

fn parse_optional_bool(env_name: &str, value: Option<String>) -> Result<Option<bool>, AppError> {
    match normalize_string(value).as_deref() {
        None => Ok(None),
        Some("1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON") => Ok(Some(true)),
        Some("0" | "false" | "FALSE" | "no" | "NO" | "off" | "OFF") => Ok(Some(false)),
        Some(other) => invalid_config(format!("invalid value for {env_name}: `{other}`")),
    }
}

fn parse_node_auth_credentials(
    env_name: &str,
    value: Option<String>,
) -> Result<HashMap<String, String>, AppError> {
    let Some(value) = normalize_string(value) else {
        return Ok(HashMap::new());
    };

    let mut credentials = HashMap::new();
    for entry in value
        .split([',', ';'])
        .map(str::trim)
        .filter(|entry| !entry.is_empty())
    {
        let Some((node_id, token)) = entry.split_once('=') else {
            return invalid_config(format!(
                "invalid value for {env_name}: `{entry}`; expected `node_id=token`"
            ));
        };
        let node_id = node_id.trim();
        let token = token.trim();
        if node_id.is_empty() || token.is_empty() {
            return invalid_config(format!(
                "invalid value for {env_name}: `{entry}`; node id and token must not be empty"
            ));
        }
        if credentials.contains_key(node_id) {
            return invalid_config(format!(
                "invalid value for {env_name}: duplicate credential entry for `{node_id}`"
            ));
        }
        credentials.insert(node_id.to_owned(), token.to_owned());
    }

    Ok(credentials)
}

fn normalize_string(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_owned())
    })
}

fn invalid_config<T>(message: impl Into<String>) -> Result<T, AppError> {
    Err(AppError::Io(io::Error::new(
        io::ErrorKind::InvalidData,
        message.into(),
    )))
}
