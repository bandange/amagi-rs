# Amagi SDK Usage Reference

Chinese version: [sdk-reference.zh-CN.md](sdk-reference.zh-CN.md)

This document describes the stable Rust client usage surface exposed by the `client`
feature. It focuses on the interfaces that SDK consumers call directly:

- Client bootstrap and environment loading
- Shared request configuration
- Platform-scoped accessors
- Rust-native fetchers for every migrated platform
- Bound fetcher constructors for embedding the SDK without a full client

It does not repeat every response-model field or every low-level signing helper.
Those remain available in rustdoc, while this document stays focused on the
operational SDK surface.

## 1. Enable The SDK

Use the crate as a Rust SDK without pulling in the CLI or Axum web server:

```toml
[dependencies]
amagi = { version = "0.1.2", default-features = false, features = ["client"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The `client` feature already enables the static `catalog` surface.

## 2. Environment And Client Bootstrap

`ClientOptions::from_env()` and `AmagiClient::from_env()` load layered dotenv
configuration through the crate's dotenv support. Process environment variables
still override dotenv values.

Default dotenv lookup order:

1. user config file
2. current working directory `.env`

User config dotenv path:

- Linux/macOS: `~/.config/amagi/.env`
- Windows: `%APPDATA%\\amagi\\.env`

Override variable:

- `AMAGI_USER_ENV_FILE`

Supported SDK environment variables:

- `AMAGI_DOUYIN_COOKIE`
- `AMAGI_BILIBILI_COOKIE`
- `AMAGI_KUAISHOU_COOKIE`
- `AMAGI_TWITTER_COOKIE`
- `AMAGI_XIAOHONGSHU_COOKIE`
- `AMAGI_TIMEOUT_MS`
- `AMAGI_MAX_RETRIES`
- `AMAGI_USER_ENV_FILE`

Example:

```rust
use amagi::{AmagiClient, ClientOptions};

#[tokio::main]
async fn main() -> Result<(), amagi::AppError> {
    let options = ClientOptions::from_env()?;
    let client = AmagiClient::new(options);

    let work = client
        .douyin_fetcher()
        .fetch_video_work("7454762925746679055")
        .await?;

    println!("{work:#?}");
    Ok(())
}
```

If you want to load a specific dotenv file instead of the process working
directory, use `ClientOptions::from_env_path(path)`.

## 3. Core SDK Interfaces

### 3.1 Shared Types

| Item | Kind | Purpose |
| --- | --- | --- |
| `ClientOptions` | struct | Top-level SDK configuration: cookies plus shared request settings |
| `CookieConfig` | struct | Per-platform cookie storage |
| `RequestConfig` | struct | Timeout, retry, and custom-header overrides |
| `RequestProfile` | struct | Resolved per-platform request settings after defaults and overrides are merged |
| `AmagiClient` | struct | Main SDK entrypoint |
| `PlatformClient` | struct | Lightweight platform-scoped client view |
| `create_amagi_client` | function | Convenience constructor equivalent to `AmagiClient::new` |

### 3.2 `ClientOptions`, `CookieConfig`, And `RequestConfig`

| Interface | Description |
| --- | --- |
| `ClientOptions::from_env()` | Build options from layered dotenv files plus process environment |
| `ClientOptions::from_env_path(path)` | Build options from a specific dotenv file |
| `CookieConfig::for_platform(platform)` | Return the configured cookie for one platform |
| `RequestConfig::with_timeout_ms(timeout_ms)` | Override upstream timeout in milliseconds |
| `RequestConfig::with_max_retries(max_retries)` | Override retry budget for recoverable failures |
| `RequestConfig::with_header(name, value)` | Inject or replace a request header |

Typical explicit bootstrap:

```rust
use amagi::{AmagiClient, ClientOptions, CookieConfig, RequestConfig};

let client = AmagiClient::new(ClientOptions {
    cookies: CookieConfig {
        douyin: std::env::var("AMAGI_DOUYIN_COOKIE").ok(),
        bilibili: None,
        kuaishou: None,
        twitter: None,
        xiaohongshu: None,
    },
    request: RequestConfig::default()
        .with_timeout_ms(15_000)
        .with_max_retries(5)
        .with_header("x-trace-id", "demo"),
});
```

### 3.3 `AmagiClient`

| Interface | Description |
| --- | --- |
| `AmagiClient::new(options)` | Create a client from explicit options |
| `AmagiClient::from_env()` | Create a client from layered dotenv files and process environment |
| `AmagiClient::events()` | Return the shared event bus |
| `AmagiClient::options()` | Inspect the current `ClientOptions` |
| `AmagiClient::platform(platform)` | Build a `PlatformClient` for any supported platform |
| `AmagiClient::catalog()` | Return the complete static API catalog for all platforms |
| `AmagiClient::bilibili()` / `douyin()` / `kuaishou()` / `twitter()` / `xiaohongshu()` | Platform-specific `PlatformClient` shortcuts |
| `AmagiClient::bilibili_fetcher()` / `douyin_fetcher()` / `kuaishou_fetcher()` / `twitter_fetcher()` / `xiaohongshu_fetcher()` | Platform-specific fetcher shortcuts |
| `create_amagi_client(options)` | Free function alias for `AmagiClient::new(options)` |

### 3.4 `PlatformClient`

`PlatformClient` is useful when you want catalog metadata and the resolved
request profile without constructing a fetcher immediately.

| Interface | Description |
| --- | --- |
| `PlatformClient::has_cookie()` | Report whether a non-empty cookie is bound |
| `PlatformClient::api_base_path()` | Return the stable web base path, such as `/api/douyin` |
| `PlatformClient::spec()` | Return the full `PlatformSpec` catalog for the platform |
| `PlatformClient::methods()` | Return the published `ApiMethodSpec` list for the platform |
| `PlatformClient::request_profile()` | Return the resolved timeout, retry, method, and headers |

Example:

```rust
use amagi::{AmagiClient, Platform};

let client = AmagiClient::from_env()?;
let douyin = client.platform(Platform::Douyin);

assert_eq!(douyin.api_base_path(), "/api/douyin");
assert!(!douyin.methods().is_empty());
```

## 4. Bound Fetcher Constructors

Every platform module exports a `Bound*Fetcher` alias plus a
`create_bound_*_fetcher(cookie, request)` constructor. These are useful when
your application does not want to keep a full `AmagiClient`.

| Platform | Bound Type | Constructor |
| --- | --- | --- |
| Bilibili | `BoundBilibiliFetcher` | `create_bound_bilibili_fetcher(cookie, request)` |
| Douyin | `BoundDouyinFetcher` | `create_bound_douyin_fetcher(cookie, request)` |
| Kuaishou | `BoundKuaishouFetcher` | `create_bound_kuaishou_fetcher(cookie, request)` |
| Twitter/X | `BoundTwitterFetcher` | `create_bound_twitter_fetcher(cookie, request)` |
| Xiaohongshu | `BoundXiaohongshuFetcher` | `create_bound_xiaohongshu_fetcher(cookie, request)` |

Example:

```rust
use amagi::bilibili::create_bound_bilibili_fetcher;

let fetcher = create_bound_bilibili_fetcher(
    std::env::var("AMAGI_BILIBILI_COOKIE")?,
    None,
);
```

## 5. Fetcher Conventions

All platform fetcher methods:

- are async
- return `Result<T, amagi::AppError>`
- reuse the cookie and request profile bound through `AmagiClient` or
  `create_bound_*_fetcher`
- map one-to-one to the migrated platform capability surface

Each platform fetcher also exposes:

- `new(client: PlatformClient)`
- `from_cookie(cookie, request)`
- `request_profile()`

## 6. Bilibili SDK Interfaces

Module path: `amagi::bilibili`

Main fetcher type: `BilibiliFetcher`

| Method | Parameters | Description |
| --- | --- | --- |
| `fetch_video_info` | `bvid: &str` | Fetch a Bilibili video payload |
| `fetch_video_stream` | `aid: u64, cid: u64` | Fetch stream URLs for a Bilibili video |
| `fetch_video_danmaku` | `cid: u64, segment_index: Option<u32>` | Fetch one danmaku segment |
| `fetch_comments` | `oid: u64, comment_type: u32, number: Option<u32>, mode: Option<u32>` | Fetch comments for a subject |
| `fetch_comment_replies` | `oid: u64, comment_type: u32, root: u64, number: Option<u32>` | Fetch replies for one root comment |
| `fetch_user_card` | `host_mid: u64` | Fetch a user card |
| `fetch_user_dynamic_list` | `host_mid: u64` | Fetch a user's dynamic list |
| `fetch_user_space_info` | `host_mid: u64` | Fetch a user's space details |
| `fetch_uploader_total_views` | `host_mid: u64` | Fetch uploader total-play metrics |
| `fetch_dynamic_detail` | `dynamic_id: &str` | Fetch dynamic details |
| `fetch_dynamic_card` | `dynamic_id: &str` | Fetch a dynamic card |
| `fetch_bangumi_info` | `bangumi_id: &str` | Fetch bangumi metadata |
| `fetch_bangumi_stream` | `ep_id: &str, cid: u64` | Fetch bangumi stream URLs |
| `fetch_live_room_info` | `room_id: u64` | Fetch live room details |
| `fetch_live_room_init` | `room_id: u64` | Fetch live room init data |
| `fetch_article_content` | `article_id: &str` | Fetch article body content |
| `fetch_article_cards` | `ids: impl IntoIterator<Item = impl AsRef<str>>` | Fetch article cards for one or more ids |
| `fetch_article_info` | `article_id: &str` | Fetch article metadata |
| `fetch_article_list_info` | `list_id: &str` | Fetch article-list metadata |
| `fetch_login_status` | none | Fetch current login status |
| `request_login_qrcode` | none | Request a login QR code |
| `check_qrcode_status` | `qrcode_key: &str` | Poll QR code status |
| `fetch_emoji_list` | none | Fetch the emoji catalog |
| `request_captcha_from_voucher` | `v_voucher: &str, csrf: Option<&str>` | Request a captcha challenge |
| `validate_captcha_result` | `challenge: &str, token: &str, validate: &str, seccode: &str, csrf: Option<&str>` | Validate a captcha result |
| `convert_av_to_bv` | `aid: u64` | Convert AV to BV locally |
| `convert_bv_to_av` | `bvid: &str` | Convert BV to AV locally |

## 7. Douyin SDK Interfaces

Module path: `amagi::douyin`

Main fetcher type: `DouyinFetcher`

| Method | Parameters | Description |
| --- | --- | --- |
| `parse_work` | `aweme_id: &str` | Parse a work and infer its content type |
| `fetch_video_work` | `aweme_id: &str` | Fetch a video work |
| `fetch_image_album_work` | `aweme_id: &str` | Fetch an image-album work |
| `fetch_slides_work` | `aweme_id: &str` | Fetch a slides work |
| `fetch_text_work` | `aweme_id: &str` | Fetch a text work |
| `fetch_work_comments` | `aweme_id: &str, number: Option<u32>, cursor: Option<u64>` | Fetch comments for one work |
| `fetch_comment_replies` | `aweme_id: &str, comment_id: &str, number: Option<u32>, cursor: Option<u64>` | Fetch replies for one comment |
| `fetch_user_profile` | `sec_uid: &str` | Fetch a user profile |
| `fetch_user_video_list` | `sec_uid: &str, number: Option<u32>, max_cursor: Option<&str>` | Fetch published videos for a user |
| `fetch_user_favorite_list` | `sec_uid: &str, number: Option<u32>, max_cursor: Option<&str>` | Fetch favorite works for a user |
| `fetch_user_recommend_list` | `sec_uid: &str, number: Option<u32>, max_cursor: Option<&str>` | Fetch recommended works for a user |
| `fetch_music_info` | `music_id: &str` | Fetch music metadata |
| `fetch_live_room_info` | `room_id: &str, web_rid: &str` | Fetch live room information |
| `request_login_qrcode` | `verify_fp: Option<&str>` | Request a login QR code |
| `fetch_emoji_list` | none | Fetch the emoji catalog |
| `fetch_dynamic_emoji_list` | none | Fetch animated emoji data |
| `fetch_danmaku_list` | `aweme_id: &str, duration: u64, start_time: Option<u64>, end_time: Option<u64>` | Fetch danmaku entries |
| `search_content` | `query: &str, search_type: Option<DouyinSearchType>, number: Option<u32>, search_id: Option<&str>` | Search content |
| `fetch_suggest_words` | `query: &str` | Fetch search suggestions |

Supported `DouyinSearchType` values:

- `general`
- `user`
- `video`

## 8. Kuaishou SDK Interfaces

Module path: `amagi::kuaishou`

Main fetcher type: `KuaishouFetcher`

| Method | Parameters | Description |
| --- | --- | --- |
| `fetch_video_work` | `photo_id: &str` | Fetch a video work |
| `fetch_work_comments` | `photo_id: &str` | Fetch work comments |
| `fetch_user_profile` | `principal_id: &str` | Fetch a user profile |
| `fetch_user_work_list` | `principal_id: &str, count: Option<u32>, pcursor: Option<&str>` | Fetch a user's work list |
| `fetch_live_room_info` | `principal_id: &str` | Fetch live room information |
| `fetch_emoji_list` | none | Fetch the emoji catalog |

## 9. Twitter / X SDK Interfaces

Module path: `amagi::twitter`

Main fetcher type: `TwitterFetcher`

| Method | Parameters | Description |
| --- | --- | --- |
| `search_tweets` | `query: &str, search_type: Option<TwitterTweetSearchMode>, count: Option<u32>, cursor: Option<&str>` | Search tweets |
| `fetch_tweet_detail` | `tweet_id: &str` | Fetch a single tweet |
| `fetch_user_profile` | `screen_name: &str` | Fetch one user profile |
| `fetch_user_timeline` | `screen_name: &str, count: Option<u32>, cursor: Option<&str>` | Fetch a user's timeline |
| `fetch_user_replies` | `screen_name: &str, count: Option<u32>, cursor: Option<&str>` | Fetch a user's replies timeline |
| `fetch_user_media` | `screen_name: &str, count: Option<u32>, cursor: Option<&str>` | Fetch a user's media timeline |
| `fetch_user_followers` | `screen_name: &str, count: Option<u32>, cursor: Option<&str>` | Fetch a user's followers |
| `fetch_user_following` | `screen_name: &str, count: Option<u32>, cursor: Option<&str>` | Fetch a user's following list |
| `fetch_space_detail` | `space_id: &str` | Fetch a Space detail payload |

Supported `TwitterTweetSearchMode` values:

- `latest`
- `top`

## 10. Xiaohongshu SDK Interfaces

Module path: `amagi::xiaohongshu`

Main fetcher type: `XiaohongshuFetcher`

Unlike the other platforms, the Xiaohongshu fetcher uses explicit option
structs for most requests.

### 10.1 Request Option Types

| Type | Fields |
| --- | --- |
| `XiaohongshuHomeFeedOptions` | `cursor_score`, `num`, `refresh_type`, `note_index`, `category`, `search_key` |
| `XiaohongshuNoteDetailOptions` | `note_id`, `xsec_token` |
| `XiaohongshuCommentsOptions` | `note_id`, `cursor`, `xsec_token` |
| `XiaohongshuUserProfileOptions` | `user_id` |
| `XiaohongshuUserNotesOptions` | `user_id`, `cursor`, `num` |
| `XiaohongshuSearchNotesOptions` | `keyword`, `page`, `page_size`, `sort`, `note_type` |

### 10.2 Fetcher Methods

| Method | Parameters | Description |
| --- | --- | --- |
| `fetch_home_feed` | `&XiaohongshuHomeFeedOptions` | Fetch the home feed |
| `fetch_note_detail` | `&XiaohongshuNoteDetailOptions` | Fetch one note |
| `fetch_note_comments` | `&XiaohongshuCommentsOptions` | Fetch note comments |
| `fetch_user_profile` | `&XiaohongshuUserProfileOptions` | Fetch a user profile |
| `fetch_user_note_list` | `&XiaohongshuUserNotesOptions` | Fetch notes for a user |
| `search_notes` | `&XiaohongshuSearchNotesOptions` | Search notes |
| `fetch_emoji_list` | none | Fetch the emoji catalog |

Supported `XiaohongshuSearchSortType` values:

- `general`
- `time_descending`
- `popularity_descending`

Supported `XiaohongshuSearchNoteType` values:

- `all`
- `video`
- `image`

## 11. Choosing Between SDK Surfaces

Use `AmagiClient` when:

- you want one shared configuration object
- you need access to the API catalog and event bus
- you use multiple platforms in one process

Use `create_bound_*_fetcher` when:

- you only need one platform
- you already resolved the cookie elsewhere
- you want a minimal embedding surface

Use the static `catalog` helpers when:

- you need method lookup and route metadata only
- you want to generate docs, UIs, or adapters without making network requests
