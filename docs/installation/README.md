# Amagi Installation Guide

Chinese version: [README.zh-CN.md](README.zh-CN.md)

This directory documents how to install the `amagi` binary from the current
workspace or by downloading release assets from the default GitHub repository.

## 1. Installation Targets

The provided scripts install `amagi` into a user-level directory by default.

- Linux shell script: installs to `$HOME/.local/bin`
- PowerShell script: installs to `%LOCALAPPDATA%\\Programs\\amagi\\bin`

You can override the target directory:

- Linux: set `AMAGI_INSTALL_DIR`
- Linux shell profile file: set `AMAGI_PROFILE_FILE`
- PowerShell: set `AMAGI_INSTALL_DIR` or pass `-InstallDir`

## 2. Local Installation Scripts

Repository scripts:

- `scripts/install.sh`
- `scripts/install.ps1`

Linux shell:

```bash
bash scripts/install.sh
```

PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install.ps1
```

## 3. Install Modes

The generic install scripts support two modes.

1. Local mode
   When the script runs from a repository checkout or next to a local binary, it installs from local files.
2. Remote mode
   When the script runs through a streamed bootstrap such as `curl ... | bash` or `irm ... | iex`, it downloads a release asset from a predefined GitHub Releases URL.

Local mode checks these locations in order:

1. A prebuilt binary placed next to the script itself
2. When the script detects a repository workspace, it first runs `cargo build --release` to build a fresh release binary
3. The existing workspace release binary under `target/release/`
4. The workspace debug binary under `target/debug/`

Even when local mode triggers `cargo build --release`, the installer still copies
the built binary into the user-level install directory instead of writing into
Cargo's default `.cargo/bin`.

Remote mode uses this repository by default:

- repository owner: `bandange`
- repository name: `amagi-rs`

You can override them without editing the scripts:

- `AMAGI_REMOTE_REPO_OWNER`
- `AMAGI_REMOTE_REPO_NAME`
- `AMAGI_REMOTE_BASE_URL`
- `AMAGI_INSTALL_VERSION`

PowerShell bootstrap shape:

```powershell
irm https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1 | iex
```

Shell bootstrap shape:

```bash
curl -fsSL https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash
```

## 4. Typical Workflow

Build first if needed:

```bash
cargo build --release
```

Install:

```bash
bash scripts/install.sh
```

Or on Windows:

```powershell
cargo build --release
powershell -ExecutionPolicy Bypass -File .\scripts\install.ps1
```

Verify:

```bash
amagi --version
```

## 5. PATH Behavior

The scripts now persist the install directory into the user environment.

- Linux shell script: writes `export PATH="...:$PATH"` into the selected shell startup file
- PowerShell script: moves the install directory to the front of the user-level `Path`

Linux profile selection order:

1. `AMAGI_PROFILE_FILE`
2. `~/.bashrc` when the current shell is Bash
3. `~/.zshrc` when the current shell is Zsh
4. `~/.profile` for all other shells

Notes:

- Linux: if you run the script with `bash scripts/install.sh`, open a new shell or `source` the updated profile file
- PowerShell: the current PowerShell process is updated immediately, and future sessions continue to prefer the user install directory over an older copy in `.cargo\bin`

## 6. Global Command Config

After installation, `amagi` also reads a user-level dotenv file so the command
can work consistently outside a project directory.

- Linux/macOS: `~/.config/amagi/.env`
- Windows: `%APPDATA%\\amagi\\.env`

Override variable:

- `AMAGI_USER_ENV_FILE`

Current directory `.env` still overrides this user-level file.

The install scripts also sync `AMAGI_*` entries from the current directory
`.env` into the user-level `.env` during installation. Existing user-level
entries are merged by key, and the current directory values win for matching
keys.

## 7. Uninstall

Repository scripts:

- `scripts/uninstall.sh`
- `scripts/uninstall.ps1`

Linux shell:

```bash
bash scripts/uninstall.sh
```

PowerShell:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\uninstall.ps1
```

Default uninstall behavior:

- remove the installed binary from detected install directories
- ask for confirmation before removing matching persisted PATH entries
- ask for confirmation before removing `AMAGI_*` entries from the user-level dotenv file
- remove the user-level dotenv file or empty `amagi` directory when they become empty

Optional flags:

- Linux shell: `--install-dir`, `--keep-path`, `--keep-user-env`, `--yes`
- PowerShell: `-InstallDir`, `-KeepPath`, `-KeepUserEnv`, `-Force`

The current project `.env` is not modified by the uninstall scripts.
