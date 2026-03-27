use clap::{Args, Parser, Subcommand};

#[cfg(feature = "server")]
use super::ServeArgs;
use super::{BilibiliCommand, DouyinCommand, KuaishouCommand, TwitterCommand, XiaohongshuCommand};
use crate::cli::i18n::CliLanguage;
use crate::config::{LogFormat, LogLevel, OutputFormat};

/// Top-level command-line interface definition.
#[derive(Debug, Parser)]
#[command(
    name = crate::APP_NAME,
    version,
    about = "Rust SDK + CLI + Web API service for multi-platform social web adapters",
    long_about = None
)]
pub struct Cli {
    /// CLI help language. Supports `zh` and `en`.
    #[arg(long, global = true, env = "AMAGI_LANG", value_name = "LANG")]
    pub lang: Option<CliLanguage>,

    /// Output format for CLI-facing messages.
    #[arg(long, global = true, value_enum, default_value_t = OutputFormat::Text)]
    pub output: OutputFormat,

    /// Optional file path used to save CLI-facing output.
    #[arg(
        long = "output-file",
        short = 'o',
        global = true,
        env = "AMAGI_OUTPUT_FILE"
    )]
    pub output_file: Option<String>,

    /// Pretty-print JSON output instead of emitting compact JSON.
    #[arg(
        long = "pretty",
        global = true,
        env = "AMAGI_OUTPUT_PRETTY",
        default_value_t = false
    )]
    pub pretty: bool,

    /// Append to `--output-file` instead of truncating it first.
    #[arg(
        long = "append",
        global = true,
        env = "AMAGI_OUTPUT_APPEND",
        default_value_t = false
    )]
    pub append: bool,

    /// Create missing parent directories for `--output-file`.
    #[arg(
        long = "create-parent-dirs",
        global = true,
        env = "AMAGI_OUTPUT_CREATE_DIRS",
        default_value_t = false
    )]
    pub create_parent_dirs: bool,

    /// Douyin cookie shared by CLI tasks and the HTTP server.
    #[arg(long = "douyin-cookie", global = true, env = "AMAGI_DOUYIN_COOKIE")]
    pub douyin_cookie: Option<String>,

    /// Bilibili cookie shared by CLI tasks and the HTTP server.
    #[arg(long = "bilibili-cookie", global = true, env = "AMAGI_BILIBILI_COOKIE")]
    pub bilibili_cookie: Option<String>,

    /// Kuaishou cookie shared by CLI tasks and the HTTP server.
    #[arg(long = "kuaishou-cookie", global = true, env = "AMAGI_KUAISHOU_COOKIE")]
    pub kuaishou_cookie: Option<String>,

    /// Xiaohongshu cookie shared by CLI tasks and the HTTP server.
    #[arg(
        long = "xiaohongshu-cookie",
        global = true,
        env = "AMAGI_XIAOHONGSHU_COOKIE"
    )]
    pub xiaohongshu_cookie: Option<String>,

    /// Twitter/X cookie shared by CLI tasks and the HTTP server.
    #[arg(long = "twitter-cookie", global = true, env = "AMAGI_TWITTER_COOKIE")]
    pub twitter_cookie: Option<String>,

    /// Upstream request timeout in milliseconds.
    #[arg(
        long = "timeout-ms",
        global = true,
        env = "AMAGI_TIMEOUT_MS",
        default_value_t = 10_000
    )]
    pub timeout_ms: u64,

    /// Maximum retry count for recoverable upstream failures.
    #[arg(
        long = "max-retries",
        global = true,
        env = "AMAGI_MAX_RETRIES",
        default_value_t = 3
    )]
    pub max_retries: u32,

    /// Log renderer used for runtime events.
    #[arg(
        long = "log-format",
        global = true,
        env = "AMAGI_LOG_FORMAT",
        value_enum,
        default_value_t = LogFormat::Text
    )]
    pub log_format: LogFormat,

    /// Minimum log level emitted by the application.
    #[arg(
        long = "log-level",
        global = true,
        env = "AMAGI_LOG",
        value_enum,
        default_value_t = LogLevel::Info
    )]
    pub log_level: LogLevel,

    /// Optional top-level command. Defaults to `run`.
    #[command(subcommand)]
    pub command: Option<Command>,
}

/// Supported CLI subcommands.
#[derive(Debug, Subcommand)]
pub enum Command {
    /// Run the local CLI workflow.
    Run(RunArgs),
    #[cfg(feature = "server")]
    /// Start the built-in HTTP API server.
    Serve(ServeArgs),
}

/// Arguments for the `run` subcommand.
#[derive(Debug, Args, Clone, Default)]
pub struct RunArgs {
    /// Suppress normal startup output.
    #[arg(long, default_value_t = false)]
    pub quiet: bool,

    /// Optional platform task to execute.
    #[command(subcommand)]
    pub task: Option<RunTaskArgs>,
}

/// Nested tasks for the `run` subcommand.
#[derive(Debug, Subcommand, Clone)]
pub enum RunTaskArgs {
    /// Run a Bilibili-specific task.
    Bilibili(BilibiliArgs),
    /// Run a Douyin-specific task.
    Douyin(DouyinArgs),
    /// Run a Kuaishou-specific task.
    Kuaishou(KuaishouArgs),
    /// Run a Twitter/X-specific task.
    Twitter(TwitterArgs),
    /// Run a Xiaohongshu-specific task.
    Xiaohongshu(XiaohongshuArgs),
}

/// Bilibili CLI task arguments.
#[derive(Debug, Args, Clone)]
pub struct BilibiliArgs {
    /// Concrete Bilibili task.
    #[command(subcommand)]
    pub command: BilibiliCommand,
}

/// Douyin CLI task arguments.
#[derive(Debug, Args, Clone)]
pub struct DouyinArgs {
    /// Concrete Douyin task.
    #[command(subcommand)]
    pub command: DouyinCommand,
}

/// Kuaishou CLI task arguments.
#[derive(Debug, Args, Clone)]
pub struct KuaishouArgs {
    /// Concrete Kuaishou task.
    #[command(subcommand)]
    pub command: KuaishouCommand,
}

/// Xiaohongshu CLI task arguments.
#[derive(Debug, Args, Clone)]
pub struct XiaohongshuArgs {
    /// Concrete Xiaohongshu task.
    #[command(subcommand)]
    pub command: XiaohongshuCommand,
}

/// Twitter/X CLI task arguments.
#[derive(Debug, Args, Clone)]
pub struct TwitterArgs {
    /// Concrete Twitter/X task.
    #[command(subcommand)]
    pub command: TwitterCommand,
}
