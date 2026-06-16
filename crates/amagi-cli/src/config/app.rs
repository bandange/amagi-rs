use amagi_client::ClientOptions;
#[cfg(feature = "server")]
use amagi_server::ServeConfig;

/// Resolved runtime configuration assembled from CLI input.
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
#[derive(Debug, Clone)]
pub enum CommandConfig {
    /// Run local CLI workflows.
    Run(RunConfig),
    #[cfg(feature = "server")]
    /// Start the built-in HTTP server.
    Serve(ServeConfig),
}

/// Runtime options for local CLI execution.
#[derive(Debug, Clone, Default)]
pub struct RunConfig {
    /// Suppress normal startup output without affecting structured logs.
    pub quiet: bool,
    /// Selected CLI task to execute.
    pub task: crate::config::RunTask,
}
