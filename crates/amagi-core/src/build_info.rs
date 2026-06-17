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

/// Build metadata marker kept in optimized binaries for cross-build CI checks.
#[used]
pub static BUILD_METADATA_MARKER: &str = concat!(
    "AMAGI_BUILD_METADATA|version=",
    env!("AMAGI_DISPLAY_VERSION"),
    "|type=",
    env!("AMAGI_BUILD_TYPE"),
    "|rustc=",
    env!("AMAGI_BUILD_RUSTC"),
    "|target=",
    env!("AMAGI_BUILD_TARGET"),
    "|built=",
    env!("AMAGI_BUILD_TIME"),
    "|toolchain=",
    env!("AMAGI_BUILD_TOOLCHAIN"),
);

/// Make the metadata marker reachable from final binaries without changing output.
pub fn retain_metadata_marker() {
    std::hint::black_box(BUILD_METADATA_MARKER);
}
