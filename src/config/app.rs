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
    /// };
    ///
    /// assert_eq!(serve.base_url(), "http://127.0.0.1:4567");
    /// ```
    #[doc(alias = "root_url")]
    pub fn base_url(&self) -> String {
        format!("http://{}", self.bind_addr())
    }
}
