use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-env-changed=BUILD_TYPE");
    println!("cargo:rerun-if-env-changed=AMAGI_BUILD_GIT_HASH");
    println!("cargo:rerun-if-env-changed=GITHUB_SHA");
    println!("cargo:rerun-if-env-changed=RUSTUP_TOOLCHAIN");
    println!("cargo:rerun-if-env-changed=RUSTC");
    println!("cargo:rerun-if-env-changed=TARGET");

    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    let rustc_version = command_output(&rustc, &["-V"]).unwrap_or_else(|| "unknown".to_owned());
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_owned());
    let build_time = resolve_build_time();
    let build_type = env::var("BUILD_TYPE").unwrap_or_else(|_| "local".to_owned());
    let display_version = resolve_display_version(&build_type);
    let toolchain = resolve_toolchain(&rustc_version);

    println!("cargo:rustc-env=AMAGI_DISPLAY_VERSION={display_version}");
    println!("cargo:rustc-env=AMAGI_BUILD_TYPE={build_type}");
    println!("cargo:rustc-env=AMAGI_BUILD_RUSTC={rustc_version}");
    println!("cargo:rustc-env=AMAGI_BUILD_TARGET={target}");
    println!("cargo:rustc-env=AMAGI_BUILD_TIME={build_time}");
    println!("cargo:rustc-env=AMAGI_BUILD_TOOLCHAIN={toolchain}");
}

fn resolve_display_version(build_type: &str) -> String {
    let version = env::var("CARGO_PKG_VERSION").unwrap_or_else(|_| "0.0.0".to_owned());

    match build_type {
        "release" => version,
        "daily" => {
            let git_hash = resolve_git_hash().unwrap_or_else(|| "unknown".to_owned());
            format!("daily-{git_hash}")
        }
        _ => format!("{version}-local"),
    }
}

fn resolve_git_hash() -> Option<String> {
    env_git_hash("AMAGI_BUILD_GIT_HASH")
        .or_else(|| env_git_hash("GITHUB_SHA"))
        .or_else(|| command_output("git", &["rev-parse", "--short", "HEAD"]))
        .or_else(read_git_head_hash)
        .map(short_hash)
        .filter(|value| !value.is_empty())
}

fn env_git_hash(name: &str) -> Option<String> {
    env::var(name)
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
}

fn read_git_head_hash() -> Option<String> {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").ok()?);
    let repo_root = manifest_dir.parent()?.parent()?;
    let git_dir = repo_root.join(".git");
    let head = fs::read_to_string(git_dir.join("HEAD")).ok()?;
    let head = head.trim();

    if let Some(reference) = head.strip_prefix("ref: ") {
        return read_git_ref(&git_dir, reference);
    }

    Some(head.to_owned())
}

fn read_git_ref(git_dir: &Path, reference: &str) -> Option<String> {
    fs::read_to_string(git_dir.join(reference))
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty())
        .or_else(|| read_packed_git_ref(git_dir, reference))
}

fn read_packed_git_ref(git_dir: &Path, reference: &str) -> Option<String> {
    let packed_refs = fs::read_to_string(git_dir.join("packed-refs")).ok()?;
    packed_refs
        .lines()
        .filter(|line| !line.starts_with('#') && !line.starts_with('^'))
        .find_map(|line| {
            let mut parts = line.split_whitespace();
            let hash = parts.next()?;
            let packed_reference = parts.next()?;
            (packed_reference == reference).then(|| hash.to_owned())
        })
}

fn short_hash(value: String) -> String {
    value.chars().take(7).collect()
}

fn resolve_toolchain(rustc_version: &str) -> String {
    let toolchain = env::var("RUSTUP_TOOLCHAIN")
        .ok()
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    match toolchain {
        Some(toolchain) if rustc_version != "unknown" => {
            format!("{toolchain} ({rustc_version})")
        }
        Some(toolchain) => toolchain,
        None => rustc_version.to_owned(),
    }
}

fn resolve_build_time() -> String {
    if cfg!(windows) {
        if let Some(output) = command_output(
            "powershell",
            &[
                "-NoProfile",
                "-Command",
                "(Get-Date).ToUniversalTime().ToString('yyyy-MM-ddTHH:mm:ssZ')",
            ],
        ) {
            return output;
        }
    } else if let Some(output) = command_output("date", &["-u", "+%Y-%m-%dT%H:%M:%SZ"]) {
        return output;
    }

    let seconds = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs();
    format!("unix:{seconds}")
}

fn command_output(program: &str, args: &[&str]) -> Option<String> {
    Command::new(program)
        .args(args)
        .output()
        .ok()
        .filter(|output| output.status.success())
        .and_then(|output| String::from_utf8(output.stdout).ok())
        .map(|output| output.trim().to_owned())
        .filter(|output| !output.is_empty())
}
