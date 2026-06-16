use serde::Serialize;

use amagi_core::{ApiOperationSpec, Platform, PlatformApiSpec, RequestConfig, RequestProfile};

use crate::spec::{operation_specs, platform_api_spec};

/// Adapter-bound platform context.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct AdapterContext {
    /// Platform this adapter context targets.
    pub platform: Platform,
    /// Bound cookie for the platform when configured.
    pub cookie: Option<String>,
    /// Shared request overrides carried into the platform adapter.
    pub request: RequestConfig,
}

impl AdapterContext {
    /// Return whether the adapter context has a bound cookie.
    pub fn has_cookie(&self) -> bool {
        self.cookie
            .as_deref()
            .is_some_and(|value| !value.is_empty())
    }

    /// Return the published API base path for this platform.
    pub fn api_base_path(&self) -> &'static str {
        self.platform.api_base_path()
    }

    /// Return the static API metadata for this platform adapter.
    pub fn api_spec(&self) -> PlatformApiSpec {
        platform_api_spec(self.platform)
    }

    /// Return every published operation for this platform adapter.
    pub fn operations(&self) -> &'static [ApiOperationSpec] {
        operation_specs(self.platform)
    }

    /// Compatibility wrapper for the former catalog-oriented name.
    pub fn spec(&self) -> PlatformApiSpec {
        self.api_spec()
    }

    /// Compatibility wrapper for the former method-oriented name.
    pub fn methods(&self) -> &'static [ApiOperationSpec] {
        self.operations()
    }

    /// Build the effective request profile by merging defaults and overrides.
    pub fn request_profile(&self) -> RequestProfile {
        let mut headers = amagi_core::defaults::platform_default_headers(self.platform);
        let cookie_header = match self.platform {
            Platform::Xiaohongshu => "cookie",
            _ => "Cookie",
        };

        headers.insert(
            cookie_header.to_owned(),
            self.cookie
                .as_deref()
                .map(strip_matching_quotes)
                .unwrap_or_default()
                .to_owned(),
        );
        headers.extend(self.request.headers.clone());

        RequestProfile {
            platform: self.platform,
            method: amagi_core::defaults::platform_default_method(self.platform),
            timeout_ms: self.request.timeout_ms,
            max_retries: self.request.max_retries,
            headers,
        }
    }
}

/// Compatibility alias for the former platform-scoped client view name.
pub type PlatformClient = AdapterContext;

fn strip_matching_quotes(value: &str) -> &str {
    let trimmed = value.trim();

    if trimmed.len() >= 2 {
        let quote = trimmed.as_bytes()[0];
        let last = trimmed.as_bytes()[trimmed.len() - 1];

        if (quote == b'"' || quote == b'\'') && last == quote {
            return &trimmed[1..trimmed.len() - 1];
        }
    }

    trimmed
}
