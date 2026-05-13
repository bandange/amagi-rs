//! Axum router construction for web metadata and data endpoints.

use std::collections::BTreeMap;
use std::net::SocketAddr;
use std::time::Duration;

use axum::{
    Json, Router,
    body::{Body, to_bytes},
    extract::ConnectInfo,
    http::{
        HeaderMap, Request, Response, StatusCode,
        header::{ACCEPT, CONTENT_TYPE},
    },
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
};
use serde_json::json;
use tower_http::trace::{DefaultMakeSpan, DefaultOnResponse, TraceLayer};
use tracing::Level;

use super::handlers;
use super::runtime::PlatformServeMode;
use super::state::AppState;
use crate::catalog::Platform;
use crate::node::protocol::{
    NODE_CALLER_HEADER, NODE_DEADLINE_MS_HEADER, NODE_REQUEST_ID_HEADER,
    NODE_REQUESTED_AT_MS_HEADER, NODE_TRACE_ID_HEADER, NodeEnvelope, NodeTaskCancelParams,
    NodeTaskDispatchParams, NodeTaskResult, PROXY_HOP_HEADER, now_ms,
};

const GENERIC_COOKIE_HEADER: &str = "x-amagi-cookie";
const BILIBILI_COOKIE_HEADER: &str = "x-amagi-bilibili-cookie";
const DOUYIN_COOKIE_HEADER: &str = "x-amagi-douyin-cookie";
const KUAISHOU_COOKIE_HEADER: &str = "x-amagi-kuaishou-cookie";
const TWITTER_COOKIE_HEADER: &str = "x-amagi-twitter-cookie";
const XIAOHONGSHU_COOKIE_HEADER: &str = "x-amagi-xiaohongshu-cookie";

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct PropagatedNodeRequestContext {
    request_id: Option<String>,
    trace_id: Option<String>,
    deadline_ms: Option<u64>,
    caller: Option<String>,
    requested_at_ms: Option<u64>,
}

#[derive(Debug, Clone)]
struct NodeDispatchFailure {
    status: StatusCode,
    code: &'static str,
    detail: String,
}

impl NodeDispatchFailure {
    fn new(status: StatusCode, code: &'static str, detail: impl Into<String>) -> Self {
        Self {
            status,
            code,
            detail: detail.into(),
        }
    }
}

/// Build the web router for metadata, catalog, and data endpoints.
pub fn build(state: AppState) -> Router {
    Router::new()
        .route("/", get(handlers::root))
        .route("/health", get(handlers::health))
        .route("/node/ws", get(handlers::node_websocket))
        .route(
            "/internal/node/routes",
            post(handlers::control_apply_runtime_routes),
        )
        .route(
            "/internal/node/upstream/drain",
            post(handlers::control_announce_upstream_drain),
        )
        .route(
            "/internal/node/local/drain",
            post(handlers::control_set_local_drain),
        )
        .route(
            "/internal/node/local/isolate",
            post(handlers::control_set_local_isolated),
        )
        .route(
            "/internal/node/local/capacity",
            post(handlers::control_set_local_capacity),
        )
        .route(
            "/internal/node/local/ready",
            post(handlers::control_set_local_ready),
        )
        .route(
            "/internal/node/upstream/routes/claim",
            post(handlers::control_claim_published_upstream_routes),
        )
        .route(
            "/internal/node/upstream/routes/release",
            post(handlers::control_release_published_upstream_routes),
        )
        .route(
            "/internal/node/downstream/{node_id}/drain",
            post(handlers::control_send_downstream_drain),
        )
        .route(
            "/internal/node/downstream/{node_id}/ready",
            post(handlers::control_send_downstream_ready),
        )
        .route(
            "/internal/node/downstream/{node_id}/isolate",
            post(handlers::control_send_downstream_isolate),
        )
        .route(
            "/internal/node/downstream/{node_id}/capacity",
            post(handlers::control_send_downstream_capacity),
        )
        .route(
            "/internal/node/downstream/drain",
            post(handlers::control_broadcast_downstream_drain),
        )
        .route(
            "/internal/node/downstream/ready",
            post(handlers::control_broadcast_downstream_ready),
        )
        .route(
            "/internal/node/downstream/isolate",
            post(handlers::control_broadcast_downstream_isolate),
        )
        .route(
            "/internal/node/downstream/capacity",
            post(handlers::control_broadcast_downstream_capacity),
        )
        .route(
            "/internal/node/downstream/{node_id}/shutdown",
            post(handlers::control_send_downstream_shutdown_notice),
        )
        .route(
            "/internal/node/downstream/shutdown",
            post(handlers::control_broadcast_downstream_shutdown_notice),
        )
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
    let router = Router::new()
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
        .route("/danmaku/{aweme_id}", get(handlers::douyin_danmaku_list));
    router
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
    let explicit_node_route = state.platform_route_node(platform);
    let trust_internal_headers = trusts_internal_node_headers(&request);
    let current_hop = match current_proxy_hop(request.headers(), trust_internal_headers) {
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
    let propagated_node_context = if current_hop == 0 {
        PropagatedNodeRequestContext::default()
    } else {
        match current_node_request_context(request.headers()) {
            Ok(context) => context,
            Err(detail) => {
                return fetch_error_response(
                    StatusCode::BAD_REQUEST,
                    "invalid_node_context",
                    detail,
                );
            }
        }
    };

    let method = request.method().clone();
    let headers = request.headers().clone();
    let path_and_query = request
        .uri()
        .path_and_query()
        .map(|value| value.as_str().to_owned())
        .unwrap_or_else(|| request.uri().path().to_owned());
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
    let forwarded_headers = collect_forward_headers(
        platform,
        &headers,
        current_hop + 1,
        &propagated_node_context,
    );

    match dispatch_node_request(
        platform,
        &state,
        &method,
        &build_platform_path(platform, &path_and_query),
        &forwarded_headers,
        &body,
        current_hop + 1,
        propagated_node_context.request_id.as_deref(),
        propagated_node_context.trace_id.as_deref(),
        propagated_node_context.deadline_ms,
        propagated_node_context.caller.as_deref(),
        propagated_node_context.requested_at_ms,
    )
    .await
    {
        Ok(Some(response)) => return response,
        Ok(None) => {}
        Err(error) => {
            if explicit_node_route.is_some() || state.platform_upstream(platform).is_none() {
                return fetch_error_response(error.status, error.code, error.detail);
            }
        }
    }

    let Some(upstream_base) = state.platform_upstream(platform) else {
        return fetch_error_response(
            StatusCode::BAD_GATEWAY,
            "node_target_unavailable",
            format!("no online node or HTTP upstream configured for platform `{platform}`"),
        );
    };
    let upstream_url = build_upstream_url(upstream_base, platform, &path_and_query);
    let request_builder = apply_forward_headers(
        state.proxy_client.request(method, &upstream_url),
        &forwarded_headers,
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

async fn dispatch_node_request(
    platform: Platform,
    state: &AppState,
    method: &axum::http::Method,
    request_path_and_query: &str,
    headers: &BTreeMap<String, String>,
    body: &[u8],
    next_hop: u32,
    request_id: Option<&str>,
    trace_id: Option<&str>,
    deadline_ms: Option<u64>,
    caller: Option<&str>,
    requested_at_ms: Option<u64>,
) -> Result<Option<Response<Body>>, NodeDispatchFailure> {
    let registry = state.node_registry();
    let preferred_node_id = state.platform_route_node(platform);
    let selected = match preferred_node_id.as_deref() {
        Some(node_id) => {
            registry.sender_for_node_platform(node_id, platform.as_str(), state.node_heartbeat_ms())
        }
        None => registry.sender_for_platform(platform.as_str(), state.node_heartbeat_ms()),
    };
    let Some((target, sender)) = selected else {
        return match preferred_node_id {
            Some(node_id) => Err(preferred_node_dispatch_failure(
                state, &registry, &node_id, platform,
            )),
            None => Ok(None),
        };
    };

    let timeout_ms = state.node_request_timeout_ms().unwrap_or(15_000);
    if next_hop > state.node_max_hops().unwrap_or(u32::MAX) {
        return Err(NodeDispatchFailure::new(
            StatusCode::LOOP_DETECTED,
            "node_loop_detected",
            format!(
                "node hop {next_hop} exceeded configured maximum {}",
                state.node_max_hops().unwrap_or(u32::MAX)
            ),
        ));
    }
    let now = now_ms();
    let effective_deadline_ms = deadline_ms.unwrap_or_else(|| now.saturating_add(timeout_ms));
    let remaining_ms = effective_deadline_ms.saturating_sub(now);
    if remaining_ms == 0 {
        return Err(NodeDispatchFailure::new(
            StatusCode::GATEWAY_TIMEOUT,
            "node_timeout",
            format!("node deadline {effective_deadline_ms} has already expired before dispatch"),
        ));
    }
    let dispatch_timeout_ms = timeout_ms.min(remaining_ms);
    let (task_path, task_query) = split_node_task_path_and_query(platform, request_path_and_query)?;
    let effective_requested_at_ms = requested_at_ms.or(Some(now));
    let effective_caller = caller
        .map(str::to_owned)
        .or_else(|| state.node_id().map(str::to_owned))
        .or_else(|| Some("http".to_owned()));

    let mut envelope = NodeEnvelope::request(
        "task.dispatch",
        json!(NodeTaskDispatchParams {
            platform,
            http_method: method.as_str().to_owned(),
            path: task_path,
            query: task_query,
            path_and_query: request_path_and_query.to_owned(),
            headers: headers.clone(),
            body: body.to_vec(),
            caller: effective_caller,
            requested_at_ms: effective_requested_at_ms,
        }),
    );
    if let Some(request_id) = request_id {
        envelope.id = request_id.to_owned();
    }
    envelope.from = state.node_id().map(str::to_owned);
    envelope.to = Some(target.node_id.clone());
    envelope.session_id = Some(target.session_id.clone());
    envelope.trace_id = Some(
        trace_id
            .map(str::to_owned)
            .unwrap_or_else(|| crate::node::protocol::new_message_id("trace")),
    );
    envelope.hop_count = Some(next_hop);
    envelope.deadline_ms = Some(effective_deadline_ms);

    let request_id = envelope.id.clone();
    let request_trace_id = envelope.trace_id.clone();
    let request_hop_count = envelope.hop_count;
    let request_deadline_ms = envelope.deadline_ms;
    let pending = registry.register_pending(request_id.clone(), target.clone(), sender.clone());
    if sender.send(envelope).is_err() {
        registry.cancel_pending(&request_id);
        let _ = registry.set_session_state(
            &target.node_id,
            crate::node::session::NodeSessionState::Degraded,
        );
        return Err(NodeDispatchFailure::new(
            StatusCode::BAD_GATEWAY,
            "node_unavailable",
            format!("node `{}` is no longer writable", target.node_id),
        ));
    }

    let response =
        match tokio::time::timeout(Duration::from_millis(dispatch_timeout_ms), pending).await {
            Ok(Ok(response)) => response,
            Ok(Err(_)) => {
                registry.cancel_pending(&request_id);
                let _ = registry.set_session_state(
                    &target.node_id,
                    crate::node::session::NodeSessionState::Degraded,
                );
                return Err(NodeDispatchFailure::new(
                    StatusCode::BAD_GATEWAY,
                    "node_unavailable",
                    format!(
                        "node `{}` response channel closed before completion",
                        target.node_id
                    ),
                ));
            }
            Err(_) => {
                registry.cancel_pending(&request_id);
                let _ = registry.set_session_state(
                    &target.node_id,
                    crate::node::session::NodeSessionState::Degraded,
                );
                let _ = send_task_cancel_request(
                    state,
                    &target.node_id,
                    &target.session_id,
                    &sender,
                    &request_id,
                    request_trace_id.as_deref(),
                    request_hop_count,
                    request_deadline_ms,
                    "node request timeout exceeded",
                );
                return Err(NodeDispatchFailure::new(
                    StatusCode::GATEWAY_TIMEOUT,
                    "node_timeout",
                    format!(
                        "node `{}` did not finish the task within {dispatch_timeout_ms}ms",
                        target.node_id,
                    ),
                ));
            }
        };

    if let Some(error) = response.error {
        let should_degrade = matches!(
            error.code.as_str(),
            "task_failed" | "node_protocol_invalid" | "node_timeout" | "node_unavailable"
        );
        if should_degrade {
            let _ = registry.set_session_state(
                &target.node_id,
                crate::node::session::NodeSessionState::Degraded,
            );
        }
        return Err(NodeDispatchFailure::new(
            node_error_status(&error.code),
            node_error_code(&error.code),
            format!(
                "node `{}` returned {} for `{}`: {}",
                target.node_id, error.code, response.method, error.message
            ),
        ));
    }

    if response.method != "task.result" {
        let _ = registry.set_session_state(
            &target.node_id,
            crate::node::session::NodeSessionState::Degraded,
        );
        return Err(NodeDispatchFailure::new(
            StatusCode::BAD_GATEWAY,
            "node_protocol_invalid",
            format!(
                "node `{}` returned unexpected response method `{}`",
                target.node_id, response.method
            ),
        ));
    }

    let result_value = response.result.ok_or_else(|| {
        let _ = registry.set_session_state(
            &target.node_id,
            crate::node::session::NodeSessionState::Degraded,
        );
        NodeDispatchFailure::new(
            StatusCode::BAD_GATEWAY,
            "node_protocol_invalid",
            format!("node `{}` returned an empty task result", target.node_id),
        )
    })?;
    let result: NodeTaskResult = serde_json::from_value(result_value).map_err(|error| {
        let _ = registry.set_session_state(
            &target.node_id,
            crate::node::session::NodeSessionState::Degraded,
        );
        NodeDispatchFailure::new(
            StatusCode::BAD_GATEWAY,
            "node_protocol_invalid",
            format!(
                "node `{}` returned invalid task result: {error}",
                target.node_id
            ),
        )
    })?;

    response_from_node_task(result).map(Some).map_err(|error| {
        let _ = registry.set_session_state(
            &target.node_id,
            crate::node::session::NodeSessionState::Degraded,
        );
        NodeDispatchFailure::new(StatusCode::BAD_GATEWAY, "node_protocol_invalid", error)
    })
}

fn preferred_node_dispatch_failure(
    state: &AppState,
    registry: &crate::node::registry::NodeRegistry,
    node_id: &str,
    platform: Platform,
) -> NodeDispatchFailure {
    let Some(record) = registry.record_for_node(node_id, state.node_heartbeat_ms()) else {
        return NodeDispatchFailure::new(
            StatusCode::BAD_GATEWAY,
            "node_unavailable",
            format!("configured node route `node:{node_id}` is offline"),
        );
    };

    if !record
        .platforms
        .iter()
        .any(|item| item == platform.as_str())
    {
        return NodeDispatchFailure::new(
            StatusCode::BAD_GATEWAY,
            "node_route_missing",
            format!(
                "configured node route `node:{node_id}` does not publish platform `{platform}`"
            ),
        );
    }

    if matches!(
        record.session_state,
        crate::node::session::NodeSessionState::Authenticating
    ) {
        return NodeDispatchFailure::new(
            StatusCode::BAD_GATEWAY,
            "node_unavailable",
            format!("configured node route `node:{node_id}` is still authenticating"),
        );
    }

    if matches!(
        record.session_state,
        crate::node::session::NodeSessionState::Degraded
    ) {
        return NodeDispatchFailure::new(
            StatusCode::BAD_GATEWAY,
            "node_unavailable",
            format!("configured node route `node:{node_id}` is degraded"),
        );
    }

    match record.availability {
        crate::node::registry::NodeAvailability::Draining => {
            return NodeDispatchFailure::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "node_draining",
                format!("configured node route `node:{node_id}` is draining"),
            );
        }
        crate::node::registry::NodeAvailability::Isolated => {
            return NodeDispatchFailure::new(
                StatusCode::SERVICE_UNAVAILABLE,
                "node_isolated",
                format!("configured node route `node:{node_id}` is isolated"),
            );
        }
        crate::node::registry::NodeAvailability::Ready => {}
    }

    if record
        .max_concurrent_tasks
        .is_some_and(|limit| record.active_tasks >= limit)
    {
        return NodeDispatchFailure::new(
            StatusCode::SERVICE_UNAVAILABLE,
            "node_capacity_exceeded",
            format!(
                "configured node route `node:{node_id}` is at capacity ({}/{})",
                record.active_tasks,
                record.max_concurrent_tasks.unwrap_or_default()
            ),
        );
    }

    NodeDispatchFailure::new(
        StatusCode::BAD_GATEWAY,
        "node_unavailable",
        format!("configured node route `node:{node_id}` is unavailable for platform `{platform}`"),
    )
}

fn send_task_cancel_request(
    state: &AppState,
    target_node_id: &str,
    session_id: &str,
    sender: &tokio::sync::mpsc::UnboundedSender<NodeEnvelope>,
    request_id: &str,
    trace_id: Option<&str>,
    hop_count: Option<u32>,
    deadline_ms: Option<u64>,
    reason: &str,
) -> Result<(), String> {
    let mut envelope = NodeEnvelope::request(
        "task.cancel",
        json!(NodeTaskCancelParams {
            request_id: request_id.to_owned(),
            reason: Some(reason.to_owned()),
        }),
    );
    envelope.from = state.node_id().map(str::to_owned);
    envelope.to = Some(target_node_id.to_owned());
    envelope.session_id = Some(session_id.to_owned());
    envelope.trace_id = trace_id.map(str::to_owned);
    envelope.hop_count = hop_count;
    envelope.deadline_ms = deadline_ms;

    sender.send(envelope).map_err(|error| {
        format!(
            "node `{}` could not receive task.cancel: {}",
            target_node_id, error.0.method
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

fn current_proxy_hop(headers: &HeaderMap, trust_internal_headers: bool) -> Result<u32, String> {
    if !trust_internal_headers {
        return Ok(0);
    }

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

fn current_node_request_context(
    headers: &HeaderMap,
) -> Result<PropagatedNodeRequestContext, String> {
    let request_id = match headers.get(NODE_REQUEST_ID_HEADER) {
        Some(value) => {
            let value = value
                .to_str()
                .map_err(|_| format!("`{NODE_REQUEST_ID_HEADER}` is not valid UTF-8"))?;
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return Err(format!("`{NODE_REQUEST_ID_HEADER}` must not be empty"));
            }
            Some(trimmed.to_owned())
        }
        None => None,
    };
    let trace_id = match headers.get(NODE_TRACE_ID_HEADER) {
        Some(value) => {
            let value = value
                .to_str()
                .map_err(|_| format!("`{NODE_TRACE_ID_HEADER}` is not valid UTF-8"))?;
            let trimmed = value.trim();
            if trimmed.is_empty() {
                return Err(format!("`{NODE_TRACE_ID_HEADER}` must not be empty"));
            }
            Some(trimmed.to_owned())
        }
        None => None,
    };
    let deadline_ms =
        match headers.get(NODE_DEADLINE_MS_HEADER) {
            Some(value) => {
                let value = value
                    .to_str()
                    .map_err(|_| format!("`{NODE_DEADLINE_MS_HEADER}` is not valid UTF-8"))?;
                Some(value.trim().parse::<u64>().map_err(|error| {
                    format!("invalid `{NODE_DEADLINE_MS_HEADER}` value: {error}")
                })?)
            }
            None => None,
        };
    let caller = match headers.get(NODE_CALLER_HEADER) {
        Some(value) => {
            let value = value
                .to_str()
                .map_err(|_| format!("`{NODE_CALLER_HEADER}` is not valid UTF-8"))?;
            let trimmed = value.trim();
            (!trimmed.is_empty()).then(|| trimmed.to_owned())
        }
        None => None,
    };
    let requested_at_ms = match headers.get(NODE_REQUESTED_AT_MS_HEADER) {
        Some(value) => {
            let value = value
                .to_str()
                .map_err(|_| format!("`{NODE_REQUESTED_AT_MS_HEADER}` is not valid UTF-8"))?;
            Some(value.trim().parse::<u64>().map_err(|error| {
                format!("invalid `{NODE_REQUESTED_AT_MS_HEADER}` value: {error}")
            })?)
        }
        None => None,
    };

    Ok(PropagatedNodeRequestContext {
        request_id,
        trace_id,
        deadline_ms,
        caller,
        requested_at_ms,
    })
}

fn collect_forward_headers(
    platform: Platform,
    headers: &HeaderMap,
    next_hop: u32,
    propagated_node_context: &PropagatedNodeRequestContext,
) -> BTreeMap<String, String> {
    let mut forwarded = BTreeMap::new();

    for header_name in [ACCEPT.as_str(), CONTENT_TYPE.as_str()] {
        if let Some(value) = headers
            .get(header_name)
            .and_then(|value| value.to_str().ok())
        {
            forwarded.insert(header_name.to_owned(), value.to_owned());
        }
    }

    for header_name in [GENERIC_COOKIE_HEADER, platform_cookie_header(platform)] {
        if let Some(value) = headers
            .get(header_name)
            .and_then(|value| value.to_str().ok())
        {
            forwarded.insert(header_name.to_owned(), value.to_owned());
        }
    }

    if let Some(request_id) = propagated_node_context.request_id.as_deref() {
        forwarded.insert(NODE_REQUEST_ID_HEADER.to_owned(), request_id.to_owned());
    }
    if let Some(trace_id) = propagated_node_context.trace_id.as_deref() {
        forwarded.insert(NODE_TRACE_ID_HEADER.to_owned(), trace_id.to_owned());
    }
    if let Some(deadline_ms) = propagated_node_context.deadline_ms {
        forwarded.insert(NODE_DEADLINE_MS_HEADER.to_owned(), deadline_ms.to_string());
    }
    if let Some(requested_at_ms) = propagated_node_context.requested_at_ms {
        forwarded.insert(
            NODE_REQUESTED_AT_MS_HEADER.to_owned(),
            requested_at_ms.to_string(),
        );
    }
    if let Some(caller) = propagated_node_context.caller.as_deref() {
        forwarded.insert(NODE_CALLER_HEADER.to_owned(), caller.to_owned());
    }

    forwarded.insert(PROXY_HOP_HEADER.to_owned(), next_hop.to_string());
    forwarded
}

fn trusts_internal_node_headers(request: &Request<Body>) -> bool {
    request
        .extensions()
        .get::<ConnectInfo<SocketAddr>>()
        .is_some_and(|connect_info| connect_info.0.ip().is_loopback())
}

fn apply_forward_headers(
    mut builder: reqwest::RequestBuilder,
    headers: &BTreeMap<String, String>,
) -> reqwest::RequestBuilder {
    for (name, value) in headers {
        builder = builder.header(name, value);
    }

    builder
}

fn build_platform_path(platform: Platform, path_and_query: &str) -> String {
    let mut path = platform.api_base_path().to_owned();
    if !path_and_query.starts_with('/') {
        path.push('/');
    }
    path.push_str(path_and_query);
    path
}

fn normalize_path_and_query(path_and_query: &str) -> String {
    if path_and_query.starts_with('/') {
        path_and_query.to_owned()
    } else {
        format!("/{path_and_query}")
    }
}

fn response_from_node_task(result: NodeTaskResult) -> Result<Response<Body>, String> {
    let status = StatusCode::from_u16(result.status)
        .map_err(|error| format!("invalid node response status {}: {error}", result.status))?;
    let mut response_builder = Response::builder().status(status);
    if let Some(content_type) = result.content_type {
        response_builder = response_builder.header(CONTENT_TYPE, content_type);
    }
    for (name, value) in result.headers {
        response_builder = response_builder.header(name, value);
    }

    response_builder
        .body(Body::from(result.body))
        .map_err(|error| format!("invalid node response body: {error}"))
}

fn split_node_task_path_and_query(
    platform: Platform,
    request_path_and_query: &str,
) -> Result<(String, Vec<(String, String)>), NodeDispatchFailure> {
    let path_and_query = normalize_path_and_query(request_path_and_query);
    let (raw_path, raw_query) = match path_and_query.split_once('?') {
        Some((path, query)) => (path, Some(query)),
        None => (path_and_query.as_str(), None),
    };
    let path = raw_path
        .strip_prefix(platform.api_base_path())
        .unwrap_or(raw_path)
        .to_owned();
    let path = if path.is_empty() {
        "/".to_owned()
    } else {
        path
    };
    let query: Vec<(String, String)> = raw_query.map(split_query_pairs).unwrap_or_default();

    Ok((path, query))
}

fn split_query_pairs(raw_query: &str) -> Vec<(String, String)> {
    raw_query
        .split('&')
        .filter(|segment| !segment.is_empty())
        .map(|segment| match segment.split_once('=') {
            Some((key, value)) => (key.to_owned(), value.to_owned()),
            None => (segment.to_owned(), String::new()),
        })
        .collect()
}

fn node_error_status(code: &str) -> StatusCode {
    match code {
        "node_timeout" => StatusCode::GATEWAY_TIMEOUT,
        "node_loop_detected" => StatusCode::LOOP_DETECTED,
        "node_draining" => StatusCode::SERVICE_UNAVAILABLE,
        "node_isolated" => StatusCode::SERVICE_UNAVAILABLE,
        "node_capacity_exceeded" => StatusCode::SERVICE_UNAVAILABLE,
        "node_unavailable" | "node_route_missing" | "task_failed" | "task_not_found" => {
            StatusCode::BAD_GATEWAY
        }
        "task_cancelled" => StatusCode::CONFLICT,
        _ => StatusCode::BAD_GATEWAY,
    }
}

fn node_error_code(code: &str) -> &'static str {
    match code {
        "node_timeout" => "node_timeout",
        "node_loop_detected" => "node_loop_detected",
        "node_draining" => "node_draining",
        "node_isolated" => "node_isolated",
        "node_capacity_exceeded" => "node_capacity_exceeded",
        "node_unavailable" => "node_unavailable",
        "node_route_missing" => "node_route_missing",
        "node_protocol_invalid" => "node_protocol_invalid",
        "task_cancelled" => "task_cancelled",
        "task_not_found" => "task_not_found",
        "task_failed" => "task_failed",
        _ => "node_dispatch_failed",
    }
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
