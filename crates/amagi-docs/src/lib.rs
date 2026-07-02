//! Documentation-site workspace crate.
//!
//! This crate is intentionally not published. It gives the documentation site a
//! stable workspace home without committing the project to a specific static
//! site generator yet.
#![warn(missing_docs)]

/// Marker used by workspace checks to prove the documentation crate builds.
pub const DOCS_CRATE: &str = "amagi-docs";
