//! Runtime configuration for node-aware server behavior.

use std::collections::HashMap;
use std::fmt;
use std::io;
use std::time::Duration;

use reqwest::Client;
use serde::Serialize;

use crate::catalog::Platform;
use crate::env::{dotenv_values, env_or_dotenv};
use crate::error::AppError;

const DEFAULT_PROXY_TIMEOUT_MS: u64 = 15_000;
const DEFAULT_PROXY_MAX_HOPS: u32 = 1;

/// Serving behavior for one platform.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum PlatformServeMode {
    /// Serve the platform from the current node.
    #[default]
    Local,
    /// Proxy the platform to an upstream node.
    Upstream,
    /// Keep the route shape but reject requests for the platform.
    Disabled,
}

/// Per-platform serving policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlatformServePolicy {
    /// Serving mode for the platform.
    pub mode: PlatformServeMode,
    /// Upstream base URL used when the platform is proxied.
    pub upstream: Option<String>,
}

/// Shared proxy controls for upstream forwarding.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProxyRuntimeConfig {
    /// Request timeout for node-to-node proxy calls.
    pub timeout_ms: u64,
    /// Maximum number of proxy hops allowed for a request.
    pub max_hops: u32,
}

/// Resolved server runtime configuration.
#[derive(Debug, Clone)]
pub struct ServerRuntimeConfig {
    proxy: ProxyRuntimeConfig,
    platforms: HashMap<Platform, PlatformServePolicy>,
}

impl ServerRuntimeConfig {
    /// Resolve the runtime configuration from process env and layered dotenv,
    /// with optional per-process overrides applied first.
    ///
    /// # Errors
    ///
    /// Returns an error when the configuration contains invalid values.
    pub fn from_env_with_overrides<F>(overrides: F) -> Result<Self, AppError>
    where
        F: Fn(&str) -> Option<String>,
    {
        let dotenv = dotenv_values()?;
        Self::from_value_lookup(|name| overrides(name).or_else(|| env_or_dotenv(name, &dotenv)))
    }

    fn from_value_lookup<F>(lookup: F) -> Result<Self, AppError>
    where
        F: Fn(&str) -> Option<String>,
    {
        let proxy = ProxyRuntimeConfig {
            timeout_ms: resolve_u64("AMAGI_PROXY_TIMEOUT_MS", &lookup, DEFAULT_PROXY_TIMEOUT_MS)?,
            max_hops: resolve_u32("AMAGI_PROXY_MAX_HOPS", &lookup, DEFAULT_PROXY_MAX_HOPS)?,
        };

        let platforms = Platform::ALL
            .into_iter()
            .map(|platform| {
                let mode = parse_platform_mode(lookup(platform_mode_env(platform)))?;
                let upstream = normalize_string(lookup(platform_upstream_env(platform)));
                Ok((platform, PlatformServePolicy { mode, upstream }))
            })
            .collect::<Result<HashMap<_, _>, AppError>>()?;

        let config = Self { proxy, platforms };

        config.validate()?;
        Ok(config)
    }

    /// Return the maximum allowed proxy hop count.
    pub const fn proxy_max_hops(&self) -> u32 {
        self.proxy.max_hops
    }

    /// Return the resolved serving policy for a platform.
    pub fn platform_policy(&self, platform: Platform) -> &PlatformServePolicy {
        self.platforms
            .get(&platform)
            .expect("every supported platform should have a runtime policy")
    }

    /// Return the resolved mode for a platform.
    pub fn platform_mode(&self, platform: Platform) -> PlatformServeMode {
        self.platform_policy(platform).mode
    }

    /// Return whether the platform is published by the current node.
    pub fn is_platform_published(&self, platform: Platform) -> bool {
        !matches!(self.platform_mode(platform), PlatformServeMode::Disabled)
    }

    /// Return the configured upstream base URL for a platform when present.
    pub fn platform_upstream(&self, platform: Platform) -> Option<&str> {
        self.platform_policy(platform).upstream.as_deref()
    }

    /// Build the shared HTTP client used for proxy requests.
    ///
    /// # Errors
    ///
    /// Returns an error when the client cannot be initialized.
    pub fn build_proxy_client(&self) -> Result<Client, AppError> {
        Client::builder()
            .timeout(Duration::from_millis(self.proxy.timeout_ms))
            .build()
            .map_err(AppError::from)
    }

    fn validate(&self) -> Result<(), AppError> {
        if self.proxy.max_hops == 0 {
            return invalid_config("AMAGI_PROXY_MAX_HOPS must be greater than 0");
        }

        for platform in Platform::ALL {
            let policy = self.platform_policy(platform);
            if matches!(policy.mode, PlatformServeMode::Upstream) && policy.upstream.is_none() {
                return invalid_config(format!(
                    "{} requires {} when set to `upstream`",
                    platform_mode_env(platform),
                    platform_upstream_env(platform)
                ));
            }
        }

        Ok(())
    }
}

fn platform_mode_env(platform: Platform) -> &'static str {
    match platform {
        Platform::Bilibili => "AMAGI_PLATFORM_BILIBILI_MODE",
        Platform::Douyin => "AMAGI_PLATFORM_DOUYIN_MODE",
        Platform::Kuaishou => "AMAGI_PLATFORM_KUAISHOU_MODE",
        Platform::Twitter => "AMAGI_PLATFORM_TWITTER_MODE",
        Platform::Xiaohongshu => "AMAGI_PLATFORM_XIAOHONGSHU_MODE",
    }
}

fn platform_upstream_env(platform: Platform) -> &'static str {
    match platform {
        Platform::Bilibili => "AMAGI_PLATFORM_BILIBILI_UPSTREAM",
        Platform::Douyin => "AMAGI_PLATFORM_DOUYIN_UPSTREAM",
        Platform::Kuaishou => "AMAGI_PLATFORM_KUAISHOU_UPSTREAM",
        Platform::Twitter => "AMAGI_PLATFORM_TWITTER_UPSTREAM",
        Platform::Xiaohongshu => "AMAGI_PLATFORM_XIAOHONGSHU_UPSTREAM",
    }
}

fn parse_platform_mode(value: Option<String>) -> Result<PlatformServeMode, AppError> {
    match normalize_string(value).as_deref() {
        None => Ok(PlatformServeMode::Local),
        Some("enabled") => Ok(PlatformServeMode::Local),
        Some("local") => Ok(PlatformServeMode::Local),
        Some("upstream") => Ok(PlatformServeMode::Upstream),
        Some("disabled") => Ok(PlatformServeMode::Disabled),
        Some(other) => invalid_config(format!(
            "invalid platform mode value: `{other}`; expected `enabled`, `local`, `upstream`, or `disabled`"
        )),
    }
}

fn resolve_u64<F>(env_name: &str, lookup: &F, default: u64) -> Result<u64, AppError>
where
    F: Fn(&str) -> Option<String>,
{
    resolve_number(env_name, lookup)?.map_or(Ok(default), Ok)
}

fn resolve_u32<F>(env_name: &str, lookup: &F, default: u32) -> Result<u32, AppError>
where
    F: Fn(&str) -> Option<String>,
{
    resolve_number(env_name, lookup)?.map_or(Ok(default), Ok)
}

fn resolve_number<T, F>(env_name: &str, lookup: &F) -> Result<Option<T>, AppError>
where
    T: std::str::FromStr,
    T::Err: fmt::Display,
    F: Fn(&str) -> Option<String>,
{
    match lookup(env_name) {
        Some(value) => value.parse::<T>().map(Some).map_err(|error| {
            AppError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid value for {env_name}: {error}"),
            ))
        }),
        None => Ok(None),
    }
}

fn normalize_string(value: Option<String>) -> Option<String> {
    value.and_then(|value| {
        let trimmed = value.trim();
        (!trimmed.is_empty()).then(|| trimmed.to_owned())
    })
}

fn invalid_config<T>(message: impl Into<String>) -> Result<T, AppError> {
    Err(AppError::Io(io::Error::new(
        io::ErrorKind::InvalidData,
        message.into(),
    )))
}
