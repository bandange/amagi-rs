#![allow(missing_docs)]

use clap::Subcommand;

/// Douyin tasks exposed through the CLI.
#[derive(Debug, Subcommand, Clone)]
pub enum DouyinCommand {
    #[command(name = "parse-work")]
    ParseWork { aweme_id: String },
    #[command(name = "video-work")]
    VideoWork { aweme_id: String },
    #[command(name = "image-album-work")]
    ImageAlbumWork { aweme_id: String },
    #[command(name = "slides-work")]
    SlidesWork { aweme_id: String },
    #[command(name = "text-work")]
    TextWork { aweme_id: String },
    #[command(name = "work-comments")]
    WorkComments {
        aweme_id: String,
        #[arg(long)]
        number: Option<u32>,
        #[arg(long)]
        cursor: Option<u64>,
    },
    #[command(name = "comment-replies")]
    CommentReplies {
        aweme_id: String,
        comment_id: String,
        #[arg(long)]
        number: Option<u32>,
        #[arg(long)]
        cursor: Option<u64>,
    },
    #[command(name = "user-profile")]
    UserProfile { sec_uid: String },
    #[command(name = "user-video-list")]
    UserVideoList {
        sec_uid: String,
        #[arg(long)]
        number: Option<u32>,
        #[arg(long)]
        max_cursor: Option<String>,
    },
    #[command(name = "user-favorite-list")]
    UserFavoriteList {
        sec_uid: String,
        #[arg(long)]
        number: Option<u32>,
        #[arg(long)]
        max_cursor: Option<String>,
    },
    #[command(name = "user-recommend-list")]
    UserRecommendList {
        sec_uid: String,
        #[arg(long)]
        number: Option<u32>,
        #[arg(long)]
        max_cursor: Option<String>,
    },
    #[command(name = "search")]
    Search {
        query: String,
        #[arg(long = "type", value_enum)]
        search_type: Option<crate::platforms::douyin::DouyinSearchType>,
        #[arg(long)]
        number: Option<u32>,
        #[arg(long)]
        search_id: Option<String>,
    },
    #[command(name = "suggest-words")]
    SuggestWords { query: String },
    #[command(name = "music-info")]
    MusicInfo { music_id: String },
    #[command(name = "live-room-info")]
    LiveRoomInfo {
        room_id: String,
        #[arg(long)]
        web_rid: String,
    },
    #[command(name = "login-qrcode")]
    LoginQrcode {
        #[arg(long)]
        verify_fp: Option<String>,
    },
    #[command(name = "emoji-list")]
    EmojiList,
    #[command(name = "dynamic-emoji-list")]
    DynamicEmojiList,
    #[command(name = "danmaku-list")]
    DanmakuList {
        aweme_id: String,
        #[arg(long)]
        duration: u64,
        #[arg(long)]
        start_time: Option<u64>,
        #[arg(long)]
        end_time: Option<u64>,
    },
}
