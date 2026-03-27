//! Shared catalog metadata and route descriptors.
//!
//! This module ports the cross-cutting API specification from the TypeScript
//! `dev/types/api-spec.ts` entrypoint into Rust so the client and server can
//! share one canonical catalog.

mod catalog;
mod lookup;
mod types;

pub use catalog::{all_platform_specs, platform_spec};
pub use lookup::{
    find_method, get_api_route, get_chinese_method_name, get_english_method_name, method_specs,
};
pub use types::{ApiMethodSpec, HttpMethod, ParsePlatformError, Platform, PlatformSpec};
