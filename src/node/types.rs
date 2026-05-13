use serde::{Deserialize, Serialize};

/// Runtime node role used by the multi-node WSS transport.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NodeRole {
    /// Entry node that accepts downstream node sessions.
    Root,
    /// Leaf node that connects to an upstream node and executes local tasks.
    Worker,
    /// Intermediate node that both connects upward and accepts downstream sessions.
    Relay,
    /// Mixed local node role without explicit upstream or downstream defaults.
    Hybrid,
}
