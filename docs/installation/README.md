# Amagi Installation Guide

Chinese version: [README.zh-CN.md](README.zh-CN.md)

Use this guide to install `amagi`, update it, run it directly from source, and remove it.

## 1. Install

Default install locations:

- Linux/macOS: `$HOME/.local/bin`
- PowerShell: `%LOCALAPPDATA%\\Programs\\amagi\\bin`

Install from a repository checkout:

Linux/macOS:

```bash
bash scripts/install.sh
```

PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install.ps1
```

Install the latest release directly:

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

Optional install variables:

- `AMAGI_INSTALL_DIR`: override the install directory
- `AMAGI_PROFILE_FILE`: on Linux/macOS, write the shell hook into one specific profile file
- `AMAGI_INSTALL_VERSION`: install a specific release instead of `latest`
- `AMAGI_REMOTE_REPO_OWNER`, `AMAGI_REMOTE_REPO_NAME`, `AMAGI_REMOTE_BASE_URL`: override the default download source

## 2. Run From Source

Update to the latest release:

Linux/macOS:

```bash
bash scripts/update.sh
```

PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\update.ps1
```

Remote one-liner:

Linux/macOS:

```bash
curl -fsSL https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/update.sh | bash
```

PowerShell:

```powershell
irm https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/update.ps1 | iex
```

Optional update arguments:

- Linux/macOS: `--source local|remote`, `--version`, `--install-dir`
- PowerShell: `-Source Local|Remote`, `-Version`, `-InstallDir`

## 3. Run From Source

If you only want to try the project without installing the binary, run it in the repository directory. This requires a local Rust toolchain.

Examples:

```bash
cargo run -- run
cargo run -- run douyin video-work <aweme_id>
cargo run -- serve --host 127.0.0.1 --port 4567
```

## 4. Config

`amagi` reads configuration in this order:

1. the user-level dotenv file
2. the current directory `.env`

User-level dotenv path:

- Linux/macOS: `~/.config/amagi/.env`
- Windows: `%APPDATA%\\amagi\\.env`

Override variable:

- `AMAGI_USER_ENV_FILE`

During installation, the current directory `.env` file is scanned for `AMAGI_*` keys and synced into the user-level dotenv file.

On Linux/macOS, the install script also writes shell integration files:

- POSIX shells: `shell.sh` in the same directory as `AMAGI_USER_ENV_FILE`
- `fish`: `~/.config/fish/conf.d/amagi.fish`
- if `AMAGI_*` entries exist, export helper files are also generated next to `AMAGI_USER_ENV_FILE`

After installation, open a new shell. If needed, you can also `source` the generated helper file manually.

## 5. Uninstall

Linux/macOS:

```bash
bash scripts/uninstall.sh
```

PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\uninstall.ps1
```

Default uninstall behavior:

- remove the installed binary
- ask before removing persisted PATH entries and shell helper files
- ask before removing `AMAGI_*` entries from the user-level dotenv file

Optional flags:

- Linux/macOS: `--install-dir`, `--keep-path`, `--keep-user-env`, `--yes`
- PowerShell: `-InstallDir`, `-KeepPath`, `-KeepUserEnv`, `-Force`
