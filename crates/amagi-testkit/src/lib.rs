//! Test-only workspace crate.
#![warn(missing_docs)]

/// Shared environment and filesystem helpers for integration tests.
pub mod env;

/// SDK client constructors used by integration tests.
#[cfg(feature = "client")]
pub mod client;

/// Bilibili fetcher constructors used by integration tests.
#[cfg(any(feature = "adapters", feature = "platforms", feature = "client"))]
pub mod bilibili;

/// Douyin fetcher constructors, seed discovery, and assertion helpers.
#[cfg(any(feature = "adapters", feature = "platforms", feature = "client"))]
pub mod douyin;

/// Kuaishou fetcher constructors used by integration tests.
#[cfg(any(feature = "adapters", feature = "platforms", feature = "client"))]
pub mod kuaishou;

/// Twitter/X fetcher constructors and live-room contract helpers.
#[cfg(any(feature = "adapters", feature = "platforms", feature = "client"))]
pub mod twitter;

/// Xiaohongshu fetcher constructors used by integration tests.
#[cfg(any(feature = "adapters", feature = "platforms", feature = "client"))]
pub mod xiaohongshu;
