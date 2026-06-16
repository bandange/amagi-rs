# Amagi-rs

[![Cargo CI](https://github.com/bandange/amagi-rs/actions/workflows/cargo.yml/badge.svg)](https://github.com/bandange/amagi-rs/actions/workflows/cargo.yml)
[![Crates.io](https://img.shields.io/crates/v/amagi.svg)](https://crates.io/crates/amagi)
[![docs.rs](https://docs.rs/amagi/badge.svg)](https://docs.rs/amagi)
[![License: GPL-3.0-only](https://img.shields.io/badge/license-GPL--3.0--only-blue.svg)](LICENSE)

Amagi-rs 是一个面向多平台社交服务适配器的 Rust API、命令行工具和本地 JSON API 服务。

它适合需要统一平台能力入口的自动化场景：可以在终端直接使用 `amagi`，也可以在 Rust 程序里嵌入 API，或者用 `amagi serve` 启动本地 HTTP 服务。

[在线文档](https://bandange.github.io/amagi-rs/) ·
[英文 README](README.md) ·
[Crate](https://crates.io/crates/amagi) ·
[API 文档](https://docs.rs/amagi) ·
[免责声明](DISCLAIMER.zh-CN.md)

## 你可以用它做什么

- 用一套 CLI 调用 Bilibili、Douyin、Kuaishou、Twitter/X、Xiaohongshu 等平台能力
- 启动本地 JSON API 服务，给脚本、工具或后端集成调用
- 通过 feature flags 按需嵌入 Rust API
- 使用分层 dotenv 配置用户级和项目级凭据
- 使用公开 CI 安全测试，并保留本地私有真实上游测试布局
- 从仓库文档构建多语言文档站

## 安装

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

如果你的环境访问 GitHub raw 较慢或受限，可以使用代理模式：

```bash
curl -fsSL https://gh-proxy.com/https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash -s -- --proxy
```

```powershell
& ([scriptblock]::Create((irm "https://gh-proxy.com/https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1"))) -Proxy
```

更新、卸载、PATH 和 shell 集成细节请看
[安装指南](https://bandange.github.io/amagi-rs/zh/installation)。

## 使用 CLI

```bash
amagi run douyin video-work <aweme_id>
amagi run bilibili emoji-list
amagi run kuaishou live-room-info <principal_id>
amagi run twitter user-profile <screen_name>
amagi run xiaohongshu emoji-list
```

CLI 会自动读取分层配置：

- Linux/macOS：`~/.config/amagi/.env`，然后读取当前目录 `.env`
- Windows：`%APPDATA%\\amagi\\.env`，然后读取当前目录 `.env`

常用 Cookie 变量：

```dotenv
AMAGI_DOUYIN_COOKIE=
AMAGI_BILIBILI_COOKIE=
AMAGI_KUAISHOU_COOKIE=
AMAGI_TWITTER_COOKIE=
AMAGI_XIAOHONGSHU_COOKIE=
```

## 启动本地服务

```bash
amagi serve --host 127.0.0.1 --port 4567
```

服务会通过 JSON endpoint 暴露同一套适配器能力，也支持通过 `X-Amagi-Cookie`、`X-Amagi-Kuaishou-Cookie` 等请求头为单次请求覆盖 Cookie。

```bash
curl http://127.0.0.1:4567/
curl -H "X-Amagi-Cookie:" "http://127.0.0.1:4567/api/bilibili/live/21452505"
```

路由、请求头、服务模式和响应结构请看
[服务接口参考](https://bandange.github.io/amagi-rs/zh/web-api)。

## 使用 Rust API

如果你想在 Rust 中直接使用适配器能力，可以依赖公开的 `amagi` crate：

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

默认 feature：`client`、`cli`、`server`。可选能力组：

| Feature | 用途 |
| --- | --- |
| `client` | Rust 客户端类型与上游 fetcher |
| `cli` | 命令行运行时 |
| `server` | 基于 Axum 的 JSON API 服务 |
| `adapters` | Rust 原生适配器模块、fetcher 与静态 API specs |
| `platforms` | `adapters` 的兼容别名 |
| `catalog` | 静态 API spec 元数据的兼容别名 |

完整 API 使用面请看
[Rust API 参考](https://bandange.github.io/amagi-rs/zh/rust-api)
和 [docs.rs](https://docs.rs/amagi)。

## Workspace

仓库拆分为多个职责明确的 crate：

| Crate | 职责 |
| --- | --- |
| `amagi` | 公开 crate 与二进制入口 |
| `amagi-core` | 共享错误、平台类型、请求配置与 specs |
| `amagi-adapters` | 平台适配器与 fetcher |
| `amagi-client` | 高层 Rust 客户端 |
| `amagi-cli` | CLI 运行时与本地化命令输出 |
| `amagi-server` | 本地 HTTP 服务运行时 |
| `amagi-testkit` | CI 安全的公开集成测试 |
| `amagi-docs` | Dioxus 文档站，不发布到 crates.io |

从源码运行：

```bash
cargo run -p amagi -- run
cargo run -p amagi -- run douyin video-work <aweme_id>
cargo run -p amagi -- serve --host 127.0.0.1 --port 4567
```

常用检查：

```bash
cargo fmt --check
cargo check --locked --all-features
cargo test --locked --workspace
```

## 文档

在线文档发布在
[bandange.github.io/amagi-rs](https://bandange.github.io/amagi-rs/)。

仓库内的 markdown 源文件按语言分组：

- 英文：[docs/en](docs/en)
- 中文：[docs/中文](docs/中文)

重点页面：

- [安装指南](https://bandange.github.io/amagi-rs/zh/installation)
- [命令行参考](https://bandange.github.io/amagi-rs/zh/cli)
- [Rust API 参考](https://bandange.github.io/amagi-rs/zh/rust-api)
- [服务接口参考](https://bandange.github.io/amagi-rs/zh/web-api)
- [接口规格参考](https://bandange.github.io/amagi-rs/zh/api-catalog)
- [测试分层](https://bandange.github.io/amagi-rs/zh/testing)

## 免责声明

Amagi-rs 是独立项目。它不隶属于、不代表、不受赞助、不受认可，也未获得源码、文档或示例中提到的任何第三方平台、服务、品牌或商标所有者的批准。用户需要自行确保使用方式符合适用法律、合同和平台条款。

使用前请阅读完整[免责声明](DISCLAIMER.zh-CN.md)。

## License

本项目使用 [GPL-3.0-only](LICENSE) 授权。
