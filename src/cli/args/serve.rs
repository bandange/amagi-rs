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
}
