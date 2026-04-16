# Amagi Web API Reference

Chinese version: [web-api-reference.zh-CN.md](web-api-reference.zh-CN.md)

This document covers the built-in HTTP service exposed by the `server` feature:

- how to start the server
- metadata and health endpoints
- response and error conventions
- every published business route
- path, query, and body parameters for every route

## 1. Enable And Run The Web Surface

If you embed the web service in your own Rust application:

```toml
[dependencies]
amagi = { version = "0.1.2", default-features = false, features = ["server"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

If you want to use the bundled `amagi serve` binary, keep the default features
or enable `cli` and `server` together. The repository default already does this.

### 1.1 Start Through The Bundled Binary

```bash
cargo run -- serve --host 127.0.0.1 --port 4567
```

### 1.2 Start Programmatically

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

## 2. Runtime Environment

The web runtime reads the same shared environment variables as the SDK:

- `AMAGI_DOUYIN_COOKIE`
- `AMAGI_BILIBILI_COOKIE`
- `AMAGI_KUAISHOU_COOKIE`
- `AMAGI_TWITTER_COOKIE`
- `AMAGI_XIAOHONGSHU_COOKIE`
- `AMAGI_TIMEOUT_MS`
- `AMAGI_MAX_RETRIES`
- `AMAGI_HOST`
- `AMAGI_PORT`

`amagi serve` also loads layered dotenv configuration automatically during startup.

Default dotenv lookup order:

1. user config file
2. current working directory `.env`

User config dotenv path:

- Linux/macOS: `~/.config/amagi/.env`
- Windows: `%APPDATA%\\amagi\\.env`

Override variable:

- `AMAGI_USER_ENV_FILE`

### 2.1 Per-request Cookie Override Headers

Besides startup environment variables, `amagi serve` also accepts per-request
cookie overrides through HTTP headers:

- `X-Amagi-Cookie`
- `X-Amagi-Bilibili-Cookie`
- `X-Amagi-Douyin-Cookie`
- `X-Amagi-Kuaishou-Cookie`
- `X-Amagi-Twitter-Cookie`
- `X-Amagi-Xiaohongshu-Cookie`

Precedence is:

1. platform-specific header
2. `X-Amagi-Cookie`
3. startup environment / dotenv configuration

An empty override header clears the configured startup cookie for that request.
This is useful when the server has a default authenticated cookie but a specific
request should use guest-mode behavior instead.
These headers are intended for direct HTTP callers such as `curl`, automation
scripts, reverse proxies, and backend services.

## 3. Response Conventions

### 3.1 Success Responses

There are two success shapes:

- Metadata endpoints return typed wrapper payloads such as `RootResponse`,
  `HealthResponse`, `ApiCatalogResponse`, and `PlatformCatalogResponse`.
- Business endpoints return the raw platform payload serialized from the SDK
  fetcher result.

### 3.2 Error Responses

Catalog lookup errors:

```json
{
  "error": "unknown platform",
  "platform": "invalid-name"
}
```

Fetcher errors:

```json
{
  "error": "fetch_failed",
  "detail": "..."
}
```

Status-code policy:

- `500 Internal Server Error`: local IO or invalid request configuration
- `502 Bad Gateway`: upstream HTTP, upstream JSON, or upstream response errors
- `404 Not Found`: unknown platform in `/api/spec/{platform}`

### 3.3 Parameter Conventions

- Path parameters are shown as `{name}`.
- Optional query parameters are marked with `?`.
- POST bodies are JSON.
- All routes currently return JSON.

## 4. Metadata Endpoints

| Method | Path | Parameters | Response | Description |
| --- | --- | --- | --- | --- |
| `GET` | `/` | none | `RootResponse` | Service metadata, bind address, base URL, published endpoint list, and per-platform cookie status |
| `GET` | `/health` | none | `HealthResponse` | Liveness payload |
| `GET` | `/api/spec` | none | `ApiCatalogResponse` | Full static API catalog for all platforms |
| `GET` | `/api/spec/{platform}` | path: `platform` | `PlatformCatalogResponse` or `CatalogErrorResponse` | Static API catalog for one platform |

Valid `{platform}` values:

- `bilibili`
- `douyin`
- `kuaishou`
- `twitter`
- `xiaohongshu`

## 5. Bilibili Routes

Base path: `/api/bilibili`

| Method | Path | Path Parameters | Query / Body | Description |
| --- | --- | --- | --- | --- |
| `GET` | `/api/bilibili/video/{bvid}` | `bvid` | none | Fetch a Bilibili video payload |
| `GET` | `/api/bilibili/video/{aid}/stream` | `aid` | query: `cid` | Fetch Bilibili video stream URLs |
| `GET` | `/api/bilibili/video/{cid}/danmaku` | `cid` | query: `segment_index?` | Fetch one danmaku segment |
| `GET` | `/api/bilibili/bangumi/{bangumi_id}` | `bangumi_id` | none | Fetch bangumi metadata |
| `GET` | `/api/bilibili/bangumi/{ep_id}/stream` | `ep_id` | query: `cid` | Fetch bangumi stream URLs |
| `GET` | `/api/bilibili/article/{id}/content` | `id` | none | Fetch article body content |
| `GET` | `/api/bilibili/article/cards` | none | query: `ids` | Fetch article cards for comma-separated ids |
| `GET` | `/api/bilibili/article/{id}` | `id` | none | Fetch article metadata |
| `GET` | `/api/bilibili/article-list/{id}` | `id` | none | Fetch article-list metadata |
| `POST` | `/api/bilibili/captcha` | none | body: `v_voucher`, `csrf?` | Request a captcha challenge |
| `POST` | `/api/bilibili/captcha/validate` | none | body: `challenge`, `token`, `validate`, `seccode`, `csrf?` | Validate a captcha result |
| `GET` | `/api/bilibili/user/{host_mid}` | `host_mid` | none | Fetch a user card |
| `GET` | `/api/bilibili/user/{host_mid}/dynamics` | `host_mid` | none | Fetch a user's dynamic list |
| `GET` | `/api/bilibili/user/{host_mid}/space` | `host_mid` | none | Fetch user space details |
| `GET` | `/api/bilibili/user/{host_mid}/total-views` | `host_mid` | none | Fetch uploader total-play metrics |
| `GET` | `/api/bilibili/comments/{oid}` | `oid` | query: `type`, `number?`, `mode?` | Fetch comments for one subject |
| `GET` | `/api/bilibili/comment-replies/{oid}/{root}` | `oid`, `root` | query: `type`, `number?` | Fetch replies for one root comment |
| `GET` | `/api/bilibili/dynamic/{dynamic_id}` | `dynamic_id` | none | Fetch dynamic details |
| `GET` | `/api/bilibili/dynamic/{dynamic_id}/card` | `dynamic_id` | none | Fetch a dynamic card |
| `GET` | `/api/bilibili/live/{room_id}` | `room_id` | none | Fetch live room details |
| `GET` | `/api/bilibili/live/{room_id}/init` | `room_id` | none | Fetch live room init data |
| `GET` | `/api/bilibili/auth/status` | none | none | Fetch current login status |
| `GET` | `/api/bilibili/auth/qrcode` | none | none | Request a login QR code |
| `GET` | `/api/bilibili/auth/qrcode/status` | none | query: `qrcode_key` | Poll QR code status |
| `GET` | `/api/bilibili/emoji` | none | none | Fetch the emoji catalog |
| `GET` | `/api/bilibili/convert/av/{aid}` | `aid` | none | Convert AV to BV |
| `GET` | `/api/bilibili/convert/bv/{bvid}` | `bvid` | none | Convert BV to AV |

## 6. Douyin Routes

Base path: `/api/douyin`

| Method | Path | Path Parameters | Query / Body | Description |
| --- | --- | --- | --- | --- |
| `GET` | `/api/douyin/work/{aweme_id}` | `aweme_id` | none | Parse a work and infer its type |
| `GET` | `/api/douyin/work/{aweme_id}/video` | `aweme_id` | none | Fetch a video work |
| `GET` | `/api/douyin/work/{aweme_id}/image-album` | `aweme_id` | none | Fetch an image-album work |
| `GET` | `/api/douyin/work/{aweme_id}/slides` | `aweme_id` | none | Fetch a slides work |
| `GET` | `/api/douyin/work/{aweme_id}/text` | `aweme_id` | none | Fetch a text work |
| `GET` | `/api/douyin/comments/{aweme_id}` | `aweme_id` | query: `number?`, `cursor?` | Fetch work comments |
| `GET` | `/api/douyin/comment-replies/{aweme_id}/{comment_id}` | `aweme_id`, `comment_id` | query: `number?`, `cursor?` | Fetch replies for one comment |
| `GET` | `/api/douyin/user/{sec_uid}` | `sec_uid` | none | Fetch a user profile |
| `GET` | `/api/douyin/user/{sec_uid}/videos` | `sec_uid` | query: `number?`, `max_cursor?` | Fetch a user's video list |
| `GET` | `/api/douyin/user/{sec_uid}/favorites` | `sec_uid` | query: `number?`, `max_cursor?` | Fetch a user's favorite list |
| `GET` | `/api/douyin/user/{sec_uid}/recommends` | `sec_uid` | query: `number?`, `max_cursor?` | Fetch a user's recommendation list |
| `GET` | `/api/douyin/search` | none | query: `query`, `type?`, `number?`, `search_id?` | Search Douyin content |
| `GET` | `/api/douyin/search/suggest` | none | query: `query` | Fetch search suggestions |
| `GET` | `/api/douyin/emoji` | none | none | Fetch the emoji catalog |
| `GET` | `/api/douyin/emoji/dynamic` | none | none | Fetch animated emoji data |
| `GET` | `/api/douyin/music/{music_id}` | `music_id` | none | Fetch music metadata |
| `GET` | `/api/douyin/live/{room_id}` | `room_id` | query: `web_rid` | Fetch live room information |
| `GET` | `/api/douyin/auth/qrcode` | none | query: `verify_fp?` | Request a login QR code |
| `GET` | `/api/douyin/danmaku/{aweme_id}` | `aweme_id` | query: `duration`, `start_time?`, `end_time?` | Fetch danmaku data |

Supported query enums:

- `/api/douyin/search`: `type=general|user|video`

## 7. Kuaishou Routes

Base path: `/api/kuaishou`

| Method | Path | Path Parameters | Query / Body | Description |
| --- | --- | --- | --- | --- |
| `GET` | `/api/kuaishou/work/{photo_id}` | `photo_id` | none | Fetch a video work |
| `GET` | `/api/kuaishou/comments/{photo_id}` | `photo_id` | none | Fetch work comments |
| `GET` | `/api/kuaishou/emoji` | none | none | Fetch the emoji catalog |
| `GET` | `/api/kuaishou/user/{principal_id}` | `principal_id` | none | Fetch a user profile |
| `GET` | `/api/kuaishou/user/{principal_id}/works` | `principal_id` | query: `pcursor?`, `count?` | Fetch a user's work list |
| `GET` | `/api/kuaishou/live/{principal_id}` | `principal_id` | none | Fetch live room information |

## 8. Twitter / X Routes

Base path: `/api/twitter`

| Method | Path | Path Parameters | Query / Body | Description |
| --- | --- | --- | --- | --- |
| `GET` | `/api/twitter/user/{screen_name}` | `screen_name` | none | Fetch a user profile |
| `GET` | `/api/twitter/user/{screen_name}/timeline` | `screen_name` | query: `count?`, `cursor?` | Fetch a user's timeline |
| `GET` | `/api/twitter/user/{screen_name}/replies` | `screen_name` | query: `count?`, `cursor?` | Fetch a user's replies timeline |
| `GET` | `/api/twitter/user/{screen_name}/media` | `screen_name` | query: `count?`, `cursor?` | Fetch a user's media timeline |
| `GET` | `/api/twitter/user/{screen_name}/followers` | `screen_name` | query: `count?`, `cursor?` | Fetch a user's followers |
| `GET` | `/api/twitter/user/{screen_name}/following` | `screen_name` | query: `count?`, `cursor?` | Fetch a user's following list |
| `GET` | `/api/twitter/search/tweets` | none | query: `query`, `search_type?`, `count?`, `cursor?` | Search tweets |
| `GET` | `/api/twitter/tweet/{tweet_id}` | `tweet_id` | none | Fetch a tweet detail payload |
| `GET` | `/api/twitter/space/{space_id}` | `space_id` | none | Fetch a Space detail payload |

Supported query enums:

- `/api/twitter/search/tweets`: `search_type=latest|top`

## 9. Xiaohongshu Routes

Base path: `/api/xiaohongshu`

| Method | Path | Path Parameters | Query / Body | Description |
| --- | --- | --- | --- | --- |
| `GET` | `/api/xiaohongshu/feed` | none | query: `cursor_score?`, `num?`, `refresh_type?`, `note_index?`, `category?`, `search_key?` | Fetch the home feed |
| `GET` | `/api/xiaohongshu/note/{note_id}` | `note_id` | query: `xsec_token` | Fetch one note |
| `GET` | `/api/xiaohongshu/comments/{note_id}` | `note_id` | query: `xsec_token`, `cursor?` | Fetch note comments |
| `GET` | `/api/xiaohongshu/user/{user_id}` | `user_id` | none | Fetch a user profile |
| `GET` | `/api/xiaohongshu/user/{user_id}/notes` | `user_id` | query: `cursor?`, `num?` | Fetch a user's notes |
| `GET` | `/api/xiaohongshu/emoji` | none | none | Fetch the emoji catalog |
| `GET` | `/api/xiaohongshu/search` | none | query: `keyword`, `page?`, `page_size?`, `sort?`, `note_type?` | Search notes |

Supported query enums:

- `/api/xiaohongshu/search`: `sort=general|time_descending|popularity_descending`
- `/api/xiaohongshu/search`: `note_type=all|video|image`

## 10. Example Requests

```bash
curl "http://127.0.0.1:4567/health"
curl "http://127.0.0.1:4567/api/spec/douyin"
curl "http://127.0.0.1:4567/api/douyin/search?query=openai&type=general&number=10"
curl "http://127.0.0.1:4567/api/twitter/search/tweets?query=OpenAI&search_type=latest&count=20"
curl "http://127.0.0.1:4567/api/xiaohongshu/search?keyword=%E6%8A%80%E6%9C%AF&page=1&page_size=20&sort=general&note_type=all"
curl -H "X-Amagi-Twitter-Cookie: auth_token=...; ct0=...; twid=u%3D..." "http://127.0.0.1:4567/api/twitter/user/likes?count=20"
curl -H "X-Amagi-Cookie:" "http://127.0.0.1:4567/api/bilibili/live/21452505"
```

POST example:

```bash
curl -X POST "http://127.0.0.1:4567/api/bilibili/captcha" \
  -H "Content-Type: application/json" \
  -d '{"v_voucher":"...","csrf":"..."}'
```
