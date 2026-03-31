use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use super::super::state::AppState;
use super::support::CatalogResult;
use super::{
    ApiCatalogResponse, CatalogErrorResponse, HealthResponse, PlatformCatalogResponse,
    PlatformSummary, RootResponse,
};
use crate::catalog::{Platform, PlatformSpec};

const PUBLISHED_ENDPOINTS: &[&str] = &[
    "/",
    "/health",
    "/api/spec",
    "/api/spec/{platform}",
    "/api/bilibili/video/{bvid}",
    "/api/bilibili/video/{aid}/stream?cid={cid}",
    "/api/bilibili/video/{cid}/danmaku?segment_index={segment_index}",
    "/api/bilibili/bangumi/{bangumi_id}",
    "/api/bilibili/bangumi/{ep_id}/stream?cid={cid}",
    "/api/bilibili/article/{id}/content",
    "/api/bilibili/article/cards?ids={ids}",
    "/api/bilibili/article/{id}",
    "/api/bilibili/article-list/{id}",
    "/api/bilibili/captcha",
    "/api/bilibili/captcha/validate",
    "/api/bilibili/user/{host_mid}",
    "/api/bilibili/user/{host_mid}/dynamics",
    "/api/bilibili/user/{host_mid}/space",
    "/api/bilibili/user/{host_mid}/total-views",
    "/api/bilibili/comments/{oid}",
    "/api/bilibili/comment-replies/{oid}/{root}",
    "/api/bilibili/dynamic/{dynamic_id}",
    "/api/bilibili/dynamic/{dynamic_id}/card",
    "/api/bilibili/live/{room_id}",
    "/api/bilibili/live/{room_id}/init",
    "/api/bilibili/auth/status",
    "/api/bilibili/auth/qrcode",
    "/api/bilibili/auth/qrcode/status?qrcode_key={qrcode_key}",
    "/api/bilibili/emoji",
    "/api/bilibili/convert/av/{aid}",
    "/api/bilibili/convert/bv/{bvid}",
    "/api/douyin/work/{aweme_id}",
    "/api/douyin/work/{aweme_id}/video",
    "/api/douyin/work/{aweme_id}/image-album",
    "/api/douyin/work/{aweme_id}/slides",
    "/api/douyin/work/{aweme_id}/text",
    "/api/douyin/comments/{aweme_id}",
    "/api/douyin/comment-replies/{aweme_id}/{comment_id}",
    "/api/douyin/user/{sec_uid}",
    "/api/douyin/user/{sec_uid}/videos",
    "/api/douyin/user/{sec_uid}/favorites",
    "/api/douyin/user/{sec_uid}/recommends",
    "/api/douyin/search",
    "/api/douyin/search/suggest",
    "/api/douyin/emoji",
    "/api/douyin/emoji/dynamic",
    "/api/douyin/music/{music_id}",
    "/api/douyin/live/{room_id}",
    "/api/douyin/auth/qrcode",
    "/api/douyin/danmaku/{aweme_id}",
    "/api/kuaishou/work/{photo_id}",
    "/api/kuaishou/comments/{photo_id}",
    "/api/kuaishou/emoji",
    "/api/kuaishou/user/{principal_id}",
    "/api/kuaishou/user/{principal_id}/works",
    "/api/kuaishou/live/{principal_id}",
    "/api/twitter/user/{screen_name}",
    "/api/twitter/user/{screen_name}/timeline",
    "/api/twitter/user/{screen_name}/replies",
    "/api/twitter/user/{screen_name}/media",
    "/api/twitter/user/{screen_name}/followers",
    "/api/twitter/user/{screen_name}/following",
    "/api/twitter/user/likes",
    "/api/twitter/user/bookmarks",
    "/api/twitter/user/followed",
    "/api/twitter/user/recommended",
    "/api/twitter/search/tweets?query={query}",
    "/api/twitter/search/users",
    "/api/twitter/tweet/{tweet_id}",
    "/api/twitter/tweet/{tweet_id}/replies",
    "/api/twitter/tweet/{tweet_id}/likers",
    "/api/twitter/tweet/{tweet_id}/retweeters",
    "/api/twitter/space/{space_id}",
    "/api/xiaohongshu/feed",
    "/api/xiaohongshu/note/{note_id}?xsec_token={xsec_token}",
    "/api/xiaohongshu/comments/{note_id}?xsec_token={xsec_token}",
    "/api/xiaohongshu/user/{user_id}?xsec_token={xsec_token}&xsec_source={xsec_source}",
    "/api/xiaohongshu/user/{user_id}/notes?xsec_token={xsec_token}&xsec_source={xsec_source}",
    "/api/xiaohongshu/emoji",
    "/api/xiaohongshu/search?keyword={keyword}",
];

/// Return root web metadata.
pub async fn root(State(state): State<AppState>) -> Json<RootResponse> {
    let bind = state.serve.bind_addr();
    let base_url = state.serve.base_url();
    let platforms = Platform::ALL
        .into_iter()
        .map(|platform| {
            let client = state.client.platform(platform);
            PlatformSummary {
                platform,
                api_base_path: client.api_base_path(),
                method_count: client.methods().len(),
                has_cookie: client.has_cookie(),
            }
        })
        .collect();

    Json(RootResponse {
        name: state.app_name,
        version: state.version,
        mode: "server",
        status: "ok",
        bind,
        base_url,
        endpoints: PUBLISHED_ENDPOINTS.to_vec(),
        platforms,
    })
}

/// Return a simple health check payload.
pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok",
        service: state.app_name,
        version: state.version,
    })
}

/// Return the published API catalog for every platform.
pub async fn api_catalog(State(state): State<AppState>) -> Json<ApiCatalogResponse> {
    let platforms = Platform::ALL
        .into_iter()
        .map(|platform| platform_catalog(platform, &state))
        .collect();

    Json(ApiCatalogResponse {
        version: state.version,
        platforms,
    })
}

/// Return the published API catalog for one platform.
pub async fn platform_catalog_by_name(
    Path(platform): Path<String>,
    State(state): State<AppState>,
) -> CatalogResult<PlatformCatalogResponse> {
    let parsed = platform.parse::<Platform>().map_err(|_| {
        (
            StatusCode::NOT_FOUND,
            Json(CatalogErrorResponse {
                error: "unknown platform",
                platform,
            }),
        )
    })?;

    Ok(Json(platform_catalog(parsed, &state)))
}

fn platform_catalog(platform: Platform, state: &AppState) -> PlatformCatalogResponse {
    let spec = state.client.platform(platform).spec();
    catalog_from_spec(spec)
}

fn catalog_from_spec(spec: PlatformSpec) -> PlatformCatalogResponse {
    PlatformCatalogResponse {
        platform: spec.platform,
        api_base_path: spec.api_base_path,
        method_count: spec.methods.len(),
        methods: spec.methods.to_vec(),
    }
}
