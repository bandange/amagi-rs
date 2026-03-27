use std::fmt;
use std::str::FromStr;

use serde::Serialize;

/// Supported HTTP methods for published API endpoints.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum HttpMethod {
    /// HTTP `GET`.
    Get,
    /// HTTP `POST`.
    Post,
    /// HTTP `PUT`.
    Put,
    /// HTTP `DELETE`.
    Delete,
    /// HTTP `PATCH`.
    Patch,
}

/// Supported social platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Platform {
    /// Bilibili APIs.
    Bilibili,
    /// Douyin APIs.
    Douyin,
    /// Kuaishou APIs.
    Kuaishou,
    /// Twitter/X APIs.
    Twitter,
    /// Xiaohongshu APIs.
    Xiaohongshu,
}

impl Platform {
    /// Every supported platform in display order.
    pub const ALL: [Self; 5] = [
        Self::Douyin,
        Self::Bilibili,
        Self::Kuaishou,
        Self::Xiaohongshu,
        Self::Twitter,
    ];

    /// Return the stable lowercase identifier for this platform.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Bilibili => "bilibili",
            Self::Douyin => "douyin",
            Self::Kuaishou => "kuaishou",
            Self::Twitter => "twitter",
            Self::Xiaohongshu => "xiaohongshu",
        }
    }

    /// Return the shared HTTP base path for this platform.
    pub const fn api_base_path(self) -> &'static str {
        match self {
            Self::Bilibili => "/api/bilibili",
            Self::Douyin => "/api/douyin",
            Self::Kuaishou => "/api/kuaishou",
            Self::Twitter => "/api/twitter",
            Self::Xiaohongshu => "/api/xiaohongshu",
        }
    }
}

impl fmt::Display for Platform {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Error returned when parsing an unknown platform string.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsePlatformError;

impl fmt::Display for ParsePlatformError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("unknown platform")
    }
}

impl std::error::Error for ParsePlatformError {}

impl FromStr for Platform {
    type Err = ParsePlatformError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "bilibili" => Ok(Self::Bilibili),
            "douyin" => Ok(Self::Douyin),
            "kuaishou" => Ok(Self::Kuaishou),
            "twitter" => Ok(Self::Twitter),
            "xiaohongshu" => Ok(Self::Xiaohongshu),
            _ => Err(ParsePlatformError),
        }
    }
}

/// A single published API capability.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ApiMethodSpec {
    /// Stable method key used by the web layer.
    pub method_key: &'static str,
    /// Original Chinese method label from the source API catalog.
    pub chinese_name: &'static str,
    /// Canonical English fetcher method name.
    pub fetcher_name: &'static str,
    /// Relative REST path within the platform base path.
    pub route: &'static str,
    /// HTTP method exposed by the web surface.
    pub http_method: HttpMethod,
    /// Short human-readable description.
    pub description: &'static str,
    /// Tags used to group related capabilities.
    pub tags: &'static [&'static str],
}

/// Complete API catalog for one platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct PlatformSpec {
    /// Platform this catalog belongs to.
    pub platform: Platform,
    /// Shared HTTP base path for the platform.
    pub api_base_path: &'static str,
    /// Every published method for the platform.
    pub methods: &'static [ApiMethodSpec],
}
