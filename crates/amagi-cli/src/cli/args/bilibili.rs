#![allow(missing_docs)]

use clap::Subcommand;

/// Bilibili tasks exposed through the CLI.
#[derive(Debug, Subcommand, Clone)]
pub enum BilibiliCommand {
    /// Fetch one Bilibili video payload.
    #[command(name = "video-info")]
    VideoInfo { bvid: String },
    /// Fetch one Bilibili video stream payload.
    #[command(name = "video-stream")]
    VideoStream {
        aid: u64,
        #[arg(long)]
        cid: u64,
    },
    /// Fetch one Bilibili video danmaku segment.
    #[command(name = "video-danmaku")]
    VideoDanmaku {
        cid: u64,
        #[arg(long)]
        segment_index: Option<u32>,
    },
    /// Fetch comments for one Bilibili subject.
    #[command(name = "comments")]
    Comments {
        oid: u64,
        #[arg(long = "type")]
        comment_type: u32,
        #[arg(long)]
        number: Option<u32>,
        #[arg(long)]
        mode: Option<u32>,
    },
    /// Fetch replies for one Bilibili root comment.
    #[command(name = "comment-replies")]
    CommentReplies {
        oid: u64,
        root: u64,
        #[arg(long = "type")]
        comment_type: u32,
        #[arg(long)]
        number: Option<u32>,
    },
    /// Fetch one Bilibili user card.
    #[command(name = "user-card")]
    UserCard { host_mid: u64 },
    /// Fetch one Bilibili user dynamic list.
    #[command(name = "user-dynamic-list")]
    UserDynamicList { host_mid: u64 },
    /// Fetch one Bilibili user space payload.
    #[command(name = "user-space-info")]
    UserSpaceInfo { host_mid: u64 },
    /// Fetch one uploader's total views.
    #[command(name = "uploader-total-views")]
    UploaderTotalViews { host_mid: u64 },
    /// Fetch one Bilibili dynamic detail payload.
    #[command(name = "dynamic-detail")]
    DynamicDetail { dynamic_id: String },
    /// Fetch one Bilibili dynamic card payload.
    #[command(name = "dynamic-card")]
    DynamicCard { dynamic_id: String },
    /// Fetch one Bilibili bangumi metadata payload.
    #[command(name = "bangumi-info")]
    BangumiInfo { bangumi_id: String },
    /// Fetch one Bilibili bangumi stream payload.
    #[command(name = "bangumi-stream")]
    BangumiStream {
        ep_id: String,
        #[arg(long)]
        cid: u64,
    },
    /// Fetch one Bilibili live room detail payload.
    #[command(name = "live-room-info")]
    LiveRoomInfo { room_id: u64 },
    /// Fetch one Bilibili live room init payload.
    #[command(name = "live-room-init")]
    LiveRoomInit { room_id: u64 },
    /// Fetch the current Bilibili login status.
    #[command(name = "login-status")]
    LoginStatus,
    /// Request a Bilibili login QR code.
    #[command(name = "login-qrcode")]
    LoginQrcode,
    /// Poll one Bilibili login QR code.
    #[command(name = "qrcode-status")]
    QrcodeStatus { qrcode_key: String },
    /// Fetch the Bilibili emoji catalog.
    #[command(name = "emoji-list")]
    EmojiList,
    /// Convert one AV identifier into BV.
    #[command(name = "av-to-bv")]
    AvToBv { aid: u64 },
    /// Convert one BV identifier into AV.
    #[command(name = "bv-to-av")]
    BvToAv { bvid: String },
    /// Fetch one Bilibili article content payload.
    #[command(name = "article-content")]
    ArticleContent { article_id: String },
    /// Fetch Bilibili article cards for one or more ids.
    #[command(name = "article-cards")]
    ArticleCards { ids: Vec<String> },
    /// Fetch one Bilibili article metadata payload.
    #[command(name = "article-info")]
    ArticleInfo { article_id: String },
    /// Fetch one Bilibili article-list payload.
    #[command(name = "article-list-info")]
    ArticleListInfo { list_id: String },
    /// Request a Bilibili captcha challenge from one voucher.
    #[command(name = "captcha-from-voucher")]
    CaptchaFromVoucher {
        v_voucher: String,
        #[arg(long)]
        csrf: Option<String>,
    },
    /// Validate one Bilibili captcha result.
    #[command(name = "validate-captcha")]
    ValidateCaptcha {
        challenge: String,
        token: String,
        validate: String,
        seccode: String,
        #[arg(long)]
        csrf: Option<String>,
    },
}
