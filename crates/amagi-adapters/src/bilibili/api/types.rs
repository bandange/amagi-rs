use serde_json::Value;

/// Public JSON POST request descriptor used by Bilibili builders.
#[derive(Debug, Clone, PartialEq)]
pub struct BilibiliJsonPostRequest {
    /// Target request URL.
    pub url: String,
    /// JSON body sent to the target URL.
    pub body: Value,
}

/// Login state used by [`BilibiliPlayurlQuery`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BilibiliPlayurlStatus {
    /// No valid cookie was supplied, so the request should use guest playback parameters.
    Guest,
    /// A logged-in session was supplied.
    LoggedIn {
        /// Whether the logged-in session is a VIP session.
        is_vip: bool,
    },
}

impl BilibiliPlayurlStatus {
    /// Return whether the playback request is authenticated.
    pub fn is_logged_in(self) -> bool {
        matches!(self, Self::LoggedIn { .. })
    }

    /// Return whether the playback request is authenticated with a VIP session.
    pub fn is_vip(self) -> bool {
        matches!(self, Self::LoggedIn { is_vip: true })
    }

    /// Return the legacy TypeScript status marker.
    pub fn as_legacy_str(self) -> &'static str {
        match self {
            Self::Guest => "!isLogin",
            Self::LoggedIn { .. } => "isLogin",
        }
    }
}

/// Playback query suffix generated from Bilibili login state and WBI parameters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BilibiliPlayurlQuery {
    /// Query suffix without the leading `?` or `&`.
    pub query_suffix: String,
    /// Login state used to derive this suffix.
    pub status: BilibiliPlayurlStatus,
}

impl BilibiliPlayurlQuery {
    /// Append the query suffix to a base URL.
    pub fn append_to(&self, base_url: &str) -> String {
        if self.query_suffix.is_empty() {
            return base_url.to_owned();
        }

        let separator = if base_url.contains('?') { '&' } else { '?' };
        format!("{base_url}{separator}{}", self.query_suffix)
    }
}
