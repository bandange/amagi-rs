# Amagi-rs

英文版 README：[`README.md`](README.md)

Amagi-rs 是一个面向多平台社交服务适配器的 Rust API、CLI 和 JSON API 服务。

## 它能做什么

- 通过 CLI 调用 Bilibili、Douyin、Kuaishou、Twitter/X、Xiaohongshu 等平台能力
- 通过 `amagi serve` 启动本地 JSON API 服务
- 作为 Rust crate 按需启用能力面

## 快速开始

安装最新发布版本：

Linux/macOS：

```bash
curl -fsSL https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash
```

PowerShell：

```powershell
irm https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1 | iex
```

使用代理模式：

Linux/macOS：

```bash
curl -fsSL https://gh-proxy.com/https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash -s -- --proxy
```

PowerShell：

```powershell
& ([scriptblock]::Create((irm "https://gh-proxy.com/https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1"))) -Proxy
```

验证：

```bash
amagi --version
```

常见示例：

```bash
amagi run douyin video-work <aweme_id>
amagi run bilibili emoji-list
amagi run twitter user-profile <screen_name>
amagi serve --host 127.0.0.1 --port 4567
```

如果你需要安装、更新、卸载，或者查看 shell 集成细节，请看[安装指南](docs/中文/安装指南.md)。

## 从源码运行

如果你不想先安装二进制，也可以直接在仓库目录运行。这里需要本地已经有 Rust 工具链。

```bash
cargo run -p amagi -- run
cargo run -p amagi -- run douyin video-work <aweme_id>
cargo run -p amagi -- serve --host 127.0.0.1 --port 4567
```

## 配置

配置按这个顺序加载：

1. 用户级 dotenv 文件
2. 当前目录下的 `.env`

用户级 dotenv 路径：

- Linux/macOS：`~/.config/amagi/.env`
- Windows：`%APPDATA%\\amagi\\.env`

常用文件和变量：

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

完整环境变量和命令参数说明请看 [CLI 参考](docs/中文/参考/命令行参考.md)。

## Crate Features

默认启用：`client`、`cli`、`server`

可选能力：

- `adapters`：Rust 原生适配器模块、fetcher 与静态 API spec
- `platforms`：`adapters` 的兼容别名
- `catalog`：静态 API spec 元数据的兼容别名
- `client`：Rust 客户端类型与上游 fetcher
- `cli`：命令行运行时
- `server`：基于 Axum 的 HTTP 服务

示例：

```toml
amagi = { version = "0.1.6", default-features = false, features = ["client"] }
```

## 文档

文档现在按语言分组。可以从[文档目录](docs/README.md)进入，也可以直接打开下面的中文文档。

中文文档：

- [安装指南](docs/中文/安装指南.md)
- [命令行参考](docs/中文/参考/命令行参考.md)
- [Rust API 参考](docs/中文/参考/Rust API 参考.md)
- [服务接口参考](docs/中文/参考/服务接口参考.md)
- [接口规格参考](docs/中文/参考/接口规格参考.md)
- [测试分层](docs/中文/测试分层.md)

英文文档：

- [Installation guide](docs/en/installation.md)
- [CLI reference](docs/en/reference/cli.md)
- [Rust API reference](docs/en/reference/rust-api.md)
- [Service API reference](docs/en/reference/web-api.md)
- [API spec reference](docs/en/reference/api-catalog.md)
- [Testing layout](docs/en/testing.md)
