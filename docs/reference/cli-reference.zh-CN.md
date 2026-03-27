# Amagi CLI 完整命令与参数参考

英文版：[`cli-reference.md`](cli-reference.md)

本文档覆盖当前 `amagi` CLI 的完整能力清单，包括：

- 顶层命令
- 全局参数
- `run` 子命令下的全部平台任务
- 每个任务的全部参数
- 参数可选值、默认值与环境变量来源

## 1. 调用入口

标准调用形式：

```bash
amagi [GLOBAL_OPTIONS] [COMMAND]
```

如果省略顶层 `COMMAND`，默认等价于：

```bash
amagi run
```

`serve` 子命令只在启用 `server` 功能时可用。

## 2. 顶层能力总览

| 命令 | 说明 |
| --- | --- |
| `run` | 运行本地 CLI 工作流 |
| `serve` | 启动内置 Web API 服务 |

`run` 下当前已接入的平台：

| 平台 | 任务数 | 说明 |
| --- | --- | --- |
| `bilibili` | 26 | 视频、评论、动态、番剧、直播、登录二维码、专栏、验证码、表情 |
| `douyin` | 19 | 作品、评论、用户、搜索、音乐、直播、登录二维码、表情、弹幕 |
| `kuaishou` | 6 | 作品、评论、用户主页、用户作品、直播、表情 |
| `twitter` | 17 | 搜索、用户主页、时间线、回复、媒体、关注关系、登录态时间线、推文、Space |
| `xiaohongshu` | 7 | 首页流、笔记、评论、用户、搜索、表情 |

## 3. 全局参数

全局参数可放在顶层命令前，也可以放在子命令后。

| 参数 | 值 | 默认值 | 环境变量 / `.env` | 说明 |
| --- | --- | --- | --- | --- |
| `--lang <LANG>` | `zh` / `en` | 自动解析系统语言，回退到 `en` | `AMAGI_LANG` | CLI 帮助语言 |
| `--output <OUTPUT>` | `text` / `json` | `text` | `AMAGI_OUTPUT` | CLI 输出格式 |
| `--output-file`, `-o <OUTPUT_FILE>` | 路径字符串 | 无 | `AMAGI_OUTPUT_FILE` | 将 CLI 输出写入文件 |
| `--pretty` | 布尔开关 | `false` | `AMAGI_OUTPUT_PRETTY` | JSON 输出使用缩进格式 |
| `--append` | 布尔开关 | `false` | `AMAGI_OUTPUT_APPEND` | 向 `--output-file` 追加写入 |
| `--create-parent-dirs` | 布尔开关 | `false` | `AMAGI_OUTPUT_CREATE_DIRS` | 自动创建输出文件父目录 |
| `--douyin-cookie <DOUYIN_COOKIE>` | Cookie 字符串 | 无 | `AMAGI_DOUYIN_COOKIE` | 抖音 Cookie |
| `--bilibili-cookie <BILIBILI_COOKIE>` | Cookie 字符串 | 无 | `AMAGI_BILIBILI_COOKIE` | Bilibili Cookie |
| `--kuaishou-cookie <KUAISHOU_COOKIE>` | Cookie 字符串 | 无 | `AMAGI_KUAISHOU_COOKIE` | 快手 Cookie |
| `--xiaohongshu-cookie <XIAOHONGSHU_COOKIE>` | Cookie 字符串 | 无 | `AMAGI_XIAOHONGSHU_COOKIE` | 小红书 Cookie |
| `--twitter-cookie <TWITTER_COOKIE>` | Cookie 字符串 | 无 | `AMAGI_TWITTER_COOKIE` | Twitter/X Cookie |
| `--timeout-ms <TIMEOUT_MS>` | 无符号整数 | `10000` | `AMAGI_TIMEOUT_MS` | 上游请求超时，单位毫秒 |
| `--max-retries <MAX_RETRIES>` | 无符号整数 | `3` | `AMAGI_MAX_RETRIES` | 可恢复请求失败的最大重试次数 |
| `--log-format <LOG_FORMAT>` | `text` / `json` | `text` | `AMAGI_LOG_FORMAT` | 日志输出格式 |
| `--log-level <LOG_LEVEL>` | `error` / `warn` / `info` / `debug` / `trace` | `info` | `AMAGI_LOG` | 最低日志级别 |
| `--help`, `-h` | 无 | 无 | 无 | 显示帮助 |
| `--version`, `-V` | 无 | 无 | 无 | 显示版本 |

## 4. `run` 子命令

调用形式：

```bash
amagi run [--quiet] [PLATFORM] [TASK]
```

### 4.1 `run` 级参数

| 参数 | 值 | 默认值 | 说明 |
| --- | --- | --- | --- |
| `--quiet` | 布尔开关 | `false` | 关闭常规启动输出 |

### 4.2 平台子命令

| 平台命令 | 说明 |
| --- | --- |
| `bilibili` | 运行 Bilibili 专用任务 |
| `douyin` | 运行抖音专用任务 |
| `kuaishou` | 运行快手专用任务 |
| `twitter` | 运行 Twitter/X 专用任务 |
| `xiaohongshu` | 运行小红书专用任务 |

## 5. Bilibili 命令参考

调用前缀：

```bash
amagi run bilibili <TASK> ...
```

| 任务 | 说明 | 状态 | 必填位置参数 | 可选参数 |
| --- | --- | --- | --- | --- |
| `video-info` | 获取单个 Bilibili 视频详情 | 已测试可用 | `<bvid>` | 无 |
| `video-stream` | 获取单个 Bilibili 视频流信息 | 已测试可用 | `<aid>` | `--cid <u64>` 必填命名参数 |
| `video-danmaku` | 获取单个 Bilibili 视频弹幕分段 | 已测试可用 | `<cid>` | `--segment-index <u32>` |
| `comments` | 获取单个 Bilibili 内容的评论列表 | 已测试可用 | `<oid>` | `--type <u32>` 必填，`--number <u32>`，`--mode <u32>` |
| `comment-replies` | 获取单个 Bilibili 根评论的回复列表 | 已测试可用 | `<oid> <root>` | `--type <u32>` 必填，`--number <u32>` |
| `user-card` | 获取单个 Bilibili 用户卡片 | 已测试可用 | `<host_mid>` | 无 |
| `user-dynamic-list` | 获取单个 Bilibili 用户动态列表 | 已测试可用 | `<host_mid>` | 无 |
| `user-space-info` | 获取单个 Bilibili 用户空间信息 | 已测试可用 | `<host_mid>` | 无 |
| `uploader-total-views` | 获取单个 UP 主的总播放量 | 已测试可用 | `<host_mid>` | 无 |
| `dynamic-detail` | 获取单个 Bilibili 动态详情 | 已测试可用 | `<dynamic_id>` | 无 |
| `dynamic-card` | 获取单个 Bilibili 动态卡片 | 已测试可用 | `<dynamic_id>` | 无 |
| `bangumi-info` | 获取单个 Bilibili 番剧元数据 | 未单独复测 | `<bangumi_id>` | 无 |
| `bangumi-stream` | 获取单个 Bilibili 番剧播放流 | 未单独复测 | `<ep_id>` | `--cid <u64>` 必填命名参数 |
| `live-room-info` | 获取单个 Bilibili 直播间详情 | 未单独复测 | `<room_id>` | 无 |
| `live-room-init` | 获取单个 Bilibili 直播间初始化信息 | 未单独复测 | `<room_id>` | 无 |
| `login-status` | 获取当前 Bilibili 登录状态 | 已测试可用 | 无 | 无 |
| `login-qrcode` | 请求一个 Bilibili 登录二维码 | 已测试可用 | 无 | 无 |
| `qrcode-status` | 轮询一个 Bilibili 登录二维码状态 | 未单独复测 | `<qrcode_key>` | 无 |
| `emoji-list` | 获取 Bilibili 表情列表 | 已测试可用 | 无 | 无 |
| `av-to-bv` | 将 AV 号转换为 BV 号 | 已测试可用 | `<aid>` | 无 |
| `bv-to-av` | 将 BV 号转换为 AV 号 | 已测试可用 | `<bvid>` | 无 |
| `article-content` | 获取单个 Bilibili 专栏正文 | 未单独复测 | `<article_id>` | 无 |
| `article-cards` | 获取一个或多个 Bilibili 专栏卡片 | 未单独复测 | `<ids>...` | 无 |
| `article-info` | 获取单个 Bilibili 专栏元数据 | 未单独复测 | `<article_id>` | 无 |
| `article-list-info` | 获取单个 Bilibili 文集信息 | 未单独复测 | `<list_id>` | 无 |
| `captcha-from-voucher` | 根据验证码凭证请求 Bilibili 验证挑战 | 未单独复测 | `<v_voucher>` | `--csrf <string>` |
| `validate-captcha` | 提交并验证一个 Bilibili 验证码结果 | 未单独复测 | `<challenge> <token> <validate> <seccode>` | `--csrf <string>` |

### 5.1 Bilibili 参数说明

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| `bvid` | 字符串 | 视频 BV 号 |
| `aid` | `u64` | 视频 AV 号 |
| `cid` | `u64` | 视频或番剧的内容 ID |
| `segment_index` | `u32` | 弹幕分段索引 |
| `oid` | `u64` | 评论目标对象 ID |
| `comment_type` | `u32` | 评论类型 |
| `number` | `u32` | 评论条数 |
| `mode` | `u32` | 评论模式 |
| `root` | `u64` | 根评论 ID |
| `host_mid` | `u64` | 用户 mid |
| `dynamic_id` | 字符串 | 动态 ID |
| `bangumi_id` | 字符串 | 番剧 ID |
| `ep_id` | 字符串 | 剧集 ID |
| `room_id` | `u64` | 直播间 ID |
| `qrcode_key` | 字符串 | 登录二维码 key |
| `article_id` | 字符串 | 专栏 ID |
| `ids` | 字符串数组 | 一个或多个专栏 ID |
| `list_id` | 字符串 | 文集 ID |
| `v_voucher` | 字符串 | 验证码凭证 |
| `csrf` | 字符串 | 可选的 csrf token |
| `challenge` | 字符串 | 验证 challenge |
| `token` | 字符串 | 验证 token |
| `validate` | 字符串 | 验证 validate 值 |
| `seccode` | 字符串 | 验证 seccode |

## 6. 抖音命令参考

调用前缀：

```bash
amagi run douyin <TASK> ...
```

| 任务 | 说明 | 状态 | 必填位置参数 | 可选参数 |
| --- | --- | --- | --- | --- |
| `parse-work` | 解析单个抖音作品 | 已测试可用 | `<aweme_id>` | 无 |
| `video-work` | 获取单个抖音视频作品 | 已测试可用 | `<aweme_id>` | 无 |
| `image-album-work` | 获取单个抖音图文作品 | 已测试可用 | `<aweme_id>` | 无 |
| `slides-work` | 获取单个抖音图集作品 | 已测试可用 | `<aweme_id>` | 无 |
| `text-work` | 获取单个抖音文字作品 | 已测试可用 | `<aweme_id>` | 无 |
| `work-comments` | 获取单个抖音作品的评论列表 | 已测试可用 | `<aweme_id>` | `--number <u32>`，`--cursor <u64>` |
| `comment-replies` | 获取单个抖音评论的回复列表 | 已测试可用 | `<aweme_id> <comment_id>` | `--number <u32>`，`--cursor <u64>` |
| `user-profile` | 获取单个抖音用户资料 | 已测试可用 | `<sec_uid>` | 无 |
| `user-video-list` | 获取单个抖音用户的视频列表 | 已测试可用 | `<sec_uid>` | `--number <u32>`，`--max-cursor <string>` |
| `user-favorite-list` | 获取单个抖音用户的收藏列表 | 已测试，当前实现失败 | `<sec_uid>` | `--number <u32>`，`--max-cursor <string>` |
| `user-recommend-list` | 获取单个抖音用户的推荐列表 | 已测试，当前实现失败 | `<sec_uid>` | `--number <u32>`，`--max-cursor <string>` |
| `search` | 搜索抖音内容 | 已测试可用 | `<query>` | `--type <search_type>`，`--number <u32>`，`--search-id <string>` |
| `suggest-words` | 获取抖音搜索联想词 | 已测试可用 | `<query>` | 无 |
| `music-info` | 获取抖音音乐信息 | 已测试可用 | `<music_id>` | 无 |
| `live-room-info` | 获取抖音直播间信息 | 已测试可用 | `<room_id>` | `--web-rid <string>` 必填命名参数 |
| `login-qrcode` | 请求抖音登录二维码 | 已测试，当前实现失败 | 无 | `--verify-fp <string>` |
| `emoji-list` | 获取抖音表情列表 | 已测试可用 | 无 | 无 |
| `dynamic-emoji-list` | 获取抖音动态表情列表 | 已测试可用 | 无 | 无 |
| `danmaku-list` | 获取抖音弹幕列表 | 已测试，有已知偏差 | `<aweme_id>` | `--duration <u64>` 必填，`--start-time <u64>`，`--end-time <u64>` |

### 6.1 抖音枚举参数

| 参数 | 可选值 |
| --- | --- |
| `search_type` | `general`、`user`、`video` |

### 6.2 抖音参数说明

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| `aweme_id` | 字符串 | 作品 ID |
| `comment_id` | 字符串 | 评论 ID |
| `sec_uid` | 字符串 | 用户 `sec_uid` |
| `number` | `u32` | 每页数量 |
| `cursor` | `u64` | 分页游标 |
| `max_cursor` | 字符串 | 用户列表分页游标 |
| `query` | 字符串 | 搜索关键词 |
| `search_id` | 字符串 | 搜索游标 ID |
| `music_id` | 字符串 | 音乐 ID |
| `room_id` | 字符串 | 直播间 ID |
| `web_rid` | 字符串 | 直播间 `web_rid` |
| `verify_fp` | 字符串 | 登录二维码使用的 `verify_fp` 覆盖值 |
| `duration` | `u64` | 作品时长，毫秒 |
| `start_time` | `u64` | 弹幕起始时间 |
| `end_time` | `u64` | 弹幕结束时间 |

## 7. 快手命令参考

调用前缀：

```bash
amagi run kuaishou <TASK> ...
```

| 任务 | 说明 | 状态 | 必填位置参数 | 可选参数 |
| --- | --- | --- | --- | --- |
| `video-work` | 获取单个快手作品 | 已测试可用 | `<photo_id>` | 无 |
| `work-comments` | 获取单个快手作品的评论列表 | 已测试可用 | `<photo_id>` | 无 |
| `emoji-list` | 获取快手表情列表 | 已测试可用 | 无 | 无 |
| `user-profile` | 获取单个快手用户资料 | 已测试可用 | `<principal_id>` | 无 |
| `user-work-list` | 获取单个快手用户的作品列表 | 已测试可用 | `<principal_id>` | `--pcursor <string>`，`--count <u32>` |
| `live-room-info` | 获取快手直播间信息 | 已测试可用 | `<principal_id>` | 无 |

### 7.1 快手参数说明

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| `photo_id` | 字符串 | 作品 ID |
| `principal_id` | 字符串 | 用户或直播间 `principal_id` |
| `pcursor` | 字符串 | 分页游标 |
| `count` | `u32` | 请求数量 |

## 8. Twitter / X 命令参考

调用前缀：

```bash
amagi run twitter <TASK> ...
```

| 任务 | 说明 | 状态 | 必填位置参数 | 可选参数 |
| --- | --- | --- | --- | --- |
| `search-tweets` | 搜索 Twitter/X 推文 | 已测试可用 | `<query>` | `--search-type <mode>`，`--count <u32>`，`--cursor <string>` |
| `search-users` | 搜索 Twitter/X 用户 | 已测试可用 | `<query>` | `--count <u32>`，`--cursor <string>` |
| `user-profile` | 获取单个 Twitter/X 用户资料 | 已测试可用 | `<screen_name>` | 无 |
| `user-timeline` | 获取单个 Twitter/X 用户时间线 | 已测试可用 | `<screen_name>` | `--count <u32>`，`--cursor <string>` |
| `user-replies` | 获取单个 Twitter/X 用户回复流 | 已测试，有已知偏差 | `<screen_name>` | `--count <u32>`，`--cursor <string>` |
| `user-media` | 获取单个 Twitter/X 用户媒体流 | 已测试，有已知偏差 | `<screen_name>` | `--count <u32>`，`--cursor <string>` |
| `user-followers` | 获取单个 Twitter/X 用户粉丝列表 | 已测试可用 | `<screen_name>` | `--count <u32>`，`--cursor <string>` |
| `user-following` | 获取单个 Twitter/X 用户关注列表 | 已测试可用 | `<screen_name>` | `--count <u32>`，`--cursor <string>` |
| `user-likes` | 获取当前已登录 Twitter/X 账户的点赞列表 | 已测试可用 | 无 | `--count <u32>`，`--cursor <string>` |
| `user-bookmarks` | 获取当前已登录 Twitter/X 账户的书签列表 | 已测试可用 | 无 | `--count <u32>`，`--cursor <string>` |
| `user-followed` | 获取当前已登录 Twitter/X 账户的关注时间线 | 已测试可用 | 无 | `--count <u32>`，`--cursor <string>` |
| `user-recommended` | 获取当前已登录 Twitter/X 账户的推荐时间线 | 已测试可用 | 无 | `--count <u32>`，`--cursor <string>` |
| `tweet-detail` | 获取单条 Twitter/X 推文详情 | 已测试可用 | `<tweet_id>` | 无 |
| `tweet-replies` | 获取单条 Twitter/X 推文的回复列表 | 已测试，有已知偏差 | `<tweet_id>` | `--cursor <string>`，`--sort-by <sort_by>` |
| `tweet-likers` | 获取点赞单条 Twitter/X 推文的用户列表 | 已测试可用 | `<tweet_id>` | `--count <u32>`，`--cursor <string>` |
| `tweet-retweeters` | 获取转推单条 Twitter/X 推文的用户列表 | 已测试可用 | `<tweet_id>` | `--count <u32>`，`--cursor <string>` |
| `space-detail` | 获取单个 Twitter/X Space 详情 | 未单独复测 | `<space_id>` | 无 |

CLI 实测状态说明：

- `已测试可用`：命令链路已实际验证，可作为当前可用接口使用
- `已测试，有已知偏差`：命令能跑通，但当前结果与命令语义还存在偏差
- `已测试，需要有效登录态`：命令链路已实际执行，但当前环境缺少可用登录态，接口返回上游鉴权失败
- `已测试，当前实现失败`：本轮已经实际执行，但当前实现报错，修复代码前不可作为可用接口使用
- `未单独复测`：文档已列出接口，但这轮 CLI 样例未单独覆盖

当前已知偏差：

- `user-replies`：当前结果会混入普通推文，不是严格的“仅回复”列表
- `danmaku-list`：已对 `danmaku_cnt = 1` 的抖音作品实测，当前响应仍返回 `total = 0` 且没有弹幕条目
- `user-media`：当前结果里可能包含 `media: []` 的项目，媒体过滤或媒体实体解析仍需收敛
- `tweet-replies`：当前结果会包含根推文本身，不是严格的“仅回复”列表

### 8.1 Twitter / X 枚举参数

| 参数 | 可选值 |
| --- | --- |
| `search_type` | `latest`、`top` |

### 8.2 Twitter / X 参数说明

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| `query` | 字符串 | 搜索关键词 |
| `search_type` | 枚举 | 搜索模式 |
| `count` | `u32` | 请求数量 |
| `cursor` | 字符串 | 分页游标 |
| `screen_name` | 字符串 | 用户 `screen_name` |
| `tweet_id` | 字符串 | 推文 ID |
| `space_id` | 字符串 | Space ID |

## 9. 小红书命令参考

调用前缀：

```bash
amagi run xiaohongshu <TASK> ...
```

| 任务 | 说明 | 状态 | 必填位置参数 | 可选参数 |
| --- | --- | --- | --- | --- |
| `home-feed` | 获取小红书首页推荐流 | 已测试可用 | 无 | `--cursor-score <string>`，`--num <u32>`，`--refresh-type <u32>`，`--note-index <u32>`，`--category <string>`，`--search-key <string>` |
| `note-detail` | 获取单个小红书笔记详情 | 已测试可用 | `<note_id>` | `--xsec-token <string>` 必填命名参数 |
| `note-comments` | 获取单个小红书笔记的评论列表 | 已测试可用 | `<note_id>` | `--xsec-token <string>` 必填，`--cursor <string>` |
| `user-profile` | 获取单个小红书用户资料 | 已测试可用 | `<user_id>` | `--xsec-token <string>` 必填，`--xsec-source <string>` 可选 |
| `user-note-list` | 获取单个小红书用户的笔记列表 | 已测试可用 | `<user_id>` | `--xsec-token <string>` 必填，`--xsec-source <string>` 可选，`--cursor <string>`，`--num <u32>` |
| `emoji-list` | 获取小红书表情列表 | 已测试可用 | 无 | 无 |
| `search` | 搜索小红书笔记 | 已测试可用 | `<keyword>` | `--page <u32>`，`--page-size <u32>`，`--sort <sort>`，`--note-type <note_type>` |

### 9.1 小红书枚举参数

| 参数 | 可选值 |
| --- | --- |
| `sort` | `general`、`time_descending`、`popularity_descending` |
| `note_type` | `all`、`video`、`image` |

### 9.2 小红书参数说明

| 参数名 | 类型 | 说明 |
| --- | --- | --- |
| `cursor_score` | 字符串 | 首页流游标分数 |
| `num` | `u32` | 请求数量 |
| `refresh_type` | `u32` | 刷新类型 |
| `note_index` | `u32` | 笔记索引 |
| `category` | 字符串 | feed 分类 |
| `search_key` | 字符串 | feed 搜索关键字 |
| `note_id` | 字符串 | 笔记 ID |
| `xsec_token` | 字符串 | 笔记 `xsec_token` |
| `cursor` | 字符串 | 分页游标 |
| `user_id` | 字符串 | 用户 ID |
| `keyword` | 字符串 | 搜索关键词 |
| `page` | `u32` | 页码 |
| `page_size` | `u32` | 每页数量 |

## 10. `serve` 命令参考

调用形式：

```bash
amagi serve [OPTIONS]
```

| 参数 | 值 | 默认值 | 环境变量 / `.env` | 说明 |
| --- | --- | --- | --- | --- |
| `--host <HOST>` | 主机名或 IP | `127.0.0.1` | `AMAGI_HOST` | 服务绑定地址 |
| `--port <PORT>` | `u16` | `4567` | `AMAGI_PORT` | 服务绑定端口 |

## 11. 环境变量总表

dotenv、进程环境变量和显式命令行参数的优先级为：

```text
命令行参数 > 进程环境变量 > 当前目录 .env > 用户级配置 .env > 内置默认值
```

用户级配置 dotenv 路径：

- Linux/macOS：`~/.config/amagi/.env`
- Windows：`%APPDATA%\\amagi\\.env`

覆盖变量：

- `AMAGI_USER_ENV_FILE`

当前支持的变量如下：

| 变量名 | 作用 |
| --- | --- |
| `AMAGI_LANG` | CLI 帮助语言 |
| `AMAGI_OUTPUT` | CLI 输出格式 |
| `AMAGI_USER_ENV_FILE` | 覆盖用户级 dotenv 路径 |
| `AMAGI_OUTPUT_FILE` | CLI 输出文件路径 |
| `AMAGI_OUTPUT_PRETTY` | JSON 美化输出 |
| `AMAGI_OUTPUT_APPEND` | 输出文件追加模式 |
| `AMAGI_OUTPUT_CREATE_DIRS` | 自动创建输出目录 |
| `AMAGI_DOUYIN_COOKIE` | 抖音 Cookie |
| `AMAGI_BILIBILI_COOKIE` | Bilibili Cookie |
| `AMAGI_KUAISHOU_COOKIE` | 快手 Cookie |
| `AMAGI_XIAOHONGSHU_COOKIE` | 小红书 Cookie |
| `AMAGI_TWITTER_COOKIE` | Twitter/X Cookie |
| `AMAGI_TIMEOUT_MS` | 请求超时 |
| `AMAGI_MAX_RETRIES` | 最大重试次数 |
| `AMAGI_LOG_FORMAT` | 日志格式 |
| `AMAGI_LOG` | 日志级别 |
| `AMAGI_HOST` | 服务端监听地址 |
| `AMAGI_PORT` | 服务端监听端口 |

## 12. 维护规则

新增 CLI 能力时，同步更新以下位置：

- `src/cli/args/*.rs`
- `locales/cli/zh-CN.json`
- `locales/cli/en-US.json`
- `docs/reference/cli-reference.md`
- `docs/reference/cli-reference.zh-CN.md`
