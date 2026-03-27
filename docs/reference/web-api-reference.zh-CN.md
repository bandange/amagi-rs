# Amagi Web API 参考

英文版：[`web-api-reference.md`](web-api-reference.md)

本文档覆盖 `server` feature 暴露的内置 HTTP 服务：

- 如何启动服务
- 元数据与健康检查接口
- 响应与错误约定
- 全部已发布业务路由
- 每个路由的路径参数、查询参数与请求体

## 1. 启用与启动 Web 面

如果你想在自己的 Rust 程序中嵌入这个 Web 服务：

```toml
[dependencies]
amagi = { version = "0.1.0", default-features = false, features = ["server"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

如果你想直接使用仓库内置的 `amagi serve` 二进制命令，需要保留默认 feature，
或者同时启用 `cli` 与 `server`。当前仓库默认就是这样配置的。

### 1.1 通过内置命令启动

```bash
cargo run -- serve --host 127.0.0.1 --port 4567
```

### 1.2 以库方式启动

```rust
use amagi::config::{OutputConfig, OutputFormat, ServeConfig};
use amagi::output::Printer;
use amagi::server;
use amagi::AmagiClient;

#[tokio::main]
async fn main() -> Result<(), amagi::AppError> {
    let client = AmagiClient::from_env()?;
    let printer = Printer::new(OutputConfig {
        format: OutputFormat::Text,
        file: None,
        pretty: false,
        append: false,
        create_parent_dirs: false,
    });

    server::serve(
        ServeConfig {
            host: "127.0.0.1".into(),
            port: 4567,
        },
        client,
        &printer,
    )
    .await
}
```

## 2. 运行时环境变量

Web 运行时读取和 SDK 相同的共享环境变量：

- `AMAGI_DOUYIN_COOKIE`
- `AMAGI_BILIBILI_COOKIE`
- `AMAGI_KUAISHOU_COOKIE`
- `AMAGI_TWITTER_COOKIE`
- `AMAGI_XIAOHONGSHU_COOKIE`
- `AMAGI_TIMEOUT_MS`
- `AMAGI_MAX_RETRIES`
- `AMAGI_HOST`
- `AMAGI_PORT`

`amagi serve` 在启动时也会自动加载分层 dotenv 配置。

默认 dotenv 查找顺序：

1. 用户级配置文件
2. 当前工作目录下的 `.env`

用户级配置 dotenv 路径：

- Linux/macOS：`~/.config/amagi/.env`
- Windows：`%APPDATA%\\amagi\\.env`

覆盖变量：

- `AMAGI_USER_ENV_FILE`

## 3. 响应约定

### 3.1 成功响应

成功响应分为两类：

- 元数据接口返回包装类型，如 `RootResponse`、`HealthResponse`、
  `ApiCatalogResponse`、`PlatformCatalogResponse`。
- 业务接口返回 SDK fetcher 的原始平台 JSON 结果。

### 3.2 错误响应

Catalog 查询错误：

```json
{
  "error": "unknown platform",
  "platform": "invalid-name"
}
```

Fetcher 执行错误：

```json
{
  "error": "fetch_failed",
  "detail": "..."
}
```

状态码策略：

- `500 Internal Server Error`：本地 IO 或请求配置非法
- `502 Bad Gateway`：上游 HTTP、上游 JSON、上游响应错误
- `404 Not Found`：`/api/spec/{platform}` 中的平台名非法

### 3.3 参数约定

- 路径参数使用 `{name}` 表示。
- 可选查询参数使用 `?` 标记。
- `POST` 请求体为 JSON。
- 当前全部路由都返回 JSON。

## 4. 元数据接口

| 方法 | 路径 | 参数 | 响应 | 说明 |
| --- | --- | --- | --- | --- |
| `GET` | `/` | 无 | `RootResponse` | 返回服务元数据、绑定地址、基础 URL、已发布接口列表和每个平台的 Cookie 状态 |
| `GET` | `/health` | 无 | `HealthResponse` | 返回健康检查结果 |
| `GET` | `/api/spec` | 无 | `ApiCatalogResponse` | 返回全部平台的静态 API catalog |
| `GET` | `/api/spec/{platform}` | 路径：`platform` | `PlatformCatalogResponse` 或 `CatalogErrorResponse` | 返回单个平台的静态 API catalog |

合法 `{platform}` 取值：

- `bilibili`
- `douyin`
- `kuaishou`
- `twitter`
- `xiaohongshu`

## 5. Bilibili 路由

基础路径：`/api/bilibili`

| 方法 | 路径 | 路径参数 | 查询 / 请求体 | 说明 |
| --- | --- | --- | --- | --- |
| `GET` | `/api/bilibili/video/{bvid}` | `bvid` | 无 | 获取 Bilibili 视频详情 |
| `GET` | `/api/bilibili/video/{aid}/stream` | `aid` | 查询：`cid` | 获取 Bilibili 视频播放流地址 |
| `GET` | `/api/bilibili/video/{cid}/danmaku` | `cid` | 查询：`segment_index?` | 获取单个弹幕分段 |
| `GET` | `/api/bilibili/bangumi/{bangumi_id}` | `bangumi_id` | 无 | 获取番剧元数据 |
| `GET` | `/api/bilibili/bangumi/{ep_id}/stream` | `ep_id` | 查询：`cid` | 获取番剧播放流地址 |
| `GET` | `/api/bilibili/article/{id}/content` | `id` | 无 | 获取专栏正文 |
| `GET` | `/api/bilibili/article/cards` | 无 | 查询：`ids` | 按逗号分隔的多个 id 获取专栏卡片 |
| `GET` | `/api/bilibili/article/{id}` | `id` | 无 | 获取专栏元数据 |
| `GET` | `/api/bilibili/article-list/{id}` | `id` | 无 | 获取文集元数据 |
| `POST` | `/api/bilibili/captcha` | 无 | 请求体：`v_voucher`、`csrf?` | 申请验证码挑战 |
| `POST` | `/api/bilibili/captcha/validate` | 无 | 请求体：`challenge`、`token`、`validate`、`seccode`、`csrf?` | 校验验证码结果 |
| `GET` | `/api/bilibili/user/{host_mid}` | `host_mid` | 无 | 获取用户卡片 |
| `GET` | `/api/bilibili/user/{host_mid}/dynamics` | `host_mid` | 无 | 获取用户动态列表 |
| `GET` | `/api/bilibili/user/{host_mid}/space` | `host_mid` | 无 | 获取用户空间详情 |
| `GET` | `/api/bilibili/user/{host_mid}/total-views` | `host_mid` | 无 | 获取 UP 主总播放量 |
| `GET` | `/api/bilibili/comments/{oid}` | `oid` | 查询：`type`、`number?`、`mode?` | 获取评论列表 |
| `GET` | `/api/bilibili/comment-replies/{oid}/{root}` | `oid`、`root` | 查询：`type`、`number?` | 获取根评论回复列表 |
| `GET` | `/api/bilibili/dynamic/{dynamic_id}` | `dynamic_id` | 无 | 获取动态详情 |
| `GET` | `/api/bilibili/dynamic/{dynamic_id}/card` | `dynamic_id` | 无 | 获取动态卡片 |
| `GET` | `/api/bilibili/live/{room_id}` | `room_id` | 无 | 获取直播间详情 |
| `GET` | `/api/bilibili/live/{room_id}/init` | `room_id` | 无 | 获取直播间初始化数据 |
| `GET` | `/api/bilibili/auth/status` | 无 | 无 | 获取当前登录状态 |
| `GET` | `/api/bilibili/auth/qrcode` | 无 | 无 | 申请登录二维码 |
| `GET` | `/api/bilibili/auth/qrcode/status` | 无 | 查询：`qrcode_key` | 轮询二维码状态 |
| `GET` | `/api/bilibili/emoji` | 无 | 无 | 获取表情列表 |
| `GET` | `/api/bilibili/convert/av/{aid}` | `aid` | 无 | AV 转 BV |
| `GET` | `/api/bilibili/convert/bv/{bvid}` | `bvid` | 无 | BV 转 AV |

## 6. 抖音路由

基础路径：`/api/douyin`

| 方法 | 路径 | 路径参数 | 查询 / 请求体 | 说明 |
| --- | --- | --- | --- | --- |
| `GET` | `/api/douyin/work/{aweme_id}` | `aweme_id` | 无 | 聚合解析作品并推断类型 |
| `GET` | `/api/douyin/work/{aweme_id}/video` | `aweme_id` | 无 | 获取视频作品 |
| `GET` | `/api/douyin/work/{aweme_id}/image-album` | `aweme_id` | 无 | 获取图文作品 |
| `GET` | `/api/douyin/work/{aweme_id}/slides` | `aweme_id` | 无 | 获取图集作品 |
| `GET` | `/api/douyin/work/{aweme_id}/text` | `aweme_id` | 无 | 获取文字作品 |
| `GET` | `/api/douyin/comments/{aweme_id}` | `aweme_id` | 查询：`number?`、`cursor?` | 获取作品评论 |
| `GET` | `/api/douyin/comment-replies/{aweme_id}/{comment_id}` | `aweme_id`、`comment_id` | 查询：`number?`、`cursor?` | 获取评论回复 |
| `GET` | `/api/douyin/user/{sec_uid}` | `sec_uid` | 无 | 获取用户主页 |
| `GET` | `/api/douyin/user/{sec_uid}/videos` | `sec_uid` | 查询：`number?`、`max_cursor?` | 获取用户视频列表 |
| `GET` | `/api/douyin/user/{sec_uid}/favorites` | `sec_uid` | 查询：`number?`、`max_cursor?` | 获取用户收藏列表 |
| `GET` | `/api/douyin/user/{sec_uid}/recommends` | `sec_uid` | 查询：`number?`、`max_cursor?` | 获取用户推荐列表 |
| `GET` | `/api/douyin/search` | 无 | 查询：`query`、`type?`、`number?`、`search_id?` | 搜索抖音内容 |
| `GET` | `/api/douyin/search/suggest` | 无 | 查询：`query` | 获取搜索联想词 |
| `GET` | `/api/douyin/emoji` | 无 | 无 | 获取表情列表 |
| `GET` | `/api/douyin/emoji/dynamic` | 无 | 无 | 获取动态表情数据 |
| `GET` | `/api/douyin/music/{music_id}` | `music_id` | 无 | 获取音乐元数据 |
| `GET` | `/api/douyin/live/{room_id}` | `room_id` | 查询：`web_rid` | 获取直播间信息 |
| `GET` | `/api/douyin/auth/qrcode` | 无 | 查询：`verify_fp?` | 申请登录二维码 |
| `GET` | `/api/douyin/danmaku/{aweme_id}` | `aweme_id` | 查询：`duration`、`start_time?`、`end_time?` | 获取弹幕数据 |

支持的查询枚举：

- `/api/douyin/search`：`type=general|user|video`

## 7. 快手路由

基础路径：`/api/kuaishou`

| 方法 | 路径 | 路径参数 | 查询 / 请求体 | 说明 |
| --- | --- | --- | --- | --- |
| `GET` | `/api/kuaishou/work/{photo_id}` | `photo_id` | 无 | 获取单个作品 |
| `GET` | `/api/kuaishou/comments/{photo_id}` | `photo_id` | 无 | 获取作品评论 |
| `GET` | `/api/kuaishou/emoji` | 无 | 无 | 获取表情列表 |
| `GET` | `/api/kuaishou/user/{principal_id}` | `principal_id` | 无 | 获取用户主页 |
| `GET` | `/api/kuaishou/user/{principal_id}/works` | `principal_id` | 查询：`pcursor?`、`count?` | 获取用户作品列表 |
| `GET` | `/api/kuaishou/live/{principal_id}` | `principal_id` | 无 | 获取直播间信息 |

## 8. Twitter / X 路由

基础路径：`/api/twitter`

| 方法 | 路径 | 路径参数 | 查询 / 请求体 | 说明 |
| --- | --- | --- | --- | --- |
| `GET` | `/api/twitter/user/{screen_name}` | `screen_name` | 无 | 获取用户资料 |
| `GET` | `/api/twitter/user/{screen_name}/timeline` | `screen_name` | 查询：`count?`、`cursor?` | 获取用户时间线 |
| `GET` | `/api/twitter/user/{screen_name}/replies` | `screen_name` | 查询：`count?`、`cursor?` | 获取用户回复流 |
| `GET` | `/api/twitter/user/{screen_name}/media` | `screen_name` | 查询：`count?`、`cursor?` | 获取用户媒体流 |
| `GET` | `/api/twitter/user/{screen_name}/followers` | `screen_name` | 查询：`count?`、`cursor?` | 获取用户粉丝列表 |
| `GET` | `/api/twitter/user/{screen_name}/following` | `screen_name` | 查询：`count?`、`cursor?` | 获取用户关注列表 |
| `GET` | `/api/twitter/search/tweets` | 无 | 查询：`query`、`search_type?`、`count?`、`cursor?` | 搜索推文 |
| `GET` | `/api/twitter/tweet/{tweet_id}` | `tweet_id` | 无 | 获取推文详情 |
| `GET` | `/api/twitter/space/{space_id}` | `space_id` | 无 | 获取 Space 详情 |

支持的查询枚举：

- `/api/twitter/search/tweets`：`search_type=latest|top`

## 9. 小红书路由

基础路径：`/api/xiaohongshu`

| 方法 | 路径 | 路径参数 | 查询 / 请求体 | 说明 |
| --- | --- | --- | --- | --- |
| `GET` | `/api/xiaohongshu/feed` | 无 | 查询：`cursor_score?`、`num?`、`refresh_type?`、`note_index?`、`category?`、`search_key?` | 获取首页流 |
| `GET` | `/api/xiaohongshu/note/{note_id}` | `note_id` | 查询：`xsec_token` | 获取单个笔记 |
| `GET` | `/api/xiaohongshu/comments/{note_id}` | `note_id` | 查询：`xsec_token`、`cursor?` | 获取笔记评论 |
| `GET` | `/api/xiaohongshu/user/{user_id}` | `user_id` | 无 | 获取用户资料 |
| `GET` | `/api/xiaohongshu/user/{user_id}/notes` | `user_id` | 查询：`cursor?`、`num?` | 获取用户笔记列表 |
| `GET` | `/api/xiaohongshu/emoji` | 无 | 无 | 获取表情列表 |
| `GET` | `/api/xiaohongshu/search` | 无 | 查询：`keyword`、`page?`、`page_size?`、`sort?`、`note_type?` | 搜索笔记 |

支持的查询枚举：

- `/api/xiaohongshu/search`：`sort=general|time_descending|popularity_descending`
- `/api/xiaohongshu/search`：`note_type=all|video|image`

## 10. 请求示例

```bash
curl "http://127.0.0.1:4567/health"
curl "http://127.0.0.1:4567/api/spec/douyin"
curl "http://127.0.0.1:4567/api/douyin/search?query=openai&type=general&number=10"
curl "http://127.0.0.1:4567/api/twitter/search/tweets?query=OpenAI&search_type=latest&count=20"
curl "http://127.0.0.1:4567/api/xiaohongshu/search?keyword=%E6%8A%80%E6%9C%AF&page=1&page_size=20&sort=general&note_type=all"
```

`POST` 示例：

```bash
curl -X POST "http://127.0.0.1:4567/api/bilibili/captcha" \
  -H "Content-Type: application/json" \
  -d '{"v_voucher":"...","csrf":"..."}'
```
