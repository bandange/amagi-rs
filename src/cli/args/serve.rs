use clap::Args;

use crate::{DEFAULT_HOST, DEFAULT_PORT};

/// Arguments for the `serve` subcommand.
#[derive(Debug, Args, Clone)]
pub struct ServeArgs {
    /// Host or IP address to bind.
    #[arg(long, env = "AMAGI_HOST", default_value = DEFAULT_HOST)]
    pub host: String,

    /// Port to bind.
    #[arg(long, env = "AMAGI_PORT", default_value_t = DEFAULT_PORT)]
    pub port: u16,

    /// Optional node-aware serving overrides.
    #[command(flatten)]
    pub runtime: ServeRuntimeArgs,
}

/// Node-aware serving overrides accepted by `amagi serve`.
#[derive(Debug, Args, Clone, Default)]
pub struct ServeRuntimeArgs {
    /// Request timeout for node-to-node proxy traffic in milliseconds.
    #[arg(long = "proxy-timeout-ms", env = "AMAGI_PROXY_TIMEOUT_MS")]
    pub proxy_timeout_ms: Option<u64>,

    /// Maximum number of proxy hops allowed for one request.
    #[arg(long = "proxy-max-hops", env = "AMAGI_PROXY_MAX_HOPS")]
    pub proxy_max_hops: Option<u32>,

    /// Stable node identifier used by the WSS node transport.
    #[arg(long = "node-id", env = "AMAGI_NODE_ID")]
    pub node_id: Option<String>,

    /// Runtime node role used to derive WSS behavior.
    #[arg(
        long = "node-role",
        env = "AMAGI_NODE_ROLE",
        value_parser = ["root", "worker", "relay", "hybrid"]
    )]
    pub node_role: Option<String>,

    /// Whether this node accepts downstream WSS sessions.
    #[arg(long = "node-accept-downstream", env = "AMAGI_NODE_ACCEPT_DOWNSTREAM")]
    pub node_accept_downstream: Option<bool>,

    /// Upstream WSS endpoint used when this node actively connects upward.
    #[arg(long = "node-connect-upstream", env = "AMAGI_NODE_CONNECT_UPSTREAM")]
    pub node_connect_upstream: Option<String>,

    /// Shared bearer-like token used by the minimum node auth flow.
    #[arg(long = "node-auth-token", env = "AMAGI_NODE_AUTH_TOKEN")]
    pub node_auth_token: Option<String>,

    /// Optional per-node credential table, for example `worker-a=secret-a,worker-b=secret-b`.
    #[arg(long = "node-auth-credentials", env = "AMAGI_NODE_AUTH_CREDENTIALS")]
    pub node_auth_credentials: Option<String>,

    /// Optional bearer token for internal control APIs. Falls back to node auth token when unset.
    #[arg(long = "node-control-token", env = "AMAGI_NODE_CONTROL_TOKEN")]
    pub node_control_token: Option<String>,

    /// Whether to allow insecure `ws://` upstream transport instead of `wss://`.
    #[arg(long = "node-allow-insecure-ws", env = "AMAGI_NODE_ALLOW_INSECURE_WS")]
    pub node_allow_insecure_ws: Option<bool>,

    /// Heartbeat interval in milliseconds for node sessions.
    #[arg(long = "node-heartbeat-ms", env = "AMAGI_NODE_HEARTBEAT_MS")]
    pub node_heartbeat_ms: Option<u64>,

    /// Timeout budget in milliseconds for one node task.
    #[arg(
        long = "node-request-timeout-ms",
        env = "AMAGI_NODE_REQUEST_TIMEOUT_MS"
    )]
    pub node_request_timeout_ms: Option<u64>,

    /// Maximum number of node hops allowed for one task.
    #[arg(long = "node-max-hops", env = "AMAGI_NODE_MAX_HOPS")]
    pub node_max_hops: Option<u32>,

    /// Maximum number of local node tasks allowed to run at the same time.
    #[arg(
        long = "node-max-concurrent-tasks",
        env = "AMAGI_NODE_MAX_CONCURRENT_TASKS"
    )]
    pub node_max_concurrent_tasks: Option<u32>,

    /// Whether the node should automatically claim its published platforms upstream after connect.
    #[arg(
        long = "node-auto-claim-published-routes",
        env = "AMAGI_NODE_AUTO_CLAIM_PUBLISHED_ROUTES"
    )]
    pub node_auto_claim_published_routes: Option<bool>,

    /// Douyin service mode. `enabled` maps to local handling.
    #[arg(
        long = "douyin-mode",
        env = "AMAGI_PLATFORM_DOUYIN_MODE",
        value_parser = ["enabled", "local", "upstream", "disabled"]
    )]
    pub douyin_mode: Option<String>,

    /// Douyin route target. Accepts `local`, `disabled`, or `node:<id>`.
    #[arg(long = "douyin-route", env = "AMAGI_PLATFORM_DOUYIN_ROUTE")]
    pub douyin_route: Option<String>,

    /// Child-node upstream base URL used when Douyin runs in `upstream` mode.
    #[arg(long = "douyin-upstream", env = "AMAGI_PLATFORM_DOUYIN_UPSTREAM")]
    pub douyin_upstream: Option<String>,

    /// Bilibili service mode. `enabled` maps to local handling.
    #[arg(
        long = "bilibili-mode",
        env = "AMAGI_PLATFORM_BILIBILI_MODE",
        value_parser = ["enabled", "local", "upstream", "disabled"]
    )]
    pub bilibili_mode: Option<String>,

    /// Bilibili route target. Accepts `local`, `disabled`, or `node:<id>`.
    #[arg(long = "bilibili-route", env = "AMAGI_PLATFORM_BILIBILI_ROUTE")]
    pub bilibili_route: Option<String>,

    /// Child-node upstream base URL used when Bilibili runs in `upstream` mode.
    #[arg(long = "bilibili-upstream", env = "AMAGI_PLATFORM_BILIBILI_UPSTREAM")]
    pub bilibili_upstream: Option<String>,

    /// Kuaishou service mode. `enabled` maps to local handling.
    #[arg(
        long = "kuaishou-mode",
        env = "AMAGI_PLATFORM_KUAISHOU_MODE",
        value_parser = ["enabled", "local", "upstream", "disabled"]
    )]
    pub kuaishou_mode: Option<String>,

    /// Kuaishou route target. Accepts `local`, `disabled`, or `node:<id>`.
    #[arg(long = "kuaishou-route", env = "AMAGI_PLATFORM_KUAISHOU_ROUTE")]
    pub kuaishou_route: Option<String>,

    /// Child-node upstream base URL used when Kuaishou runs in `upstream` mode.
    #[arg(long = "kuaishou-upstream", env = "AMAGI_PLATFORM_KUAISHOU_UPSTREAM")]
    pub kuaishou_upstream: Option<String>,

    /// Xiaohongshu service mode. `enabled` maps to local handling.
    #[arg(
        long = "xiaohongshu-mode",
        env = "AMAGI_PLATFORM_XIAOHONGSHU_MODE",
        value_parser = ["enabled", "local", "upstream", "disabled"]
    )]
    pub xiaohongshu_mode: Option<String>,

    /// Xiaohongshu route target. Accepts `local`, `disabled`, or `node:<id>`.
    #[arg(long = "xiaohongshu-route", env = "AMAGI_PLATFORM_XIAOHONGSHU_ROUTE")]
    pub xiaohongshu_route: Option<String>,

    /// Child-node upstream base URL used when Xiaohongshu runs in `upstream` mode.
    #[arg(
        long = "xiaohongshu-upstream",
        env = "AMAGI_PLATFORM_XIAOHONGSHU_UPSTREAM"
    )]
    pub xiaohongshu_upstream: Option<String>,

    /// Twitter/X service mode. `enabled` maps to local handling.
    #[arg(
        long = "twitter-mode",
        env = "AMAGI_PLATFORM_TWITTER_MODE",
        value_parser = ["enabled", "local", "upstream", "disabled"]
    )]
    pub twitter_mode: Option<String>,

    /// Twitter/X route target. Accepts `local`, `disabled`, or `node:<id>`.
    #[arg(long = "twitter-route", env = "AMAGI_PLATFORM_TWITTER_ROUTE")]
    pub twitter_route: Option<String>,

    /// Child-node upstream base URL used when Twitter/X runs in `upstream` mode.
    #[arg(long = "twitter-upstream", env = "AMAGI_PLATFORM_TWITTER_UPSTREAM")]
    pub twitter_upstream: Option<String>,
}
