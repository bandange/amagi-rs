//! SDK client constructors for integration tests.

use std::path::Path;

use amagi::{AmagiClient, ClientOptions};

use crate::env::{TestResult, load_local_test_env};

/// Build an [`AmagiClient`] from layered local test environment files.
///
/// # Errors
///
/// Returns an error when client options cannot be loaded from the environment.
pub fn client_from_env(manifest_dir: impl AsRef<Path>) -> TestResult<AmagiClient> {
    load_local_test_env(manifest_dir);
    Ok(AmagiClient::new(ClientOptions::from_env()?))
}

/// Build an unauthenticated [`AmagiClient`] with default options.
pub fn unauthenticated_client() -> AmagiClient {
    AmagiClient::new(ClientOptions::default())
}
