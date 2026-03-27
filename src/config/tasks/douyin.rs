/// Douyin tasks exposed by the CLI runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DouyinRunTask {
    /// Parse a work and infer its type.
    ParseWork {
        /// Aweme id of the target work.
        aweme_id: String,
    },
    /// Fetch a video work.
    VideoWork {
        /// Aweme id of the target work.
        aweme_id: String,
    },
    /// Fetch an image album work.
    ImageAlbumWork {
        /// Aweme id of the target work.
        aweme_id: String,
    },
    /// Fetch a slides work.
    SlidesWork {
        /// Aweme id of the target work.
        aweme_id: String,
    },
    /// Fetch a text work.
    TextWork {
        /// Aweme id of the target work.
        aweme_id: String,
    },
    /// Fetch work comments.
    WorkComments {
        /// Aweme id of the target work.
        aweme_id: String,
        /// Optional page size.
        number: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<u64>,
    },
    /// Fetch replies for one comment.
    CommentReplies {
        /// Aweme id of the target work.
        aweme_id: String,
        /// Comment id to expand.
        comment_id: String,
        /// Optional page size.
        number: Option<u32>,
        /// Optional pagination cursor.
        cursor: Option<u64>,
    },
    /// Fetch a user profile.
    UserProfile {
        /// Target sec_uid.
        sec_uid: String,
    },
    /// Fetch a user's public videos.
    UserVideoList {
        /// Target sec_uid.
        sec_uid: String,
        /// Optional page size.
        number: Option<u32>,
        /// Optional pagination cursor.
        max_cursor: Option<String>,
    },
    /// Fetch a user's favorites.
    UserFavoriteList {
        /// Target sec_uid.
        sec_uid: String,
        /// Optional page size.
        number: Option<u32>,
        /// Optional pagination cursor.
        max_cursor: Option<String>,
    },
    /// Fetch a user's recommendations.
    UserRecommendList {
        /// Target sec_uid.
        sec_uid: String,
        /// Optional page size.
        number: Option<u32>,
        /// Optional pagination cursor.
        max_cursor: Option<String>,
    },
    /// Search content.
    Search {
        /// Search keyword.
        query: String,
        /// Optional search type.
        search_type: Option<crate::platforms::douyin::DouyinSearchType>,
        /// Optional page size.
        number: Option<u32>,
        /// Optional search cursor id.
        search_id: Option<String>,
    },
    /// Fetch suggestion keywords.
    SuggestWords {
        /// Search keyword.
        query: String,
    },
    /// Fetch music information.
    MusicInfo {
        /// Target music id.
        music_id: String,
    },
    /// Fetch live room information.
    LiveRoomInfo {
        /// Target room id.
        room_id: String,
        /// Target web_rid.
        web_rid: String,
    },
    /// Request a login QR code.
    LoginQrcode {
        /// Optional verify_fp override.
        verify_fp: Option<String>,
    },
    /// Fetch the emoji catalog.
    EmojiList,
    /// Fetch animated emoji configuration.
    DynamicEmojiList,
    /// Fetch danmaku for one work.
    DanmakuList {
        /// Aweme id of the target work.
        aweme_id: String,
        /// Total work duration.
        duration: u64,
        /// Optional segment start.
        start_time: Option<u64>,
        /// Optional segment end.
        end_time: Option<u64>,
    },
}
