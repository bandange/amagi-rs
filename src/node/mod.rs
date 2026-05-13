//! Node-network support for server-mode multi-node deployments.
//!
//! This module defines the stable user-facing node concepts used by the
//! server-mode WSS transport. Low-level frame formats and session registries
//! remain internal implementation details.

mod types;

pub(crate) mod client;
pub(crate) mod protocol;
pub(crate) mod registry;
pub(crate) mod session;
pub(crate) mod upstream;

pub use types::NodeRole;
