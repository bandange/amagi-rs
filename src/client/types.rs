use std::collections::BTreeMap;
use std::io;
use std::path::Path;

use serde::Serialize;

use crate::catalog::{HttpMethod, Platform};
use crate::env::{DotenvMap, dotenv_values, dotenv_values_from_path, env_or_dotenv};
use crate::error::AppError;

/// Shared cookie configuration for all supported platforms.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct CookieConfig {
    /// Cookie used for Douyin requests.
    pub douyin: Option<String>,
    /// Cookie used for Bilibili requests.
    pub bilibili: Option<String>,
    /// Cookie used for Kuaishou requests.
    pub kuaishou: Option<String>,
    /// Cookie used for Twitter/X requests.
    pub twitter: Option<String>,
    /// Cookie used for Xiaohongshu requests.
    pub xiaohongshu: Option<String>,
}

impl CookieConfig {
    /// Return the configured cookie for a single platform.
    pub fn for_platform(&self, platform: Platform) -> Option<&str> {
        match platform {
            Platform::Bilibili => self.bilibili.as_deref(),
            Platform::Douyin => self.douyin.as_deref(),
            Platform::Kuaishou => self.kuaishou.as_deref(),
            Platform::Twitter => self.twitter.as_deref(),
            Platform::Xiaohongshu => self.xiaohongshu.as_deref(),
        }
    }
}

/// Shared network request configuration.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RequestConfig {
    /// Request timeout in milliseconds.
    pub timeout_ms: u64,
    /// Maximum retry count for recoverable failures.
    pub max_retries: u32,
    /// User-provided request headers that override per-platform defaults.
    pub headers: BTreeMap<String, String>,
}

impl Default for RequestConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 10_000,
            max_retries: 3,
            headers: BTreeMap::new(),
        }
    }
}

impl RequestConfig {
    /// Override the request timeout.
    pub fn with_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }

    /// Override the retry budget.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }

    /// Insert or replace a header value.
    pub fn with_header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }
}

/// Options used to create an [`crate::client::AmagiClient`].
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize)]
pub struct ClientOptions {
    /// Optional per-platform cookies.
    pub cookies: CookieConfig,
    /// Shared request overrides.
    pub request: RequestConfig,
}

impl ClientOptions {
    /// Build client options from process environment variables and layered
    /// dotenv files.
    ///
    /// Process environment variables take precedence over dotenv values. The
    /// default dotenv lookup order is user-level config first, then the current
    /// working directory `.env`.
    ///
    /// # Errors
    ///
    /// Returns an error when any discovered dotenv file cannot be read or
    /// contains invalid values.
    pub fn from_env() -> Result<Self, AppError> {
        let dotenv = dotenv_values()?;
        Self::from_dotenv_map(&dotenv)
    }

    /// Build client options from a specific dotenv file path.
    ///
    /// Process environment variables still take precedence over values loaded
    /// from the provided file.
    ///
    /// # Errors
    ///
    /// Returns an error when the file cannot be read or contains invalid
    /// values.
    pub fn from_env_path(path: impl AsRef<Path>) -> Result<Self, AppError> {
        let dotenv = dotenv_values_from_path(path)?;
        Self::from_dotenv_map(&dotenv)
    }

    fn from_dotenv_map(dotenv: &DotenvMap) -> Result<Self, AppError> {
        Ok(Self {
            cookies: CookieConfig {
                douyin: env_or_dotenv("AMAGI_DOUYIN_COOKIE", dotenv),
                bilibili: env_or_dotenv("AMAGI_BILIBILI_COOKIE", dotenv),
                kuaishou: env_or_dotenv("AMAGI_KUAISHOU_COOKIE", dotenv),
                twitter: env_or_dotenv("AMAGI_TWITTER_COOKIE", dotenv),
                xiaohongshu: env_or_dotenv("AMAGI_XIAOHONGSHU_COOKIE", dotenv),
            },
            request: RequestConfig::default()
                .with_timeout_ms(resolve_u64("AMAGI_TIMEOUT_MS", dotenv, 10_000)?)
                .with_max_retries(resolve_u32("AMAGI_MAX_RETRIES", dotenv, 3)?),
        })
    }
}

/// Resolved request profile for one platform.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct RequestProfile {
    /// Platform the profile is built for.
    pub platform: Platform,
    /// Default HTTP method used by the platform adapters.
    pub method: HttpMethod,
    /// Effective request timeout in milliseconds.
    pub timeout_ms: u64,
    /// Effective retry budget.
    pub max_retries: u32,
    /// Effective headers after merging defaults and user overrides.
    pub headers: BTreeMap<String, String>,
}

fn resolve_u64(env_name: &str, dotenv: &DotenvMap, default: u64) -> Result<u64, AppError> {
    resolve_number(env_name, dotenv)?.map_or(Ok(default), Ok)
}

fn resolve_u32(env_name: &str, dotenv: &DotenvMap, default: u32) -> Result<u32, AppError> {
    resolve_number(env_name, dotenv)?.map_or(Ok(default), Ok)
}

fn resolve_number<T>(env_name: &str, dotenv: &DotenvMap) -> Result<Option<T>, AppError>
where
    T: std::str::FromStr,
    T::Err: std::fmt::Display,
{
    match env_or_dotenv(env_name, dotenv) {
        Some(value) => value.parse::<T>().map(Some).map_err(|error| {
            AppError::Io(io::Error::new(
                io::ErrorKind::InvalidData,
                format!("invalid value for {env_name}: {error}"),
            ))
        }),
        None => Ok(None),
    }
}
