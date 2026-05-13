use axum::{
    Json,
    extract::{Path, State},
    http::{HeaderMap, StatusCode, header::AUTHORIZATION},
};
use serde_json::{Value, json};
use tokio::time::Duration;

use super::super::state::AppState;
use super::{
    ControlActionResponse, ControlErrorResponse, RuntimeRouteMutationResponse, RuntimeRouteSummary,
};
use crate::events::EventLogLevel;
use crate::node::protocol::{
    NodeCapacityParams, NodeDrainParams, NodeEnvelope, NodeIsolateParams, NodeReadyParams,
    NodeRouteUpdateParams, NodeShutdownNoticeParams, new_message_id,
};
use crate::node::registry::NodeAvailability;

type ControlResult<T> = Result<Json<T>, (StatusCode, Json<ControlErrorResponse>)>;

/// Apply one batch of runtime route overrides from the local control plane.
pub async fn control_apply_runtime_routes(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeRouteUpdateParams>,
) -> ControlResult<RuntimeRouteMutationResponse> {
    authorize_control(&state, &headers)?;

    let mut applied = Vec::with_capacity(request.updates.len());
    let mut cleared_platforms = Vec::new();
    for update in request.updates {
        match update.route_node {
            Some(route_node) => {
                let route_node = route_node.trim();
                if route_node.is_empty() {
                    return Err(control_error(
                        StatusCode::BAD_REQUEST,
                        "invalid_route_update",
                        format!(
                            "platform `{}` route_node must not be empty",
                            update.platform
                        ),
                    ));
                }
                state.set_runtime_platform_route(update.platform, route_node.to_owned());
                applied.push(format!("{}=node:{route_node}", update.platform));
            }
            None => {
                if state
                    .remove_runtime_platform_route(update.platform)
                    .is_some()
                {
                    cleared_platforms.push(update.platform);
                    applied.push(format!("{}=cleared", update.platform));
                }
            }
        }
    }

    state.client.events().emit_log(
        EventLogLevel::Info,
        "internal runtime route update",
        applied,
    );

    Ok(Json(RuntimeRouteMutationResponse {
        ok: true,
        runtime_routes: current_runtime_routes(&state),
        cleared_platforms,
    }))
}

/// Send one drain request to one connected downstream node.
pub async fn control_send_downstream_drain(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeDrainParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    let detail = request_downstream_drain(&state, &node_id, request.reason.as_deref())
        .await
        .map_err(|error| control_error(StatusCode::BAD_GATEWAY, "downstream_unavailable", error))?;
    let _ = state
        .node_registry()
        .set_availability(&node_id, NodeAvailability::Draining);

    state.client.events().emit_log(
        EventLogLevel::Info,
        format!("internal downstream drain {node_id}"),
        [
            format!("reason={}", request.reason.as_deref().unwrap_or_default()),
            detail.clone(),
        ],
    );

    Ok(Json(ControlActionResponse {
        ok: true,
        action: "downstream_drain",
        target: Some(node_id),
        detail: Some(detail),
    }))
}

/// Send one ready request to one connected downstream node.
pub async fn control_send_downstream_ready(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeReadyParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    let detail = request_downstream_ready(&state, &node_id, request.reason.as_deref())
        .await
        .map_err(|error| control_error(StatusCode::BAD_GATEWAY, "downstream_unavailable", error))?;
    let _ = state
        .node_registry()
        .set_availability(&node_id, NodeAvailability::Ready);

    state.client.events().emit_log(
        EventLogLevel::Info,
        format!("internal downstream ready {node_id}"),
        [
            format!("reason={}", request.reason.as_deref().unwrap_or_default()),
            detail.clone(),
        ],
    );

    Ok(Json(ControlActionResponse {
        ok: true,
        action: "downstream_ready",
        target: Some(node_id),
        detail: Some(detail),
    }))
}

/// Send one isolation request to one connected downstream node.
pub async fn control_send_downstream_isolate(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeIsolateParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    let detail = request_downstream_isolate(&state, &node_id, request.reason.as_deref())
        .await
        .map_err(|error| control_error(StatusCode::BAD_GATEWAY, "downstream_unavailable", error))?;
    let _ = state
        .node_registry()
        .set_availability(&node_id, NodeAvailability::Isolated);

    state.client.events().emit_log(
        EventLogLevel::Info,
        format!("internal downstream isolate {node_id}"),
        [
            format!("reason={}", request.reason.as_deref().unwrap_or_default()),
            detail.clone(),
        ],
    );

    Ok(Json(ControlActionResponse {
        ok: true,
        action: "downstream_isolate",
        target: Some(node_id),
        detail: Some(detail),
    }))
}

/// Send one capacity update request to one connected downstream node.
pub async fn control_send_downstream_capacity(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeCapacityParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    let detail = request_downstream_capacity(
        &state,
        &node_id,
        request.max_concurrent_tasks,
        request.reason.as_deref(),
    )
    .await
    .map_err(|error| control_error(StatusCode::BAD_GATEWAY, "downstream_unavailable", error))?;
    let _ = state
        .node_registry()
        .set_capacity(&node_id, request.max_concurrent_tasks);

    state.client.events().emit_log(
        EventLogLevel::Info,
        format!("internal downstream capacity {node_id}"),
        [
            format!(
                "max_concurrent_tasks={}",
                request.max_concurrent_tasks.unwrap_or_default()
            ),
            format!("reason={}", request.reason.as_deref().unwrap_or_default()),
            detail.clone(),
        ],
    );

    Ok(Json(ControlActionResponse {
        ok: true,
        action: "downstream_capacity",
        target: Some(node_id),
        detail: Some(detail),
    }))
}

/// Broadcast one drain request to every connected downstream node.
pub async fn control_broadcast_downstream_drain(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeDrainParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    let node_ids = downstream_node_ids(&state);
    let mut drained = 0usize;
    let mut failures = Vec::new();
    for node_id in node_ids {
        match request_downstream_drain(&state, &node_id, request.reason.as_deref()).await {
            Ok(_) => {
                let _ = state
                    .node_registry()
                    .set_availability(&node_id, NodeAvailability::Draining);
                drained += 1;
            }
            Err(error) => failures.push(format!("{node_id}={error}")),
        }
    }

    let detail = if failures.is_empty() {
        format!(
            "drain requested for {drained} downstream node(s); reason={}",
            request.reason.as_deref().unwrap_or_default()
        )
    } else {
        format!(
            "drain requested for {drained} downstream node(s), failed={} ; reason={}",
            failures.join(","),
            request.reason.as_deref().unwrap_or_default()
        )
    };
    state.client.events().emit_log(
        EventLogLevel::Info,
        "internal downstream drain broadcast",
        [
            format!("count={drained}"),
            format!("reason={}", request.reason.as_deref().unwrap_or_default()),
            format!("failures={}", failures.join(",")),
        ],
    );

    Ok(Json(ControlActionResponse {
        ok: failures.is_empty(),
        action: "broadcast_downstream_drain",
        target: Some("downstream:*".to_owned()),
        detail: Some(detail),
    }))
}

/// Broadcast one ready request to every connected downstream node.
pub async fn control_broadcast_downstream_ready(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeReadyParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    let node_ids = downstream_node_ids(&state);
    let mut readied = 0usize;
    let mut failures = Vec::new();
    for node_id in node_ids {
        match request_downstream_ready(&state, &node_id, request.reason.as_deref()).await {
            Ok(_) => {
                let _ = state
                    .node_registry()
                    .set_availability(&node_id, NodeAvailability::Ready);
                readied += 1;
            }
            Err(error) => failures.push(format!("{node_id}={error}")),
        }
    }

    let detail = if failures.is_empty() {
        format!(
            "ready requested for {readied} downstream node(s); reason={}",
            request.reason.as_deref().unwrap_or_default()
        )
    } else {
        format!(
            "ready requested for {readied} downstream node(s), failed={} ; reason={}",
            failures.join(","),
            request.reason.as_deref().unwrap_or_default()
        )
    };
    state.client.events().emit_log(
        EventLogLevel::Info,
        "internal downstream ready broadcast",
        [
            format!("count={readied}"),
            format!("reason={}", request.reason.as_deref().unwrap_or_default()),
            format!("failures={}", failures.join(",")),
        ],
    );

    Ok(Json(ControlActionResponse {
        ok: failures.is_empty(),
        action: "broadcast_downstream_ready",
        target: Some("downstream:*".to_owned()),
        detail: Some(detail),
    }))
}

/// Broadcast one isolation request to every connected downstream node.
pub async fn control_broadcast_downstream_isolate(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeIsolateParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    let node_ids = downstream_node_ids(&state);
    let mut isolated = 0usize;
    let mut failures = Vec::new();
    for node_id in node_ids {
        match request_downstream_isolate(&state, &node_id, request.reason.as_deref()).await {
            Ok(_) => {
                let _ = state
                    .node_registry()
                    .set_availability(&node_id, NodeAvailability::Isolated);
                isolated += 1;
            }
            Err(error) => failures.push(format!("{node_id}={error}")),
        }
    }

    let detail = if failures.is_empty() {
        format!(
            "isolation requested for {isolated} downstream node(s); reason={}",
            request.reason.as_deref().unwrap_or_default()
        )
    } else {
        format!(
            "isolation requested for {isolated} downstream node(s), failed={} ; reason={}",
            failures.join(","),
            request.reason.as_deref().unwrap_or_default()
        )
    };
    state.client.events().emit_log(
        EventLogLevel::Info,
        "internal downstream isolate broadcast",
        [
            format!("count={isolated}"),
            format!("reason={}", request.reason.as_deref().unwrap_or_default()),
            format!("failures={}", failures.join(",")),
        ],
    );

    Ok(Json(ControlActionResponse {
        ok: failures.is_empty(),
        action: "broadcast_downstream_isolate",
        target: Some("downstream:*".to_owned()),
        detail: Some(detail),
    }))
}

/// Broadcast one capacity update request to every connected downstream node.
pub async fn control_broadcast_downstream_capacity(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeCapacityParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    let node_ids = downstream_node_ids(&state);
    let mut updated_count = 0usize;
    let mut failures = Vec::new();
    for node_id in node_ids {
        match request_downstream_capacity(
            &state,
            &node_id,
            request.max_concurrent_tasks,
            request.reason.as_deref(),
        )
        .await
        {
            Ok(_) => {
                let _ = state
                    .node_registry()
                    .set_capacity(&node_id, request.max_concurrent_tasks);
                updated_count += 1;
            }
            Err(error) => failures.push(format!("{node_id}={error}")),
        }
    }

    let detail = if failures.is_empty() {
        format!(
            "capacity requested for {updated_count} downstream node(s); max_concurrent_tasks={} ; reason={}",
            request.max_concurrent_tasks.unwrap_or_default(),
            request.reason.as_deref().unwrap_or_default()
        )
    } else {
        format!(
            "capacity requested for {updated_count} downstream node(s), failed={} ; max_concurrent_tasks={} ; reason={}",
            failures.join(","),
            request.max_concurrent_tasks.unwrap_or_default(),
            request.reason.as_deref().unwrap_or_default()
        )
    };
    state.client.events().emit_log(
        EventLogLevel::Info,
        "internal downstream capacity broadcast",
        [
            format!("count={updated_count}"),
            format!(
                "max_concurrent_tasks={}",
                request.max_concurrent_tasks.unwrap_or_default()
            ),
            format!("reason={}", request.reason.as_deref().unwrap_or_default()),
            format!("failures={}", failures.join(",")),
        ],
    );

    Ok(Json(ControlActionResponse {
        ok: failures.is_empty(),
        action: "broadcast_downstream_capacity",
        target: Some("downstream:*".to_owned()),
        detail: Some(detail),
    }))
}

/// Send one shutdown notice to one connected downstream node.
pub async fn control_send_downstream_shutdown_notice(
    Path(node_id): Path<String>,
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeShutdownNoticeParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    state
        .send_shutdown_notice_to_node(
            &node_id,
            request.reason.as_deref(),
            request.reconnect_after_ms,
        )
        .map_err(|error| {
            control_error(
                StatusCode::NOT_FOUND,
                "downstream_not_found",
                error.to_string(),
            )
        })?;

    state.client.events().emit_log(
        EventLogLevel::Info,
        format!("internal downstream shutdown_notice {node_id}"),
        [
            format!("reason={}", request.reason.as_deref().unwrap_or_default()),
            format!(
                "reconnect_after_ms={}",
                request.reconnect_after_ms.unwrap_or_default()
            ),
        ],
    );

    Ok(Json(ControlActionResponse {
        ok: true,
        action: "downstream_shutdown_notice",
        target: Some(node_id),
        detail: request.reason,
    }))
}

/// Broadcast one shutdown notice to every connected downstream node.
pub async fn control_broadcast_downstream_shutdown_notice(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeShutdownNoticeParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    let count = state
        .broadcast_shutdown_notice_count(request.reason.as_deref(), request.reconnect_after_ms);
    let detail = format!(
        "broadcast to {count} downstream node(s); reason={}",
        request.reason.as_deref().unwrap_or_default()
    );
    state.client.events().emit_log(
        EventLogLevel::Info,
        "internal downstream shutdown_notice broadcast",
        [
            format!("count={count}"),
            format!("reason={}", request.reason.as_deref().unwrap_or_default()),
            format!(
                "reconnect_after_ms={}",
                request.reconnect_after_ms.unwrap_or_default()
            ),
        ],
    );

    Ok(Json(ControlActionResponse {
        ok: true,
        action: "broadcast_downstream_shutdown_notice",
        target: Some("downstream:*".to_owned()),
        detail: Some(detail),
    }))
}

/// Mark the current node as draining and stop accepting new node-routed tasks.
pub async fn control_set_local_drain(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeDrainParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    let notes = state.enter_local_node_drain(request.reason.as_deref());

    state.client.events().emit_log(
        EventLogLevel::Info,
        "internal local drain",
        [
            format!("reason={}", request.reason.as_deref().unwrap_or_default()),
            notes.join("; "),
        ],
    );

    Ok(Json(ControlActionResponse {
        ok: true,
        action: "local_drain",
        target: state.node_id().map(str::to_owned),
        detail: Some(notes.join("; ")),
    }))
}

/// Mark the current node as isolated and stop accepting new node-routed tasks.
pub async fn control_set_local_isolated(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeIsolateParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    let notes = state.enter_local_node_isolation(request.reason.as_deref());

    state.client.events().emit_log(
        EventLogLevel::Info,
        "internal local isolate",
        [
            format!("reason={}", request.reason.as_deref().unwrap_or_default()),
            notes.join("; "),
        ],
    );

    Ok(Json(ControlActionResponse {
        ok: true,
        action: "local_isolate",
        target: state.node_id().map(str::to_owned),
        detail: Some(notes.join("; ")),
    }))
}

/// Set or clear the local node capacity override and refresh upstream advertisement state.
pub async fn control_set_local_capacity(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeCapacityParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    state.set_local_node_capacity_override(request.max_concurrent_tasks);
    let detail = match request.max_concurrent_tasks {
        Some(limit) => format!("local capacity override set to {limit}"),
        None => "local capacity override cleared".to_owned(),
    };

    let mut notes = vec![detail.clone()];
    if state.node_connect_upstream().is_some() {
        match state
            .announce_upstream_capacity(request.max_concurrent_tasks, request.reason.as_deref())
        {
            Ok(()) => notes.push("upstream capacity update announced".to_owned()),
            Err(envelope) => notes.push(format!(
                "upstream capacity update skipped because no session is ready for `{}`",
                envelope.method
            )),
        }

        match state.announce_upstream_advertisement(state.local_node_active_tasks()) {
            Ok(()) => notes.push("upstream advertisement refreshed".to_owned()),
            Err(envelope) => notes.push(format!(
                "upstream advertisement skipped because no session is ready for `{}`",
                envelope.method
            )),
        }
    }

    state.client.events().emit_log(
        EventLogLevel::Info,
        "internal local capacity",
        [
            format!(
                "max_concurrent_tasks={}",
                request.max_concurrent_tasks.unwrap_or_default()
            ),
            format!("reason={}", request.reason.as_deref().unwrap_or_default()),
            notes.join("; "),
        ],
    );

    Ok(Json(ControlActionResponse {
        ok: true,
        action: "local_capacity",
        target: state.node_id().map(str::to_owned),
        detail: Some(notes.join("; ")),
    }))
}

/// Mark the current node as ready again and synchronise readiness upstream.
pub async fn control_set_local_ready(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    let notes = state.restore_local_node_ready(None);

    state.client.events().emit_log(
        EventLogLevel::Info,
        "internal local ready",
        [notes.join("; ")],
    );

    Ok(Json(ControlActionResponse {
        ok: true,
        action: "local_ready",
        target: state.node_id().map(str::to_owned),
        detail: Some(notes.join("; ")),
    }))
}

/// Announce that the current node is draining to its upstream session.
pub async fn control_announce_upstream_drain(
    headers: HeaderMap,
    State(state): State<AppState>,
    Json(request): Json<NodeDrainParams>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    state
        .announce_upstream_drain(request.reason.as_deref())
        .map_err(|envelope| {
            control_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "upstream_unavailable",
                format!(
                    "failed to send `{}` because no upstream session is ready",
                    envelope.method
                ),
            )
        })?;

    state.client.events().emit_log(
        EventLogLevel::Info,
        "internal upstream drain",
        [format!(
            "reason={}",
            request.reason.as_deref().unwrap_or_default()
        )],
    );

    Ok(Json(ControlActionResponse {
        ok: true,
        action: "upstream_drain",
        target: Some("upstream".to_owned()),
        detail: request.reason,
    }))
}

/// Claim every published platform on the current upstream session.
pub async fn control_claim_published_upstream_routes(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    state
        .claim_published_upstream_routes()
        .map_err(|envelope| {
            control_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "upstream_unavailable",
                format!(
                    "failed to send `{}` because no upstream session is ready",
                    envelope.method
                ),
            )
        })?;

    state.client.events().emit_log(
        EventLogLevel::Info,
        "internal upstream route claim",
        std::iter::empty::<String>(),
    );

    Ok(Json(ControlActionResponse {
        ok: true,
        action: "claim_published_upstream_routes",
        target: Some("upstream".to_owned()),
        detail: None,
    }))
}

/// Release every published platform from the current upstream session.
pub async fn control_release_published_upstream_routes(
    headers: HeaderMap,
    State(state): State<AppState>,
) -> ControlResult<ControlActionResponse> {
    authorize_control(&state, &headers)?;

    state
        .release_published_upstream_routes()
        .map_err(|envelope| {
            control_error(
                StatusCode::SERVICE_UNAVAILABLE,
                "upstream_unavailable",
                format!(
                    "failed to send `{}` because no upstream session is ready",
                    envelope.method
                ),
            )
        })?;

    state.client.events().emit_log(
        EventLogLevel::Info,
        "internal upstream route release",
        std::iter::empty::<String>(),
    );

    Ok(Json(ControlActionResponse {
        ok: true,
        action: "release_published_upstream_routes",
        target: Some("upstream".to_owned()),
        detail: None,
    }))
}

fn current_runtime_routes(state: &AppState) -> Vec<RuntimeRouteSummary> {
    state
        .runtime_platform_routes()
        .into_iter()
        .map(|(platform, route_node)| RuntimeRouteSummary {
            platform,
            route_node,
        })
        .collect()
}

fn downstream_node_ids(state: &AppState) -> Vec<String> {
    state
        .node_registry()
        .records(state.node_heartbeat_ms())
        .into_iter()
        .map(|record| record.node_id)
        .collect()
}

struct DownstreamRpcRequest {
    method: &'static str,
    params: Value,
    completion: &'static str,
    success_detail: &'static str,
}

async fn request_downstream_rpc(
    state: &AppState,
    node_id: &str,
    request: DownstreamRpcRequest,
) -> Result<String, String> {
    let registry = state.node_registry();
    let Some((record, sender)) = registry.sender_for_node(node_id) else {
        return Err(format!("downstream node `{node_id}` is not connected"));
    };

    let mut envelope = NodeEnvelope::request(request.method, request.params);
    envelope.from = state.node_id().map(str::to_owned);
    envelope.to = Some(record.node_id.clone());
    envelope.session_id = Some(record.session_id.clone());
    envelope.trace_id = Some(new_message_id("trace"));

    let request_id = envelope.id.clone();
    let pending = registry.register_pending(request_id.clone(), record, sender.clone());
    if sender.send(envelope).is_err() {
        registry.cancel_pending(&request_id);
        return Err(format!("downstream node `{node_id}` is no longer writable"));
    }

    let timeout_ms = state.node_request_timeout_ms().unwrap_or(15_000);
    let response = match tokio::time::timeout(Duration::from_millis(timeout_ms), pending).await {
        Ok(Ok(response)) => response,
        Ok(Err(_)) => {
            registry.cancel_pending(&request_id);
            return Err(format!(
                "downstream node `{node_id}` response channel closed before {} completed",
                request.completion
            ));
        }
        Err(_) => {
            registry.cancel_pending(&request_id);
            return Err(format!(
                "downstream node `{node_id}` did not confirm {} within {timeout_ms}ms",
                request.completion
            ));
        }
    };

    if let Some(error) = response.error {
        return Err(format!(
            "downstream node `{node_id}` returned {} for `{}`: {}",
            error.code, response.method, error.message
        ));
    }
    if response.method != request.method {
        return Err(format!(
            "downstream node `{node_id}` returned unexpected response method `{}`",
            response.method
        ));
    }

    Ok(response
        .result
        .as_ref()
        .and_then(|value| value.get("detail"))
        .and_then(serde_json::Value::as_str)
        .map(str::to_owned)
        .unwrap_or_else(|| request.success_detail.to_owned()))
}

async fn request_downstream_drain(
    state: &AppState,
    node_id: &str,
    reason: Option<&str>,
) -> Result<String, String> {
    request_downstream_rpc(
        state,
        node_id,
        DownstreamRpcRequest {
            method: "node.drain",
            params: json!(NodeDrainParams {
                reason: reason.map(str::to_owned),
            }),
            completion: "drain",
            success_detail: "downstream node accepted drain",
        },
    )
    .await
}

async fn request_downstream_ready(
    state: &AppState,
    node_id: &str,
    reason: Option<&str>,
) -> Result<String, String> {
    request_downstream_rpc(
        state,
        node_id,
        DownstreamRpcRequest {
            method: "node.ready",
            params: json!(NodeReadyParams {
                reason: reason.map(str::to_owned),
            }),
            completion: "ready",
            success_detail: "downstream node accepted ready",
        },
    )
    .await
}

async fn request_downstream_isolate(
    state: &AppState,
    node_id: &str,
    reason: Option<&str>,
) -> Result<String, String> {
    request_downstream_rpc(
        state,
        node_id,
        DownstreamRpcRequest {
            method: "node.isolate",
            params: json!(NodeIsolateParams {
                reason: reason.map(str::to_owned),
            }),
            completion: "isolation",
            success_detail: "downstream node accepted isolation",
        },
    )
    .await
}

async fn request_downstream_capacity(
    state: &AppState,
    node_id: &str,
    max_concurrent_tasks: Option<u32>,
    reason: Option<&str>,
) -> Result<String, String> {
    request_downstream_rpc(
        state,
        node_id,
        DownstreamRpcRequest {
            method: "node.capacity",
            params: json!(NodeCapacityParams {
                max_concurrent_tasks,
                reason: reason.map(str::to_owned),
            }),
            completion: "capacity update",
            success_detail: "downstream node accepted capacity update",
        },
    )
    .await
}

fn authorize_control(
    state: &AppState,
    headers: &HeaderMap,
) -> Result<(), (StatusCode, Json<ControlErrorResponse>)> {
    let Some(expected_token) = state.node_control_token() else {
        return Err(control_error(
            StatusCode::SERVICE_UNAVAILABLE,
            "node_control_unavailable",
            "node control token is not configured for this server",
        ));
    };

    let Some(value) = headers.get(AUTHORIZATION) else {
        return Err(control_error(
            StatusCode::UNAUTHORIZED,
            "control_unauthorized",
            "missing Authorization bearer token",
        ));
    };
    let Ok(value) = value.to_str() else {
        return Err(control_error(
            StatusCode::UNAUTHORIZED,
            "control_unauthorized",
            "invalid Authorization header encoding",
        ));
    };
    let candidate = value
        .strip_prefix("Bearer ")
        .or_else(|| value.strip_prefix("bearer "))
        .unwrap_or(value)
        .trim();
    if candidate != expected_token {
        return Err(control_error(
            StatusCode::UNAUTHORIZED,
            "control_unauthorized",
            "Authorization bearer token did not match",
        ));
    }

    Ok(())
}

fn control_error(
    status: StatusCode,
    error: &'static str,
    detail: impl Into<String>,
) -> (StatusCode, Json<ControlErrorResponse>) {
    (
        status,
        Json(ControlErrorResponse {
            error,
            detail: detail.into(),
        }),
    )
}
