# Amagi-rs CLI Command and Parameter Reference

Chinese version: [../../中文/参考/命令行参考.md](../../中文/参考/命令行参考.md)

This document covers the current `amagi` CLI surface in full:

- Top-level commands
- Global options
- All `run` platform tasks
- All task parameters
- Supported enum values, defaults, and environment-variable bindings

## 1. Invocation

Standard form:

```bash
amagi [GLOBAL_OPTIONS] [COMMAND]
```

If the top-level `COMMAND` is omitted, the CLI behaves as:

```bash
amagi run
```

The `serve` subcommand is only available when the `server` feature is enabled.

## 2. Top-Level Capabilities

| Command | Description |
| --- | --- |
| `run` | Execute local CLI workflows |
| `serve` | Start the built-in JSON API service |

Currently available `run` platforms:

| Platform | Task Count | Coverage |
| --- | --- | --- |
| `bilibili` | 26 | video, comments, dynamics, bangumi, live, login QR, articles, captcha, emoji |
| `douyin` | 19 | works, comments, users, search, music, live, login QR, emoji, danmaku |
| `kuaishou` | 6 | works, comments, user profile, user works, live, emoji |
| `twitter` | 19 | search, profiles, timelines, replies, media, follower graph, authenticated timelines, live room info, live room streams, tweets, Spaces |
| `xiaohongshu` | 7 | home feed, notes, comments, profiles, search, emoji |

## 3. Global Options

Global options can appear before the top-level command or after nested subcommands.

| Option | Values | Default | Environment / `.env` | Description |
| --- | --- | --- | --- | --- |
| `--lang <LANG>` | `zh` / `en` | auto-detected from system locale, fallback `en` | `AMAGI_LANG` | CLI help language |
| `--output <OUTPUT>` | `text` / `json` | `text` | `AMAGI_OUTPUT` | CLI-facing output format |
| `--output-file`, `-o <OUTPUT_FILE>` | file path | none | `AMAGI_OUTPUT_FILE` | Write CLI-facing output to a file |
| `--pretty` | boolean flag | `false` | `AMAGI_OUTPUT_PRETTY` | Pretty-print JSON output |
| `--append` | boolean flag | `false` | `AMAGI_OUTPUT_APPEND` | Append to `--output-file` instead of truncating |
| `--create-parent-dirs` | boolean flag | `false` | `AMAGI_OUTPUT_CREATE_DIRS` | Create missing parent directories for the output file |
| `--douyin-cookie <DOUYIN_COOKIE>` | cookie string | none | `AMAGI_DOUYIN_COOKIE` | Douyin cookie |
| `--bilibili-cookie <BILIBILI_COOKIE>` | cookie string | none | `AMAGI_BILIBILI_COOKIE` | Bilibili cookie |
| `--kuaishou-cookie <KUAISHOU_COOKIE>` | cookie string | none | `AMAGI_KUAISHOU_COOKIE` | Kuaishou cookie |
| `--xiaohongshu-cookie <XIAOHONGSHU_COOKIE>` | cookie string | none | `AMAGI_XIAOHONGSHU_COOKIE` | Xiaohongshu cookie |
| `--twitter-cookie <TWITTER_COOKIE>` | cookie string | none | `AMAGI_TWITTER_COOKIE` | Twitter/X cookie |
| `--timeout-ms <TIMEOUT_MS>` | unsigned integer | `10000` | `AMAGI_TIMEOUT_MS` | Upstream request timeout in milliseconds |
| `--max-retries <MAX_RETRIES>` | unsigned integer | `3` | `AMAGI_MAX_RETRIES` | Maximum retries for recoverable upstream failures |
| `--log-format <LOG_FORMAT>` | `text` / `json` | `text` | `AMAGI_LOG_FORMAT` | Log renderer format |
| `--log-level <LOG_LEVEL>` | `error` / `warn` / `info` / `debug` / `trace` | `info` | `AMAGI_LOG` | Minimum log level |
| `--help`, `-h` | none | none | none | Print help |
| `--version`, `-V` | none | none | none | Print version |

## 4. The `run` Command

Invocation:

```bash
amagi run [--quiet] [PLATFORM] [TASK]
```

### 4.1 `run`-Level Options

| Option | Values | Default | Description |
| --- | --- | --- | --- |
| `--quiet` | boolean flag | `false` | Suppress normal startup output |

### 4.2 Platform Subcommands

| Platform | Description |
| --- | --- |
| `bilibili` | Execute Bilibili-specific tasks |
| `douyin` | Execute Douyin-specific tasks |
| `kuaishou` | Execute Kuaishou-specific tasks |
| `twitter` | Execute Twitter/X-specific tasks |
| `xiaohongshu` | Execute Xiaohongshu-specific tasks |

## 5. Bilibili Commands

Prefix:

```bash
amagi run bilibili <TASK> ...
```

| Task | Description | Status | Required Positional Parameters | Optional / Named Parameters |
| --- | --- | --- | --- | --- |
| `video-info` | Fetch one Bilibili video payload | 2.0发布待测试 | `<bvid>` | none |
| `video-stream` | Fetch one Bilibili video stream payload | 2.0发布待测试 | `<aid>` | required named `--cid <u64>` |
| `video-danmaku` | Fetch one Bilibili video danmaku segment | 2.0发布待测试 | `<cid>` | `--segment-index <u32>` |
| `comments` | Fetch comments for one Bilibili subject | 2.0发布待测试 | `<oid>` | required `--type <u32>`, `--number <u32>`, `--mode <u32>` |
| `comment-replies` | Fetch replies for one Bilibili root comment | 2.0发布待测试 | `<oid> <root>` | required `--type <u32>`, `--number <u32>` |
| `user-card` | Fetch one Bilibili user card | 2.0发布待测试 | `<host_mid>` | none |
| `user-dynamic-list` | Fetch one Bilibili user dynamic list | 2.0发布待测试 | `<host_mid>` | none |
| `user-space-info` | Fetch one Bilibili user space payload | 2.0发布待测试 | `<host_mid>` | none |
| `uploader-total-views` | Fetch one uploader's total views | 2.0发布待测试 | `<host_mid>` | none |
| `dynamic-detail` | Fetch one Bilibili dynamic detail payload | 2.0发布待测试 | `<dynamic_id>` | none |
| `dynamic-card` | Fetch one Bilibili dynamic card payload | 2.0发布待测试 | `<dynamic_id>` | none |
| `bangumi-info` | Fetch one Bilibili bangumi metadata payload | 2.0发布待测试 | `<bangumi_id>` | none |
| `bangumi-stream` | Fetch one Bilibili bangumi stream payload | 2.0发布待测试 | `<ep_id>` | required named `--cid <u64>` |
| `live-room-info` | Fetch one Bilibili live room detail payload | 2.0发布待测试 | `<room_id>` | none |
| `live-room-init` | Fetch one Bilibili live room init payload | 2.0发布待测试 | `<room_id>` | none |
| `login-status` | Fetch the current Bilibili login status | 2.0发布待测试 | none | none |
| `login-qrcode` | Request a Bilibili login QR code | 2.0发布待测试 | none | none |
| `qrcode-status` | Poll one Bilibili login QR code | 2.0发布待测试 | `<qrcode_key>` | none |
| `emoji-list` | Fetch the Bilibili emoji catalog | 2.0发布待测试 | none | none |
| `av-to-bv` | Convert one AV identifier into BV | 2.0发布待测试 | `<aid>` | none |
| `bv-to-av` | Convert one BV identifier into AV | 2.0发布待测试 | `<bvid>` | none |
| `article-content` | Fetch one Bilibili article content payload | 2.0发布待测试 | `<article_id>` | none |
| `article-cards` | Fetch article cards for one or more ids | 2.0发布待测试 | `<ids>...` | none |
| `article-info` | Fetch one Bilibili article metadata payload | 2.0发布待测试 | `<article_id>` | none |
| `article-list-info` | Fetch one Bilibili article-list payload | 2.0发布待测试 | `<list_id>` | none |
| `captcha-from-voucher` | Request a Bilibili captcha challenge from one voucher | 2.0发布待测试 | `<v_voucher>` | `--csrf <string>` |
| `validate-captcha` | Validate one Bilibili captcha result | 2.0发布待测试 | `<challenge> <token> <validate> <seccode>` | `--csrf <string>` |

### 5.1 Bilibili Parameter Reference

| Parameter | Type | Description |
| --- | --- | --- |
| `bvid` | string | video BV identifier |
| `aid` | `u64` | video AV identifier |
| `cid` | `u64` | video or bangumi content id |
| `segment_index` | `u32` | danmaku segment index |
| `oid` | `u64` | comment subject id |
| `comment_type` | `u32` | comment type |
| `number` | `u32` | comment page size |
| `mode` | `u32` | comment mode |
| `root` | `u64` | root comment id |
| `host_mid` | `u64` | user mid |
| `dynamic_id` | string | dynamic id |
| `bangumi_id` | string | bangumi id |
| `ep_id` | string | episode id |
| `room_id` | `u64` | live room id |
| `qrcode_key` | string | login QR key |
| `article_id` | string | article id |
| `ids` | string array | one or more article ids |
| `list_id` | string | article-list id |
| `v_voucher` | string | captcha voucher |
| `csrf` | string | optional csrf token |
| `challenge` | string | captcha challenge |
| `token` | string | captcha token |
| `validate` | string | captcha validate value |
| `seccode` | string | captcha seccode |

## 6. Douyin Commands

Prefix:

```bash
amagi run douyin <TASK> ...
```

| Task | Description | Status | Required Positional Parameters | Optional / Named Parameters |
| --- | --- | --- | --- | --- |
| `parse-work` | Parse one Douyin work | 2.0发布待测试 | `<aweme_id>` | none |
| `video-work` | Fetch one Douyin video work | 2.0发布待测试 | `<aweme_id>` | none |
| `image-album-work` | Fetch one Douyin image album work | 2.0发布待测试 | `<aweme_id>` | none |
| `slides-work` | Fetch one Douyin slides work | 2.0发布待测试 | `<aweme_id>` | none |
| `text-work` | Fetch one Douyin text work | 2.0发布待测试 | `<aweme_id>` | none |
| `work-comments` | Fetch comments for one Douyin work | 2.0发布待测试 | `<aweme_id>` | `--number <u32>`, `--cursor <u64>` |
| `comment-replies` | Fetch replies for one Douyin comment | 2.0发布待测试 | `<aweme_id> <comment_id>` | `--number <u32>`, `--cursor <u64>` |
| `user-profile` | Fetch one Douyin user profile | 2.0发布待测试 | `<sec_uid>` | none |
| `user-video-list` | Fetch one Douyin user video list | 2.0发布待测试 | `<sec_uid>` | `--number <u32>`, `--max-cursor <string>` |
| `user-favorite-list` | Fetch one Douyin user favorite list | 2.0发布待测试 | `<sec_uid>` | `--number <u32>`, `--max-cursor <string>` |
| `user-recommend-list` | Fetch one Douyin user recommend list | 2.0发布待测试 | `<sec_uid>` | `--number <u32>`, `--max-cursor <string>` |
| `search` | Search Douyin content | 2.0发布待测试 | `<query>` | `--type <search_type>`, `--number <u32>`, `--search-id <string>` |
| `suggest-words` | Fetch Douyin suggest words | 2.0发布待测试 | `<query>` | none |
| `music-info` | Fetch Douyin music metadata | 2.0发布待测试 | `<music_id>` | none |
| `live-room-info` | Fetch Douyin live room info | 2.0发布待测试 | `<room_id>` | required named `--web-rid <string>` |
| `login-qrcode` | Request a Douyin login QR code | 2.0发布待测试 | none | `--verify-fp <string>` |
| `emoji-list` | Fetch the Douyin emoji list | 2.0发布待测试 | none | none |
| `dynamic-emoji-list` | Fetch the Douyin dynamic emoji list | 2.0发布待测试 | none | none |
| `danmaku-list` | Fetch the Douyin danmaku list | 2.0发布待测试 | `<aweme_id>` | required `--duration <u64>`, `--start-time <u64>`, `--end-time <u64>` |

### 6.1 Douyin Enum Parameters

| Parameter | Supported Values |
| --- | --- |
| `search_type` | `general`, `user`, `video` |

### 6.2 Douyin Parameter Reference

| Parameter | Type | Description |
| --- | --- | --- |
| `aweme_id` | string | work id |
| `comment_id` | string | comment id |
| `sec_uid` | string | user `sec_uid` |
| `number` | `u32` | page size |
| `cursor` | `u64` | pagination cursor |
| `max_cursor` | string | user-list pagination cursor |
| `query` | string | search keyword |
| `search_id` | string | search cursor id |
| `music_id` | string | music id |
| `room_id` | string | live room id |
| `web_rid` | string | live room `web_rid` |
| `verify_fp` | string | optional `verify_fp` override for login QR |
| `duration` | `u64` | work duration in milliseconds |
| `start_time` | `u64` | danmaku start time |
| `end_time` | `u64` | danmaku end time |

## 7. Kuaishou Commands

Prefix:

```bash
amagi run kuaishou <TASK> ...
```

| Task | Description | Status | Required Positional Parameters | Optional / Named Parameters |
| --- | --- | --- | --- | --- |
| `video-work` | Fetch one Kuaishou video work | 2.0发布待测试 | `<photo_id>` | none |
| `work-comments` | Fetch comments for one Kuaishou work | 2.0发布待测试 | `<photo_id>` | none |
| `emoji-list` | Fetch the Kuaishou emoji catalog | 2.0发布待测试 | none | none |
| `user-profile` | Fetch one Kuaishou user profile | 2.0发布待测试 | `<principal_id>` | none |
| `user-work-list` | Fetch one Kuaishou user work list | 2.0发布待测试 | `<principal_id>` | `--pcursor <string>`, `--count <u32>` |
| `live-room-info` | Fetch Kuaishou live room info | 2.0发布待测试 | `<principal_id>` | none |

### 7.1 Kuaishou Parameter Reference

| Parameter | Type | Description |
| --- | --- | --- |
| `photo_id` | string | work id |
| `principal_id` | string | user or live-room `principal_id` |
| `pcursor` | string | pagination cursor |
| `count` | `u32` | item count |

## 8. Twitter / X Commands

Prefix:

```bash
amagi run twitter <TASK> ...
```

| Task | Description | Status | Required Positional Parameters | Optional / Named Parameters |
| --- | --- | --- | --- | --- |
| `search-tweets` | Search Twitter/X tweets | 2.0发布待测试 | `<query>` | `--search-type <mode>`, `--count <u32>`, `--cursor <string>` |
| `search-users` | Search Twitter/X users | 2.0发布待测试 | `<query>` | `--count <u32>`, `--cursor <string>` |
| `user-profile` | Fetch one Twitter/X user profile | 2.0发布待测试 | `<screen_name>` | none |
| `user-timeline` | Fetch one Twitter/X user timeline | 2.0发布待测试 | `<screen_name>` | `--count <u32>`, `--cursor <string>` |
| `user-replies` | Fetch one Twitter/X user replies timeline | 2.0发布待测试 | `<screen_name>` | `--count <u32>`, `--cursor <string>` |
| `user-media` | Fetch one Twitter/X user media timeline | 2.0发布待测试 | `<screen_name>` | `--count <u32>`, `--cursor <string>` |
| `user-followers` | Fetch one Twitter/X user followers page | 2.0发布待测试 | `<screen_name>` | `--count <u32>`, `--cursor <string>` |
| `user-following` | Fetch one Twitter/X user following page | 2.0发布待测试 | `<screen_name>` | `--count <u32>`, `--cursor <string>` |
| `user-likes` | Fetch liked tweets for the authenticated Twitter/X account | 2.0发布待测试 | none | `--count <u32>`, `--cursor <string>` |
| `user-bookmarks` | Fetch bookmarks for the authenticated Twitter/X account | 2.0发布待测试 | none | `--count <u32>`, `--cursor <string>` |
| `user-followed` | Fetch the followed home timeline for the authenticated Twitter/X account | 2.0发布待测试 | none | `--count <u32>`, `--cursor <string>` |
| `user-recommended` | Fetch the recommended home timeline for the authenticated Twitter/X account | 2.0发布待测试 | none | `--count <u32>`, `--cursor <string>` |
| `live-room-info` | Fetch current live room information for a Twitter/X account | 2.0发布待测试 | optional `<screen_name>` | `--user-id <user_id>`; choose either `screen_name` or `--user-id` |
| `live-room-stream` | Resolve a Twitter/X live room master HLS stream | 2.0发布待测试 | optional `<broadcast_id>` | `--media-key <media_key>` or `--tweet-id <tweet_id>`; choose exactly one input |
| `tweet-detail` | Fetch one Twitter/X tweet detail | 2.0发布待测试 | `<tweet_id>` | none |
| `tweet-replies` | Fetch replies for one Twitter/X tweet | 2.0发布待测试 | `<tweet_id>` | `--cursor <string>`, `--sort-by <sort_by>` |
| `tweet-likers` | Fetch users who liked one Twitter/X tweet | 2.0发布待测试 | `<tweet_id>` | `--count <u32>`, `--cursor <string>` |
| `tweet-retweeters` | Fetch users who retweeted one Twitter/X tweet | 2.0发布待测试 | `<tweet_id>` | `--count <u32>`, `--cursor <string>` |
| `space-detail` | Fetch one Twitter/X Space detail | 2.0发布待测试 | `<space_id>` | none |

CLI verification status meanings:

- `2.0发布待测试`: pending verification for the 2.0 release.

Current caveats:

- `user-replies`: current output mixes normal tweets into the replies timeline instead of returning replies only
- `danmaku-list`: verified against a Douyin work with `danmaku_cnt = 1`, but the current response still returned `total = 0` and no danmaku entries
- `user-media`: current output may still contain items with `media: []`; media filtering or media entity parsing needs tightening
- `tweet-replies`: current output includes the root tweet itself instead of returning replies only

### 8.1 Twitter / X Enum Parameters

| Parameter | Supported Values |
| --- | --- |
| `search_type` | `latest`, `top` |

### 8.2 Twitter / X Parameter Reference

| Parameter | Type | Description |
| --- | --- | --- |
| `query` | string | search keyword |
| `search_type` | enum | search mode |
| `count` | `u32` | item count |
| `cursor` | string | pagination cursor |
| `screen_name` | string | user `screen_name` |
| `user_id` | string | numeric Twitter/X user id |
| `tweet_id` | string | tweet id |
| `space_id` | string | Space id |

## 9. Xiaohongshu Commands

Prefix:

```bash
amagi run xiaohongshu <TASK> ...
```

| Task | Description | Status | Required Positional Parameters | Optional / Named Parameters |
| --- | --- | --- | --- | --- |
| `home-feed` | Fetch the Xiaohongshu home feed | 2.0发布待测试 | none | `--cursor-score <string>`, `--num <u32>`, `--refresh-type <u32>`, `--note-index <u32>`, `--category <string>`, `--search-key <string>` |
| `note-detail` | Fetch one Xiaohongshu note detail | 2.0发布待测试 | `<note_id>` | required named `--xsec-token <string>` |
| `note-comments` | Fetch one page of Xiaohongshu note comments | 2.0发布待测试 | `<note_id>` | required `--xsec-token <string>`, `--cursor <string>` |
| `user-profile` | Fetch one Xiaohongshu user profile | 2.0发布待测试 | `<user_id>` | required `--xsec-token <string>`, optional `--xsec-source <string>` |
| `user-note-list` | Fetch one page of Xiaohongshu user notes | 2.0发布待测试 | `<user_id>` | required `--xsec-token <string>`, optional `--xsec-source <string>`, `--cursor <string>`, `--num <u32>` |
| `emoji-list` | Fetch the Xiaohongshu emoji catalog | 2.0发布待测试 | none | none |
| `search` | Search Xiaohongshu notes | 2.0发布待测试 | `<keyword>` | `--page <u32>`, `--page-size <u32>`, `--sort <sort>`, `--note-type <note_type>` |

### 9.1 Xiaohongshu Enum Parameters

| Parameter | Supported Values |
| --- | --- |
| `sort` | `general`, `time_descending`, `popularity_descending` |
| `note_type` | `all`, `video`, `image` |

### 9.2 Xiaohongshu Parameter Reference

| Parameter | Type | Description |
| --- | --- | --- |
| `cursor_score` | string | home-feed cursor score |
| `num` | `u32` | item count |
| `refresh_type` | `u32` | refresh type |
| `note_index` | `u32` | note index |
| `category` | string | feed category |
| `search_key` | string | feed search key |
| `note_id` | string | note id |
| `xsec_token` | string | note `xsec_token` |
| `cursor` | string | pagination cursor |
| `user_id` | string | user id |
| `keyword` | string | search keyword |
| `page` | `u32` | page number |
| `page_size` | `u32` | page size |

## 10. The `serve` Command

Invocation:

```bash
amagi serve [OPTIONS]
```

| Option | Values | Default | Environment / `.env` | Description |
| --- | --- | --- | --- | --- |
| `--host <HOST>` | host or IP | `127.0.0.1` | `AMAGI_HOST` | bind address |
| `--port <PORT>` | `u16` | `4567` | `AMAGI_PORT` | bind port |
| `--proxy-timeout-ms <MS>` | `u64` | `15000` | `AMAGI_PROXY_TIMEOUT_MS` | timeout used by node-to-node proxy requests |
| `--proxy-max-hops <COUNT>` | `u32` | `4` | `AMAGI_PROXY_MAX_HOPS` | maximum HTTP upstream proxy hop count |
| `--node-id <ID>` | string | none | `AMAGI_NODE_ID` | stable node id once node transport is enabled |
| `--node-role <ROLE>` | `root`, `worker`, `relay`, `hybrid` | inferred | `AMAGI_NODE_ROLE` | node role that shapes upstream/downstream behavior |
| `--node-accept-downstream <BOOL>` | `bool` | role-derived | `AMAGI_NODE_ACCEPT_DOWNSTREAM` | whether this node accepts downstream WSS sessions |
| `--node-connect-upstream <URL>` | URL | none | `AMAGI_NODE_CONNECT_UPSTREAM` | upstream `wss://` endpoint this node connects to |
| `--node-auth-token <TOKEN>` | string | none | `AMAGI_NODE_AUTH_TOKEN` | shared token for the minimum node-auth flow |
| `--node-auth-credentials <MAP>` | `node=token,...` | none | `AMAGI_NODE_AUTH_CREDENTIALS` | optional per-node credential table |
| `--node-control-token <TOKEN>` | string | falls back to `--node-auth-token` | `AMAGI_NODE_CONTROL_TOKEN` | bearer token for internal control APIs |
| `--node-allow-insecure-ws <BOOL>` | `bool` | `false` | `AMAGI_NODE_ALLOW_INSECURE_WS` | allow `ws://` instead of requiring `wss://` |
| `--node-heartbeat-ms <MS>` | `u64` | `10000` | `AMAGI_NODE_HEARTBEAT_MS` | node heartbeat interval |
| `--node-request-timeout-ms <MS>` | `u64` | `15000` | `AMAGI_NODE_REQUEST_TIMEOUT_MS` | timeout budget for one node task |
| `--node-max-hops <COUNT>` | `u32` | `4` | `AMAGI_NODE_MAX_HOPS` | maximum node-routing hop count |
| `--node-max-concurrent-tasks <COUNT>` | `u32` | `8` | `AMAGI_NODE_MAX_CONCURRENT_TASKS` | maximum number of concurrent node tasks executed locally |
| `--node-auto-claim-published-routes <BOOL>` | `bool` | `false` | `AMAGI_NODE_AUTO_CLAIM_PUBLISHED_ROUTES` | auto-claim locally executable platforms upstream after connect |
| `--douyin-mode <MODE>` | `enabled`, `local`, `upstream`, `disabled` | `local` | `AMAGI_PLATFORM_DOUYIN_MODE` | Douyin serving mode; `enabled` maps to local handling |
| `--douyin-route <TARGET>` | `local`, `disabled`, `node:<id>` | none | `AMAGI_PLATFORM_DOUYIN_ROUTE` | static Douyin route target; can pin the platform to one node |
| `--douyin-upstream <URL>` | URL | none | `AMAGI_PLATFORM_DOUYIN_UPSTREAM` | upstream node base URL used when Douyin runs in `upstream` mode |
| `--bilibili-mode <MODE>` | `enabled`, `local`, `upstream`, `disabled` | `local` | `AMAGI_PLATFORM_BILIBILI_MODE` | Bilibili serving mode; `enabled` maps to local handling |
| `--bilibili-route <TARGET>` | `local`, `disabled`, `node:<id>` | none | `AMAGI_PLATFORM_BILIBILI_ROUTE` | static Bilibili route target; can pin the platform to one node |
| `--bilibili-upstream <URL>` | URL | none | `AMAGI_PLATFORM_BILIBILI_UPSTREAM` | upstream node base URL used when Bilibili runs in `upstream` mode |
| `--kuaishou-mode <MODE>` | `enabled`, `local`, `upstream`, `disabled` | `local` | `AMAGI_PLATFORM_KUAISHOU_MODE` | Kuaishou serving mode; `enabled` maps to local handling |
| `--kuaishou-route <TARGET>` | `local`, `disabled`, `node:<id>` | none | `AMAGI_PLATFORM_KUAISHOU_ROUTE` | static Kuaishou route target; can pin the platform to one node |
| `--kuaishou-upstream <URL>` | URL | none | `AMAGI_PLATFORM_KUAISHOU_UPSTREAM` | upstream node base URL used when Kuaishou runs in `upstream` mode |
| `--xiaohongshu-mode <MODE>` | `enabled`, `local`, `upstream`, `disabled` | `local` | `AMAGI_PLATFORM_XIAOHONGSHU_MODE` | Xiaohongshu serving mode; `enabled` maps to local handling |
| `--xiaohongshu-route <TARGET>` | `local`, `disabled`, `node:<id>` | none | `AMAGI_PLATFORM_XIAOHONGSHU_ROUTE` | static Xiaohongshu route target; can pin the platform to one node |
| `--xiaohongshu-upstream <URL>` | URL | none | `AMAGI_PLATFORM_XIAOHONGSHU_UPSTREAM` | upstream node base URL used when Xiaohongshu runs in `upstream` mode |
| `--twitter-mode <MODE>` | `enabled`, `local`, `upstream`, `disabled` | `local` | `AMAGI_PLATFORM_TWITTER_MODE` | Twitter/X serving mode; `enabled` maps to local handling |
| `--twitter-route <TARGET>` | `local`, `disabled`, `node:<id>` | none | `AMAGI_PLATFORM_TWITTER_ROUTE` | static Twitter/X route target; can pin the platform to one node |
| `--twitter-upstream <URL>` | URL | none | `AMAGI_PLATFORM_TWITTER_UPSTREAM` | upstream node base URL used when Twitter/X runs in `upstream` mode |

Notes:

- As soon as any `--node-*` value is provided, the runtime attempts to resolve the full node configuration.
- Once node transport is enabled, `--node-id` and `--node-auth-token` are both required.
- `--node-role worker` and `relay` require `--node-connect-upstream`.
- `--node-connect-upstream` must use `wss://` unless `--node-allow-insecure-ws true` is set.
- `--*-route node:<id>` also requires node-transport configuration on the current process.
- `--*-route node:<id>` means "pin this platform to one node", not "switch to a separate platform mode".

## 11. Environment Variables

Priority order:

```text
command line > process environment > current directory .env > user config .env > built-in defaults
```

User config dotenv path:

- Linux/macOS: `~/.config/amagi/.env`
- Windows: `%APPDATA%\\amagi\\.env`

Override variable:

- `AMAGI_USER_ENV_FILE`

Supported variables:

| Variable | Purpose |
| --- | --- |
| `AMAGI_LANG` | CLI help language |
| `AMAGI_OUTPUT` | CLI output format |
| `AMAGI_USER_ENV_FILE` | override the user-level dotenv path |
| `AMAGI_OUTPUT_FILE` | CLI output file path |
| `AMAGI_OUTPUT_PRETTY` | pretty-print JSON output |
| `AMAGI_OUTPUT_APPEND` | append mode for output files |
| `AMAGI_OUTPUT_CREATE_DIRS` | create missing output directories |
| `AMAGI_DOUYIN_COOKIE` | Douyin cookie |
| `AMAGI_BILIBILI_COOKIE` | Bilibili cookie |
| `AMAGI_KUAISHOU_COOKIE` | Kuaishou cookie |
| `AMAGI_XIAOHONGSHU_COOKIE` | Xiaohongshu cookie |
| `AMAGI_TWITTER_COOKIE` | Twitter/X cookie |
| `AMAGI_TIMEOUT_MS` | request timeout |
| `AMAGI_MAX_RETRIES` | maximum retries |
| `AMAGI_LOG_FORMAT` | log format |
| `AMAGI_LOG` | log level |
| `AMAGI_HOST` | server bind address |
| `AMAGI_PORT` | server bind port |
| `AMAGI_PROXY_TIMEOUT_MS` | node-to-node proxy timeout |
| `AMAGI_PROXY_MAX_HOPS` | maximum proxy hop count |
| `AMAGI_NODE_ID` | current node id |
| `AMAGI_NODE_ROLE` | current node role |
| `AMAGI_NODE_ACCEPT_DOWNSTREAM` | whether downstream node sessions are accepted |
| `AMAGI_NODE_CONNECT_UPSTREAM` | upstream WSS node endpoint |
| `AMAGI_NODE_AUTH_TOKEN` | node auth token |
| `AMAGI_NODE_AUTH_CREDENTIALS` | per-node credential table |
| `AMAGI_NODE_CONTROL_TOKEN` | internal control-plane token |
| `AMAGI_NODE_ALLOW_INSECURE_WS` | allow insecure `ws://` upstream transport |
| `AMAGI_NODE_HEARTBEAT_MS` | node heartbeat interval |
| `AMAGI_NODE_REQUEST_TIMEOUT_MS` | node task timeout budget |
| `AMAGI_NODE_MAX_HOPS` | maximum node-routing hop count |
| `AMAGI_NODE_MAX_CONCURRENT_TASKS` | maximum local concurrent node tasks |
| `AMAGI_NODE_AUTO_CLAIM_PUBLISHED_ROUTES` | auto-claim locally executable platforms upstream |
| `AMAGI_PLATFORM_DOUYIN_MODE` | Douyin serving mode |
| `AMAGI_PLATFORM_DOUYIN_ROUTE` | static Douyin route target |
| `AMAGI_PLATFORM_DOUYIN_UPSTREAM` | Douyin upstream node base URL |
| `AMAGI_PLATFORM_BILIBILI_MODE` | Bilibili serving mode |
| `AMAGI_PLATFORM_BILIBILI_ROUTE` | static Bilibili route target |
| `AMAGI_PLATFORM_BILIBILI_UPSTREAM` | Bilibili upstream node base URL |
| `AMAGI_PLATFORM_KUAISHOU_MODE` | Kuaishou serving mode |
| `AMAGI_PLATFORM_KUAISHOU_ROUTE` | static Kuaishou route target |
| `AMAGI_PLATFORM_KUAISHOU_UPSTREAM` | Kuaishou upstream node base URL |
| `AMAGI_PLATFORM_XIAOHONGSHU_MODE` | Xiaohongshu serving mode |
| `AMAGI_PLATFORM_XIAOHONGSHU_ROUTE` | static Xiaohongshu route target |
| `AMAGI_PLATFORM_XIAOHONGSHU_UPSTREAM` | Xiaohongshu upstream node base URL |
| `AMAGI_PLATFORM_TWITTER_MODE` | Twitter/X serving mode |
| `AMAGI_PLATFORM_TWITTER_ROUTE` | static Twitter/X route target |
| `AMAGI_PLATFORM_TWITTER_UPSTREAM` | Twitter/X upstream node base URL |

## 12. Maintenance Rules

When you add new CLI capabilities, update these locations together:

- `crates/amagi-cli/src/cli/args/*.rs`
- `crates/amagi-cli/locales/cli/zh-CN.json`
- `crates/amagi-cli/locales/cli/en-US.json`
- `docs/en/reference/cli.md`
- `docs/中文/参考/命令行参考.md`
