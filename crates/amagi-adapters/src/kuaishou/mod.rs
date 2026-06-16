//! Rust-native Kuaishou platform fetchers.

mod api;
mod bound;
mod fetcher;
/// Pure Rust Kuaishou signing algorithms and request helpers.
pub mod sign;
mod types;

pub use api::{KuaishouApiUrls, KuaishouGraphqlRequest, create_kuaishou_api_urls};
pub use bound::{BoundKuaishouFetcher, create_bound_kuaishou_fetcher};
pub use fetcher::KuaishouFetcher;
pub use sign::{
    KuaishouGeneratedHxfalcon, KuaishouHeContext, KuaishouHeResult, KuaishouHudrContext,
    KuaishouHudrResult, KuaishouHxfalconPayload, KuaishouJsonObject, KuaishouJsonValue,
    KuaishouLiveApiMethod, KuaishouLiveApiRequest, KuaishouLiveApiSignature,
    KuaishouPureRuntimeState, KuaishouPureSignContext, KuaishouPureSignResult, KuaishouSecsState,
    build_kuaishou_hudr_info_cache, build_kuaishou_hudr_payload, build_kuaishou_hxfalcon_payload,
    build_kuaishou_hxfalcon_sign_input, bytes_to_lower_hex, compute_kuaishou_lrc_hex,
    derive_kuaishou_anonymous_kww, derive_kuaishou_b2has, derive_kuaishou_b2sa,
    derive_kuaishou_cts, derive_kuaishou_he_hash_field_hex, derive_kuaishou_he_hex,
    derive_kuaishou_hudr_body, derive_kuaishou_kww, derive_kuaishou_pure_signature,
    derive_kuaishou_secs_stack_tail, derive_kuaishou_secs_state, extract_cookie_value,
    generate_hxfalcon_from_payload, generate_kww, get_cat_version, get_kuaishou_pure_runtime_state,
    hex_to_signed_bytes, mask_kuaishou_hudr_payload, resolve_kuaishou_hxfalcon_sign_path,
    sign_live_api_request, sign_live_api_url, to_little_endian_hex, transform_kuaishou_he_hex,
    xor_byte_arrays,
};
pub use types::{
    KuaishouBannedStatus, KuaishouCategoryMask, KuaishouFollowButtonState, KuaishouFollowState,
    KuaishouLiveRoomEmojiState, KuaishouLiveRoomGameInfo, KuaishouLiveRoomInfo,
    KuaishouLiveRoomPlayItem, KuaishouLiveStreamInfo, KuaishouUserProfile,
    KuaishouUserProfileAuthor, KuaishouUserProfilePage, KuaishouUserProfilePublicTabData,
    KuaishouUserProfileTabData, KuaishouUserProfileUserInfo, KuaishouVerifiedStatus,
};
pub use types::{
    KuaishouEmojiCatalog, KuaishouEmojiList, KuaishouEmojiListData, KuaishouUserWorkList,
    KuaishouVideoWork, KuaishouVideoWorkData, KuaishouWorkComments, KuaishouWorkCommentsData,
};
