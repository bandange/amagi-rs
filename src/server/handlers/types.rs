use serde::Serialize;

use crate::catalog::{ApiMethodSpec, Platform};

/// Summary of one published platform.
#[derive(Debug, Serialize)]
pub struct PlatformSummary {
    /// Stable platform identifier.
    pub platform: Platform,
    /// Shared API base path for the platform.
    pub api_base_path: &'static str,
    /// Number of published methods for the platform.
    pub method_count: usize,
    /// Whether the current client has a bound cookie for the platform.
    pub has_cookie: bool,
}

/// Root metadata payload for the HTTP server.
#[derive(Debug, Serialize)]
pub struct RootResponse {
    /// Application name.
    pub name: &'static str,
    /// Application version.
    pub version: &'static str,
    /// Service mode.
    pub mode: &'static str,
    /// Service status.
    pub status: &'static str,
    /// Effective bind address.
    pub bind: String,
    /// Effective base URL.
    pub base_url: String,
    /// Published metadata endpoints.
    pub endpoints: Vec<&'static str>,
    /// Published platform summaries.
    pub platforms: Vec<PlatformSummary>,
}

/// Health check payload.
#[derive(Debug, Serialize)]
pub struct HealthResponse {
    /// Health status.
    pub status: &'static str,
    /// Service name.
    pub service: &'static str,
    /// Service version.
    pub version: &'static str,
}

/// Catalog payload for every platform.
#[derive(Debug, Serialize)]
pub struct ApiCatalogResponse {
    /// Service version used to build the catalog.
    pub version: &'static str,
    /// Every published platform catalog.
    pub platforms: Vec<PlatformCatalogResponse>,
}

/// Catalog payload for one platform.
#[derive(Debug, Serialize)]
pub struct PlatformCatalogResponse {
    /// Platform this catalog belongs to.
    pub platform: Platform,
    /// Shared API base path for the platform.
    pub api_base_path: &'static str,
    /// Number of published methods for the platform.
    pub method_count: usize,
    /// Every published method for the platform.
    pub methods: Vec<ApiMethodSpec>,
}

/// Error payload returned for invalid catalog requests.
#[derive(Debug, Serialize)]
pub struct CatalogErrorResponse {
    /// Human-readable error message.
    pub error: &'static str,
    /// Raw platform segment from the request path.
    pub platform: String,
}

/// Error payload returned for fetch failures.
#[derive(Debug, Serialize)]
pub struct FetchErrorResponse {
    /// Human-readable error category.
    pub error: &'static str,
    /// Detailed error message.
    pub detail: String,
}
