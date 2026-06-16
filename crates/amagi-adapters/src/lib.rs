//! Rust-native platform fetchers and shared upstream transport helpers.

pub mod bilibili;
mod context;
pub mod douyin;
mod http;
mod internal;
pub mod spec;

/// Typed Kuaishou fetchers migrated from the original TypeScript platform layer.
pub mod kuaishou;
pub mod twitter;
pub mod xiaohongshu;

pub use context::{AdapterContext, PlatformClient};
