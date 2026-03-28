# amagi

英文版 README：[`README.md`](README.md)

面向多平台社交网页适配器的 Rust SDK、CLI 与 Web API 服务骨架。

## 概览

- 使用 `amagi run` 运行本地 CLI 工作流
- 使用 `amagi serve` 启动 JSON Web API 服务
- 使用 `cargo doc --no-deps --open` 生成 Rust API 文档
- 发布 crate 后可将文档发布到 `docs.rs`
- 完整 CLI 命令与参数参考：[`docs/reference/cli-reference.zh-CN.md`](docs/reference/cli-reference.zh-CN.md)
- 英文版 CLI 参考：[`docs/reference/cli-reference.md`](docs/reference/cli-reference.md)
- SDK 使用参考：[`docs/reference/sdk-reference.zh-CN.md`](docs/reference/sdk-reference.zh-CN.md)
- 英文版 SDK 参考：[`docs/reference/sdk-reference.md`](docs/reference/sdk-reference.md)
- Web API 参考：[`docs/reference/web-api-reference.zh-CN.md`](docs/reference/web-api-reference.zh-CN.md)
- 英文版 Web API 参考：[`docs/reference/web-api-reference.md`](docs/reference/web-api-reference.md)
- API Catalog 参考：[`docs/reference/api-catalog-reference.zh-CN.md`](docs/reference/api-catalog-reference.zh-CN.md)
- 英文版 API Catalog 参考：[`docs/reference/api-catalog-reference.md`](docs/reference/api-catalog-reference.md)
- 安装指南：[`docs/installation/README.zh-CN.md`](docs/installation/README.zh-CN.md)
- 英文版安装指南：[`docs/installation/README.md`](docs/installation/README.md)

## 快速开始

先安装 CLI。若要从仓库本地安装或查看其它安装方式，请参考
[`docs/installation/README.zh-CN.md`](docs/installation/README.zh-CN.md)。

Linux/macOS：

```bash
curl -fsSL https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.sh | bash
```

PowerShell：

```powershell
irm https://raw.githubusercontent.com/bandange/amagi-rs/main/scripts/install.ps1 | iex
```

验证命令：

```bash
amagi --version
```

常用命令示例：

```bash
amagi run
amagi run douyin video-work <aweme_id>
amagi run douyin work-comments <aweme_id> --number 20
amagi run douyin user-profile <sec_uid>
amagi run douyin search "关键词" --type general --number 10
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

## 已测试的 Twitter CLI 接口

Twitter/X CLI 参考文档已经直接标记出已测试可用的接口，以及当前存在已知偏差的接口：

- [`docs/reference/cli-reference.zh-CN.md`](docs/reference/cli-reference.zh-CN.md)

## 环境配置

二进制启动流程会自动加载分层 dotenv 配置。SDK 使用方也可以通过
`ClientOptions::from_env()` 或 `AmagiClient::from_env()` 从 dotenv
构建配置。

默认 dotenv 查找顺序：

1. 用户级配置文件
2. 当前工作目录下的 `.env`

进程环境变量仍然优先于这两层 dotenv。

用户级 dotenv 路径：

- Linux/macOS：`~/.config/amagi/.env`
- Windows：`%APPDATA%\\amagi\\.env`

如果要显式覆盖这一路径，可以设置 `AMAGI_USER_ENV_FILE`。

`.env` 示例：

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

也可以直接使用模板文件 [`.env.example`](.env.example)。

CLI 与 Web 运行时都会从进程环境变量读取平台 Cookie：

```bash
export AMAGI_DOUYIN_COOKIE="..."
export AMAGI_BILIBILI_COOKIE="..."
export AMAGI_KUAISHOU_COOKIE="..."
export AMAGI_TWITTER_COOKIE="..."
export AMAGI_XIAOHONGSHU_COOKIE="..."
```

PowerShell：

```powershell
$env:AMAGI_DOUYIN_COOKIE = "..."
$env:AMAGI_BILIBILI_COOKIE = "..."
$env:AMAGI_KUAISHOU_COOKIE = "..."
$env:AMAGI_TWITTER_COOKIE = "..."
$env:AMAGI_XIAOHONGSHU_COOKIE = "..."
```

共享运行时变量：

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

示例：

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

## CLI 输出

CLI 可以把人类可读文本或机器可读 JSON 输出到标准输出或文件。

- `--output text|json`：选择文本或 JSON 输出
- `--output-file`, `-o`：将 CLI 输出写入文件，而不是标准输出
- `--pretty`：美化 JSON 输出
- `--append`：向已有输出文件追加内容
- `--create-parent-dirs`：自动创建输出路径缺失的父目录

示例：

```bash
amagi --output json --pretty --output-file tmp/bili/login.json --create-parent-dirs run bilibili login-status
amagi --output json --output-file tmp/bili/poll.json --append run bilibili qrcode-status <qrcode_key>
```

## 功能选择

在 `Cargo.toml` 中只启用需要的能力面：

```toml
# 只使用 API 目录
amagi = { version = "0.1.0", default-features = false, features = ["catalog"] }

# 只使用 Rust SDK
amagi = { version = "0.1.0", default-features = false, features = ["client"] }

# 只使用 CLI 运行时
amagi = { version = "0.1.0", default-features = false, features = ["cli"] }

# 只使用 Web API 服务
amagi = { version = "0.1.0", default-features = false, features = ["server"] }
```

功能说明：

- `catalog`：仅导出静态目录与路由元数据。
- `client`：导出 `catalog`、客户端类型、事件、错误和 Rust 原生平台 fetcher。
- `cli`：启用命令解析器和本地运行时，不强制启用服务端能力面。
- `server`：启用基于 Axum 的 HTTP 服务端能力面，不强制启用 CLI 解析器。

## 项目结构

- `src/catalog/`：共享目录与路由元数据
- `src/client/`：客户端配置和请求默认值
- `src/events/`：类型化事件总线与事件负载模型
- `src/platforms/`：Rust 原生平台 fetcher 与共享上游传输层
- `src/server/`：HTTP 服务入口和处理器
- `src/cli/`、`src/output/`、`src/app/`：CLI、输出和运行时编排
- `src/config/`、`src/error/`、`src/telemetry/`、`src/env/`：共享运行时支持
- `docs/reference/`、`docs/installation/`：参考文档与安装说明
- `scripts/`：Linux Shell 和 PowerShell 本地安装脚本
- `src/lib.rs`：公共导出与 crate 入口
- `src/main.rs`：二进制启动入口

## Rustdoc 示例

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

## 文档约定

- 公共 API 文档注释使用 `///`
- crate 级和模块级文档使用 `//!`
- 每个公开项以一句简短总结句开头，并以句号结束
- 优先使用 intra-doc link 链接相关类型、模块和辅助函数
- 在行为可观察时优先提供 `# Examples`、`# Errors` 和 `# Panics`
- 记录公开契约，不重复实现细节
- 可以使用 `#[doc(alias = "...")]` 增加可搜索别名

## 建议风格

````text
/// 构建 HTTP 服务使用的 socket 地址。
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
