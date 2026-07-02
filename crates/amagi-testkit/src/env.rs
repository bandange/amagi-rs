//! Environment loading helpers shared by integration tests.

use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

/// Result type used by test helpers that may return arbitrary boxed errors.
pub type TestResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

/// Resolve the workspace root from a crate manifest directory.
pub fn repo_root_from_manifest(manifest_dir: impl AsRef<Path>) -> PathBuf {
    manifest_dir
        .as_ref()
        .join("..")
        .join("..")
        .canonicalize()
        .unwrap_or_else(|_| manifest_dir.as_ref().join("..").join(".."))
}

/// Load local test environment files used by client-backed integration tests.
///
/// Files are loaded in best-effort order from the crate-local test env, the
/// current directory, and the workspace root.
#[cfg(feature = "client")]
pub fn load_local_test_env(manifest_dir: impl AsRef<Path>) {
    let manifest_dir = manifest_dir.as_ref();
    let repo_root = repo_root_from_manifest(manifest_dir);

    let _ = dotenvy::from_path(manifest_dir.join(".env.test.local"));
    let _ = dotenvy::from_filename(".env.test.local");
    let _ = dotenvy::from_path(repo_root.join(".env"));
}

/// Return a non-empty environment variable value after trimming whitespace.
pub fn private_env(name: &str) -> Option<String> {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}
