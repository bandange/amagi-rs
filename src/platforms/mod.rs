//! Rust-native platform fetchers and shared upstream transport helpers.

pub mod bilibili;
pub mod douyin;
mod http;
mod internal;

/// Typed Kuaishou fetchers migrated from the original TypeScript platform layer.
pub mod kuaishou;
pub mod twitter;
pub mod xiaohongshu;
