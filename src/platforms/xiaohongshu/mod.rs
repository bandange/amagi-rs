//! Rust-native Xiaohongshu fetchers and pure-protocol signing helpers.

mod api;
mod bound;
mod config;
mod fetcher;
mod ordered_json;
mod session;
pub mod sign;
mod types;
mod utils;

pub use api::{
    XiaohongshuApiUrls, XiaohongshuCommentsOptions, XiaohongshuHomeFeedOptions,
    XiaohongshuNoteDetailOptions, XiaohongshuRequestSpec, XiaohongshuSearchNotesOptions,
    XiaohongshuUserNotesOptions, XiaohongshuUserProfileOptions, create_xiaohongshu_api_urls,
};
pub use bound::{BoundXiaohongshuFetcher, create_bound_xiaohongshu_fetcher};
pub use fetcher::XiaohongshuFetcher;
pub use ordered_json::OrderedJson;
pub use session::{XiaohongshuSession, XiaohongshuSignState};
pub use sign::{
    CookieJar, XiaohongshuBrowserState, XiaohongshuHeaders, XiaohongshuMethod,
    XiaohongshuMnsv2Input, XiaohongshuSigner, XiaohongshuXsCommonPayload, XiaohongshuXsEnvelope,
};
pub use types::{
    XiaohongshuComment, XiaohongshuCommentPicture, XiaohongshuEmojiCollection,
    XiaohongshuEmojiItem, XiaohongshuEmojiList, XiaohongshuFeedItem, XiaohongshuFeedNoteCard,
    XiaohongshuHomeFeed, XiaohongshuImageAsset, XiaohongshuImageInfo, XiaohongshuInteractInfo,
    XiaohongshuJsonResponse, XiaohongshuNoteComments, XiaohongshuNoteDetail, XiaohongshuNoteTag,
    XiaohongshuSearchItem, XiaohongshuSearchNoteType, XiaohongshuSearchNotes,
    XiaohongshuSearchSortType, XiaohongshuStatusResult, XiaohongshuSubComment,
    XiaohongshuUserNoteList, XiaohongshuUserProfile, XiaohongshuUserProfileBasicInfo,
    XiaohongshuUserSummary,
};

use crate::platforms::internal::random::{PseudoRandom, now_unix_ms};

/// Extract the `a1` cookie value from a full Xiaohongshu cookie string.
pub fn extract_a1_from_cookie(cookie: &str) -> Option<String> {
    CookieJar::parse(cookie).get("a1").map(str::to_owned)
}

/// Generate the Xiaohongshu `x-t` header value from a millisecond timestamp.
pub fn generate_x_t(timestamp_ms: u64) -> u64 {
    timestamp_ms
}

/// Generate a Xiaohongshu-style `x-b3-traceid`.
pub fn generate_x_b3_trace_id() -> String {
    let mut signer = XiaohongshuSigner::new();
    signer.get_b3_trace_id()
}

/// Generate a Xiaohongshu-style `x-xray-traceid`.
pub fn generate_x_xray_trace_id() -> String {
    let mut signer = XiaohongshuSigner::new();
    signer.get_xray_trace_id(None, None)
}

/// Generate the Xiaohongshu search id used by the original API layer.
pub fn generate_search_id(timestamp_ms: u64, random_value: u32) -> String {
    utils::generate_search_id(timestamp_ms, random_value)
}

/// Generate a runtime Xiaohongshu search id with local randomness.
pub fn generate_runtime_search_id() -> String {
    let mut random = PseudoRandom::from_system();
    utils::generate_search_id(now_unix_ms(), random.next_u32())
}
