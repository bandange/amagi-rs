# CLI Usage

Use the command name `amagi`.

## Quick Start

Install the latest release:

```bash
curl -fsSL https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash
```

PowerShell:

```powershell
irm https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1 | iex
```

Proxy mode:

```bash
curl -fsSL https://gh-proxy.com/https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash -s -- --proxy
```

PowerShell proxy mode:

```powershell
& ([scriptblock]::Create((irm "https://gh-proxy.com/https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1"))) -Proxy
```

Verify:

```bash
amagi --version
```

## Common Examples

```bash
amagi run douyin video-work <aweme_id>
amagi run bilibili emoji-list
amagi run twitter user-profile <screen_name>
amagi serve --host 127.0.0.1 --port 4567
```

## Source Run

When the user does not want to install the binary, run from the repository root:

```bash
cargo run -p amagi -- run
cargo run -p amagi -- run douyin video-work <aweme_id>
cargo run -p amagi -- serve --host 127.0.0.1 --port 4567
```

## Terminology

- Use `Amagi-rs` for the project.
- Use `amagi` for the command, binary, or crate.
- Use "local JSON API service" or "service" instead of "web page" when describing `amagi serve`.

