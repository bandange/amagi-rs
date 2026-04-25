# amagi

英文版 README：[`README.md`](README.md)

`amagi` 是一个面向多平台社交网页适配器的 Rust SDK、CLI 和 JSON Web API 服务。

## 它能做什么

- 通过 CLI 调用 Bilibili、Douyin、Kuaishou、Twitter/X、Xiaohongshu 等平台能力
- 通过 `amagi serve` 启动本地 JSON Web API
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
curl -fsSL https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash -s -- --proxy
```

PowerShell：

```powershell
$Proxy = $true; irm https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1 | iex
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

如果你需要安装、更新、卸载，或者查看 shell 集成细节，请看[安装指南](docs/installation/README.zh-CN.md)。

## 从源码运行

如果你不想先安装二进制，也可以直接在仓库目录运行。这里需要本地已经有 Rust 工具链。

```bash
cargo run -- run
cargo run -- run douyin video-work <aweme_id>
cargo run -- serve --host 127.0.0.1 --port 4567
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
- `AMAGI_USER_ENV_FILE`
- `AMAGI_DOUYIN_COOKIE`
- `AMAGI_BILIBILI_COOKIE`
- `AMAGI_KUAISHOU_COOKIE`
- `AMAGI_TWITTER_COOKIE`
- `AMAGI_XIAOHONGSHU_COOKIE`
- `AMAGI_HOST`
- `AMAGI_PORT`

完整环境变量和命令参数说明请看 [CLI 参考](docs/reference/cli-reference.zh-CN.md)。

## Crate Features

默认启用：`client`、`cli`、`server`

可选能力：

- `catalog`：仅静态目录和路由元数据
- `client`：Rust 客户端类型与上游 fetcher
- `cli`：命令行运行时
- `server`：基于 Axum 的 HTTP 服务

示例：

```toml
amagi = { version = "0.1.2", default-features = false, features = ["client"] }
```

## 文档

- 安装指南：[docs/installation/README.zh-CN.md](docs/installation/README.zh-CN.md)
- CLI 参考：[docs/reference/cli-reference.zh-CN.md](docs/reference/cli-reference.zh-CN.md)
- SDK 参考：[docs/reference/sdk-reference.zh-CN.md](docs/reference/sdk-reference.zh-CN.md)
- Web API 参考：[docs/reference/web-api-reference.zh-CN.md](docs/reference/web-api-reference.zh-CN.md)
- API Catalog 参考：[docs/reference/api-catalog-reference.zh-CN.md](docs/reference/api-catalog-reference.zh-CN.md)

英文文档：

- 安装指南：[docs/installation/README.md](docs/installation/README.md)
- CLI 参考：[docs/reference/cli-reference.md](docs/reference/cli-reference.md)
- SDK 参考：[docs/reference/sdk-reference.md](docs/reference/sdk-reference.md)
- Web API 参考：[docs/reference/web-api-reference.md](docs/reference/web-api-reference.md)
- API Catalog 参考：[docs/reference/api-catalog-reference.md](docs/reference/api-catalog-reference.md)
