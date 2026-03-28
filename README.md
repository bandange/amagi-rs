# amagi

中文 README: [`README.zh-CN.md`](README.zh-CN.md)

Rust SDK, CLI, and Web API service skeleton for multi-platform social web adapters.

## Overview

- Build local CLI workflows with `amagi run`
- Start a JSON Web API service with `amagi serve`
- Generate API docs with `cargo doc --no-deps --open`
- Publish generated docs to `docs.rs` after releasing the crate
- Full CLI command and parameter reference: [`docs/reference/cli-reference.md`](docs/reference/cli-reference.md)
- Chinese CLI reference: [`docs/reference/cli-reference.zh-CN.md`](docs/reference/cli-reference.zh-CN.md)
- SDK usage reference: [`docs/reference/sdk-reference.md`](docs/reference/sdk-reference.md)
- Chinese SDK reference: [`docs/reference/sdk-reference.zh-CN.md`](docs/reference/sdk-reference.zh-CN.md)
- Web API reference: [`docs/reference/web-api-reference.md`](docs/reference/web-api-reference.md)
- Chinese Web API reference: [`docs/reference/web-api-reference.zh-CN.md`](docs/reference/web-api-reference.zh-CN.md)
- API catalog reference: [`docs/reference/api-catalog-reference.md`](docs/reference/api-catalog-reference.md)
- Chinese API catalog reference: [`docs/reference/api-catalog-reference.zh-CN.md`](docs/reference/api-catalog-reference.zh-CN.md)
- Installation guide: [`docs/installation/README.md`](docs/installation/README.md)
- Chinese installation guide: [`docs/installation/README.zh-CN.md`](docs/installation/README.zh-CN.md)

## Quick Start

Install the CLI first. For local installation from a repository checkout and
other options, see [`docs/installation/README.md`](docs/installation/README.md).

Linux/macOS:

```bash
curl -fsSL https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash
```

PowerShell:

```powershell
irm https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1 | iex
```

Verify the command:

```bash
amagi --version
```

Run common commands:

```bash
amagi run
amagi run douyin video-work <aweme_id>
amagi run douyin work-comments <aweme_id> --number 20
amagi run douyin user-profile <sec_uid>
amagi run douyin search "keyword" --type general --number 10
amagi run douyin emoji-list
amagi run kuaishou video-work <photo_id>
amagi run kuaishou work-comments <photo_id>
amagi run kuaishou emoji-list
amagi run kuaishou user-profile <principal_id>
amagi run kuaishou user-work-list <principal_id> --count 24 --pcursor ""
amagi run kuaishou live-room-info <principal_id>
amagi run twitter search-tweets OpenAI --search-type latest --count 20
amagi run twitter user-profile <screen_name>
amagi run twitter user-timeline <screen_name> --count 20
amagi run twitter tweet-detail <tweet_id>
amagi run twitter space-detail <space_id>
amagi --output json --pretty --output-file tmp/emoji.json --create-parent-dirs run bilibili emoji-list
amagi --output json --output-file tmp/events.json --append run bilibili qrcode-status <qrcode_key>
amagi serve --host 127.0.0.1 --port 4567
```

## Verified Twitter CLI Interfaces

The Twitter/X CLI reference now marks tested usable interfaces directly, and
also calls out interfaces with known behavior caveats:

- [`docs/reference/cli-reference.md`](docs/reference/cli-reference.md)

## Environment

The binary startup path loads layered dotenv configuration automatically, and
SDK consumers can also build config from dotenv files through `ClientOptions::from_env()` or
`AmagiClient::from_env()`.

Default dotenv lookup order:

1. user-level config file
2. current working directory `.env`

Process environment variables still override both dotenv layers.

User-level dotenv path:

- Linux/macOS: `~/.config/amagi/.env`
- Windows: `%APPDATA%\\amagi\\.env`

Set `AMAGI_USER_ENV_FILE` to override this user-level dotenv path explicitly.

Example `.env`:

```dotenv
AMAGI_DOUYIN_COOKIE=
AMAGI_BILIBILI_COOKIE=
AMAGI_KUAISHOU_COOKIE=token=...
AMAGI_TWITTER_COOKIE=
AMAGI_XIAOHONGSHU_COOKIE=

AMAGI_TIMEOUT_MS=10000
AMAGI_MAX_RETRIES=3
AMAGI_LOG_FORMAT=text
AMAGI_LOG=info
AMAGI_OUTPUT=text
AMAGI_OUTPUT_FILE=
AMAGI_OUTPUT_PRETTY=false
AMAGI_OUTPUT_APPEND=false
AMAGI_OUTPUT_CREATE_DIRS=false
AMAGI_HOST=127.0.0.1
AMAGI_PORT=4567
```

You can also keep a template in [`.env.example`](.env.example).

The CLI and Web runtime both read platform cookies from process environment variables:

```bash
export AMAGI_DOUYIN_COOKIE="..."
export AMAGI_BILIBILI_COOKIE="..."
export AMAGI_KUAISHOU_COOKIE="..."
export AMAGI_TWITTER_COOKIE="..."
export AMAGI_XIAOHONGSHU_COOKIE="..."
```

PowerShell:

```powershell
$env:AMAGI_DOUYIN_COOKIE = "..."
$env:AMAGI_BILIBILI_COOKIE = "..."
$env:AMAGI_KUAISHOU_COOKIE = "..."
$env:AMAGI_TWITTER_COOKIE = "..."
$env:AMAGI_XIAOHONGSHU_COOKIE = "..."
```

Shared runtime variables:

- `AMAGI_USER_ENV_FILE`
- `AMAGI_TIMEOUT_MS`
- `AMAGI_MAX_RETRIES`
- `AMAGI_OUTPUT`
- `AMAGI_OUTPUT_FILE`
- `AMAGI_OUTPUT_PRETTY`
- `AMAGI_OUTPUT_APPEND`
- `AMAGI_OUTPUT_CREATE_DIRS`
- `AMAGI_LOG_FORMAT`
- `AMAGI_LOG`
- `AMAGI_HOST`
- `AMAGI_PORT`

Example:

```bash
AMAGI_DOUYIN_COOKIE="sid_guard=..." amagi run douyin emoji-list
AMAGI_DOUYIN_COOKIE="sid_guard=..." amagi run douyin user-video-list <sec_uid> --number 18
AMAGI_KUAISHOU_COOKIE="token=..." amagi run kuaishou emoji-list
AMAGI_KUAISHOU_COOKIE="token=..." amagi run kuaishou user-work-list 3xuser --count 24
AMAGI_TWITTER_COOKIE="auth_token=...; ct0=...; twid=..." amagi run twitter search-tweets OpenAI --search-type latest --count 20
AMAGI_TWITTER_COOKIE="auth_token=...; ct0=...; twid=..." amagi run twitter user-profile OpenAI
AMAGI_TWITTER_COOKIE="auth_token=...; ct0=...; twid=..." amagi run twitter tweet-detail 2028909019977703752
AMAGI_OUTPUT=json AMAGI_OUTPUT_FILE=tmp/emoji.json AMAGI_OUTPUT_PRETTY=true amagi run bilibili emoji-list
AMAGI_OUTPUT=json AMAGI_OUTPUT_FILE=tmp/events.json AMAGI_OUTPUT_APPEND=true amagi run bilibili qrcode-status <qrcode_key>
AMAGI_KUAISHOU_COOKIE="token=..." amagi serve --host 127.0.0.1 --port 4567
```

## CLI Output

The CLI can write either human-readable text or machine-readable JSON to stdout
or a file.

- `--output text|json`: choose text or JSON output
- `--output-file`, `-o`: write CLI-facing output to a file instead of stdout
- `--pretty`: pretty-print JSON payloads
- `--append`: append to an existing output file
- `--create-parent-dirs`: create missing parent directories for the output path

Examples:

```bash
amagi --output json --pretty --output-file tmp/bili/login.json --create-parent-dirs run bilibili login-status
amagi --output json --output-file tmp/bili/poll.json --append run bilibili qrcode-status <qrcode_key>
```

## Feature Selection

Choose only the surface you need in `Cargo.toml`:

```toml
# API catalog only
amagi = { version = "0.1.0", default-features = false, features = ["catalog"] }

# Rust SDK only
amagi = { version = "0.1.0", default-features = false, features = ["client"] }

# CLI runtime only
amagi = { version = "0.1.0", default-features = false, features = ["cli"] }

# Web API service only
amagi = { version = "0.1.0", default-features = false, features = ["server"] }
```

Feature notes:

- `catalog`: only exports the static catalog and route metadata.
- `client`: exports `catalog`, client types, events, errors, and Rust-native platform fetchers.
- `cli`: enables the command parser and local runtime without forcing the server surface.
- `server`: enables the Axum-based HTTP server surface without forcing the CLI parser.

## Project Layout

- `src/catalog/`: shared catalog and route metadata
- `src/client/`: client configuration and request defaults
- `src/events/`: typed event bus and payload models
- `src/platforms/`: Rust-native platform fetchers and shared upstream transport
- `src/server/`: HTTP server entrypoint and handlers
- `src/cli/`, `src/output/`, `src/app/`: CLI, output, and runtime orchestration
- `src/config/`, `src/error/`, `src/telemetry/`, `src/env/`: shared runtime support
- `docs/reference/`, `docs/installation/`: reference docs and installation guides
- `scripts/`: local installation scripts for Linux shells and PowerShell
- `src/lib.rs`: public exports and crate-level entrypoints
- `src/main.rs`: binary bootstrap

## Rustdoc Example

```rust
#[cfg(feature = "cli")]
{
    use amagi::cli::parse_from;
    use amagi::config::CommandConfig;

    let config = parse_from(["amagi", "run", "douyin", "emoji-list"]);

    match config.command {
        CommandConfig::Run(run) => assert!(!run.quiet),
        #[cfg(feature = "server")]
        CommandConfig::Serve(_) => unreachable!("expected the run subcommand"),
    }
}
```

## Documentation Conventions

- Use `///` for public API documentation comments
- Use `//!` for crate-level and module-level documentation
- Start each public item with a one-line summary sentence ending in a period
- Prefer intra-doc links for related types, modules, and helper functions
- Prefer `# Examples`, `# Errors`, and `# Panics` sections when they describe observable behavior
- Document the public contract rather than repeating implementation details
- Use `#[doc(alias = "...")]` for useful alternate English search terms

## Suggested Style

````text
/// Build the socket address used by the HTTP server.
///
/// # Examples
///
/// ```rust
/// use amagi::config::ServeConfig;
///
/// let serve = ServeConfig {
///     host: "127.0.0.1".into(),
///     port: 4567,
/// };
///
/// assert_eq!(serve.bind_addr(), "127.0.0.1:4567");
/// ```
````
