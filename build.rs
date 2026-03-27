use std::env;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=Cargo.toml");
    println!("cargo:rerun-if-env-changed=RUSTUP_TOOLCHAIN");
    println!("cargo:rerun-if-env-changed=RUSTC");
    println!("cargo:rerun-if-env-changed=TARGET");

    let rustc = env::var("RUSTC").unwrap_or_else(|_| "rustc".to_owned());
    let rustc_version = command_output(&rustc, &["-V"]).unwrap_or_else(|| "unknown".to_owned());
    let target = env::var("TARGET").unwrap_or_else(|_| "unknown".to_owned());
    let build_time = resolve_build_time();
    let toolchain = resolve_toolchain(&rustc_version);

    println!("cargo:rustc-env=AMAGI_BUILD_RUSTC={rustc_version}");
    println!("cargo:rustc-env=AMAGI_BUILD_TARGET={target}");
    println!("cargo:rustc-env=AMAGI_BUILD_TIME={build_time}");
    println!("cargo:rustc-env=AMAGI_BUILD_TOOLCHAIN={toolchain}");
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
