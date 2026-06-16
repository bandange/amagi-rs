//! Layered dotenv loading and environment resolution helpers.

use std::collections::BTreeMap;
use std::io;
use std::path::{Path, PathBuf};

const USER_ENV_FILE_ENV: &str = "AMAGI_USER_ENV_FILE";

/// Parsed key-value pairs loaded from a `.env` file.
pub type DotenvMap = BTreeMap<String, String>;

/// Load key-value pairs from layered dotenv sources.
///
/// The default lookup order is:
///
/// 1. user-level config file
/// 2. current working directory `.env`
///
/// Later files override earlier files. Missing files are treated as an empty
/// configuration.
///
/// # Errors
///
/// Returns an error when any discovered dotenv file cannot be read or contains
/// invalid syntax.
pub fn dotenv_values() -> io::Result<DotenvMap> {
    dotenv_values_from_layers(user_dotenv_path(), Some(PathBuf::from(".env")))
}

/// Load key-value pairs from the user-level Amagi dotenv file.
///
/// By default the path is:
///
/// - Linux/macOS: `~/.config/amagi/.env`
/// - Windows: `%APPDATA%\\amagi\\.env`
///
/// Set `AMAGI_USER_ENV_FILE` to override this path explicitly.
///
/// Missing files are treated as an empty configuration.
///
/// # Errors
///
/// Returns an error when the discovered dotenv file cannot be read or contains
/// invalid syntax.
pub fn user_dotenv_values() -> io::Result<DotenvMap> {
    match user_dotenv_path() {
        Some(path) => dotenv_values_from_path(path),
        None => Ok(DotenvMap::new()),
    }
}

/// Return the user-level Amagi dotenv path when it can be resolved.
///
/// Set `AMAGI_USER_ENV_FILE` to override the default location explicitly.
pub fn user_dotenv_path() -> Option<PathBuf> {
    let override_path = std::env::var(USER_ENV_FILE_ENV).ok();
    let appdata = std::env::var("APPDATA").ok();
    let home = std::env::var("HOME").ok();

    resolve_user_dotenv_path(
        override_path.as_deref(),
        appdata.as_deref(),
        home.as_deref(),
    )
}

/// Load and merge dotenv files from layered paths.
///
/// Values from `project_path` override values from `user_path`.
/// Missing files are treated as an empty configuration.
///
/// # Errors
///
/// Returns an error when any provided dotenv file cannot be read or contains
/// invalid syntax.
pub fn dotenv_values_from_layers<U, P>(
    user_path: Option<U>,
    project_path: Option<P>,
) -> io::Result<DotenvMap>
where
    U: AsRef<Path>,
    P: AsRef<Path>,
{
    let mut values = DotenvMap::new();

    if let Some(path) = user_path {
        values.extend(dotenv_values_from_path(path)?);
    }

    if let Some(path) = project_path {
        values.extend(dotenv_values_from_path(path)?);
    }

    Ok(values)
}

/// Load key-value pairs from a specific dotenv file path.
///
/// Missing files are treated as an empty configuration.
///
/// # Errors
///
/// Returns an error when the file cannot be read or contains invalid syntax.
pub fn dotenv_values_from_path(path: impl AsRef<Path>) -> io::Result<DotenvMap> {
    match dotenvy::from_path_iter(path.as_ref()) {
        Ok(iter) => iter
            .map(|entry| entry.map_err(dotenv_error_to_io))
            .collect::<io::Result<DotenvMap>>(),
        Err(error) if error.not_found() => Ok(DotenvMap::new()),
        Err(error) => Err(dotenv_error_to_io(error)),
    }
}

/// Resolve an environment variable from the process first, then fall back to
/// the parsed `.env` file contents.
pub fn env_or_dotenv(name: &str, dotenv: &DotenvMap) -> Option<String> {
    std::env::var(name)
        .ok()
        .or_else(|| dotenv.get(name).cloned())
}

fn resolve_user_dotenv_path(
    override_path: Option<&str>,
    appdata: Option<&str>,
    home: Option<&str>,
) -> Option<PathBuf> {
    if let Some(path) = override_path.map(str::trim).filter(|path| !path.is_empty()) {
        return Some(PathBuf::from(path));
    }

    #[cfg(windows)]
    {
        let _ = home;
        appdata
            .map(str::trim)
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)
            .map(|path| path.join("amagi").join(".env"))
    }

    #[cfg(not(windows))]
    {
        let _ = appdata;
        home.map(str::trim)
            .filter(|path| !path.is_empty())
            .map(PathBuf::from)
            .map(|path| path.join(".config").join("amagi").join(".env"))
    }
}

fn dotenv_error_to_io(error: dotenvy::Error) -> io::Error {
    let kind = if error.not_found() {
        io::ErrorKind::NotFound
    } else {
        io::ErrorKind::InvalidData
    };

    io::Error::new(kind, error)
}
