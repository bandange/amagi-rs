//! Test-only workspace crate.

pub mod env;

#[cfg(feature = "client")]
pub mod client;

#[cfg(any(feature = "adapters", feature = "platforms", feature = "client"))]
pub mod bilibili;

#[cfg(any(feature = "adapters", feature = "platforms", feature = "client"))]
pub mod douyin;

#[cfg(any(feature = "adapters", feature = "platforms", feature = "client"))]
pub mod kuaishou;

#[cfg(any(feature = "adapters", feature = "platforms", feature = "client"))]
pub mod twitter;

#[cfg(any(feature = "adapters", feature = "platforms", feature = "client"))]
pub mod xiaohongshu;
