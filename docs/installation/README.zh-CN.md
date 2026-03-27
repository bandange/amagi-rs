# Amagi 安装指南

英文版：[README.md](README.md)

这里说明如何从当前工作区安装 `amagi` 二进制，或者通过默认 GitHub 仓库的 Releases 远程下载安装。

## 1. 默认安装位置

提供的脚本默认安装到用户级目录。

- Linux Shell 脚本：安装到 `$HOME/.local/bin`
- PowerShell 脚本：安装到 `%LOCALAPPDATA%\\Programs\\amagi\\bin`

也可以覆盖默认目录：

- Linux：设置 `AMAGI_INSTALL_DIR`
- Linux Shell 启动文件：设置 `AMAGI_PROFILE_FILE`
- PowerShell：设置 `AMAGI_INSTALL_DIR`，或传入 `-InstallDir`

## 2. 本地安装脚本

仓库脚本：

- `scripts/install.sh`
- `scripts/install.ps1`

Linux Shell：

```bash
bash scripts/install.sh
```

PowerShell：

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install.ps1
```

## 3. 安装模式

通用安装脚本支持两种模式。

1. 本地模式
   当脚本在仓库工作区内执行，或者脚本同级存在本地二进制时，优先走本地安装。
2. 远程模式
   当脚本通过 `curl ... | bash` 或 `irm ... | iex` 这类流式方式执行时，走预定义 GitHub Releases 地址下载。

本地模式按以下顺序查找可安装的二进制：

1. 脚本所在目录下的预编译二进制
2. 如果检测到当前是仓库工作区，则优先执行 `cargo build --release` 现编最新 release
3. 工作区 `target/release/` 下已有的 release 二进制
4. 工作区 `target/debug/` 下的 debug 二进制

即使在本地模式下触发了 `cargo build --release`，安装脚本也只会把构建产物复制到用户级安装目录，
不会写入 Cargo 默认的 `.cargo\bin`。

远程模式默认使用当前仓库：

- 仓库用户：`bandange`
- 仓库名称：`amagi-rs`

不改脚本也可以通过环境变量覆盖：

- `AMAGI_REMOTE_REPO_OWNER`
- `AMAGI_REMOTE_REPO_NAME`
- `AMAGI_REMOTE_BASE_URL`
- `AMAGI_INSTALL_VERSION`

PowerShell 远程安装示例：

```powershell
irm https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1 | iex
```

Shell 远程安装示例：

```bash
curl -fsSL https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash
```

## 4. 常见安装流程

如果还没有 release 构建产物，可以先编译：

```bash
cargo build --release
```

然后安装：

```bash
bash scripts/install.sh
```

Windows：

```powershell
cargo build --release
powershell -ExecutionPolicy Bypass -File .\scripts\install.ps1
```

验证：

```bash
amagi --version
```

## 5. PATH 行为

脚本现在会把安装目录持久化到用户环境中。

- Linux Shell 脚本：向选定的 shell 启动文件写入 `export PATH="...:$PATH"`
- PowerShell 脚本：把安装目录放到用户级 `Path` 的最前面

Linux 启动文件选择顺序：

1. `AMAGI_PROFILE_FILE`
2. 当前 shell 为 Bash 时使用 `~/.bashrc`
3. 当前 shell 为 Zsh 时使用 `~/.zshrc`
4. 其他情况使用 `~/.profile`

说明：

- Linux：如果你是通过 `bash scripts/install.sh` 执行脚本，需要重新打开一个 shell，或者手动 `source` 对应启动文件
- PowerShell：当前 PowerShell 进程会立即更新，后续新开终端会继续优先使用用户安装目录，而不是较早安装在 `.cargo\bin` 的旧副本

## 6. 全局命令配置

安装完成后，`amagi` 还会读取用户级 dotenv 文件，这样在任意目录执行全局命令时也能有稳定配置。

- Linux/macOS：`~/.config/amagi/.env`
- Windows：`%APPDATA%\\amagi\\.env`

覆盖变量：

- `AMAGI_USER_ENV_FILE`

当前工作目录下的 `.env` 仍然会覆盖这份用户级配置。

安装脚本还会在安装时把当前目录 `.env` 里的 `AMAGI_*` 项同步到这份用户级 `.env`。
如果用户级文件已存在，则按 key 合并；同名项以当前目录 `.env` 为准。

## 7. 卸载

仓库脚本：

- `scripts/uninstall.sh`
- `scripts/uninstall.ps1`

Linux Shell：

```bash
bash scripts/uninstall.sh
```

PowerShell：

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\uninstall.ps1
```

默认卸载行为：

- 从检测到的安装目录中删除已安装二进制
- 删除匹配的 PATH 持久化入口前先交互确认
- 删除用户级 dotenv 文件中的 `AMAGI_*` 项前先交互确认
- 当用户级 dotenv 文件或 `amagi` 目录变空时一并删除

可选参数：

- Linux Shell：`--install-dir`、`--keep-path`、`--keep-user-env`、`--yes`
- PowerShell：`-InstallDir`、`-KeepPath`、`-KeepUserEnv`、`-Force`

卸载脚本不会修改当前项目目录下的 `.env`。
