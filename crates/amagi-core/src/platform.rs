use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

/// Supported social platforms.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
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
