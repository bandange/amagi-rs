use std::ffi::OsString;

use clap::{CommandFactory, FromArgMatches, parser::ValueSource};

use super::args::{Cli, Command, RunArgs};
use super::map::map_run_task;
use crate::config::{
    AppConfig, CommandConfig, LoggingConfig, OutputConfig, OutputFormat, RunConfig,
};
#[cfg(feature = "server")]
use crate::config::{ServeConfig, ServeRuntimeOverrides};
use amagi_client::{ClientOptions, CookieConfig, RequestConfig};
use amagi_core::AppError;
use amagi_core::{DotenvMap, env_or_dotenv};

pub(super) fn parse_process_args_with_dotenv<I, T>(
    args: I,
    dotenv: &DotenvMap,
) -> Result<AppConfig, AppError>
where
    I: IntoIterator<Item = T>,
    T: Into<OsString> + Clone,
{
    let args = args.into_iter().map(Into::into).collect::<Vec<OsString>>();
    let lang = super::i18n::resolve_cli_language(&args, Some(dotenv));
    let matches = match super::i18n::localize_command(Cli::command(), lang)
        .color(super::i18n::default_cli_color())
        .try_get_matches_from(args)
    {
        Ok(matches) => matches,
        Err(error) => {
            let rendered = super::i18n::render_clap_error(&error, lang);
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

    app_config_from_cli(cli, &matches, dotenv)
}

pub(super) fn app_config_from_plain_cli(cli: Cli) -> AppConfig {
    let Cli {
        lang,
        output,
        output_file,
        pretty,
        append,
        create_parent_dirs,
        douyin_cookie,
        bilibili_cookie,
        kuaishou_cookie,
        twitter_cookie,
        xiaohongshu_cookie,
        timeout_ms,
        max_retries,
        log_format,
        log_level,
        command,
    } = cli;
    let output_locale = super::i18n::resolve_runtime_language(lang, None)
        .code()
        .to_owned();

    let client = ClientOptions {
        cookies: CookieConfig {
            douyin: douyin_cookie,
            bilibili: bilibili_cookie,
            kuaishou: kuaishou_cookie,
            twitter: twitter_cookie,
            xiaohongshu: xiaohongshu_cookie,
            ..CookieConfig::default()
        },
        request: RequestConfig::default()
            .with_timeout_ms(timeout_ms)
            .with_max_retries(max_retries),
    };

    let command = match command.unwrap_or(Command::Run(RunArgs::default())) {
        Command::Run(run) => CommandConfig::Run(RunConfig {
            quiet: run.quiet,
            task: map_run_task(run.task),
        }),
        #[cfg(feature = "server")]
        Command::Serve(serve) => CommandConfig::Serve(ServeConfig {
            host: serve.host,
            port: serve.port,
            runtime_overrides: ServeRuntimeOverrides {
                proxy_timeout_ms: serve.runtime.proxy_timeout_ms,
                proxy_max_hops: serve.runtime.proxy_max_hops,
                node_id: serve.runtime.node_id,
                node_role: serve.runtime.node_role,
                node_accept_downstream: serve.runtime.node_accept_downstream,
                node_connect_upstream: serve.runtime.node_connect_upstream,
                node_auth_token: serve.runtime.node_auth_token,
                node_auth_credentials: serve.runtime.node_auth_credentials,
                node_control_token: serve.runtime.node_control_token,
                node_allow_insecure_ws: serve.runtime.node_allow_insecure_ws,
                node_heartbeat_ms: serve.runtime.node_heartbeat_ms,
                node_request_timeout_ms: serve.runtime.node_request_timeout_ms,
                node_max_hops: serve.runtime.node_max_hops,
                node_max_concurrent_tasks: serve.runtime.node_max_concurrent_tasks,
                node_auto_claim_published_routes: serve.runtime.node_auto_claim_published_routes,
                douyin_mode: serve.runtime.douyin_mode,
                douyin_route: serve.runtime.douyin_route,
                douyin_upstream: serve.runtime.douyin_upstream,
                bilibili_mode: serve.runtime.bilibili_mode,
                bilibili_route: serve.runtime.bilibili_route,
                bilibili_upstream: serve.runtime.bilibili_upstream,
                kuaishou_mode: serve.runtime.kuaishou_mode,
                kuaishou_route: serve.runtime.kuaishou_route,
                kuaishou_upstream: serve.runtime.kuaishou_upstream,
                xiaohongshu_mode: serve.runtime.xiaohongshu_mode,
                xiaohongshu_route: serve.runtime.xiaohongshu_route,
                xiaohongshu_upstream: serve.runtime.xiaohongshu_upstream,
                twitter_mode: serve.runtime.twitter_mode,
                twitter_route: serve.runtime.twitter_route,
                twitter_upstream: serve.runtime.twitter_upstream,
            },
        }),
    };

    AppConfig {
        command,
        output: OutputConfig {
            locale: output_locale,
            format: output,
            file: output_file,
            pretty,
            append,
            create_parent_dirs,
        },
        logging: LoggingConfig {
            level: log_level,
            format: log_format,
        },
        client,
    }
}

fn app_config_from_cli(
    cli: Cli,
    matches: &clap::ArgMatches,
    dotenv: &DotenvMap,
) -> Result<AppConfig, AppError> {
    let Cli {
        lang,
        output,
        output_file,
        pretty,
        append,
        create_parent_dirs,
        douyin_cookie,
        bilibili_cookie,
        kuaishou_cookie,
        twitter_cookie,
        xiaohongshu_cookie,
        timeout_ms,
        max_retries,
        log_format,
        log_level,
        command,
    } = cli;

    let client = ClientOptions {
        cookies: CookieConfig {
            douyin: resolve_string(
                douyin_cookie,
                matches,
                "douyin_cookie",
                "AMAGI_DOUYIN_COOKIE",
                dotenv,
            ),
            bilibili: resolve_string(
                bilibili_cookie,
                matches,
                "bilibili_cookie",
                "AMAGI_BILIBILI_COOKIE",
                dotenv,
            ),
            kuaishou: resolve_string(
                kuaishou_cookie,
                matches,
                "kuaishou_cookie",
                "AMAGI_KUAISHOU_COOKIE",
                dotenv,
            ),
            twitter: resolve_string(
                twitter_cookie,
                matches,
                "twitter_cookie",
                "AMAGI_TWITTER_COOKIE",
                dotenv,
            ),
            xiaohongshu: resolve_string(
                xiaohongshu_cookie,
                matches,
                "xiaohongshu_cookie",
                "AMAGI_XIAOHONGSHU_COOKIE",
                dotenv,
            ),
        },
        request: RequestConfig::default()
            .with_timeout_ms(resolve_u64(
                timeout_ms,
                matches,
                "timeout_ms",
                "AMAGI_TIMEOUT_MS",
                dotenv,
            )?)
            .with_max_retries(resolve_u32(
                max_retries,
                matches,
                "max_retries",
                "AMAGI_MAX_RETRIES",
                dotenv,
            )?),
    };

    let output = OutputConfig {
        locale: super::i18n::resolve_runtime_language(lang, Some(dotenv))
            .code()
            .to_owned(),
        format: resolve_output_format(output, matches, "output", dotenv)?,
        file: resolve_string(
            output_file,
            matches,
            "output_file",
            "AMAGI_OUTPUT_FILE",
            dotenv,
        ),
        pretty: resolve_bool(pretty, matches, "pretty", "AMAGI_OUTPUT_PRETTY", dotenv)?,
        append: resolve_bool(append, matches, "append", "AMAGI_OUTPUT_APPEND", dotenv)?,
        create_parent_dirs: resolve_bool(
            create_parent_dirs,
            matches,
            "create_parent_dirs",
            "AMAGI_OUTPUT_CREATE_DIRS",
            dotenv,
        )?,
    };
    let logging = LoggingConfig {
        level: resolve_log_level(log_level, matches, "log_level", dotenv)?,
        format: resolve_log_format(log_format, matches, "log_format", dotenv)?,
    };

    let command = match command.unwrap_or(Command::Run(RunArgs::default())) {
        Command::Run(run) => CommandConfig::Run(RunConfig {
            quiet: run.quiet,
            task: map_run_task(run.task),
        }),
        #[cfg(feature = "server")]
        Command::Serve(serve) => {
            let Some(serve_matches) = matches.subcommand_matches("serve") else {
                return Err(AppError::Io(std::io::Error::new(
                    std::io::ErrorKind::InvalidData,
                    "missing `serve` subcommand matches",
                )));
            };

            CommandConfig::Serve(ServeConfig {
                host: resolve_string(
                    Some(serve.host),
                    serve_matches,
                    "host",
                    "AMAGI_HOST",
                    dotenv,
                )
                .unwrap_or_else(|| crate::DEFAULT_HOST.to_owned()),
                port: resolve_u16(serve.port, serve_matches, "port", "AMAGI_PORT", dotenv)?,
                runtime_overrides: ServeRuntimeOverrides {
                    proxy_timeout_ms: resolve_optional_u64(
                        serve.runtime.proxy_timeout_ms,
                        serve_matches,
                        "proxy_timeout_ms",
                        "AMAGI_PROXY_TIMEOUT_MS",
                        dotenv,
                    )?,
                    proxy_max_hops: resolve_optional_u32(
                        serve.runtime.proxy_max_hops,
                        serve_matches,
                        "proxy_max_hops",
                        "AMAGI_PROXY_MAX_HOPS",
                        dotenv,
                    )?,
                    node_id: resolve_string(
                        serve.runtime.node_id,
                        serve_matches,
                        "node_id",
                        "AMAGI_NODE_ID",
                        dotenv,
                    ),
                    node_role: resolve_string(
                        serve.runtime.node_role,
                        serve_matches,
                        "node_role",
                        "AMAGI_NODE_ROLE",
                        dotenv,
                    ),
                    node_accept_downstream: resolve_optional_bool(
                        serve.runtime.node_accept_downstream,
                        serve_matches,
                        "node_accept_downstream",
                        "AMAGI_NODE_ACCEPT_DOWNSTREAM",
                        dotenv,
                    )?,
                    node_connect_upstream: resolve_string(
                        serve.runtime.node_connect_upstream,
                        serve_matches,
                        "node_connect_upstream",
                        "AMAGI_NODE_CONNECT_UPSTREAM",
                        dotenv,
                    ),
                    node_auth_token: resolve_string(
                        serve.runtime.node_auth_token,
                        serve_matches,
                        "node_auth_token",
                        "AMAGI_NODE_AUTH_TOKEN",
                        dotenv,
                    ),
                    node_auth_credentials: resolve_string(
                        serve.runtime.node_auth_credentials,
                        serve_matches,
                        "node_auth_credentials",
                        "AMAGI_NODE_AUTH_CREDENTIALS",
                        dotenv,
                    ),
                    node_control_token: resolve_string(
                        serve.runtime.node_control_token,
                        serve_matches,
                        "node_control_token",
                        "AMAGI_NODE_CONTROL_TOKEN",
                        dotenv,
                    ),
                    node_allow_insecure_ws: resolve_optional_bool(
                        serve.runtime.node_allow_insecure_ws,
                        serve_matches,
                        "node_allow_insecure_ws",
                        "AMAGI_NODE_ALLOW_INSECURE_WS",
                        dotenv,
                    )?,
                    node_heartbeat_ms: resolve_optional_u64(
                        serve.runtime.node_heartbeat_ms,
                        serve_matches,
                        "node_heartbeat_ms",
                        "AMAGI_NODE_HEARTBEAT_MS",
                        dotenv,
                    )?,
                    node_request_timeout_ms: resolve_optional_u64(
                        serve.runtime.node_request_timeout_ms,
                        serve_matches,
                        "node_request_timeout_ms",
                        "AMAGI_NODE_REQUEST_TIMEOUT_MS",
                        dotenv,
                    )?,
                    node_max_hops: resolve_optional_u32(
                        serve.runtime.node_max_hops,
                        serve_matches,
                        "node_max_hops",
                        "AMAGI_NODE_MAX_HOPS",
                        dotenv,
                    )?,
                    node_max_concurrent_tasks: resolve_optional_u32(
                        serve.runtime.node_max_concurrent_tasks,
                        serve_matches,
                        "node_max_concurrent_tasks",
                        "AMAGI_NODE_MAX_CONCURRENT_TASKS",
                        dotenv,
                    )?,
                    node_auto_claim_published_routes: resolve_optional_bool(
                        serve.runtime.node_auto_claim_published_routes,
                        serve_matches,
                        "node_auto_claim_published_routes",
                        "AMAGI_NODE_AUTO_CLAIM_PUBLISHED_ROUTES",
                        dotenv,
                    )?,
                    douyin_mode: resolve_string(
                        serve.runtime.douyin_mode,
                        serve_matches,
                        "douyin_mode",
                        "AMAGI_PLATFORM_DOUYIN_MODE",
                        dotenv,
                    ),
                    douyin_route: resolve_string(
                        serve.runtime.douyin_route,
                        serve_matches,
                        "douyin_route",
                        "AMAGI_PLATFORM_DOUYIN_ROUTE",
                        dotenv,
                    ),
                    douyin_upstream: resolve_string(
                        serve.runtime.douyin_upstream,
                        serve_matches,
                        "douyin_upstream",
                        "AMAGI_PLATFORM_DOUYIN_UPSTREAM",
                        dotenv,
                    ),
                    bilibili_mode: resolve_string(
                        serve.runtime.bilibili_mode,
                        serve_matches,
                        "bilibili_mode",
                        "AMAGI_PLATFORM_BILIBILI_MODE",
                        dotenv,
                    ),
                    bilibili_route: resolve_string(
                        serve.runtime.bilibili_route,
                        serve_matches,
                        "bilibili_route",
                        "AMAGI_PLATFORM_BILIBILI_ROUTE",
                        dotenv,
                    ),
                    bilibili_upstream: resolve_string(
                        serve.runtime.bilibili_upstream,
                        serve_matches,
                        "bilibili_upstream",
                        "AMAGI_PLATFORM_BILIBILI_UPSTREAM",
                        dotenv,
                    ),
                    kuaishou_mode: resolve_string(
                        serve.runtime.kuaishou_mode,
                        serve_matches,
                        "kuaishou_mode",
                        "AMAGI_PLATFORM_KUAISHOU_MODE",
                        dotenv,
                    ),
                    kuaishou_route: resolve_string(
                        serve.runtime.kuaishou_route,
                        serve_matches,
                        "kuaishou_route",
                        "AMAGI_PLATFORM_KUAISHOU_ROUTE",
                        dotenv,
                    ),
                    kuaishou_upstream: resolve_string(
                        serve.runtime.kuaishou_upstream,
                        serve_matches,
                        "kuaishou_upstream",
                        "AMAGI_PLATFORM_KUAISHOU_UPSTREAM",
                        dotenv,
                    ),
                    xiaohongshu_mode: resolve_string(
                        serve.runtime.xiaohongshu_mode,
                        serve_matches,
                        "xiaohongshu_mode",
                        "AMAGI_PLATFORM_XIAOHONGSHU_MODE",
                        dotenv,
                    ),
                    xiaohongshu_route: resolve_string(
                        serve.runtime.xiaohongshu_route,
                        serve_matches,
                        "xiaohongshu_route",
                        "AMAGI_PLATFORM_XIAOHONGSHU_ROUTE",
                        dotenv,
                    ),
                    xiaohongshu_upstream: resolve_string(
                        serve.runtime.xiaohongshu_upstream,
                        serve_matches,
                        "xiaohongshu_upstream",
                        "AMAGI_PLATFORM_XIAOHONGSHU_UPSTREAM",
                        dotenv,
                    ),
                    twitter_mode: resolve_string(
                        serve.runtime.twitter_mode,
                        serve_matches,
                        "twitter_mode",
                        "AMAGI_PLATFORM_TWITTER_MODE",
                        dotenv,
                    ),
                    twitter_route: resolve_string(
                        serve.runtime.twitter_route,
                        serve_matches,
                        "twitter_route",
                        "AMAGI_PLATFORM_TWITTER_ROUTE",
                        dotenv,
                    ),
                    twitter_upstream: resolve_string(
                        serve.runtime.twitter_upstream,
                        serve_matches,
                        "twitter_upstream",
                        "AMAGI_PLATFORM_TWITTER_UPSTREAM",
                        dotenv,
                    ),
                },
            })
        }
    };

    Ok(AppConfig {
        command,
        output,
        logging,
        client,
    })
}

fn resolve_string(
    current: Option<String>,
    matches: &clap::ArgMatches,
    arg_id: &str,
    env_name: &str,
    dotenv: &DotenvMap,
) -> Option<String> {
    let normalize = |value: String| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_owned())
    };

    if prefers_existing_value(matches, arg_id) {
        current.and_then(normalize)
    } else {
        env_or_dotenv(env_name, dotenv)
            .and_then(normalize)
            .or_else(|| current.and_then(normalize))
    }
}

fn resolve_u64(
    current: u64,
    matches: &clap::ArgMatches,
    arg_id: &str,
    env_name: &str,
    dotenv: &DotenvMap,
) -> Result<u64, AppError> {
    resolve_number(current, matches, arg_id, env_name, dotenv)
}

fn resolve_u32(
    current: u32,
    matches: &clap::ArgMatches,
    arg_id: &str,
    env_name: &str,
    dotenv: &DotenvMap,
) -> Result<u32, AppError> {
    resolve_number(current, matches, arg_id, env_name, dotenv)
}

#[cfg(feature = "server")]
fn resolve_optional_u64(
    current: Option<u64>,
    matches: &clap::ArgMatches,
    arg_id: &str,
    env_name: &str,
    dotenv: &DotenvMap,
) -> Result<Option<u64>, AppError> {
    resolve_optional_number(current, matches, arg_id, env_name, dotenv)
}

#[cfg(feature = "server")]
fn resolve_optional_u32(
    current: Option<u32>,
    matches: &clap::ArgMatches,
    arg_id: &str,
    env_name: &str,
    dotenv: &DotenvMap,
) -> Result<Option<u32>, AppError> {
    resolve_optional_number(current, matches, arg_id, env_name, dotenv)
}

#[cfg(feature = "server")]
fn resolve_optional_bool(
    current: Option<bool>,
    matches: &clap::ArgMatches,
    arg_id: &str,
    env_name: &str,
    dotenv: &DotenvMap,
) -> Result<Option<bool>, AppError> {
    if prefers_existing_value(matches, arg_id) {
        return Ok(current);
    }

    parse_bool(env_or_dotenv(env_name, dotenv), env_name).map(|value| value.or(current))
}

fn resolve_bool(
    current: bool,
    matches: &clap::ArgMatches,
    arg_id: &str,
    env_name: &str,
    dotenv: &DotenvMap,
) -> Result<bool, AppError> {
    if prefers_existing_value(matches, arg_id) {
        return Ok(current);
    }

    parse_bool(env_or_dotenv(env_name, dotenv), env_name).map(|value| value.unwrap_or(current))
}

#[cfg(feature = "server")]
fn resolve_u16(
    current: u16,
    matches: &clap::ArgMatches,
    arg_id: &str,
    env_name: &str,
    dotenv: &DotenvMap,
) -> Result<u16, AppError> {
    resolve_number(current, matches, arg_id, env_name, dotenv)
}

fn resolve_number<T>(
    current: T,
    matches: &clap::ArgMatches,
    arg_id: &str,
    env_name: &str,
    dotenv: &DotenvMap,
) -> Result<T, AppError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    if prefers_existing_value(matches, arg_id) {
        return Ok(current);
    }

    match env_or_dotenv(env_name, dotenv) {
        Some(value) => value.parse::<T>().map_err(|error| {
            AppError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid value for {env_name}: {error}"),
            ))
        }),
        None => Ok(current),
    }
}

#[cfg(feature = "server")]
fn resolve_optional_number<T>(
    current: Option<T>,
    matches: &clap::ArgMatches,
    arg_id: &str,
    env_name: &str,
    dotenv: &DotenvMap,
) -> Result<Option<T>, AppError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    if prefers_existing_value(matches, arg_id) {
        return Ok(current);
    }

    match env_or_dotenv(env_name, dotenv) {
        Some(value) => value.parse::<T>().map(Some).map_err(|error| {
            AppError::Io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("invalid value for {env_name}: {error}"),
            ))
        }),
        None => Ok(current),
    }
}

fn resolve_output_format(
    current: OutputFormat,
    matches: &clap::ArgMatches,
    arg_id: &str,
    dotenv: &DotenvMap,
) -> Result<OutputFormat, AppError> {
    if prefers_existing_value(matches, arg_id) {
        return Ok(current);
    }

    parse_output_format(env_or_dotenv("AMAGI_OUTPUT", dotenv)).map(|value| value.unwrap_or(current))
}

fn resolve_log_format(
    current: crate::config::LogFormat,
    matches: &clap::ArgMatches,
    arg_id: &str,
    dotenv: &DotenvMap,
) -> Result<crate::config::LogFormat, AppError> {
    if prefers_existing_value(matches, arg_id) {
        return Ok(current);
    }

    parse_log_format(env_or_dotenv("AMAGI_LOG_FORMAT", dotenv))
        .map(|value| value.unwrap_or(current))
}

fn resolve_log_level(
    current: crate::config::LogLevel,
    matches: &clap::ArgMatches,
    arg_id: &str,
    dotenv: &DotenvMap,
) -> Result<crate::config::LogLevel, AppError> {
    if prefers_existing_value(matches, arg_id) {
        return Ok(current);
    }

    parse_log_level(env_or_dotenv("AMAGI_LOG", dotenv)).map(|value| value.unwrap_or(current))
}

fn prefers_existing_value(matches: &clap::ArgMatches, arg_id: &str) -> bool {
    matches
        .value_source(arg_id)
        .is_some_and(|source| matches!(source, ValueSource::CommandLine | ValueSource::EnvVariable))
}

fn parse_output_format(value: Option<String>) -> Result<Option<OutputFormat>, AppError> {
    match value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        None => Ok(None),
        Some("text") => Ok(Some(OutputFormat::Text)),
        Some("json") => Ok(Some(OutputFormat::Json)),
        Some(other) => invalid_enum("AMAGI_OUTPUT", other),
    }
}

fn parse_log_format(value: Option<String>) -> Result<Option<crate::config::LogFormat>, AppError> {
    match value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        None => Ok(None),
        Some("text") => Ok(Some(crate::config::LogFormat::Text)),
        Some("json") => Ok(Some(crate::config::LogFormat::Json)),
        Some(other) => invalid_enum("AMAGI_LOG_FORMAT", other),
    }
}

fn parse_log_level(value: Option<String>) -> Result<Option<crate::config::LogLevel>, AppError> {
    match value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        None => Ok(None),
        Some("error") => Ok(Some(crate::config::LogLevel::Error)),
        Some("warn") => Ok(Some(crate::config::LogLevel::Warn)),
        Some("info") => Ok(Some(crate::config::LogLevel::Info)),
        Some("debug") => Ok(Some(crate::config::LogLevel::Debug)),
        Some("trace") => Ok(Some(crate::config::LogLevel::Trace)),
        Some(other) => invalid_enum("AMAGI_LOG", other),
    }
}

fn parse_bool(value: Option<String>, env_name: &str) -> Result<Option<bool>, AppError> {
    match value
        .as_deref()
        .map(str::trim)
        .filter(|value| !value.is_empty())
    {
        None => Ok(None),
        Some("1" | "true" | "TRUE" | "yes" | "YES" | "on" | "ON") => Ok(Some(true)),
        Some("0" | "false" | "FALSE" | "no" | "NO" | "off" | "OFF") => Ok(Some(false)),
        Some(other) => Err(AppError::Io(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("invalid value for {env_name}: `{other}`"),
        ))),
    }
}

fn invalid_enum<T>(env_name: &str, value: &str) -> Result<T, AppError> {
    Err(AppError::Io(std::io::Error::new(
        std::io::ErrorKind::InvalidData,
        format!("invalid value for {env_name}: `{value}`"),
    )))
}
