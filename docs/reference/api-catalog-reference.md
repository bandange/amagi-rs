# Amagi API Catalog Reference

Chinese version: [api-catalog-reference.zh-CN.md](api-catalog-reference.zh-CN.md)

The `catalog` feature exposes Amagi's static platform catalog. This surface is
pure metadata:

- no upstream HTTP requests
- no cookies required
- no runtime fetcher construction required

Use it when you need to inspect published methods, map method keys to routes,
or build tooling such as documentation generators, dashboards, adapters, or
code generation pipelines.

## 1. Enable The API Catalog

```toml
[dependencies]
amagi = { version = "0.1.2", default-features = false, features = ["catalog"] }
```

## 2. Quick Start

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

## 3. Core Types

| Type | Description |
| --- | --- |
| `HttpMethod` | Published HTTP verb for a catalog entry |
| `Platform` | Supported platform enum: `bilibili`, `douyin`, `kuaishou`, `twitter`, `xiaohongshu` |
| `ParsePlatformError` | Error returned when parsing an unknown platform string |
| `ApiMethodSpec` | One published method entry |
| `PlatformSpec` | Complete method catalog for one platform |

### 3.1 `Platform`

Useful `Platform` helpers:

| Interface | Description |
| --- | --- |
| `Platform::ALL` | Every supported platform in display order |
| `Platform::as_str()` | Stable lowercase platform identifier |
| `Platform::api_base_path()` | Stable base path such as `/api/douyin` |
| `"douyin".parse::<Platform>()` | Parse from a lowercase string |

### 3.2 `ApiMethodSpec`

Each catalog entry contains:

| Field | Meaning |
| --- | --- |
| `method_key` | Stable method key |
| `chinese_name` | Original Chinese label preserved from the source catalog |
| `fetcher_name` | Canonical English fetcher method name |
| `route` | Catalog-relative route |
| `http_method` | Published HTTP verb |
| `description` | Short human-readable description |
| `tags` | Grouping tags such as `user`, `work`, `emoji`, or `search` |

Important: catalog `route` is the canonical metadata route, not always the full
concrete Axum path template. If you need the exact HTTP path exposed by the web
server, use the Web API reference.

## 4. Lookup Helpers

| Interface | Description |
| --- | --- |
| `all_platform_specs()` | Return the full catalog for every platform |
| `platform_spec(platform)` | Return the full catalog for one platform |
| `method_specs(platform)` | Return only the method list for one platform |
| `find_method(platform, method_key)` | Find one `ApiMethodSpec` by stable key |
| `get_api_route(platform, method_key)` | Resolve the catalog route for a stable method key |
| `get_english_method_name(platform, chinese_method)` | Map the original Chinese label to the canonical English fetcher name |
| `get_chinese_method_name(platform, english_method)` | Map the canonical English fetcher name back to the Chinese label |

## 5. Platform Coverage Summary

| Platform | Method Count | Coverage |
| --- | --- | --- |
| `bilibili` | 27 | video, comments, user, dynamics, bangumi, live, articles, auth, tools, captcha, emoji |
| `douyin` | 19 | works, comments, user, search, music, live, auth, emoji, danmaku |
| `kuaishou` | 6 | works, comments, user, live, emoji |
| `twitter` | 9 | search, user graph, tweets, Spaces |
| `xiaohongshu` | 7 | feed, notes, comments, user, search, emoji |

## 6. Bilibili Catalog

Platform base path: `/api/bilibili`

| method_key | Chinese Name | fetcher_name | HTTP | route | tags | Description |
| --- | --- | --- | --- | --- | --- | --- |
| `videoInfo` | `单个视频作品数据` | `fetchVideoInfo` | `GET` | `/video` | `work` | Fetch a Bilibili video payload. |
| `videoStream` | `单个视频下载信息数据` | `fetchVideoStreamUrl` | `GET` | `/video/stream` | `work` | Fetch stream URLs for a Bilibili video. |
| `videoDanmaku` | `视频弹幕分段数据` | `fetchVideoDanmaku` | `GET` | `/video/{cid}/danmaku` | `danmaku` | Fetch one danmaku segment for a Bilibili video. |
| `comments` | `评论数据` | `fetchComments` | `GET` | `/comments` | `comment` | Fetch Bilibili comments. |
| `commentReplies` | `指定评论的回复` | `fetchCommentReplies` | `GET` | `/comment-replies` | `comment` | Fetch replies to a Bilibili comment. |
| `userCard` | `用户主页数据` | `fetchUserCard` | `GET` | `/user` | `user` | Fetch a Bilibili user card. |
| `userDynamicList` | `用户主页动态列表数据` | `fetchUserDynamicList` | `GET` | `/user/dynamics` | `user` | Fetch a Bilibili user's dynamic list. |
| `userSpaceInfo` | `用户空间详细信息` | `fetchUserSpaceInfo` | `GET` | `/user/space` | `user` | Fetch Bilibili user space details. |
| `uploaderTotalViews` | `获取UP主总播放量` | `fetchUploaderTotalViews` | `GET` | `/user/total-views` | `user` | Fetch total play counts for a Bilibili uploader. |
| `dynamicDetail` | `动态详情数据` | `fetchDynamicDetail` | `GET` | `/dynamic` | `dynamic` | Fetch Bilibili dynamic details. |
| `dynamicCard` | `动态卡片数据` | `fetchDynamicCard` | `GET` | `/dynamic/card` | `dynamic` | Fetch a Bilibili dynamic card. |
| `bangumiInfo` | `番剧基本信息数据` | `fetchBangumiInfo` | `GET` | `/bangumi` | `bangumi` | Fetch Bilibili bangumi metadata. |
| `bangumiStream` | `番剧下载信息数据` | `fetchBangumiStreamUrl` | `GET` | `/bangumi/stream` | `bangumi` | Fetch Bilibili bangumi stream URLs. |
| `liveRoomInfo` | `直播间信息` | `fetchLiveRoomInfo` | `GET` | `/live` | `live` | Fetch Bilibili live room details. |
| `liveRoomInit` | `直播间初始化信息` | `fetchLiveRoomInitInfo` | `GET` | `/live/init` | `live` | Fetch Bilibili live room init data. |
| `articleContent` | `专栏正文内容` | `fetchArticleContent` | `GET` | `/article/content` | `article` | Fetch Bilibili article body content. |
| `articleCards` | `专栏显示卡片信息` | `fetchArticleCards` | `GET` | `/article/cards` | `article` | Fetch Bilibili article cards. |
| `articleInfo` | `专栏文章基本信息` | `fetchArticleInfo` | `GET` | `/article` | `article` | Fetch Bilibili article metadata. |
| `articleListInfo` | `文集基本信息` | `fetchArticleListInfo` | `GET` | `/article-list` | `article` | Fetch Bilibili article list metadata. |
| `loginStatus` | `登录基本信息` | `fetchLoginStatus` | `GET` | `/auth/status` | `auth` | Fetch Bilibili login status. |
| `loginQrcode` | `申请二维码` | `requestLoginQrcode` | `GET` | `/auth/qrcode` | `auth` | Request a Bilibili login QR code. |
| `qrcodeStatus` | `二维码状态` | `checkQrcodeStatus` | `GET` | `/auth/qrcode/status` | `auth` | Check the status of a Bilibili login QR code. |
| `avToBv` | `AV转BV` | `convertAvToBv` | `GET` | `/convert/av-to-bv` | `tool` | Convert a Bilibili AV identifier into a BV identifier. |
| `bvToAv` | `BV转AV` | `convertBvToAv` | `GET` | `/convert/bv-to-av` | `tool` | Convert a Bilibili BV identifier into an AV identifier. |
| `emojiList` | `Emoji数据` | `fetchEmojiList` | `GET` | `/emoji` | `emoji` | Fetch the Bilibili emoji catalog. |
| `captchaFromVoucher` | `从_v_voucher_申请_captcha` | `requestCaptchaFromVoucher` | `POST` | `/captcha` | `captcha` | Request a Bilibili captcha challenge from a voucher. |
| `validateCaptcha` | `验证验证码结果` | `validateCaptchaResult` | `POST` | `/captcha/validate` | `captcha` | Validate a Bilibili captcha result. |

## 7. Douyin Catalog

Platform base path: `/api/douyin`

| method_key | Chinese Name | fetcher_name | HTTP | route | tags | Description |
| --- | --- | --- | --- | --- | --- | --- |
| `parseWork` | `聚合解析` | `parseWork` | `GET` | `/work/{aweme_id}` | `work` | Parse a work URL or identifier and infer the content type. |
| `videoWork` | `视频作品数据` | `fetchVideoWork` | `GET` | `/work/{aweme_id}/video` | `work` | Fetch a Douyin video work. |
| `imageAlbumWork` | `图集作品数据` | `fetchImageAlbumWork` | `GET` | `/work/{aweme_id}/image-album` | `work` | Fetch a Douyin image album work. |
| `slidesWork` | `合辑作品数据` | `fetchSlidesWork` | `GET` | `/work/{aweme_id}/slides` | `work` | Fetch a Douyin slides work. |
| `textWork` | `文字作品数据` | `fetchTextWork` | `GET` | `/work/{aweme_id}/text` | `work` | Fetch a Douyin text work. |
| `comments` | `评论数据` | `fetchWorkComments` | `GET` | `/comments/{aweme_id}` | `comment` | Fetch comments for a Douyin work. |
| `commentReplies` | `指定评论回复数据` | `fetchCommentReplies` | `GET` | `/comment-replies/{aweme_id}/{comment_id}` | `comment` | Fetch replies for a Douyin comment. |
| `userProfile` | `用户主页数据` | `fetchUserProfile` | `GET` | `/user/{sec_uid}` | `user` | Fetch a Douyin user profile. |
| `userVideoList` | `用户主页视频列表数据` | `fetchUserVideoList` | `GET` | `/user/{sec_uid}/videos` | `user` | Fetch published videos for a Douyin user. |
| `userFavoriteList` | `用户主页喜欢列表数据` | `fetchUserFavoriteList` | `GET` | `/user/{sec_uid}/favorites` | `user` | Fetch favorite works for a Douyin user. |
| `userRecommendList` | `用户主页推荐列表数据` | `fetchUserRecommendList` | `GET` | `/user/{sec_uid}/recommends` | `user` | Fetch recommendation feeds for a Douyin user. |
| `search` | `搜索数据` | `searchContent` | `GET` | `/search` | `search` | Search Douyin content. |
| `suggestWords` | `热点词数据` | `fetchSuggestWords` | `GET` | `/search/suggest` | `search` | Fetch Douyin search suggestion keywords. |
| `musicInfo` | `音乐数据` | `fetchMusicInfo` | `GET` | `/music/{music_id}` | `music` | Fetch Douyin music metadata. |
| `liveRoomInfo` | `直播间信息数据` | `fetchLiveRoomInfo` | `GET` | `/live/{room_id}` | `live` | Fetch Douyin live room information. |
| `loginQrcode` | `申请二维码数据` | `requestLoginQrcode` | `GET` | `/auth/qrcode` | `auth` | Request a Douyin login QR code. |
| `emojiList` | `Emoji数据` | `fetchEmojiList` | `GET` | `/emoji` | `emoji` | Fetch the Douyin emoji catalog. |
| `dynamicEmojiList` | `动态表情数据` | `fetchDynamicEmojiList` | `GET` | `/emoji/dynamic` | `emoji` | Fetch Douyin animated emoji data. |
| `danmakuList` | `弹幕数据` | `fetchDanmakuList` | `GET` | `/danmaku/{aweme_id}` | `danmaku` | Fetch Douyin danmaku data. |

## 8. Kuaishou Catalog

Platform base path: `/api/kuaishou`

| method_key | Chinese Name | fetcher_name | HTTP | route | tags | Description |
| --- | --- | --- | --- | --- | --- | --- |
| `videoWork` | `单个视频作品数据` | `fetchVideoWork` | `GET` | `/work/{photo_id}` | `work` | Fetch a Kuaishou video work. |
| `comments` | `评论数据` | `fetchWorkComments` | `GET` | `/comments/{photo_id}` | `comment` | Fetch Kuaishou work comments. |
| `userProfile` | `用户主页数据` | `fetchUserProfile` | `GET` | `/user/{principal_id}` | `user` | Fetch a Kuaishou user profile. |
| `userWorkList` | `用户作品列表数据` | `fetchUserWorkList` | `GET` | `/user/{principal_id}/works` | `user` | Fetch Kuaishou works for a user. |
| `liveRoomInfo` | `直播间信息数据` | `fetchLiveRoomInfo` | `GET` | `/live/{principal_id}` | `live` | Fetch Kuaishou live room information. |
| `emojiList` | `Emoji数据` | `fetchEmojiList` | `GET` | `/emoji` | `emoji` | Fetch the Kuaishou emoji catalog. |

## 9. Twitter / X Catalog

Platform base path: `/api/twitter`

| method_key | Chinese Name | fetcher_name | HTTP | route | tags | Description |
| --- | --- | --- | --- | --- | --- | --- |
| `searchTweets` | `搜索推文` | `searchTweets` | `GET` | `/search/tweets` | `search` | Search Twitter/X tweets by raw query. |
| `userProfile` | `用户资料` | `fetchUserProfile` | `GET` | `/user/{screen_name}` | `user` | Fetch a Twitter/X user profile by screen name. |
| `userTimeline` | `用户时间线` | `fetchUserTimeline` | `GET` | `/user/{screen_name}/timeline` | `user` | Fetch a Twitter/X user timeline by screen name. |
| `userReplies` | `用户回复` | `fetchUserReplies` | `GET` | `/user/{screen_name}/replies` | `user` | Fetch a Twitter/X user's replies by screen name. |
| `userMedia` | `用户媒体` | `fetchUserMedia` | `GET` | `/user/{screen_name}/media` | `user` | Fetch a Twitter/X user's media timeline by screen name. |
| `userFollowers` | `用户粉丝` | `fetchUserFollowers` | `GET` | `/user/{screen_name}/followers` | `user` | Fetch a Twitter/X user's followers by screen name. |
| `userFollowing` | `用户关注` | `fetchUserFollowing` | `GET` | `/user/{screen_name}/following` | `user` | Fetch a Twitter/X user's following list by screen name. |
| `tweetDetail` | `推文详情` | `fetchTweetDetail` | `GET` | `/tweet/{tweet_id}` | `tweet` | Fetch a Twitter/X tweet by tweet id. |
| `spaceDetail` | `Space详情` | `fetchSpaceDetail` | `GET` | `/space/{space_id}` | `space` | Fetch a Twitter/X Space by space id. |

## 10. Xiaohongshu Catalog

Platform base path: `/api/xiaohongshu`

| method_key | Chinese Name | fetcher_name | HTTP | route | tags | Description |
| --- | --- | --- | --- | --- | --- | --- |
| `homeFeed` | `首页推荐数据` | `fetchHomeFeed` | `GET` | `/feed` | `feed` | Fetch the Xiaohongshu home feed. |
| `noteDetail` | `单个笔记数据` | `fetchNoteDetail` | `GET` | `/note` | `work` | Fetch a Xiaohongshu note. |
| `noteComments` | `评论数据` | `fetchNoteComments` | `GET` | `/comments` | `comment` | Fetch comments for a Xiaohongshu note. |
| `userProfile` | `用户数据` | `fetchUserProfile` | `GET` | `/user` | `user` | Fetch a Xiaohongshu user profile. |
| `userNoteList` | `用户笔记数据` | `fetchUserNoteList` | `GET` | `/user/notes` | `user` | Fetch Xiaohongshu notes for a user. |
| `emojiList` | `表情列表` | `fetchEmojiList` | `GET` | `/emoji` | `emoji` | Fetch the Xiaohongshu emoji catalog. |
| `searchNotes` | `搜索笔记` | `searchNotes` | `GET` | `/search` | `search` | Search Xiaohongshu notes. |
