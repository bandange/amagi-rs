/// Network options for the built-in HTTP server.
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
#[derive(Debug, Clone, Default)]
pub struct ServeRuntimeOverrides {
    /// Override for `AMAGI_PROXY_TIMEOUT_MS`.
    pub proxy_timeout_ms: Option<u64>,
    /// Override for `AMAGI_PROXY_MAX_HOPS`.
    pub proxy_max_hops: Option<u32>,
    /// Override for `AMAGI_NODE_ID`.
    pub node_id: Option<String>,
    /// Override for `AMAGI_NODE_ROLE`.
    pub node_role: Option<String>,
    /// Override for `AMAGI_NODE_ACCEPT_DOWNSTREAM`.
    pub node_accept_downstream: Option<bool>,
    /// Override for `AMAGI_NODE_CONNECT_UPSTREAM`.
    pub node_connect_upstream: Option<String>,
    /// Override for `AMAGI_NODE_AUTH_TOKEN`.
    pub node_auth_token: Option<String>,
    /// Override for `AMAGI_NODE_AUTH_CREDENTIALS`.
    pub node_auth_credentials: Option<String>,
    /// Override for `AMAGI_NODE_CONTROL_TOKEN`.
    pub node_control_token: Option<String>,
    /// Override for `AMAGI_NODE_ALLOW_INSECURE_WS`.
    pub node_allow_insecure_ws: Option<bool>,
    /// Override for `AMAGI_NODE_HEARTBEAT_MS`.
    pub node_heartbeat_ms: Option<u64>,
    /// Override for `AMAGI_NODE_REQUEST_TIMEOUT_MS`.
    pub node_request_timeout_ms: Option<u64>,
    /// Override for `AMAGI_NODE_MAX_HOPS`.
    pub node_max_hops: Option<u32>,
    /// Override for `AMAGI_NODE_MAX_CONCURRENT_TASKS`.
    pub node_max_concurrent_tasks: Option<u32>,
    /// Override for `AMAGI_NODE_AUTO_CLAIM_PUBLISHED_ROUTES`.
    pub node_auto_claim_published_routes: Option<bool>,
    /// Override for `AMAGI_PLATFORM_DOUYIN_MODE`.
    pub douyin_mode: Option<String>,
    /// Override for `AMAGI_PLATFORM_DOUYIN_ROUTE`.
    pub douyin_route: Option<String>,
    /// Override for `AMAGI_PLATFORM_DOUYIN_UPSTREAM`.
    pub douyin_upstream: Option<String>,
    /// Override for `AMAGI_PLATFORM_BILIBILI_MODE`.
    pub bilibili_mode: Option<String>,
    /// Override for `AMAGI_PLATFORM_BILIBILI_ROUTE`.
    pub bilibili_route: Option<String>,
    /// Override for `AMAGI_PLATFORM_BILIBILI_UPSTREAM`.
    pub bilibili_upstream: Option<String>,
    /// Override for `AMAGI_PLATFORM_KUAISHOU_MODE`.
    pub kuaishou_mode: Option<String>,
    /// Override for `AMAGI_PLATFORM_KUAISHOU_ROUTE`.
    pub kuaishou_route: Option<String>,
    /// Override for `AMAGI_PLATFORM_KUAISHOU_UPSTREAM`.
    pub kuaishou_upstream: Option<String>,
    /// Override for `AMAGI_PLATFORM_XIAOHONGSHU_MODE`.
    pub xiaohongshu_mode: Option<String>,
    /// Override for `AMAGI_PLATFORM_XIAOHONGSHU_ROUTE`.
    pub xiaohongshu_route: Option<String>,
    /// Override for `AMAGI_PLATFORM_XIAOHONGSHU_UPSTREAM`.
    pub xiaohongshu_upstream: Option<String>,
    /// Override for `AMAGI_PLATFORM_TWITTER_MODE`.
    pub twitter_mode: Option<String>,
    /// Override for `AMAGI_PLATFORM_TWITTER_ROUTE`.
    pub twitter_route: Option<String>,
    /// Override for `AMAGI_PLATFORM_TWITTER_UPSTREAM`.
    pub twitter_upstream: Option<String>,
}

impl ServeConfig {
    /// Return the `host:port` socket address string for this configuration.
    ///
    /// This method formats the configured values without performing DNS or
    /// socket validation.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use amagi_server::ServeConfig;
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
    /// use amagi_server::ServeConfig;
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

impl ServeRuntimeOverrides {
    /// Return an optional runtime override using the env-style key.
    pub fn lookup(&self, name: &str) -> Option<String> {
        match name {
            "AMAGI_PROXY_TIMEOUT_MS" => self.proxy_timeout_ms.map(|value| value.to_string()),
            "AMAGI_PROXY_MAX_HOPS" => self.proxy_max_hops.map(|value| value.to_string()),
            "AMAGI_NODE_ID" => self.node_id.clone(),
            "AMAGI_NODE_ROLE" => self.node_role.clone(),
            "AMAGI_NODE_ACCEPT_DOWNSTREAM" => {
                self.node_accept_downstream.map(|value| value.to_string())
            }
            "AMAGI_NODE_CONNECT_UPSTREAM" => self.node_connect_upstream.clone(),
            "AMAGI_NODE_AUTH_TOKEN" => self.node_auth_token.clone(),
            "AMAGI_NODE_AUTH_CREDENTIALS" => self.node_auth_credentials.clone(),
            "AMAGI_NODE_CONTROL_TOKEN" => self.node_control_token.clone(),
            "AMAGI_NODE_ALLOW_INSECURE_WS" => {
                self.node_allow_insecure_ws.map(|value| value.to_string())
            }
            "AMAGI_NODE_HEARTBEAT_MS" => self.node_heartbeat_ms.map(|value| value.to_string()),
            "AMAGI_NODE_REQUEST_TIMEOUT_MS" => {
                self.node_request_timeout_ms.map(|value| value.to_string())
            }
            "AMAGI_NODE_MAX_HOPS" => self.node_max_hops.map(|value| value.to_string()),
            "AMAGI_NODE_MAX_CONCURRENT_TASKS" => self
                .node_max_concurrent_tasks
                .map(|value| value.to_string()),
            "AMAGI_NODE_AUTO_CLAIM_PUBLISHED_ROUTES" => self
                .node_auto_claim_published_routes
                .map(|value| value.to_string()),
            "AMAGI_PLATFORM_DOUYIN_MODE" => self.douyin_mode.clone(),
            "AMAGI_PLATFORM_DOUYIN_ROUTE" => self.douyin_route.clone(),
            "AMAGI_PLATFORM_DOUYIN_UPSTREAM" => self.douyin_upstream.clone(),
            "AMAGI_PLATFORM_BILIBILI_MODE" => self.bilibili_mode.clone(),
            "AMAGI_PLATFORM_BILIBILI_ROUTE" => self.bilibili_route.clone(),
            "AMAGI_PLATFORM_BILIBILI_UPSTREAM" => self.bilibili_upstream.clone(),
            "AMAGI_PLATFORM_KUAISHOU_MODE" => self.kuaishou_mode.clone(),
            "AMAGI_PLATFORM_KUAISHOU_ROUTE" => self.kuaishou_route.clone(),
            "AMAGI_PLATFORM_KUAISHOU_UPSTREAM" => self.kuaishou_upstream.clone(),
            "AMAGI_PLATFORM_XIAOHONGSHU_MODE" => self.xiaohongshu_mode.clone(),
            "AMAGI_PLATFORM_XIAOHONGSHU_ROUTE" => self.xiaohongshu_route.clone(),
            "AMAGI_PLATFORM_XIAOHONGSHU_UPSTREAM" => self.xiaohongshu_upstream.clone(),
            "AMAGI_PLATFORM_TWITTER_MODE" => self.twitter_mode.clone(),
            "AMAGI_PLATFORM_TWITTER_ROUTE" => self.twitter_route.clone(),
            "AMAGI_PLATFORM_TWITTER_UPSTREAM" => self.twitter_upstream.clone(),
            _ => None,
        }
    }
}
