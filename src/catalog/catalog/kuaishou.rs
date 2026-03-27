use super::super::{ApiMethodSpec, HttpMethod};
use super::{TAG_COMMENT, TAG_EMOJI, TAG_LIVE, TAG_USER, TAG_WORK};

pub(super) const KUAISHOU_METHODS: [ApiMethodSpec; 6] = [
    ApiMethodSpec {
        method_key: "videoWork",
        chinese_name: "单个视频作品数据",
        fetcher_name: "fetchVideoWork",
        route: "/work/{photo_id}",
        http_method: HttpMethod::Get,
        description: "Fetch a Kuaishou video work.",
        tags: TAG_WORK,
    },
    ApiMethodSpec {
        method_key: "comments",
        chinese_name: "评论数据",
        fetcher_name: "fetchWorkComments",
        route: "/comments/{photo_id}",
        http_method: HttpMethod::Get,
        description: "Fetch Kuaishou work comments.",
        tags: TAG_COMMENT,
    },
    ApiMethodSpec {
        method_key: "userProfile",
        chinese_name: "用户主页数据",
        fetcher_name: "fetchUserProfile",
        route: "/user/{principal_id}",
        http_method: HttpMethod::Get,
        description: "Fetch a Kuaishou user profile.",
        tags: TAG_USER,
    },
    ApiMethodSpec {
        method_key: "userWorkList",
        chinese_name: "用户作品列表数据",
        fetcher_name: "fetchUserWorkList",
        route: "/user/{principal_id}/works",
        http_method: HttpMethod::Get,
        description: "Fetch Kuaishou works for a user.",
        tags: TAG_USER,
    },
    ApiMethodSpec {
        method_key: "liveRoomInfo",
        chinese_name: "直播间信息数据",
        fetcher_name: "fetchLiveRoomInfo",
        route: "/live/{principal_id}",
        http_method: HttpMethod::Get,
        description: "Fetch Kuaishou live room information.",
        tags: TAG_LIVE,
    },
    ApiMethodSpec {
        method_key: "emojiList",
        chinese_name: "Emoji数据",
        fetcher_name: "fetchEmojiList",
        route: "/emoji",
        http_method: HttpMethod::Get,
        description: "Fetch the Kuaishou emoji catalog.",
        tags: TAG_EMOJI,
    },
];
