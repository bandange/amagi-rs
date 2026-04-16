# amagi

Chinese README: [`README.zh-CN.md`](README.zh-CN.md)

`amagi` is a Rust SDK, CLI, and JSON Web API service for multi-platform social web adapters.

## What It Does

- Run CLI tasks against Bilibili, Douyin, Kuaishou, Twitter/X, and Xiaohongshu
- Start a local JSON Web API with `amagi serve`
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

For repository install, uninstall, and shell integration details, see the [installation guide](docs/installation/README.md).

## Run From Source

If you do not want to install the binary, run it directly in the repository. This requires a local Rust toolchain.

```bash
cargo run -- run
cargo run -- run douyin video-work <aweme_id>
cargo run -- serve --host 127.0.0.1 --port 4567
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
- `AMAGI_USER_ENV_FILE`
- `AMAGI_DOUYIN_COOKIE`
- `AMAGI_BILIBILI_COOKIE`
- `AMAGI_KUAISHOU_COOKIE`
- `AMAGI_TWITTER_COOKIE`
- `AMAGI_XIAOHONGSHU_COOKIE`
- `AMAGI_HOST`
- `AMAGI_PORT`

For the full environment and CLI option list, see the [CLI reference](docs/reference/cli-reference.md).

## Crate Features

Default features: `client`, `cli`, `server`

Optional features:

- `catalog`: static catalog and route metadata only
- `client`: Rust client types and upstream fetchers
- `cli`: command-line runtime
- `server`: Axum-based HTTP service

Example:

```toml
amagi = { version = "0.1.2", default-features = false, features = ["client"] }
```

## Documentation

- Installation guide: [docs/installation/README.md](docs/installation/README.md)
- CLI reference: [docs/reference/cli-reference.md](docs/reference/cli-reference.md)
- SDK reference: [docs/reference/sdk-reference.md](docs/reference/sdk-reference.md)
- Web API reference: [docs/reference/web-api-reference.md](docs/reference/web-api-reference.md)
- API catalog reference: [docs/reference/api-catalog-reference.md](docs/reference/api-catalog-reference.md)

Chinese documentation:

- Installation guide: [docs/installation/README.zh-CN.md](docs/installation/README.zh-CN.md)
- CLI reference: [docs/reference/cli-reference.zh-CN.md](docs/reference/cli-reference.zh-CN.md)
- SDK reference: [docs/reference/sdk-reference.zh-CN.md](docs/reference/sdk-reference.zh-CN.md)
- Web API reference: [docs/reference/web-api-reference.zh-CN.md](docs/reference/web-api-reference.zh-CN.md)
- API catalog reference: [docs/reference/api-catalog-reference.zh-CN.md](docs/reference/api-catalog-reference.zh-CN.md)
