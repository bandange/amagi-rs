use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};

use super::super::state::AppState;
use super::support::CatalogResult;
use super::{
    ApiCatalogResponse, CatalogErrorResponse, DownstreamNodeSummary, HealthResponse,
    NodeHealthSummary, NodeSummary, PlatformCatalogResponse, PlatformRouteSource, PlatformSummary,
    RootResponse, RuntimeRouteSummary, UpstreamConnectionSummary,
};
use crate::catalog::{Platform, PlatformSpec};
use crate::node::registry::NodeAvailability;
use crate::node::session::NodeSessionState;

const METADATA_ENDPOINTS: &[&str] = &["/", "/health", "/api/spec", "/api/spec/{platform}"];

const BILIBILI_ENDPOINTS: &[&str] = &[
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
];

const DOUYIN_ENDPOINTS: &[&str] = &[
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
];

const KUAISHOU_ENDPOINTS: &[&str] = &[
    "/api/kuaishou/work/{photo_id}",
    "/api/kuaishou/comments/{photo_id}",
    "/api/kuaishou/emoji",
    "/api/kuaishou/user/{principal_id}",
    "/api/kuaishou/user/{principal_id}/works",
    "/api/kuaishou/live/{principal_id}",
];

const TWITTER_ENDPOINTS: &[&str] = &[
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
];

const XIAOHONGSHU_ENDPOINTS: &[&str] = &[
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
    let downstream_nodes = downstream_node_summaries(&state);
    let (
        downstream_authenticating,
        downstream_ready,
        downstream_degraded,
        downstream_draining,
        downstream_isolated,
    ) = downstream_node_counts(&downstream_nodes);
    let upstream = upstream_summary(&state);
    let bind = state.serve.bind_addr();
    let base_url = state.serve.base_url();
    let platforms = Platform::ALL
        .into_iter()
        .map(|platform| {
            let client = state.client.platform(platform);
            let route_node = state.platform_route_node(platform);
            let route_source = route_node.as_ref().map(|_| {
                if state.platform_route_is_runtime(platform) {
                    PlatformRouteSource::Runtime
                } else {
                    PlatformRouteSource::Configured
                }
            });
            PlatformSummary {
                platform,
                api_base_path: client.api_base_path(),
                method_count: client.methods().len(),
                has_cookie: client.has_cookie(),
                mode: state.platform_mode(platform),
                published: state.is_platform_published(platform),
                route_node,
                route_source,
            }
        })
        .collect();
    let runtime_routes = state
        .runtime_platform_routes()
        .into_iter()
        .map(|(platform, route_node)| RuntimeRouteSummary {
            platform,
            route_node,
        })
        .collect();

    Json(RootResponse {
        name: state.app_name,
        version: state.version,
        mode: "server",
        status: "ok",
        bind,
        base_url,
        endpoints: published_endpoints(&state),
        platforms,
        node: state.node_id().map(|node_id| NodeSummary {
            node_id: Some(node_id.to_owned()),
            role: state.node_role(),
            availability: state.local_node_availability(),
            capabilities: state.node_capabilities(),
            max_concurrent_tasks: state.node_max_concurrent_tasks(),
            upstream,
            downstream_authenticating,
            downstream_ready,
            downstream_degraded,
            downstream_draining,
            downstream_isolated,
        }),
        runtime_routes,
        downstream_nodes,
    })
}

/// Return a simple health check payload.
pub async fn health(State(state): State<AppState>) -> Json<HealthResponse> {
    let downstream_nodes = downstream_node_summaries(&state);
    let (
        downstream_authenticating,
        downstream_ready,
        downstream_degraded,
        downstream_draining,
        downstream_isolated,
    ) = downstream_node_counts(&downstream_nodes);
    let upstream = upstream_summary(&state);
    let ready = state.is_local_node_ready_for_tasks()
        && (state.node_connect_upstream().is_none()
            || upstream
                .as_ref()
                .is_some_and(|summary| summary.state == NodeSessionState::Ready));

    Json(HealthResponse {
        status: "ok",
        service: state.app_name,
        version: state.version,
        node: state.node_id().map(|node_id| NodeHealthSummary {
            node_id: Some(node_id.to_owned()),
            role: state.node_role(),
            availability: state.local_node_availability(),
            ready,
            upstream,
            downstream_total: downstream_nodes.len(),
            downstream_authenticating,
            downstream_ready,
            downstream_degraded,
            downstream_draining,
            downstream_isolated,
        }),
    })
}

/// Return the published API catalog for every platform.
pub async fn api_catalog(State(state): State<AppState>) -> Json<ApiCatalogResponse> {
    let platforms = Platform::ALL
        .into_iter()
        .filter(|platform| state.is_platform_published(*platform))
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

    if !state.is_platform_published(parsed) {
        return Err((
            StatusCode::NOT_FOUND,
            Json(CatalogErrorResponse {
                error: "platform_disabled",
                platform: parsed.to_string(),
            }),
        ));
    }

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

fn published_endpoints(state: &AppState) -> Vec<&'static str> {
    let mut endpoints = METADATA_ENDPOINTS.to_vec();

    for platform in Platform::ALL {
        if state.is_platform_published(platform) {
            endpoints.extend_from_slice(platform_endpoints(platform));
        }
    }

    endpoints
}

fn downstream_node_summaries(state: &AppState) -> Vec<DownstreamNodeSummary> {
    state
        .node_registry()
        .records(state.node_heartbeat_ms())
        .into_iter()
        .map(|record| DownstreamNodeSummary {
            session_id: record.session_id,
            node_id: record.node_id,
            role: record.role,
            version: record.version,
            session_state: record.session_state,
            availability: record.availability,
            capabilities: record.capabilities,
            platforms: record.platforms,
            max_concurrent_tasks: record.max_concurrent_tasks,
            active_tasks: record.active_tasks,
            connected_at_ms: record.connected_at_ms,
            last_seen_ms: record.last_seen_ms,
        })
        .collect()
}

fn downstream_node_counts(nodes: &[DownstreamNodeSummary]) -> (usize, usize, usize, usize, usize) {
    nodes.iter().fold(
        (0, 0, 0, 0, 0),
        |(authenticating, ready, degraded, draining, isolated), node| match (
            node.availability,
            node.session_state,
        ) {
            (NodeAvailability::Draining, _) => {
                (authenticating, ready, degraded, draining + 1, isolated)
            }
            (NodeAvailability::Isolated, _) => {
                (authenticating, ready, degraded, draining, isolated + 1)
            }
            (NodeAvailability::Ready, NodeSessionState::Authenticating) => {
                (authenticating + 1, ready, degraded, draining, isolated)
            }
            (NodeAvailability::Ready, NodeSessionState::Ready) => {
                (authenticating, ready + 1, degraded, draining, isolated)
            }
            (NodeAvailability::Ready, NodeSessionState::Degraded) => {
                (authenticating, ready, degraded + 1, draining, isolated)
            }
            (NodeAvailability::Ready, _) => (authenticating, ready, degraded, draining, isolated),
        },
    )
}

fn upstream_summary(state: &AppState) -> Option<UpstreamConnectionSummary> {
    state.node_connect_upstream()?;
    let snapshot = state.upstream_connection_snapshot();
    Some(UpstreamConnectionSummary {
        connected: snapshot.connected,
        state: snapshot.state,
        session_id: snapshot.session_id,
        node_id: snapshot.node_id,
        role: snapshot.role,
        version: snapshot.version,
        capabilities: snapshot.capabilities,
        platforms: snapshot.platforms,
        connected_at_ms: snapshot.connected_at_ms,
        last_seen_ms: snapshot.last_seen_ms,
        last_disconnect_ms: snapshot.last_disconnect_ms,
        last_error: snapshot.last_error,
    })
}

fn platform_endpoints(platform: Platform) -> &'static [&'static str] {
    match platform {
        Platform::Bilibili => BILIBILI_ENDPOINTS,
        Platform::Douyin => DOUYIN_ENDPOINTS,
        Platform::Kuaishou => KUAISHOU_ENDPOINTS,
        Platform::Twitter => TWITTER_ENDPOINTS,
        Platform::Xiaohongshu => XIAOHONGSHU_ENDPOINTS,
    }
}
