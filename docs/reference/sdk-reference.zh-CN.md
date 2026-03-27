# Amagi SDK 使用参考

英文版：[`sdk-reference.md`](sdk-reference.md)

本文档说明 `client` feature 暴露出的稳定 Rust 客户端使用面，重点覆盖 SDK
使用方会直接调用的接口：

- 客户端初始化与环境加载
- 共享请求配置
- 平台级访问器
- 各平台 Rust 原生 fetcher
- 适合嵌入场景的 bound fetcher 构造器

本文档不重复展开每个响应模型的全部字段，也不逐个枚举底层签名辅助函数。
这些内容仍然保留在 rustdoc 中；本文档聚焦“如何使用 SDK 能力面”。

## 1. 启用 SDK

如果你只想把本项目作为 Rust SDK 依赖，而不引入 CLI 或 Axum Web 服务：

```toml
[dependencies]
amagi = { version = "0.1.0", default-features = false, features = ["client"] }
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

`client` feature 会自动启用静态 `catalog` 能力。

## 2. 环境与客户端初始化

`ClientOptions::from_env()` 与 `AmagiClient::from_env()` 会通过 crate 内部的
dotenv 支持读取分层配置。进程环境变量仍然优先于 dotenv。

默认 dotenv 查找顺序：

1. 用户级配置文件
2. 当前工作目录下的 `.env`

用户级配置 dotenv 路径：

- Linux/macOS：`~/.config/amagi/.env`
- Windows：`%APPDATA%\\amagi\\.env`

覆盖变量：

- `AMAGI_USER_ENV_FILE`

SDK 当前支持的环境变量：

- `AMAGI_DOUYIN_COOKIE`
- `AMAGI_BILIBILI_COOKIE`
- `AMAGI_KUAISHOU_COOKIE`
- `AMAGI_TWITTER_COOKIE`
- `AMAGI_XIAOHONGSHU_COOKIE`
- `AMAGI_TIMEOUT_MS`
- `AMAGI_MAX_RETRIES`
- `AMAGI_USER_ENV_FILE`

示例：

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

如果你想读取指定路径下的 dotenv 文件，而不是当前工作目录中的 `.env`，
使用 `ClientOptions::from_env_path(path)`。

## 3. 核心 SDK 接口

### 3.1 共享类型

| 项 | 类型 | 作用 |
| --- | --- | --- |
| `ClientOptions` | struct | SDK 顶层配置，包含 Cookie 与共享请求设置 |
| `CookieConfig` | struct | 各平台 Cookie 存储 |
| `RequestConfig` | struct | 超时、重试和自定义请求头覆盖 |
| `RequestProfile` | struct | 平台默认值与用户覆盖值合并后的最终请求配置 |
| `AmagiClient` | struct | SDK 主入口 |
| `PlatformClient` | struct | 平台级轻量客户端视图 |
| `create_amagi_client` | function | 等价于 `AmagiClient::new` 的便捷构造函数 |

### 3.2 `ClientOptions`、`CookieConfig` 与 `RequestConfig`

| 接口 | 说明 |
| --- | --- |
| `ClientOptions::from_env()` | 从分层 dotenv 和进程环境变量构建配置 |
| `ClientOptions::from_env_path(path)` | 从指定 dotenv 文件构建配置 |
| `CookieConfig::for_platform(platform)` | 获取某个平台当前配置的 Cookie |
| `RequestConfig::with_timeout_ms(timeout_ms)` | 覆盖上游请求超时，单位毫秒 |
| `RequestConfig::with_max_retries(max_retries)` | 覆盖可恢复失败的最大重试次数 |
| `RequestConfig::with_header(name, value)` | 注入或替换请求头 |

典型的显式初始化方式：

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

| 接口 | 说明 |
| --- | --- |
| `AmagiClient::new(options)` | 用显式配置创建客户端 |
| `AmagiClient::from_env()` | 从分层 dotenv 和进程环境创建客户端 |
| `AmagiClient::events()` | 获取共享事件总线 |
| `AmagiClient::options()` | 查看当前 `ClientOptions` |
| `AmagiClient::platform(platform)` | 为任意已支持平台构建 `PlatformClient` |
| `AmagiClient::catalog()` | 获取全部平台的静态 API catalog |
| `AmagiClient::bilibili()` / `douyin()` / `kuaishou()` / `twitter()` / `xiaohongshu()` | 平台级 `PlatformClient` 快捷方法 |
| `AmagiClient::bilibili_fetcher()` / `douyin_fetcher()` / `kuaishou_fetcher()` / `twitter_fetcher()` / `xiaohongshu_fetcher()` | 平台级 fetcher 快捷方法 |
| `create_amagi_client(options)` | `AmagiClient::new(options)` 的自由函数版本 |

### 3.4 `PlatformClient`

`PlatformClient` 适合在不立即构造 fetcher 的情况下，先查看平台元数据与最终请求配置。

| 接口 | 说明 |
| --- | --- |
| `PlatformClient::has_cookie()` | 返回当前平台是否绑定了非空 Cookie |
| `PlatformClient::api_base_path()` | 返回稳定的 Web 基础路径，例如 `/api/douyin` |
| `PlatformClient::spec()` | 返回当前平台完整的 `PlatformSpec` |
| `PlatformClient::methods()` | 返回当前平台发布的全部 `ApiMethodSpec` |
| `PlatformClient::request_profile()` | 返回最终生效的超时、重试、方法与请求头配置 |

示例：

```rust
use amagi::{AmagiClient, Platform};

let client = AmagiClient::from_env()?;
let douyin = client.platform(Platform::Douyin);

assert_eq!(douyin.api_base_path(), "/api/douyin");
assert!(!douyin.methods().is_empty());
```

## 4. Bound Fetcher 构造器

每个平台模块都导出 `Bound*Fetcher` 别名与
`create_bound_*_fetcher(cookie, request)` 构造函数。
这适合“不想保留完整 `AmagiClient`” 的嵌入式场景。

| 平台 | Bound 类型 | 构造函数 |
| --- | --- | --- |
| Bilibili | `BoundBilibiliFetcher` | `create_bound_bilibili_fetcher(cookie, request)` |
| Douyin | `BoundDouyinFetcher` | `create_bound_douyin_fetcher(cookie, request)` |
| Kuaishou | `BoundKuaishouFetcher` | `create_bound_kuaishou_fetcher(cookie, request)` |
| Twitter/X | `BoundTwitterFetcher` | `create_bound_twitter_fetcher(cookie, request)` |
| 小红书 | `BoundXiaohongshuFetcher` | `create_bound_xiaohongshu_fetcher(cookie, request)` |

示例：

```rust
use amagi::bilibili::create_bound_bilibili_fetcher;

let fetcher = create_bound_bilibili_fetcher(
    std::env::var("AMAGI_BILIBILI_COOKIE")?,
    None,
);
```

## 5. Fetcher 统一约定

所有平台 fetcher 方法都满足以下规则：

- 都是 async 方法
- 返回类型都是 `Result<T, amagi::AppError>`
- 会复用 `AmagiClient` 或 `create_bound_*_fetcher` 绑定进去的 Cookie 与请求配置
- 与平台能力面一一对应

每个平台 fetcher 还统一暴露：

- `new(client: PlatformClient)`
- `from_cookie(cookie, request)`
- `request_profile()`

## 6. Bilibili SDK 接口

模块路径：`amagi::bilibili`

主 fetcher 类型：`BilibiliFetcher`

| 方法 | 参数 | 说明 |
| --- | --- | --- |
| `fetch_video_info` | `bvid: &str` | 获取单个视频详情 |
| `fetch_video_stream` | `aid: u64, cid: u64` | 获取视频播放流地址 |
| `fetch_video_danmaku` | `cid: u64, segment_index: Option<u32>` | 获取单个弹幕分段 |
| `fetch_comments` | `oid: u64, comment_type: u32, number: Option<u32>, mode: Option<u32>` | 获取评论列表 |
| `fetch_comment_replies` | `oid: u64, comment_type: u32, root: u64, number: Option<u32>` | 获取某条根评论的回复 |
| `fetch_user_card` | `host_mid: u64` | 获取用户卡片 |
| `fetch_user_dynamic_list` | `host_mid: u64` | 获取用户动态列表 |
| `fetch_user_space_info` | `host_mid: u64` | 获取用户空间详情 |
| `fetch_uploader_total_views` | `host_mid: u64` | 获取 UP 主总播放量 |
| `fetch_dynamic_detail` | `dynamic_id: &str` | 获取动态详情 |
| `fetch_dynamic_card` | `dynamic_id: &str` | 获取动态卡片 |
| `fetch_bangumi_info` | `bangumi_id: &str` | 获取番剧元数据 |
| `fetch_bangumi_stream` | `ep_id: &str, cid: u64` | 获取番剧播放流地址 |
| `fetch_live_room_info` | `room_id: u64` | 获取直播间详情 |
| `fetch_live_room_init` | `room_id: u64` | 获取直播间初始化数据 |
| `fetch_article_content` | `article_id: &str` | 获取专栏正文 |
| `fetch_article_cards` | `ids: impl IntoIterator<Item = impl AsRef<str>>` | 批量获取专栏卡片 |
| `fetch_article_info` | `article_id: &str` | 获取专栏元数据 |
| `fetch_article_list_info` | `list_id: &str` | 获取文集元数据 |
| `fetch_login_status` | 无 | 获取当前登录状态 |
| `request_login_qrcode` | 无 | 申请登录二维码 |
| `check_qrcode_status` | `qrcode_key: &str` | 轮询二维码状态 |
| `fetch_emoji_list` | 无 | 获取表情列表 |
| `request_captcha_from_voucher` | `v_voucher: &str, csrf: Option<&str>` | 根据 voucher 申请验证码挑战 |
| `validate_captcha_result` | `challenge: &str, token: &str, validate: &str, seccode: &str, csrf: Option<&str>` | 校验验证码结果 |
| `convert_av_to_bv` | `aid: u64` | 本地执行 AV 转 BV |
| `convert_bv_to_av` | `bvid: &str` | 本地执行 BV 转 AV |

## 7. 抖音 SDK 接口

模块路径：`amagi::douyin`

主 fetcher 类型：`DouyinFetcher`

| 方法 | 参数 | 说明 |
| --- | --- | --- |
| `parse_work` | `aweme_id: &str` | 聚合解析作品并推断内容类型 |
| `fetch_video_work` | `aweme_id: &str` | 获取视频作品 |
| `fetch_image_album_work` | `aweme_id: &str` | 获取图文作品 |
| `fetch_slides_work` | `aweme_id: &str` | 获取图集作品 |
| `fetch_text_work` | `aweme_id: &str` | 获取文字作品 |
| `fetch_work_comments` | `aweme_id: &str, number: Option<u32>, cursor: Option<u64>` | 获取作品评论 |
| `fetch_comment_replies` | `aweme_id: &str, comment_id: &str, number: Option<u32>, cursor: Option<u64>` | 获取评论回复 |
| `fetch_user_profile` | `sec_uid: &str` | 获取用户资料 |
| `fetch_user_video_list` | `sec_uid: &str, number: Option<u32>, max_cursor: Option<&str>` | 获取用户视频列表 |
| `fetch_user_favorite_list` | `sec_uid: &str, number: Option<u32>, max_cursor: Option<&str>` | 获取用户收藏列表 |
| `fetch_user_recommend_list` | `sec_uid: &str, number: Option<u32>, max_cursor: Option<&str>` | 获取用户推荐列表 |
| `fetch_music_info` | `music_id: &str` | 获取音乐信息 |
| `fetch_live_room_info` | `room_id: &str, web_rid: &str` | 获取直播间信息 |
| `request_login_qrcode` | `verify_fp: Option<&str>` | 申请登录二维码 |
| `fetch_emoji_list` | 无 | 获取表情列表 |
| `fetch_dynamic_emoji_list` | 无 | 获取动态表情列表 |
| `fetch_danmaku_list` | `aweme_id: &str, duration: u64, start_time: Option<u64>, end_time: Option<u64>` | 获取弹幕数据 |
| `search_content` | `query: &str, search_type: Option<DouyinSearchType>, number: Option<u32>, search_id: Option<&str>` | 搜索内容 |
| `fetch_suggest_words` | `query: &str` | 获取搜索联想词 |

`DouyinSearchType` 支持的值：

- `general`
- `user`
- `video`

## 8. 快手 SDK 接口

模块路径：`amagi::kuaishou`

主 fetcher 类型：`KuaishouFetcher`

| 方法 | 参数 | 说明 |
| --- | --- | --- |
| `fetch_video_work` | `photo_id: &str` | 获取单个作品 |
| `fetch_work_comments` | `photo_id: &str` | 获取作品评论 |
| `fetch_user_profile` | `principal_id: &str` | 获取用户主页 |
| `fetch_user_work_list` | `principal_id: &str, count: Option<u32>, pcursor: Option<&str>` | 获取用户作品列表 |
| `fetch_live_room_info` | `principal_id: &str` | 获取直播间信息 |
| `fetch_emoji_list` | 无 | 获取表情列表 |

## 9. Twitter / X SDK 接口

模块路径：`amagi::twitter`

主 fetcher 类型：`TwitterFetcher`

| 方法 | 参数 | 说明 |
| --- | --- | --- |
| `search_tweets` | `query: &str, search_type: Option<TwitterTweetSearchMode>, count: Option<u32>, cursor: Option<&str>` | 搜索推文 |
| `fetch_tweet_detail` | `tweet_id: &str` | 获取单条推文详情 |
| `fetch_user_profile` | `screen_name: &str` | 获取用户资料 |
| `fetch_user_timeline` | `screen_name: &str, count: Option<u32>, cursor: Option<&str>` | 获取用户时间线 |
| `fetch_user_replies` | `screen_name: &str, count: Option<u32>, cursor: Option<&str>` | 获取用户回复流 |
| `fetch_user_media` | `screen_name: &str, count: Option<u32>, cursor: Option<&str>` | 获取用户媒体流 |
| `fetch_user_followers` | `screen_name: &str, count: Option<u32>, cursor: Option<&str>` | 获取用户粉丝列表 |
| `fetch_user_following` | `screen_name: &str, count: Option<u32>, cursor: Option<&str>` | 获取用户关注列表 |
| `fetch_space_detail` | `space_id: &str` | 获取 Space 详情 |

`TwitterTweetSearchMode` 支持的值：

- `latest`
- `top`

## 10. 小红书 SDK 接口

模块路径：`amagi::xiaohongshu`

主 fetcher 类型：`XiaohongshuFetcher`

与其他平台不同，小红书 fetcher 对大部分请求使用显式 options 结构体。

### 10.1 请求选项类型

| 类型 | 字段 |
| --- | --- |
| `XiaohongshuHomeFeedOptions` | `cursor_score`, `num`, `refresh_type`, `note_index`, `category`, `search_key` |
| `XiaohongshuNoteDetailOptions` | `note_id`, `xsec_token` |
| `XiaohongshuCommentsOptions` | `note_id`, `cursor`, `xsec_token` |
| `XiaohongshuUserProfileOptions` | `user_id` |
| `XiaohongshuUserNotesOptions` | `user_id`, `cursor`, `num` |
| `XiaohongshuSearchNotesOptions` | `keyword`, `page`, `page_size`, `sort`, `note_type` |

### 10.2 Fetcher 方法

| 方法 | 参数 | 说明 |
| --- | --- | --- |
| `fetch_home_feed` | `&XiaohongshuHomeFeedOptions` | 获取首页流 |
| `fetch_note_detail` | `&XiaohongshuNoteDetailOptions` | 获取单个笔记 |
| `fetch_note_comments` | `&XiaohongshuCommentsOptions` | 获取笔记评论 |
| `fetch_user_profile` | `&XiaohongshuUserProfileOptions` | 获取用户资料 |
| `fetch_user_note_list` | `&XiaohongshuUserNotesOptions` | 获取用户笔记列表 |
| `search_notes` | `&XiaohongshuSearchNotesOptions` | 搜索笔记 |
| `fetch_emoji_list` | 无 | 获取表情列表 |

`XiaohongshuSearchSortType` 支持的值：

- `general`
- `time_descending`
- `popularity_descending`

`XiaohongshuSearchNoteType` 支持的值：

- `all`
- `video`
- `image`

## 11. 如何选择不同 SDK 使用面

以下场景更适合使用 `AmagiClient`：

- 需要一个统一的共享配置对象
- 需要访问 API catalog 与事件总线
- 单进程内会调用多个平台

以下场景更适合使用 `create_bound_*_fetcher`：

- 只需要单个平台
- Cookie 已经由外层系统完成解析
- 你希望嵌入面尽量轻量

以下场景更适合直接使用静态 `catalog`：

- 只需要方法查找和路由元数据
- 需要生成文档、UI 或适配层，而不发起真实网络请求
