//! Axum router construction for web metadata and data endpoints.

use axum::{
    Json, Router,
    body::{Body, to_bytes},
    http::{
        HeaderMap, Request, Response, StatusCode,
        header::{ACCEPT, CONTENT_TYPE},
    },
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
};
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use super::handlers;
use super::runtime::PlatformServeMode;
use super::state::AppState;
use crate::catalog::Platform;

const GENERIC_COOKIE_HEADER: &str = "x-amagi-cookie";
const BILIBILI_COOKIE_HEADER: &str = "x-amagi-bilibili-cookie";
const DOUYIN_COOKIE_HEADER: &str = "x-amagi-douyin-cookie";
const KUAISHOU_COOKIE_HEADER: &str = "x-amagi-kuaishou-cookie";
const TWITTER_COOKIE_HEADER: &str = "x-amagi-twitter-cookie";
const XIAOHONGSHU_COOKIE_HEADER: &str = "x-amagi-xiaohongshu-cookie";
const PROXY_HOP_HEADER: &str = "x-amagi-proxy-hop";

/// Build the web router for metadata, catalog, and data endpoints.
pub fn build(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health))
        .route("/api/spec", get(handlers::api_catalog))
        .route(
            "/api/spec/{platform}",
            get(handlers::platform_catalog_by_name),
        )
        .nest(
            Platform::Bilibili.api_base_path(),
            platform_router(Platform::Bilibili, state.clone(), bilibili_routes()),
        )
        .nest(
            Platform::Douyin.api_base_path(),
            platform_router(Platform::Douyin, state.clone(), douyin_routes()),
        )
        .nest(
            Platform::Kuaishou.api_base_path(),
            platform_router(Platform::Kuaishou, state.clone(), kuaishou_routes()),
        )
        .nest(
            Platform::Twitter.api_base_path(),
            platform_router(Platform::Twitter, state.clone(), twitter_routes()),
        )
        .nest(
            Platform::Xiaohongshu.api_base_path(),
            platform_router(Platform::Xiaohongshu, state.clone(), xiaohongshu_routes()),
        )
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
                .on_response(DefaultOnResponse::new().level(Level::INFO)),
        )
        .with_state(state)
}

fn bilibili_routes() -> Router<AppState> {
    Router::new()
        .route("/video/{bvid}", get(handlers::bilibili_video_info))
        .route("/video/{aid}/stream", get(handlers::bilibili_video_stream))
        .route(
            "/video/{cid}/danmaku",
            get(handlers::bilibili_video_danmaku),
        )
        .route(
            "/bangumi/{bangumi_id}",
            get(handlers::bilibili_bangumi_info),
        )
        .route(
            "/bangumi/{ep_id}/stream",
            get(handlers::bilibili_bangumi_stream),
        )
        .route(
            "/article/{id}/content",
            get(handlers::bilibili_article_content),
        )
        .route("/article/cards", get(handlers::bilibili_article_cards))
        .route("/article/{id}", get(handlers::bilibili_article_info))
        .route(
            "/article-list/{id}",
            get(handlers::bilibili_article_list_info),
        )
        .route("/captcha", post(handlers::bilibili_captcha_from_voucher))
        .route(
            "/captcha/validate",
            post(handlers::bilibili_validate_captcha),
        )
        .route("/user/{host_mid}", get(handlers::bilibili_user_card))
        .route(
            "/user/{host_mid}/dynamics",
            get(handlers::bilibili_user_dynamic_list),
        )
        .route(
            "/user/{host_mid}/space",
            get(handlers::bilibili_user_space_info),
        )
        .route(
            "/user/{host_mid}/total-views",
            get(handlers::bilibili_uploader_total_views),
        )
        .route("/comments/{oid}", get(handlers::bilibili_comments))
        .route(
            "/comment-replies/{oid}/{root}",
            get(handlers::bilibili_comment_replies),
        )
        .route(
            "/dynamic/{dynamic_id}",
            get(handlers::bilibili_dynamic_detail),
        )
        .route(
            "/dynamic/{dynamic_id}/card",
            get(handlers::bilibili_dynamic_card),
        )
        .route("/live/{room_id}", get(handlers::bilibili_live_room_info))
        .route(
            "/live/{room_id}/init",
            get(handlers::bilibili_live_room_init),
        )
        .route("/auth/status", get(handlers::bilibili_login_status))
        .route("/auth/qrcode", get(handlers::bilibili_login_qrcode))
        .route("/auth/qrcode/status", get(handlers::bilibili_qrcode_status))
        .route("/emoji", get(handlers::bilibili_emoji_list))
        .route("/convert/av/{aid}", get(handlers::bilibili_av_to_bv))
        .route("/convert/bv/{bvid}", get(handlers::bilibili_bv_to_av))
}

fn douyin_routes() -> Router<AppState> {
    Router::new()
        .route("/work/{aweme_id}", get(handlers::douyin_parse_work))
        .route("/work/{aweme_id}/video", get(handlers::douyin_video_work))
        .route(
            "/work/{aweme_id}/image-album",
            get(handlers::douyin_image_album_work),
        )
        .route("/work/{aweme_id}/slides", get(handlers::douyin_slides_work))
        .route("/work/{aweme_id}/text", get(handlers::douyin_text_work))
        .route("/comments/{aweme_id}", get(handlers::douyin_work_comments))
        .route(
            "/comment-replies/{aweme_id}/{comment_id}",
            get(handlers::douyin_comment_replies),
        )
        .route("/emoji", get(handlers::douyin_emoji_list))
        .route("/emoji/dynamic", get(handlers::douyin_dynamic_emoji_list))
        .route("/user/{sec_uid}", get(handlers::douyin_user_profile))
        .route(
            "/user/{sec_uid}/videos",
            get(handlers::douyin_user_video_list),
        )
        .route(
            "/user/{sec_uid}/favorites",
            get(handlers::douyin_user_favorite_list),
        )
        .route(
            "/user/{sec_uid}/recommends",
            get(handlers::douyin_user_recommend_list),
        )
        .route("/search", get(handlers::douyin_search))
        .route("/search/suggest", get(handlers::douyin_suggest_words))
        .route("/music/{music_id}", get(handlers::douyin_music_info))
        .route("/live/{room_id}", get(handlers::douyin_live_room_info))
        .route("/auth/qrcode", get(handlers::douyin_login_qrcode))
        .route("/danmaku/{aweme_id}", get(handlers::douyin_danmaku_list))
}

fn kuaishou_routes() -> Router<AppState> {
    Router::new()
        .route("/work/{photo_id}", get(handlers::kuaishou_video_work))
        .route(
            "/comments/{photo_id}",
            get(handlers::kuaishou_work_comments),
        )
        .route("/emoji", get(handlers::kuaishou_emoji_list))
        .route("/user/{principal_id}", get(handlers::kuaishou_user_profile))
        .route(
            "/user/{principal_id}/works",
            get(handlers::kuaishou_user_work_list),
        )
        .route(
            "/live/{principal_id}",
            get(handlers::kuaishou_live_room_info),
        )
}

fn twitter_routes() -> Router<AppState> {
    Router::new()
        .route("/user/{screen_name}", get(handlers::twitter_user_profile))
        .route(
            "/user/{screen_name}/timeline",
            get(handlers::twitter_user_timeline),
        )
        .route(
            "/user/{screen_name}/replies",
            get(handlers::twitter_user_replies),
        )
        .route(
            "/user/{screen_name}/media",
            get(handlers::twitter_user_media),
        )
        .route(
            "/user/{screen_name}/followers",
            get(handlers::twitter_user_followers),
        )
        .route(
            "/user/{screen_name}/following",
            get(handlers::twitter_user_following),
        )
        .route("/user/likes", get(handlers::twitter_user_likes))
        .route("/user/bookmarks", get(handlers::twitter_user_bookmarks))
        .route("/user/followed", get(handlers::twitter_user_followed))
        .route("/user/recommended", get(handlers::twitter_user_recommended))
        .route("/search/tweets", get(handlers::twitter_search_tweets))
        .route("/search/users", get(handlers::twitter_search_users))
        .route("/tweet/{tweet_id}", get(handlers::twitter_tweet_detail))
        .route(
            "/tweet/{tweet_id}/replies",
            get(handlers::twitter_tweet_replies),
        )
        .route(
            "/tweet/{tweet_id}/likers",
            get(handlers::twitter_tweet_likers),
        )
        .route(
            "/tweet/{tweet_id}/retweeters",
            get(handlers::twitter_tweet_retweeters),
        )
        .route("/space/{space_id}", get(handlers::twitter_space_detail))
}

fn xiaohongshu_routes() -> Router<AppState> {
    Router::new()
        .route("/feed", get(handlers::xiaohongshu_home_feed))
        .route("/note/{note_id}", get(handlers::xiaohongshu_note_detail))
        .route(
            "/comments/{note_id}",
            get(handlers::xiaohongshu_note_comments),
        )
        .route("/user/{user_id}", get(handlers::xiaohongshu_user_profile))
        .route(
            "/user/{user_id}/notes",
            get(handlers::xiaohongshu_user_note_list),
        )
        .route("/emoji", get(handlers::xiaohongshu_emoji_list))
        .route("/search", get(handlers::xiaohongshu_search_notes))
}

fn platform_router(
    platform: Platform,
    state: AppState,
    router: Router<AppState>,
) -> Router<AppState> {
    router.layer(middleware::from_fn(move |request, next| {
        let state = state.clone();
        async move { dispatch_platform_request(platform, state, request, next).await }
    }))
}

async fn dispatch_platform_request(
    platform: Platform,
    state: AppState,
    request: Request<Body>,
    next: Next,
) -> Response<Body> {
    match state.platform_mode(platform) {
        PlatformServeMode::Local => next.run(request).await,
        PlatformServeMode::Disabled => platform_disabled_response(platform),
        PlatformServeMode::Upstream => proxy_platform_request(platform, state, request).await,
    }
}

async fn proxy_platform_request(
    platform: Platform,
    state: AppState,
    request: Request<Body>,
) -> Response<Body> {
    let current_hop = match current_proxy_hop(request.headers()) {
        Ok(hop) => hop,
        Err(detail) => {
            return fetch_error_response(StatusCode::BAD_REQUEST, "invalid_proxy_hop", detail);
        }
    };

    if current_hop >= state.proxy_max_hops() {
        return fetch_error_response(
            StatusCode::LOOP_DETECTED,
            "upstream_loop_detected",
            format!(
                "proxy hop {current_hop} reached configured maximum {}",
                state.proxy_max_hops()
            ),
        );
    }

    let Some(upstream_base) = state.platform_upstream(platform) else {
        return fetch_error_response(
            StatusCode::INTERNAL_SERVER_ERROR,
            "upstream_not_configured",
            format!("no upstream configured for platform `{platform}`"),
        );
    };

    let method = request.method().clone();
    let headers = request.headers().clone();
    let path_and_query = request
        .uri()
        .path_and_query()
        .map(|value| value.as_str().to_owned())
        .unwrap_or_else(|| request.uri().path().to_owned());
    let upstream_url = build_upstream_url(upstream_base, platform, &path_and_query);
    let body = match to_bytes(request.into_body(), usize::MAX).await {
        Ok(body) => body,
        Err(error) => {
            return fetch_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "upstream_request_invalid",
                error.to_string(),
            );
        }
    };

    let request_builder = forward_proxy_headers(
        state.proxy_client.request(method, &upstream_url),
        platform,
        &headers,
        current_hop + 1,
    )
    .body(body);

    let upstream_response = match request_builder.send().await {
        Ok(response) => response,
        Err(error) => {
            return fetch_error_response(
                StatusCode::BAD_GATEWAY,
                "upstream_unavailable",
                error.to_string(),
            );
        }
    };

    let status = upstream_response.status();
    let content_type = upstream_response.headers().get(CONTENT_TYPE).cloned();
    let response_body = match upstream_response.bytes().await {
        Ok(body) => body,
        Err(error) => {
            return fetch_error_response(
                StatusCode::BAD_GATEWAY,
                "upstream_unavailable",
                error.to_string(),
            );
        }
    };

    let mut response_builder = Response::builder().status(status);
    if let Some(content_type) = content_type {
        response_builder = response_builder.header(CONTENT_TYPE, content_type);
    }

    response_builder
        .body(Body::from(response_body))
        .unwrap_or_else(|error| {
            fetch_error_response(
                StatusCode::INTERNAL_SERVER_ERROR,
                "proxy_response_invalid",
                error.to_string(),
            )
        })
}

fn build_upstream_url(upstream_base: &str, platform: Platform, path_and_query: &str) -> String {
    let mut url = upstream_base.trim_end_matches('/').to_owned();
    url.push_str(platform.api_base_path());
    if !path_and_query.starts_with('/') {
        url.push('/');
    }
    url.push_str(path_and_query);
    url
}

fn current_proxy_hop(headers: &HeaderMap) -> Result<u32, String> {
    match headers.get(PROXY_HOP_HEADER) {
        Some(value) => {
            let value = value
                .to_str()
                .map_err(|_| "proxy hop header is not valid UTF-8".to_owned())?;
            value
                .trim()
                .parse::<u32>()
                .map_err(|error| format!("invalid `{PROXY_HOP_HEADER}` value: {error}"))
        }
        None => Ok(0),
    }
}

fn forward_proxy_headers(
    mut builder: reqwest::RequestBuilder,
    platform: Platform,
    headers: &HeaderMap,
    next_hop: u32,
) -> reqwest::RequestBuilder {
    for header_name in [ACCEPT, CONTENT_TYPE] {
        if let Some(value) = headers.get(&header_name) {
            builder = builder.header(header_name, value.clone());
        }
    }

    for header_name in [GENERIC_COOKIE_HEADER, platform_cookie_header(platform)] {
        if let Some(value) = headers.get(header_name) {
            builder = builder.header(header_name, value.clone());
        }
    }

    builder.header(PROXY_HOP_HEADER, next_hop.to_string())
}

fn platform_cookie_header(platform: Platform) -> &'static str {
    match platform {
        Platform::Bilibili => BILIBILI_COOKIE_HEADER,
        Platform::Douyin => DOUYIN_COOKIE_HEADER,
        Platform::Kuaishou => KUAISHOU_COOKIE_HEADER,
        Platform::Twitter => TWITTER_COOKIE_HEADER,
        Platform::Xiaohongshu => XIAOHONGSHU_COOKIE_HEADER,
    }
}

fn platform_disabled_response(platform: Platform) -> Response<Body> {
    (
        StatusCode::NOT_FOUND,
        Json(handlers::CatalogErrorResponse {
            error: "platform_disabled",
            platform: platform.to_string(),
        }),
    )
        .into_response()
}

fn fetch_error_response(
    status: StatusCode,
    error: &'static str,
    detail: impl Into<String>,
) -> Response<Body> {
    (
        status,
        Json(handlers::FetchErrorResponse {
            error,
            detail: detail.into(),
        }),
    )
        .into_response()
}
