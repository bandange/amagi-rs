# Amagi-rs

Chinese README: [`README.zh-CN.md`](README.zh-CN.md)

Amagi-rs is a Rust API, CLI, and JSON API service for multi-platform social service adapters.

## What It Does

- Run CLI tasks against Bilibili, Douyin, Kuaishou, Twitter/X, and Xiaohongshu
- Start a local JSON API service with `amagi serve`
- Reuse the same capability as a Rust crate with feature flags

## Quick Start

Install the latest release:

Linux/macOS:

```bash
curl -fsSL https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash
```

PowerShell:

```powershell
irm https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1 | iex
```

Use proxy mode:

Linux/macOS:

```bash
curl -fsSL https://gh-proxy.com/https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash -s -- --proxy
```

PowerShell:

```powershell
& ([scriptblock]::Create((irm "https://gh-proxy.com/https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1"))) -Proxy
```

Verify:

```bash
amagi --version
```

Common examples:

```bash
amagi run douyin video-work <aweme_id>
amagi run bilibili emoji-list
amagi run twitter user-profile <screen_name>
amagi serve --host 127.0.0.1 --port 4567
```

For install, update, uninstall, and shell integration details, see the [installation guide](docs/en/installation.md).

## Run From Source

If you do not want to install the binary, run it directly in the repository. This requires a local Rust toolchain.

```bash
cargo run -p amagi -- run
cargo run -p amagi -- run douyin video-work <aweme_id>
cargo run -p amagi -- serve --host 127.0.0.1 --port 4567
```

## Configuration

Configuration is loaded in this order:

1. the user-level dotenv file
2. the current directory `.env`

User-level dotenv path:

- Linux/macOS: `~/.config/amagi/.env`
- Windows: `%APPDATA%\\amagi\\.env`

Useful files and variables:

- [`.env.example`](.env.example)
- [`.env.example.zh-CN`](.env.example.zh-CN)
- `AMAGI_USER_ENV_FILE`
- `AMAGI_DOUYIN_COOKIE`
- `AMAGI_BILIBILI_COOKIE`
- `AMAGI_KUAISHOU_COOKIE`
- `AMAGI_TWITTER_COOKIE`
- `AMAGI_XIAOHONGSHU_COOKIE`
- `AMAGI_HOST`
- `AMAGI_PORT`

For the full environment and CLI option list, see the [CLI reference](docs/en/reference/cli.md).

## Crate Features

Default features: `client`, `cli`, `server`

Optional features:

- `adapters`: Rust-native adapter modules, fetchers, and static API specs
- `platforms`: compatibility alias for `adapters`
- `catalog`: compatibility alias for static API spec metadata
- `client`: Rust client types and upstream fetchers
- `cli`: command-line runtime
- `server`: Axum-based HTTP service

Example:

```toml
amagi = { version = "0.1.6", default-features = false, features = ["client"] }
```

## Documentation

Documentation is grouped by language. Start from the [docs index](docs/README.md), or open a language section directly.

English:

- [Installation guide](docs/en/installation.md)
- [CLI reference](docs/en/reference/cli.md)
- [Rust API reference](docs/en/reference/rust-api.md)
- [Service API reference](docs/en/reference/web-api.md)
- [API spec reference](docs/en/reference/api-catalog.md)
- [Testing layout](docs/en/testing.md)

Chinese documentation:

- [安装指南](docs/中文/安装指南.md)
- [命令行参考](docs/中文/参考/命令行参考.md)
- [Rust API 参考](docs/中文/参考/Rust API 参考.md)
- [服务接口参考](docs/中文/参考/服务接口参考.md)
- [接口规格参考](docs/中文/参考/接口规格参考.md)
- [测试分层](docs/中文/测试分层.md)
