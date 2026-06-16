//! Public Kuaishou request builders migrated from the TypeScript platform layer.

mod builder;
mod types;

pub use builder::{KuaishouApiUrls, create_kuaishou_api_urls};
pub use types::KuaishouGraphqlRequest;
