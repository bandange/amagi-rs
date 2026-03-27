//! Parse CLI arguments and convert them into runtime configuration values.

mod args;
mod i18n;
mod map;
mod resolve;

use std::ffi::OsString;

use clap::{CommandFactory, FromArgMatches};

use crate::config::AppConfig;
use crate::env::dotenv_values;
use crate::error::AppError;

#[cfg(feature = "server")]
pub use args::ServeArgs;
pub use args::{
    BilibiliArgs, BilibiliCommand, Cli, Command, DouyinArgs, DouyinCommand, KuaishouArgs,
    KuaishouCommand, RunArgs, RunTaskArgs, TwitterArgs, TwitterCommand, XiaohongshuArgs,
    XiaohongshuCommand,
};

/// Parse CLI arguments from the current process environment.
///
/// Invalid arguments terminate the process with clap's formatted diagnostic.
pub fn parse_env() -> AppConfig {
    parse_from(std::env::args_os())
}

/// Parse CLI arguments after resolving values from process env and layered
/// dotenv files.
///
/// Invalid arguments terminate the process with clap's formatted diagnostic.
///
/// # Errors
///
/// Returns an error when any discovered dotenv file cannot be read or contains
/// invalid values.
pub fn try_parse_env() -> Result<AppConfig, AppError> {
    let dotenv = dotenv_values()?;
    resolve::parse_process_args_with_dotenv(std::env::args_os(), &dotenv)
}

/// Parse CLI arguments from a custom iterator.
///
/// Invalid arguments terminate the process with clap's formatted diagnostic.
///
/// # Examples
///
/// ```rust
/// use amagi::cli::parse_from;
/// use amagi::config::CommandConfig;
///
/// let config = parse_from(["amagi", "run", "douyin", "emoji-list"]);
///
/// match config.command {
///     CommandConfig::Run(run) => assert!(!run.quiet),
///     _ => unreachable!("expected run"),
/// }
/// ```
pub fn parse_from<I, T>(args: I) -> AppConfig
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = args.into_iter().map(Into::into).collect::<Vec<OsString>>();
    let lang = i18n::resolve_cli_language(&args, None);
    let matches = match i18n::localize_command(Cli::command(), lang)
        .color(i18n::default_cli_color())
        .try_get_matches_from(args)
    {
        Ok(matches) => matches,
        Err(error) => {
            let rendered = i18n::render_clap_error(&error, lang);
            if error.use_stderr() {
                eprint!("{rendered}");
            } else {
                print!("{rendered}");
            }
            std::process::exit(error.exit_code());
        }
    };
    let cli = Cli::from_arg_matches(&matches)
        .expect("clap matches should deserialize into the generated CLI type");
    AppConfig::from(cli)
}

impl From<Cli> for AppConfig {
    fn from(value: Cli) -> Self {
        resolve::app_config_from_plain_cli(value)
    }
}
