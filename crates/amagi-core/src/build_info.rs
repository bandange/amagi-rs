//! Compile-time build metadata shared by CLI, server, and SDK facades.

/// Version string shown to users and exposed by service metadata.
pub const DISPLAY_VERSION: &str = env!("AMAGI_DISPLAY_VERSION");
/// Build channel, such as `local`, `daily`, or `release`.
pub const BUILD_TYPE: &str = env!("AMAGI_BUILD_TYPE");
/// UTC timestamp captured when the crate was compiled.
pub const BUILD_TIME: &str = env!("AMAGI_BUILD_TIME");
/// Rust compiler version used for the build.
pub const BUILD_RUSTC: &str = env!("AMAGI_BUILD_RUSTC");
/// Rust target triple used for the build.
pub const BUILD_TARGET: &str = env!("AMAGI_BUILD_TARGET");
/// Rustup toolchain label used for the build, when available.
pub const BUILD_TOOLCHAIN: &str = env!("AMAGI_BUILD_TOOLCHAIN");
