# Amagi 安装指南

英文版：[README.md](README.md)

这份文档只保留用户常用内容：安装、从源码运行、配置位置和卸载。

## 1. 安装

默认安装位置：

- Linux/macOS：`$HOME/.local/bin`
- PowerShell：`%LOCALAPPDATA%\\Programs\\amagi\\bin`

从仓库目录安装：

Linux/macOS：

```bash
bash scripts/install.sh
```

PowerShell：

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\install.ps1
```

直接安装最新发布版本：

Linux/macOS：

```bash
curl -fsSL https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash
```

PowerShell：

```powershell
irm https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1 | iex
```

验证：

```bash
amagi --version
```

常用安装变量：

- `AMAGI_INSTALL_DIR`：覆盖安装目录
- `AMAGI_PROFILE_FILE`：在 Linux/macOS 上只写入一个指定的 shell 启动文件
- `AMAGI_INSTALL_VERSION`：安装指定发布版本，而不是 `latest`
- `AMAGI_REMOTE_REPO_OWNER`、`AMAGI_REMOTE_REPO_NAME`、`AMAGI_REMOTE_BASE_URL`：覆盖默认下载来源

## 2. 从源码运行

如果你只是想临时试用，不想先安装二进制，可以直接在仓库目录运行。这里需要本地已经有 Rust 工具链。

示例：

```bash
cargo run -- run
cargo run -- run douyin video-work <aweme_id>
cargo run -- serve --host 127.0.0.1 --port 4567
```

## 3. 配置

`amagi` 按这个顺序读取配置：

1. 用户级 dotenv 文件
2. 当前目录下的 `.env`

用户级 dotenv 路径：

- Linux/macOS：`~/.config/amagi/.env`
- Windows：`%APPDATA%\\amagi\\.env`

覆盖变量：

- `AMAGI_USER_ENV_FILE`

安装时，当前目录 `.env` 里的 `AMAGI_*` 项会同步到用户级 dotenv 文件。

在 Linux/macOS 上，安装脚本还会写入 shell 集成文件：

- POSIX shell：在 `AMAGI_USER_ENV_FILE` 同目录下生成 `shell.sh`
- `fish`：生成 `~/.config/fish/conf.d/amagi.fish`
- 如果存在 `AMAGI_*` 项，还会在 `AMAGI_USER_ENV_FILE` 同目录下生成对应的导出辅助文件

安装完成后，重新打开一个 shell 即可。需要的话，也可以手动 `source` 生成的辅助文件。

## 4. 卸载

Linux/macOS：

```bash
bash scripts/uninstall.sh
```

PowerShell：

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\uninstall.ps1
```

默认卸载行为：

- 删除已安装的二进制
- 删除 PATH 持久化入口和 shell 辅助文件前先确认
- 删除用户级 dotenv 里的 `AMAGI_*` 项前先确认

可选参数：

- Linux/macOS：`--install-dir`、`--keep-path`、`--keep-user-env`、`--yes`
- PowerShell：`-InstallDir`、`-KeepPath`、`-KeepUserEnv`、`-Force`
