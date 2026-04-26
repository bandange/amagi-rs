#[cfg(feature = "cli")]
use crate::client::ClientOptions;

/// Resolved runtime configuration assembled from CLI input.
#[cfg(feature = "cli")]
#[derive(Debug, Clone)]
pub struct AppConfig {
    /// Selected application command.
    pub command: CommandConfig,
    /// Output behavior for stdout messages.
    pub output: crate::config::OutputConfig,
    /// Logging behavior for stderr events.
    pub logging: crate::config::LoggingConfig,
    /// Shared client configuration used by the client, CLI, and server flows.
    pub client: ClientOptions,
}

/// Supported top-level application commands.
#[cfg(feature = "cli")]
#[derive(Debug, Clone)]
pub enum CommandConfig {
    /// Run local CLI workflows.
    Run(RunConfig),
    #[cfg(feature = "server")]
    /// Start the built-in HTTP server.
    Serve(ServeConfig),
}

/// Runtime options for local CLI execution.
#[cfg(feature = "cli")]
#[derive(Debug, Clone, Default)]
pub struct RunConfig {
    /// Suppress normal startup output without affecting structured logs.
    pub quiet: bool,
    /// Selected CLI task to execute.
    pub task: crate::config::RunTask,
}

/// Network options for the built-in HTTP server.
#[cfg(feature = "server")]
#[doc(alias = "server")]
#[derive(Debug, Clone)]
pub struct ServeConfig {
    /// Host or IP address to bind.
    pub host: String,
    /// Port to bind.
    pub port: u16,
    /// Optional node-aware runtime overrides for this process.
    pub runtime_overrides: ServeRuntimeOverrides,
}

/// Optional node-aware runtime overrides accepted by `amagi serve`.
#[cfg(feature = "server")]
#[derive(Debug, Clone, Default)]
pub struct ServeRuntimeOverrides {
    /// Override for `AMAGI_PROXY_TIMEOUT_MS`.
    pub proxy_timeout_ms: Option<u64>,
    /// Override for `AMAGI_PROXY_MAX_HOPS`.
    pub proxy_max_hops: Option<u32>,
    /// Override for `AMAGI_PLATFORM_DOUYIN_MODE`.
    pub douyin_mode: Option<String>,
    /// Override for `AMAGI_PLATFORM_DOUYIN_UPSTREAM`.
    pub douyin_upstream: Option<String>,
    /// Override for `AMAGI_PLATFORM_BILIBILI_MODE`.
    pub bilibili_mode: Option<String>,
    /// Override for `AMAGI_PLATFORM_BILIBILI_UPSTREAM`.
    pub bilibili_upstream: Option<String>,
    /// Override for `AMAGI_PLATFORM_KUAISHOU_MODE`.
    pub kuaishou_mode: Option<String>,
    /// Override for `AMAGI_PLATFORM_KUAISHOU_UPSTREAM`.
    pub kuaishou_upstream: Option<String>,
    /// Override for `AMAGI_PLATFORM_XIAOHONGSHU_MODE`.
    pub xiaohongshu_mode: Option<String>,
    /// Override for `AMAGI_PLATFORM_XIAOHONGSHU_UPSTREAM`.
    pub xiaohongshu_upstream: Option<String>,
    /// Override for `AMAGI_PLATFORM_TWITTER_MODE`.
    pub twitter_mode: Option<String>,
    /// Override for `AMAGI_PLATFORM_TWITTER_UPSTREAM`.
    pub twitter_upstream: Option<String>,
}

#[cfg(feature = "server")]
impl ServeConfig {
    /// Return the `host:port` socket address string for this configuration.
    ///
    /// This method formats the configured values without performing DNS or
    /// socket validation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use amagi::config::ServeConfig;
    ///
    /// let serve = ServeConfig {
    ///     host: "127.0.0.1".into(),
    ///     port: 4567,
    ///     runtime_overrides: Default::default(),
    /// };
    ///
    /// assert_eq!(serve.bind_addr(), "127.0.0.1:4567");
    /// ```
    #[doc(alias = "bind")]
    #[doc(alias = "socket_addr")]
    pub fn bind_addr(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }

    /// Return the HTTP origin exposed by this configuration.
    ///
    /// This method assumes the built-in server is served over plain HTTP.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use amagi::config::ServeConfig;
    ///
    /// let serve = ServeConfig {
    ///     host: "127.0.0.1".into(),
    ///     port: 4567,
    ///     runtime_overrides: Default::default(),
    /// };
    ///
    /// assert_eq!(serve.base_url(), "http://127.0.0.1:4567");
    /// ```
    #[doc(alias = "root_url")]
    pub fn base_url(&self) -> String {
        format!("http://{}", self.bind_addr())
    }

    /// Return an optional runtime override using the env-style key.
    pub fn runtime_override(&self, name: &str) -> Option<String> {
        self.runtime_overrides.lookup(name)
    }
}

#[cfg(feature = "server")]
impl ServeRuntimeOverrides {
    /// Return an optional runtime override using the env-style key.
    pub fn lookup(&self, name: &str) -> Option<String> {
        match name {
            "AMAGI_PROXY_TIMEOUT_MS" => self.proxy_timeout_ms.map(|value| value.to_string()),
            "AMAGI_PROXY_MAX_HOPS" => self.proxy_max_hops.map(|value| value.to_string()),
            "AMAGI_PLATFORM_DOUYIN_MODE" => self.douyin_mode.clone(),
            "AMAGI_PLATFORM_DOUYIN_UPSTREAM" => self.douyin_upstream.clone(),
            "AMAGI_PLATFORM_BILIBILI_MODE" => self.bilibili_mode.clone(),
            "AMAGI_PLATFORM_BILIBILI_UPSTREAM" => self.bilibili_upstream.clone(),
            "AMAGI_PLATFORM_KUAISHOU_MODE" => self.kuaishou_mode.clone(),
            "AMAGI_PLATFORM_KUAISHOU_UPSTREAM" => self.kuaishou_upstream.clone(),
            "AMAGI_PLATFORM_XIAOHONGSHU_MODE" => self.xiaohongshu_mode.clone(),
            "AMAGI_PLATFORM_XIAOHONGSHU_UPSTREAM" => self.xiaohongshu_upstream.clone(),
            "AMAGI_PLATFORM_TWITTER_MODE" => self.twitter_mode.clone(),
            "AMAGI_PLATFORM_TWITTER_UPSTREAM" => self.twitter_upstream.clone(),
            _ => None,
        }
    }
}
