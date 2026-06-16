# Amagi-rs

[![Cargo CI](https://github.com/bandange/amagi-rs/actions/workflows/cargo.yml/badge.svg)](https://github.com/bandange/amagi-rs/actions/workflows/cargo.yml)
[![Crates.io](https://img.shields.io/crates/v/amagi.svg)](https://crates.io/crates/amagi)
[![docs.rs](https://docs.rs/amagi/badge.svg)](https://docs.rs/amagi)
[![License: GPL-3.0-only](https://img.shields.io/badge/license-GPL--3.0--only-blue.svg)](https://github.com/bandange/amagi-rs/blob/main/LICENSE)

Amagi-rs is a Rust API, command-line tool, and local JSON API service for
multi-platform social service adapters.

It is designed for people who want one consistent interface for platform
automation work: use `amagi` from a terminal, embed the Rust API in an
application, or run `amagi serve` as a local HTTP service.

[Documentation](https://bandange.github.io/amagi-rs/) ·
[Chinese README](README.zh-CN.md) ·
[Crate](https://crates.io/crates/amagi) ·
[API docs](https://docs.rs/amagi) ·
[Disclaimer](DISCLAIMER.md)

## What You Get

- One CLI for Bilibili, Douyin, Kuaishou, Twitter/X, and Xiaohongshu tasks
- A local JSON API service for scripts, tools, and backend integrations
- A Rust API surface with feature flags for lighter embedding
- Layered dotenv configuration for user-level and project-level credentials
- Public CI-safe tests plus a private local test layout for real upstream checks
- A multilingual documentation site built from the repository docs

## Install

Linux/macOS:

```bash
curl -fsSL https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash
```

PowerShell:

```powershell
irm https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1 | iex
```

Verify:

```bash
amagi --version
```

If GitHub raw access is slow or blocked in your environment, use proxy mode:

```bash
curl -fsSL https://gh-proxy.com/https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash -s -- --proxy
```

```powershell
& ([scriptblock]::Create((irm "https://gh-proxy.com/https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1"))) -Proxy
```

For update, uninstall, PATH, and shell integration details, see the
[installation guide](https://bandange.github.io/amagi-rs/en/installation).

## Use The CLI

```bash
amagi run douyin video-work <aweme_id>
amagi run bilibili emoji-list
amagi run kuaishou live-room-info <principal_id>
amagi run twitter user-profile <screen_name>
amagi run xiaohongshu emoji-list
```

The CLI reads layered configuration automatically:

- Linux/macOS: `~/.config/amagi/.env`, then the current directory `.env`
- Windows: `%APPDATA%\\amagi\\.env`, then the current directory `.env`

Common cookie variables:

```dotenv
AMAGI_DOUYIN_COOKIE=
AMAGI_BILIBILI_COOKIE=
AMAGI_KUAISHOU_COOKIE=
AMAGI_TWITTER_COOKIE=
AMAGI_XIAOHONGSHU_COOKIE=
```

## Run A Local Service

```bash
amagi serve --host 127.0.0.1 --port 4567
```

The service exposes the same adapter capabilities through JSON endpoints, and
also supports per-request cookie override headers such as `X-Amagi-Cookie` and
`X-Amagi-Kuaishou-Cookie`.

```bash
curl http://127.0.0.1:4567/
curl -H "X-Amagi-Cookie:" "http://127.0.0.1:4567/api/bilibili/live/21452505"
```

See the [service API reference](https://bandange.github.io/amagi-rs/en/web-api)
for routes, headers, modes, and response shapes.

## Use The Rust API

Use the public `amagi` crate when you want the adapters from Rust:

```toml
[dependencies]
amagi = { version = "0.1.6", default-features = false, features = ["client"] }
```

```rust
use amagi::{AmagiClient, ClientOptions};

#[tokio::main]
async fn main() -> Result<(), amagi::AppError> {
    let client = AmagiClient::new(ClientOptions::from_env()?);
    let specs = client.api_specs();

    println!("registered platforms: {}", specs.len());
    Ok(())
}
```

Default features are `client`, `cli`, and `server`. Optional feature groups:

| Feature | Purpose |
| --- | --- |
| `client` | Rust client types and upstream fetchers |
| `cli` | Command-line runtime |
| `server` | Axum-based JSON API service |
| `adapters` | Rust-native adapter modules, fetchers, and static API specs |
| `platforms` | Compatibility alias for `adapters` |
| `catalog` | Compatibility alias for static API spec metadata |

See the [Rust API reference](https://bandange.github.io/amagi-rs/en/rust-api)
and [docs.rs](https://docs.rs/amagi) for the full API surface.

## Workspace

The repository is split into focused crates:

| Crate | Role |
| --- | --- |
| `amagi` | Public crate and binary entrypoint |
| `amagi-core` | Shared errors, platform types, request options, and specs |
| `amagi-adapters` | Platform adapters and fetchers |
| `amagi-client` | High-level Rust client |
| `amagi-cli` | CLI runtime and localized command output |
| `amagi-server` | Local HTTP service runtime |
| `amagi-testkit` | CI-safe public integration tests |
| `amagi-docs` | Dioxus documentation site, not published to crates.io |

Run from source:

```bash
cargo run -p amagi -- run
cargo run -p amagi -- run douyin video-work <aweme_id>
cargo run -p amagi -- serve --host 127.0.0.1 --port 4567
```

Useful checks:

```bash
cargo fmt --check
cargo check --locked --all-features
cargo test --locked --workspace
```

## Documentation

The documentation site is published at
[bandange.github.io/amagi-rs](https://bandange.github.io/amagi-rs/).

Local markdown sources are grouped by language:

- English: [docs/en](docs/en)
- Chinese: [docs/中文](docs/中文)

Key pages:

- [Installation](https://bandange.github.io/amagi-rs/en/installation)
- [CLI reference](https://bandange.github.io/amagi-rs/en/cli)
- [Rust API reference](https://bandange.github.io/amagi-rs/en/rust-api)
- [Service API reference](https://bandange.github.io/amagi-rs/en/web-api)
- [API spec reference](https://bandange.github.io/amagi-rs/en/api-catalog)
- [Testing layout](https://bandange.github.io/amagi-rs/en/testing)

## Disclaimer

Amagi-rs is an independent project. It is not affiliated with, endorsed by, or
approved by any third-party platform, service, brand, or trademark owner
referenced by the code, documentation, or examples. Users are responsible for
ensuring their use complies with applicable laws, contracts, and platform terms.

Read the full [disclaimer](DISCLAIMER.md) before use.

## License

Licensed under [GPL-3.0-only](https://github.com/bandange/amagi-rs/blob/main/LICENSE).
