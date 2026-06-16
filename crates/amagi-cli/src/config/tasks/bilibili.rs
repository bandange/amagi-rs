/// Bilibili tasks exposed by the CLI runtime.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BilibiliRunTask {
    /// Fetch one video payload.
    VideoInfo {
        /// Target BVID.
        bvid: String,
    },
    /// Fetch one video stream payload.
    VideoStream {
        /// Target AVID.
        aid: u64,
        /// Target CID.
        cid: u64,
    },
    /// Fetch one danmaku segment.
    VideoDanmaku {
        /// Target CID.
        cid: u64,
        /// Optional danmaku segment index.
        segment_index: Option<u32>,
    },
    /// Fetch merged comments for one subject.
    Comments {
        /// Target subject id.
        oid: u64,
        /// Comment type code.
        comment_type: u32,
        /// Optional page size.
        number: Option<u32>,
        /// Optional sort mode.
        mode: Option<u32>,
    },
    /// Fetch replies for one root comment.
    CommentReplies {
        /// Target subject id.
        oid: u64,
        /// Comment type code.
        comment_type: u32,
        /// Root comment id.
        root: u64,
        /// Optional page size.
        number: Option<u32>,
    },
    /// Fetch one user card.
    UserCard {
        /// Target uid.
        host_mid: u64,
    },
    /// Fetch one user's dynamic list.
    UserDynamicList {
        /// Target uid.
        host_mid: u64,
    },
    /// Fetch one user's space details.
    UserSpaceInfo {
        /// Target uid.
        host_mid: u64,
    },
    /// Fetch one uploader's total views.
    UploaderTotalViews {
        /// Target uid.
        host_mid: u64,
    },
    /// Fetch one dynamic detail payload.
    DynamicDetail {
        /// Target dynamic id.
        dynamic_id: String,
    },
    /// Fetch one dynamic card payload.
    DynamicCard {
        /// Target dynamic id.
        dynamic_id: String,
    },
    /// Fetch one bangumi metadata payload.
    BangumiInfo {
        /// Target bangumi id, accepts `ep...`, `ss...`, or numeric season id.
        bangumi_id: String,
    },
    /// Fetch one bangumi stream payload.
    BangumiStream {
        /// Target episode id.
        ep_id: String,
        /// Target CID.
        cid: u64,
    },
    /// Fetch one live room detail payload.
    LiveRoomInfo {
        /// Target room id.
        room_id: u64,
    },
    /// Fetch one live room init payload.
    LiveRoomInit {
        /// Target room id.
        room_id: u64,
    },
    /// Fetch current login status.
    LoginStatus,
    /// Request a login QR code.
    LoginQrcode,
    /// Poll one login QR code.
    QrcodeStatus {
        /// QR code key returned by the generate endpoint.
        qrcode_key: String,
    },
    /// Fetch the emoji catalog.
    EmojiList,
    /// Convert one AV identifier into BV.
    AvToBv {
        /// Numeric AV identifier.
        aid: u64,
    },
    /// Convert one BV identifier into AV.
    BvToAv {
        /// Target BVID.
        bvid: String,
    },
    /// Fetch one article content payload.
    ArticleContent {
        /// Target article id.
        article_id: String,
    },
    /// Fetch article cards for a list of ids.
    ArticleCards {
        /// Target ids accepted by the upstream API.
        ids: Vec<String>,
    },
    /// Fetch one article metadata payload.
    ArticleInfo {
        /// Target article id.
        article_id: String,
    },
    /// Fetch one article-list payload.
    ArticleListInfo {
        /// Target article-list id.
        list_id: String,
    },
    /// Request a captcha challenge from one voucher.
    CaptchaFromVoucher {
        /// Target voucher value.
        v_voucher: String,
        /// Optional csrf token.
        csrf: Option<String>,
    },
    /// Validate one captcha result.
    ValidateCaptcha {
        /// Challenge returned by the captcha provider.
        challenge: String,
        /// Captcha token.
        token: String,
        /// Captcha validate field.
        validate: String,
        /// Captcha seccode field.
        seccode: String,
        /// Optional csrf token.
        csrf: Option<String>,
    },
}
