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

    /// Douyin service mode. `enabled` maps to local handling.
    #[arg(
        long = "douyin-mode",
        env = "AMAGI_PLATFORM_DOUYIN_MODE",
        value_parser = ["enabled", "local", "upstream", "disabled"]
    )]
    pub douyin_mode: Option<String>,

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

    /// Child-node upstream base URL used when Twitter/X runs in `upstream` mode.
    #[arg(long = "twitter-upstream", env = "AMAGI_PLATFORM_TWITTER_UPSTREAM")]
    pub twitter_upstream: Option<String>,
}
