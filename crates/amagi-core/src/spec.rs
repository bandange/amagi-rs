use serde::Serialize;

use crate::Platform;

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

/// A single published API operation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct ApiOperationSpec {
    /// Stable operation key used by the web layer.
    ///
    /// The serialized field name intentionally remains `method_key` for
    /// compatibility with existing `/api/spec` consumers.
    pub method_key: &'static str,
    /// Original Chinese operation label from the source API metadata.
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

/// Complete API operation metadata for one platform.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
pub struct PlatformApiSpec {
    /// Platform this API metadata belongs to.
    pub platform: Platform,
    /// Shared HTTP base path for the platform.
    pub api_base_path: &'static str,
    /// Every published operation for the platform.
    ///
    /// The serialized field name intentionally remains `methods` for
    /// compatibility with existing `/api/spec` consumers.
    pub methods: &'static [ApiOperationSpec],
}

/// Compatibility alias for the former method-oriented API metadata name.
pub type ApiMethodSpec = ApiOperationSpec;

/// Compatibility alias for the former platform catalog name.
pub type PlatformSpec = PlatformApiSpec;
