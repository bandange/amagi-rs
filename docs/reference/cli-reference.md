# Amagi CLI Command and Parameter Reference

Chinese version: [cli-reference.zh-CN.md](cli-reference.zh-CN.md)

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
| `serve` | Start the built-in Web API service |

Currently available `run` platforms:

| Platform | Task Count | Coverage |
| --- | --- | --- |
| `bilibili` | 26 | video, comments, dynamics, bangumi, live, login QR, articles, captcha, emoji |
| `douyin` | 19 | works, comments, users, search, music, live, login QR, emoji, danmaku |
| `kuaishou` | 6 | works, comments, user profile, user works, live, emoji |
| `twitter` | 17 | search, profiles, timelines, replies, media, follower graph, authenticated timelines, tweets, Spaces |
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
| `video-info` | Fetch one Bilibili video payload | tested and usable | `<bvid>` | none |
| `video-stream` | Fetch one Bilibili video stream payload | tested and usable | `<aid>` | required named `--cid <u64>` |
| `video-danmaku` | Fetch one Bilibili video danmaku segment | tested and usable | `<cid>` | `--segment-index <u32>` |
| `comments` | Fetch comments for one Bilibili subject | tested and usable | `<oid>` | required `--type <u32>`, `--number <u32>`, `--mode <u32>` |
| `comment-replies` | Fetch replies for one Bilibili root comment | tested and usable | `<oid> <root>` | required `--type <u32>`, `--number <u32>` |
| `user-card` | Fetch one Bilibili user card | tested and usable | `<host_mid>` | none |
| `user-dynamic-list` | Fetch one Bilibili user dynamic list | tested and usable | `<host_mid>` | none |
| `user-space-info` | Fetch one Bilibili user space payload | tested and usable | `<host_mid>` | none |
| `uploader-total-views` | Fetch one uploader's total views | tested and usable | `<host_mid>` | none |
| `dynamic-detail` | Fetch one Bilibili dynamic detail payload | tested and usable | `<dynamic_id>` | none |
| `dynamic-card` | Fetch one Bilibili dynamic card payload | tested and usable | `<dynamic_id>` | none |
| `bangumi-info` | Fetch one Bilibili bangumi metadata payload | not re-verified in this pass | `<bangumi_id>` | none |
| `bangumi-stream` | Fetch one Bilibili bangumi stream payload | not re-verified in this pass | `<ep_id>` | required named `--cid <u64>` |
| `live-room-info` | Fetch one Bilibili live room detail payload | not re-verified in this pass | `<room_id>` | none |
| `live-room-init` | Fetch one Bilibili live room init payload | not re-verified in this pass | `<room_id>` | none |
| `login-status` | Fetch the current Bilibili login status | tested and usable | none | none |
| `login-qrcode` | Request a Bilibili login QR code | tested and usable | none | none |
| `qrcode-status` | Poll one Bilibili login QR code | not re-verified in this pass | `<qrcode_key>` | none |
| `emoji-list` | Fetch the Bilibili emoji catalog | tested and usable | none | none |
| `av-to-bv` | Convert one AV identifier into BV | tested and usable | `<aid>` | none |
| `bv-to-av` | Convert one BV identifier into AV | tested and usable | `<bvid>` | none |
| `article-content` | Fetch one Bilibili article content payload | not re-verified in this pass | `<article_id>` | none |
| `article-cards` | Fetch article cards for one or more ids | not re-verified in this pass | `<ids>...` | none |
| `article-info` | Fetch one Bilibili article metadata payload | not re-verified in this pass | `<article_id>` | none |
| `article-list-info` | Fetch one Bilibili article-list payload | not re-verified in this pass | `<list_id>` | none |
| `captcha-from-voucher` | Request a Bilibili captcha challenge from one voucher | not re-verified in this pass | `<v_voucher>` | `--csrf <string>` |
| `validate-captcha` | Validate one Bilibili captcha result | not re-verified in this pass | `<challenge> <token> <validate> <seccode>` | `--csrf <string>` |

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
| `parse-work` | Parse one Douyin work | tested and usable | `<aweme_id>` | none |
| `video-work` | Fetch one Douyin video work | tested and usable | `<aweme_id>` | none |
| `image-album-work` | Fetch one Douyin image album work | tested and usable | `<aweme_id>` | none |
| `slides-work` | Fetch one Douyin slides work | tested and usable | `<aweme_id>` | none |
| `text-work` | Fetch one Douyin text work | tested and usable | `<aweme_id>` | none |
| `work-comments` | Fetch comments for one Douyin work | tested and usable | `<aweme_id>` | `--number <u32>`, `--cursor <u64>` |
| `comment-replies` | Fetch replies for one Douyin comment | tested and usable | `<aweme_id> <comment_id>` | `--number <u32>`, `--cursor <u64>` |
| `user-profile` | Fetch one Douyin user profile | tested and usable | `<sec_uid>` | none |
| `user-video-list` | Fetch one Douyin user video list | tested and usable | `<sec_uid>` | `--number <u32>`, `--max-cursor <string>` |
| `user-favorite-list` | Fetch one Douyin user favorite list | tested, current implementation failure | `<sec_uid>` | `--number <u32>`, `--max-cursor <string>` |
| `user-recommend-list` | Fetch one Douyin user recommend list | tested, current implementation failure | `<sec_uid>` | `--number <u32>`, `--max-cursor <string>` |
| `search` | Search Douyin content | tested and usable | `<query>` | `--type <search_type>`, `--number <u32>`, `--search-id <string>` |
| `suggest-words` | Fetch Douyin suggest words | tested and usable | `<query>` | none |
| `music-info` | Fetch Douyin music metadata | tested and usable | `<music_id>` | none |
| `live-room-info` | Fetch Douyin live room info | tested and usable | `<room_id>` | required named `--web-rid <string>` |
| `login-qrcode` | Request a Douyin login QR code | tested, current implementation failure | none | `--verify-fp <string>` |
| `emoji-list` | Fetch the Douyin emoji list | tested and usable | none | none |
| `dynamic-emoji-list` | Fetch the Douyin dynamic emoji list | tested and usable | none | none |
| `danmaku-list` | Fetch the Douyin danmaku list | tested, known caveat | `<aweme_id>` | required `--duration <u64>`, `--start-time <u64>`, `--end-time <u64>` |

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
| `video-work` | Fetch one Kuaishou video work | tested and usable | `<photo_id>` | none |
| `work-comments` | Fetch comments for one Kuaishou work | tested and usable | `<photo_id>` | none |
| `emoji-list` | Fetch the Kuaishou emoji catalog | tested and usable | none | none |
| `user-profile` | Fetch one Kuaishou user profile | tested and usable | `<principal_id>` | none |
| `user-work-list` | Fetch one Kuaishou user work list | tested and usable | `<principal_id>` | `--pcursor <string>`, `--count <u32>` |
| `live-room-info` | Fetch Kuaishou live room info | tested and usable | `<principal_id>` | none |

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
| `search-tweets` | Search Twitter/X tweets | tested and usable | `<query>` | `--search-type <mode>`, `--count <u32>`, `--cursor <string>` |
| `search-users` | Search Twitter/X users | tested and usable | `<query>` | `--count <u32>`, `--cursor <string>` |
| `user-profile` | Fetch one Twitter/X user profile | tested and usable | `<screen_name>` | none |
| `user-timeline` | Fetch one Twitter/X user timeline | tested and usable | `<screen_name>` | `--count <u32>`, `--cursor <string>` |
| `user-replies` | Fetch one Twitter/X user replies timeline | tested, known caveat | `<screen_name>` | `--count <u32>`, `--cursor <string>` |
| `user-media` | Fetch one Twitter/X user media timeline | tested, known caveat | `<screen_name>` | `--count <u32>`, `--cursor <string>` |
| `user-followers` | Fetch one Twitter/X user followers page | tested and usable | `<screen_name>` | `--count <u32>`, `--cursor <string>` |
| `user-following` | Fetch one Twitter/X user following page | tested and usable | `<screen_name>` | `--count <u32>`, `--cursor <string>` |
| `user-likes` | Fetch liked tweets for the authenticated Twitter/X account | tested and usable | none | `--count <u32>`, `--cursor <string>` |
| `user-bookmarks` | Fetch bookmarks for the authenticated Twitter/X account | tested and usable | none | `--count <u32>`, `--cursor <string>` |
| `user-followed` | Fetch the followed home timeline for the authenticated Twitter/X account | tested and usable | none | `--count <u32>`, `--cursor <string>` |
| `user-recommended` | Fetch the recommended home timeline for the authenticated Twitter/X account | tested and usable | none | `--count <u32>`, `--cursor <string>` |
| `tweet-detail` | Fetch one Twitter/X tweet detail | tested and usable | `<tweet_id>` | none |
| `tweet-replies` | Fetch replies for one Twitter/X tweet | tested, known caveat | `<tweet_id>` | `--cursor <string>`, `--sort-by <sort_by>` |
| `tweet-likers` | Fetch users who liked one Twitter/X tweet | tested and usable | `<tweet_id>` | `--count <u32>`, `--cursor <string>` |
| `tweet-retweeters` | Fetch users who retweeted one Twitter/X tweet | tested and usable | `<tweet_id>` | `--count <u32>`, `--cursor <string>` |
| `space-detail` | Fetch one Twitter/X Space detail | not re-verified in this pass | `<space_id>` | none |

CLI verification status meanings:

- `tested and usable`: the command path was verified and can be used as part of the current supported surface
- `tested, known caveat`: the command runs successfully, but the current output still differs from the intended command semantics
- `tested, requires valid authenticated session`: the command path was exercised, but the current environment did not provide a valid logged-in session and the upstream returned an authentication failure
- `tested, current implementation failure`: the command was exercised in this pass, but the current implementation failed and needs a code fix before it is usable
- `not re-verified in this pass`: the interface is documented, but this CLI verification pass did not cover it separately

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
| `tweet_id` | string | tweet id |
| `space_id` | string | Space id |

## 9. Xiaohongshu Commands

Prefix:

```bash
amagi run xiaohongshu <TASK> ...
```

| Task | Description | Status | Required Positional Parameters | Optional / Named Parameters |
| --- | --- | --- | --- | --- |
| `home-feed` | Fetch the Xiaohongshu home feed | tested and usable | none | `--cursor-score <string>`, `--num <u32>`, `--refresh-type <u32>`, `--note-index <u32>`, `--category <string>`, `--search-key <string>` |
| `note-detail` | Fetch one Xiaohongshu note detail | tested and usable | `<note_id>` | required named `--xsec-token <string>` |
| `note-comments` | Fetch one page of Xiaohongshu note comments | tested and usable | `<note_id>` | required `--xsec-token <string>`, `--cursor <string>` |
| `user-profile` | Fetch one Xiaohongshu user profile | tested and usable | `<user_id>` | required `--xsec-token <string>`, optional `--xsec-source <string>` |
| `user-note-list` | Fetch one page of Xiaohongshu user notes | tested and usable | `<user_id>` | required `--xsec-token <string>`, optional `--xsec-source <string>`, `--cursor <string>`, `--num <u32>` |
| `emoji-list` | Fetch the Xiaohongshu emoji catalog | tested and usable | none | none |
| `search` | Search Xiaohongshu notes | tested and usable | `<keyword>` | `--page <u32>`, `--page-size <u32>`, `--sort <sort>`, `--note-type <note_type>` |

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

## 12. Maintenance Rules

When you add new CLI capabilities, update these locations together:

- `src/cli/args/*.rs`
- `locales/cli/zh-CN.json`
- `locales/cli/en-US.json`
- `docs/reference/cli-reference.md`
- `docs/reference/cli-reference.zh-CN.md`
