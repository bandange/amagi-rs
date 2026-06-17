use std::{
    env,
    error::Error,
    path::{Path, PathBuf},
};

pub type TestResult<T = ()> = Result<T, Box<dyn Error + Send + Sync>>;

pub fn repo_root_from_manifest(manifest_dir: impl AsRef<Path>) -> PathBuf {
    manifest_dir
        .as_ref()
        .join("..")
        .join("..")
        .canonicalize()
        .unwrap_or_else(|_| manifest_dir.as_ref().join("..").join(".."))
}

#[cfg(feature = "client")]
pub fn load_local_test_env(manifest_dir: impl AsRef<Path>) {
    let manifest_dir = manifest_dir.as_ref();
    let repo_root = repo_root_from_manifest(manifest_dir);

    let _ = dotenvy::from_path(manifest_dir.join(".env.test.local"));
    let _ = dotenvy::from_filename(".env.test.local");
    let _ = dotenvy::from_path(repo_root.join(".env"));
}

pub fn private_env(name: &str) -> Option<String> {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}
