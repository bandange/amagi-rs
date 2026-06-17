use std::path::Path;

use amagi::{AmagiClient, ClientOptions};

use crate::env::{TestResult, load_local_test_env};

pub fn client_from_env(manifest_dir: impl AsRef<Path>) -> TestResult<AmagiClient> {
    load_local_test_env(manifest_dir);
    Ok(AmagiClient::new(ClientOptions::from_env()?))
}

pub fn unauthenticated_client() -> AmagiClient {
    AmagiClient::new(ClientOptions::default())
}
