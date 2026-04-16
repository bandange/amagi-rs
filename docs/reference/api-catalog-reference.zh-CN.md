# Amagi API Catalog 参考

英文版：[`api-catalog-reference.md`](api-catalog-reference.md)

`catalog` feature 暴露的是 Amagi 的静态平台 catalog。这一层是纯元数据接口：

- 不发起任何上游 HTTP 请求
- 不需要 Cookie
- 不需要构造运行时 fetcher

适用场景包括：

- 查看已发布的方法清单
- 把方法键映射到路由
- 生成文档、控制台、适配层或代码生成管线

## 1. 启用 API Catalog

```toml
[dependencies]
amagi = { version = "0.1.2", default-features = false, features = ["catalog"] }
```

## 2. 快速开始

```rust
use amagi::{Platform, all_platform_specs, find_method, get_api_route, method_specs};

let douyin_methods = method_specs(Platform::Douyin);
assert!(!douyin_methods.is_empty());

let search = find_method(Platform::Douyin, "search").unwrap();
assert_eq!(search.fetcher_name, "searchContent");
assert_eq!(get_api_route(Platform::Douyin, "search"), Some("/search"));

let all = all_platform_specs();
assert_eq!(all.len(), 5);
```

## 3. 核心类型

| 类型 | 说明 |
| --- | --- |
| `HttpMethod` | catalog 条目声明的 HTTP 方法 |
| `Platform` | 已支持平台枚举：`bilibili`、`douyin`、`kuaishou`、`twitter`、`xiaohongshu` |
| `ParsePlatformError` | 解析未知平台字符串时返回的错误 |
| `ApiMethodSpec` | 单个已发布方法的元数据 |
| `PlatformSpec` | 单个平台的完整方法目录 |

### 3.1 `Platform`

`Platform` 常用接口：

| 接口 | 说明 |
| --- | --- |
| `Platform::ALL` | 显示顺序下的全部平台 |
| `Platform::as_str()` | 稳定的小写平台标识 |
| `Platform::api_base_path()` | 稳定基础路径，例如 `/api/douyin` |
| `"douyin".parse::<Platform>()` | 从小写字符串解析平台 |

### 3.2 `ApiMethodSpec`

每个 catalog 条目包含以下字段：

| 字段 | 含义 |
| --- | --- |
| `method_key` | 稳定方法键 |
| `chinese_name` | 从源目录保留的中文方法名 |
| `fetcher_name` | 规范英文 fetcher 方法名 |
| `route` | catalog 相对路由 |
| `http_method` | 已发布 HTTP 方法 |
| `description` | 简要说明 |
| `tags` | 分组标签，如 `user`、`work`、`emoji`、`search` |

注意：catalog 中的 `route` 是规范元数据路由，不一定总是 Web 服务最终暴露的
完整 Axum 路由模板。如果你需要精确 HTTP 路径，请看 Web API 参考文档。

## 4. Lookup Helper

| 接口 | 说明 |
| --- | --- |
| `all_platform_specs()` | 返回全部平台的完整目录 |
| `platform_spec(platform)` | 返回某个平台完整目录 |
| `method_specs(platform)` | 只返回某个平台的方法列表 |
| `find_method(platform, method_key)` | 按稳定方法键查找 `ApiMethodSpec` |
| `get_api_route(platform, method_key)` | 解析稳定方法键对应的 catalog 路由 |
| `get_english_method_name(platform, chinese_method)` | 把中文方法名映射到规范英文 fetcher 名 |
| `get_chinese_method_name(platform, english_method)` | 把规范英文 fetcher 名映射回中文方法名 |

## 5. 平台覆盖总览

| 平台 | 方法数 | 覆盖能力 |
| --- | --- | --- |
| `bilibili` | 27 | 视频、评论、用户、动态、番剧、直播、专栏、登录、工具、验证码、表情 |
| `douyin` | 19 | 作品、评论、用户、搜索、音乐、直播、登录、表情、弹幕 |
| `kuaishou` | 6 | 作品、评论、用户、直播、表情 |
| `twitter` | 9 | 搜索、用户关系、推文、Space |
| `xiaohongshu` | 7 | 首页流、笔记、评论、用户、搜索、表情 |

## 6. Bilibili Catalog

平台基础路径：`/api/bilibili`

| method_key | 中文名 | fetcher_name | HTTP | route | tags | 说明 |
| --- | --- | --- | --- | --- | --- | --- |
| `videoInfo` | `单个视频作品数据` | `fetchVideoInfo` | `GET` | `/video` | `work` | 获取单个 Bilibili 视频数据。 |
| `videoStream` | `单个视频下载信息数据` | `fetchVideoStreamUrl` | `GET` | `/video/stream` | `work` | 获取 Bilibili 视频播放流地址。 |
| `videoDanmaku` | `视频弹幕分段数据` | `fetchVideoDanmaku` | `GET` | `/video/{cid}/danmaku` | `danmaku` | 获取单个 Bilibili 视频弹幕分段。 |
| `comments` | `评论数据` | `fetchComments` | `GET` | `/comments` | `comment` | 获取 Bilibili 评论。 |
| `commentReplies` | `指定评论的回复` | `fetchCommentReplies` | `GET` | `/comment-replies` | `comment` | 获取指定 Bilibili 评论的回复。 |
| `userCard` | `用户主页数据` | `fetchUserCard` | `GET` | `/user` | `user` | 获取 Bilibili 用户卡片。 |
| `userDynamicList` | `用户主页动态列表数据` | `fetchUserDynamicList` | `GET` | `/user/dynamics` | `user` | 获取 Bilibili 用户动态列表。 |
| `userSpaceInfo` | `用户空间详细信息` | `fetchUserSpaceInfo` | `GET` | `/user/space` | `user` | 获取 Bilibili 用户空间详情。 |
| `uploaderTotalViews` | `获取UP主总播放量` | `fetchUploaderTotalViews` | `GET` | `/user/total-views` | `user` | 获取 Bilibili UP 主总播放量。 |
| `dynamicDetail` | `动态详情数据` | `fetchDynamicDetail` | `GET` | `/dynamic` | `dynamic` | 获取 Bilibili 动态详情。 |
| `dynamicCard` | `动态卡片数据` | `fetchDynamicCard` | `GET` | `/dynamic/card` | `dynamic` | 获取 Bilibili 动态卡片。 |
| `bangumiInfo` | `番剧基本信息数据` | `fetchBangumiInfo` | `GET` | `/bangumi` | `bangumi` | 获取 Bilibili 番剧元数据。 |
| `bangumiStream` | `番剧下载信息数据` | `fetchBangumiStreamUrl` | `GET` | `/bangumi/stream` | `bangumi` | 获取 Bilibili 番剧播放流地址。 |
| `liveRoomInfo` | `直播间信息` | `fetchLiveRoomInfo` | `GET` | `/live` | `live` | 获取 Bilibili 直播间详情。 |
| `liveRoomInit` | `直播间初始化信息` | `fetchLiveRoomInitInfo` | `GET` | `/live/init` | `live` | 获取 Bilibili 直播间初始化数据。 |
| `articleContent` | `专栏正文内容` | `fetchArticleContent` | `GET` | `/article/content` | `article` | 获取 Bilibili 专栏正文。 |
| `articleCards` | `专栏显示卡片信息` | `fetchArticleCards` | `GET` | `/article/cards` | `article` | 获取 Bilibili 专栏卡片。 |
| `articleInfo` | `专栏文章基本信息` | `fetchArticleInfo` | `GET` | `/article` | `article` | 获取 Bilibili 专栏元数据。 |
| `articleListInfo` | `文集基本信息` | `fetchArticleListInfo` | `GET` | `/article-list` | `article` | 获取 Bilibili 文集元数据。 |
| `loginStatus` | `登录基本信息` | `fetchLoginStatus` | `GET` | `/auth/status` | `auth` | 获取 Bilibili 登录状态。 |
| `loginQrcode` | `申请二维码` | `requestLoginQrcode` | `GET` | `/auth/qrcode` | `auth` | 申请 Bilibili 登录二维码。 |
| `qrcodeStatus` | `二维码状态` | `checkQrcodeStatus` | `GET` | `/auth/qrcode/status` | `auth` | 查询 Bilibili 登录二维码状态。 |
| `avToBv` | `AV转BV` | `convertAvToBv` | `GET` | `/convert/av-to-bv` | `tool` | 将 Bilibili AV 号转换为 BV 号。 |
| `bvToAv` | `BV转AV` | `convertBvToAv` | `GET` | `/convert/bv-to-av` | `tool` | 将 Bilibili BV 号转换为 AV 号。 |
| `emojiList` | `Emoji数据` | `fetchEmojiList` | `GET` | `/emoji` | `emoji` | 获取 Bilibili 表情目录。 |
| `captchaFromVoucher` | `从_v_voucher_申请_captcha` | `requestCaptchaFromVoucher` | `POST` | `/captcha` | `captcha` | 根据 voucher 申请 Bilibili 验证码挑战。 |
| `validateCaptcha` | `验证验证码结果` | `validateCaptchaResult` | `POST` | `/captcha/validate` | `captcha` | 校验 Bilibili 验证码结果。 |

## 7. Douyin Catalog

平台基础路径：`/api/douyin`

| method_key | 中文名 | fetcher_name | HTTP | route | tags | 说明 |
| --- | --- | --- | --- | --- | --- | --- |
| `parseWork` | `聚合解析` | `parseWork` | `GET` | `/work/{aweme_id}` | `work` | 聚合解析作品 URL 或标识，并推断内容类型。 |
| `videoWork` | `视频作品数据` | `fetchVideoWork` | `GET` | `/work/{aweme_id}/video` | `work` | 获取抖音视频作品。 |
| `imageAlbumWork` | `图集作品数据` | `fetchImageAlbumWork` | `GET` | `/work/{aweme_id}/image-album` | `work` | 获取抖音图文作品。 |
| `slidesWork` | `合辑作品数据` | `fetchSlidesWork` | `GET` | `/work/{aweme_id}/slides` | `work` | 获取抖音图集作品。 |
| `textWork` | `文字作品数据` | `fetchTextWork` | `GET` | `/work/{aweme_id}/text` | `work` | 获取抖音文字作品。 |
| `comments` | `评论数据` | `fetchWorkComments` | `GET` | `/comments/{aweme_id}` | `comment` | 获取抖音作品评论。 |
| `commentReplies` | `指定评论回复数据` | `fetchCommentReplies` | `GET` | `/comment-replies/{aweme_id}/{comment_id}` | `comment` | 获取抖音评论回复。 |
| `userProfile` | `用户主页数据` | `fetchUserProfile` | `GET` | `/user/{sec_uid}` | `user` | 获取抖音用户主页。 |
| `userVideoList` | `用户主页视频列表数据` | `fetchUserVideoList` | `GET` | `/user/{sec_uid}/videos` | `user` | 获取抖音用户发布视频列表。 |
| `userFavoriteList` | `用户主页喜欢列表数据` | `fetchUserFavoriteList` | `GET` | `/user/{sec_uid}/favorites` | `user` | 获取抖音用户喜欢列表。 |
| `userRecommendList` | `用户主页推荐列表数据` | `fetchUserRecommendList` | `GET` | `/user/{sec_uid}/recommends` | `user` | 获取抖音用户推荐流。 |
| `search` | `搜索数据` | `searchContent` | `GET` | `/search` | `search` | 搜索抖音内容。 |
| `suggestWords` | `热点词数据` | `fetchSuggestWords` | `GET` | `/search/suggest` | `search` | 获取抖音搜索联想词。 |
| `musicInfo` | `音乐数据` | `fetchMusicInfo` | `GET` | `/music/{music_id}` | `music` | 获取抖音音乐元数据。 |
| `liveRoomInfo` | `直播间信息数据` | `fetchLiveRoomInfo` | `GET` | `/live/{room_id}` | `live` | 获取抖音直播间信息。 |
| `loginQrcode` | `申请二维码数据` | `requestLoginQrcode` | `GET` | `/auth/qrcode` | `auth` | 申请抖音登录二维码。 |
| `emojiList` | `Emoji数据` | `fetchEmojiList` | `GET` | `/emoji` | `emoji` | 获取抖音表情目录。 |
| `dynamicEmojiList` | `动态表情数据` | `fetchDynamicEmojiList` | `GET` | `/emoji/dynamic` | `emoji` | 获取抖音动态表情数据。 |
| `danmakuList` | `弹幕数据` | `fetchDanmakuList` | `GET` | `/danmaku/{aweme_id}` | `danmaku` | 获取抖音弹幕数据。 |

## 8. 快手 Catalog

平台基础路径：`/api/kuaishou`

| method_key | 中文名 | fetcher_name | HTTP | route | tags | 说明 |
| --- | --- | --- | --- | --- | --- | --- |
| `videoWork` | `单个视频作品数据` | `fetchVideoWork` | `GET` | `/work/{photo_id}` | `work` | 获取单个快手视频作品。 |
| `comments` | `评论数据` | `fetchWorkComments` | `GET` | `/comments/{photo_id}` | `comment` | 获取快手作品评论。 |
| `userProfile` | `用户主页数据` | `fetchUserProfile` | `GET` | `/user/{principal_id}` | `user` | 获取快手用户主页。 |
| `userWorkList` | `用户作品列表数据` | `fetchUserWorkList` | `GET` | `/user/{principal_id}/works` | `user` | 获取快手用户作品列表。 |
| `liveRoomInfo` | `直播间信息数据` | `fetchLiveRoomInfo` | `GET` | `/live/{principal_id}` | `live` | 获取快手直播间信息。 |
| `emojiList` | `Emoji数据` | `fetchEmojiList` | `GET` | `/emoji` | `emoji` | 获取快手表情目录。 |

## 9. Twitter / X Catalog

平台基础路径：`/api/twitter`

| method_key | 中文名 | fetcher_name | HTTP | route | tags | 说明 |
| --- | --- | --- | --- | --- | --- | --- |
| `searchTweets` | `搜索推文` | `searchTweets` | `GET` | `/search/tweets` | `search` | 按原始查询条件搜索 Twitter/X 推文。 |
| `userProfile` | `用户资料` | `fetchUserProfile` | `GET` | `/user/{screen_name}` | `user` | 按 `screen_name` 获取 Twitter/X 用户资料。 |
| `userTimeline` | `用户时间线` | `fetchUserTimeline` | `GET` | `/user/{screen_name}/timeline` | `user` | 按 `screen_name` 获取 Twitter/X 用户时间线。 |
| `userReplies` | `用户回复` | `fetchUserReplies` | `GET` | `/user/{screen_name}/replies` | `user` | 按 `screen_name` 获取 Twitter/X 用户回复流。 |
| `userMedia` | `用户媒体` | `fetchUserMedia` | `GET` | `/user/{screen_name}/media` | `user` | 按 `screen_name` 获取 Twitter/X 用户媒体流。 |
| `userFollowers` | `用户粉丝` | `fetchUserFollowers` | `GET` | `/user/{screen_name}/followers` | `user` | 按 `screen_name` 获取 Twitter/X 用户粉丝列表。 |
| `userFollowing` | `用户关注` | `fetchUserFollowing` | `GET` | `/user/{screen_name}/following` | `user` | 按 `screen_name` 获取 Twitter/X 用户关注列表。 |
| `tweetDetail` | `推文详情` | `fetchTweetDetail` | `GET` | `/tweet/{tweet_id}` | `tweet` | 按 `tweet_id` 获取 Twitter/X 推文详情。 |
| `spaceDetail` | `Space详情` | `fetchSpaceDetail` | `GET` | `/space/{space_id}` | `space` | 按 `space_id` 获取 Twitter/X Space 详情。 |

## 10. 小红书 Catalog

平台基础路径：`/api/xiaohongshu`

| method_key | 中文名 | fetcher_name | HTTP | route | tags | 说明 |
| --- | --- | --- | --- | --- | --- | --- |
| `homeFeed` | `首页推荐数据` | `fetchHomeFeed` | `GET` | `/feed` | `feed` | 获取小红书首页推荐流。 |
| `noteDetail` | `单个笔记数据` | `fetchNoteDetail` | `GET` | `/note` | `work` | 获取单个小红书笔记。 |
| `noteComments` | `评论数据` | `fetchNoteComments` | `GET` | `/comments` | `comment` | 获取小红书笔记评论。 |
| `userProfile` | `用户数据` | `fetchUserProfile` | `GET` | `/user` | `user` | 获取小红书用户资料。 |
| `userNoteList` | `用户笔记数据` | `fetchUserNoteList` | `GET` | `/user/notes` | `user` | 获取小红书用户笔记列表。 |
| `emojiList` | `表情列表` | `fetchEmojiList` | `GET` | `/emoji` | `emoji` | 获取小红书表情目录。 |
| `searchNotes` | `搜索笔记` | `searchNotes` | `GET` | `/search` | `search` | 搜索小红书笔记。 |
