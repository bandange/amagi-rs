use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::http::{HeaderMap, StatusCode, header::AUTHORIZATION};
use axum::response::{IntoResponse, Response};
use futures_util::{SinkExt, StreamExt};
use serde_json::{Value, json};
use tokio::sync::mpsc;
use tracing::{info, warn};

use crate::error::AppError;
use crate::events::EventLogLevel;
use crate::node::NodeRole;
use crate::node::protocol::{
    AUTH_MAX_SKEW_MS, NODE_HANDSHAKE_NODE_ID_HEADER, NodeAdvertiseParams, NodeAuthParams,
    NodeCapacityParams, NodeDrainParams, NodeEnvelope, NodeHeartbeatParams, NodeHelloAck,
    NodeHelloParams, NodeIsolateParams, NodeReadyParams, NodeRouteUpdateParams,
    NodeTaskProgressParams, now_ms,
};
use crate::node::registry::{NodeAvailability, NodeRecord};
use crate::node::session::NodeSessionState;

use super::super::state::AppState;

/// Upgrade a downstream node connection into a WSS session when enabled.
pub async fn node_websocket(
    ws: WebSocketUpgrade,
    headers: HeaderMap,
    State(state): State<AppState>,
) -> Response {
    if !state.accepts_downstream_nodes() {
        return StatusCode::NOT_FOUND.into_response();
    }

    let handshake_auth = match parse_handshake_auth(&headers) {
        Ok(handshake_auth) => handshake_auth,
        Err(response) => return response,
    };
    if let Some(handshake_auth) = handshake_auth.as_ref() {
        if let Err(error) =
            state.validate_incoming_node_auth(&handshake_auth.node_id, &handshake_auth.token)
        {
            warn!(
                node_id = %handshake_auth.node_id,
                error = %error,
                "downstream websocket handshake bearer auth failed"
            );
            return StatusCode::UNAUTHORIZED.into_response();
        }
    }

    ws.on_upgrade(move |socket| async move {
        if let Err(error) = handle_node_socket(state, socket, handshake_auth).await {
            warn!(error = %error, "downstream node session failed");
        }
    })
}

async fn handle_node_socket(
    state: AppState,
    socket: WebSocket,
    handshake_auth: Option<NodeHandshakeAuth>,
) -> Result<(), AppError> {
    let session_id = crate::node::protocol::new_message_id("sess");
    let registry = state.node_registry();
    let local_node_id = state.node_id().unwrap_or("server").to_owned();
    let local_role = state
        .node_role()
        .map(|role| format!("{role:?}").to_lowercase())
        .unwrap_or_else(|| "server".to_owned());
    let (mut sink, mut source) = socket.split();
    let (tx, mut rx) = mpsc::unbounded_channel::<NodeEnvelope>();

    let writer = tokio::spawn(async move {
        while let Some(envelope) = rx.recv().await {
            let payload = match serde_json::to_string(&envelope) {
                Ok(payload) => payload,
                Err(_) => break,
            };
            if sink.send(Message::Text(payload.into())).await.is_err() {
                break;
            }
        }
    });

    let auth_message = read_text_envelope(&mut source).await?;
    if auth_message.method != "node.auth" {
        let _ = tx.send(NodeEnvelope::response_error(
            auth_message.id,
            auth_message.method,
            Some(local_node_id.clone()),
            None,
            Some(session_id.clone()),
            "node_protocol_invalid",
            "the first node message must be `node.auth`",
        ));
        return Ok(());
    }

    let auth: NodeAuthParams = match auth_message.parse_params() {
        Ok(auth) => auth,
        Err(error) => {
            respond_handshake_error(
                &tx,
                auth_message.id,
                "node.auth",
                &local_node_id,
                None,
                &session_id,
                "node_protocol_invalid",
                error.to_string(),
            );
            return Ok(());
        }
    };
    if let Some(handshake_auth) = handshake_auth.as_ref() {
        if auth.node_id != handshake_auth.node_id || auth.token != handshake_auth.token {
            warn!(
                handshake_node_id = %handshake_auth.node_id,
                auth_node_id = %auth.node_id,
                "downstream node auth frame did not match websocket handshake headers"
            );
            respond_handshake_error(
                &tx,
                auth_message.id,
                "node.auth",
                &local_node_id,
                Some(auth.node_id.as_str()),
                &session_id,
                "node_auth_failed",
                "node.auth must match the authenticated websocket handshake".to_owned(),
            );
            return Ok(());
        }
    }
    if let Err(error) = validate_node_auth(&state, &auth) {
        warn!(
            node_id = %auth.node_id,
            session_id = %session_id,
            error = %error,
            "downstream node auth failed"
        );
        respond_handshake_error(
            &tx,
            auth_message.id,
            "node.auth",
            &local_node_id,
            Some(auth.node_id.as_str()),
            &session_id,
            "node_auth_failed",
            error.to_string(),
        );
        return Ok(());
    }

    let _ = tx.send(NodeEnvelope::response_ok(
        auth_message.id,
        "node.auth",
        Some(local_node_id.clone()),
        Some(auth.node_id.clone()),
        Some(session_id.clone()),
        json!({ "accepted": true }),
    ));

    registry.upsert(
        NodeRecord {
            session_id: session_id.clone(),
            node_id: auth.node_id.clone(),
            role: NodeRole::Hybrid,
            version: None,
            session_state: NodeSessionState::Authenticating,
            capabilities: Vec::new(),
            platforms: Vec::new(),
            availability: NodeAvailability::Ready,
            max_concurrent_tasks: None,
            active_tasks: 0,
            connected_at_ms: now_ms(),
            last_seen_ms: now_ms(),
        },
        tx.clone(),
    );

    let hello_message = match read_text_envelope(&mut source).await {
        Ok(hello_message) => hello_message,
        Err(error) => {
            registry.remove_if_session(&auth.node_id, &session_id);
            return Err(error);
        }
    };
    if hello_message.method != "node.hello" {
        registry.remove_if_session(&auth.node_id, &session_id);
        let _ = tx.send(NodeEnvelope::response_error(
            hello_message.id,
            hello_message.method,
            Some(local_node_id.clone()),
            Some(auth.node_id.clone()),
            Some(session_id.clone()),
            "node_protocol_invalid",
            "the second node message must be `node.hello`",
        ));
        return Ok(());
    }

    let hello: NodeHelloParams = match hello_message.parse_params() {
        Ok(hello) => hello,
        Err(error) => {
            registry.remove_if_session(&auth.node_id, &session_id);
            respond_handshake_error(
                &tx,
                hello_message.id,
                "node.hello",
                &local_node_id,
                Some(auth.node_id.as_str()),
                &session_id,
                "node_protocol_invalid",
                error.to_string(),
            );
            return Ok(());
        }
    };
    if hello.node_id != auth.node_id {
        registry.remove_if_session(&auth.node_id, &session_id);
        let _ = tx.send(NodeEnvelope::response_error(
            hello_message.id,
            "node.hello",
            Some(local_node_id.clone()),
            Some(auth.node_id.clone()),
            Some(session_id.clone()),
            "node_auth_failed",
            "node.hello node_id must match the authenticated node id",
        ));
        return Ok(());
    }

    registry.upsert(
        NodeRecord {
            session_id: session_id.clone(),
            node_id: hello.node_id.clone(),
            role: hello.role,
            version: Some(hello.version.clone()),
            session_state: NodeSessionState::Ready,
            capabilities: hello.capabilities.clone(),
            platforms: hello.platforms.clone(),
            availability: NodeAvailability::Ready,
            max_concurrent_tasks: None,
            active_tasks: 0,
            connected_at_ms: now_ms(),
            last_seen_ms: now_ms(),
        },
        tx.clone(),
    );

    info!(
        node_id = %hello.node_id,
        session_id = %session_id,
        remote_role = %format!("{:?}", hello.role).to_lowercase(),
        local_role = %local_role,
        "downstream node session ready"
    );

    let _ = tx.send(NodeEnvelope::response_ok(
        hello_message.id,
        "node.hello",
        Some(local_node_id.clone()),
        Some(hello.node_id.clone()),
        Some(session_id.clone()),
        json!(NodeHelloAck {
            state: "ready".to_owned(),
            session_id: session_id.clone(),
            node_id: local_node_id.clone(),
            role: state.node_role().unwrap_or(NodeRole::Hybrid),
            version: state.version.to_owned(),
            capabilities: state.node_capabilities(),
            platforms: state.published_platform_names(),
        }),
    ));

    while let Some(message) = source.next().await {
        match message {
            Ok(Message::Text(text)) => {
                let envelope: NodeEnvelope = serde_json::from_str(&text).map_err(AppError::from)?;
                match envelope.kind.clone() {
                    crate::node::protocol::NodeEnvelopeKind::Response => {
                        registry.fulfill_pending(envelope);
                    }
                    crate::node::protocol::NodeEnvelopeKind::Event => {
                        if envelope.method == "node.heartbeat" {
                            let heartbeat: NodeHeartbeatParams = envelope.parse_params()?;
                            registry.update_last_seen(&hello.node_id, heartbeat.timestamp_ms);
                            let _ = tx.send(NodeEnvelope::event(
                                "node.heartbeat",
                                Some(local_node_id.clone()),
                                Some(hello.node_id.clone()),
                                Some(session_id.clone()),
                                json!(NodeHeartbeatParams {
                                    timestamp_ms: now_ms(),
                                }),
                            ));
                            continue;
                        }

                        if envelope.method == "task.progress" {
                            handle_task_progress_event(&state, &hello.node_id, envelope)?;
                        }
                    }
                    crate::node::protocol::NodeEnvelopeKind::Request => {
                        if envelope.method == "node.advertise" {
                            handle_node_advertise_event(
                                &state,
                                &hello.node_id,
                                &local_node_id,
                                &session_id,
                                &tx,
                                envelope,
                            )?;
                            continue;
                        }

                        if envelope.method == "node.drain" {
                            handle_node_drain_request(
                                &state,
                                &hello.node_id,
                                &local_node_id,
                                &session_id,
                                &tx,
                                envelope,
                            )?;
                            continue;
                        }

                        if envelope.method == "node.ready" {
                            handle_node_ready_request(
                                &state,
                                &hello.node_id,
                                &local_node_id,
                                &session_id,
                                &tx,
                                envelope,
                            )?;
                            continue;
                        }

                        if envelope.method == "node.isolate" {
                            handle_node_isolate_request(
                                &state,
                                &hello.node_id,
                                &local_node_id,
                                &session_id,
                                &tx,
                                envelope,
                            )?;
                            continue;
                        }

                        if envelope.method == "node.capacity" {
                            handle_node_capacity_request(
                                &state,
                                &hello.node_id,
                                &local_node_id,
                                &session_id,
                                &tx,
                                envelope,
                            )?;
                            continue;
                        }

                        if envelope.method == "route.update" {
                            handle_route_update_request(
                                &state,
                                &hello.node_id,
                                &local_node_id,
                                &session_id,
                                &tx,
                                envelope,
                            )?;
                        }
                    }
                }
            }
            Ok(Message::Ping(_)) | Ok(Message::Pong(_)) | Ok(Message::Binary(_)) => {}
            Ok(Message::Close(_)) => break,
            Err(error) => {
                if registry.remove_if_session(&hello.node_id, &session_id) {
                    state.clear_runtime_platform_routes_for_node(&hello.node_id);
                }
                return Err(AppError::Io(std::io::Error::other(format!(
                    "downstream websocket read failed: {error}"
                ))));
            }
        }
    }

    if registry.remove_if_session(&hello.node_id, &session_id) {
        state.clear_runtime_platform_routes_for_node(&hello.node_id);
    }
    drop(tx);
    let _ = writer.await;
    info!(node_id = %hello.node_id, "downstream node session closed");

    Ok(())
}

#[derive(Debug, Clone)]
struct NodeHandshakeAuth {
    node_id: String,
    token: String,
}

fn parse_handshake_auth(headers: &HeaderMap) -> Result<Option<NodeHandshakeAuth>, Response> {
    let node_id = headers
        .get(NODE_HANDSHAKE_NODE_ID_HEADER)
        .map(|value| value.to_str().map(str::trim).map(str::to_owned));
    let node_id = match node_id {
        Some(Ok(node_id)) if !node_id.is_empty() => Some(node_id),
        Some(Ok(_)) => return Err(StatusCode::UNAUTHORIZED.into_response()),
        Some(Err(_)) => return Err(StatusCode::UNAUTHORIZED.into_response()),
        None => None,
    };

    let token = headers
        .get(AUTHORIZATION)
        .map(|value| value.to_str().map(parse_bearer_token));
    let token = match token {
        Some(Ok(Some(token))) => Some(token),
        Some(Ok(None)) => return Err(StatusCode::UNAUTHORIZED.into_response()),
        Some(Err(_)) => return Err(StatusCode::UNAUTHORIZED.into_response()),
        None => None,
    };

    match (node_id, token) {
        (Some(node_id), Some(token)) => Ok(Some(NodeHandshakeAuth { node_id, token })),
        (None, None) => Ok(None),
        _ => Err(StatusCode::UNAUTHORIZED.into_response()),
    }
}

fn parse_bearer_token(value: &str) -> Option<String> {
    let candidate = value
        .strip_prefix("Bearer ")
        .or_else(|| value.strip_prefix("bearer "))
        .unwrap_or(value)
        .trim();
    (!candidate.is_empty()).then(|| candidate.to_owned())
}

fn send_request_ok_response(
    tx: &mpsc::UnboundedSender<NodeEnvelope>,
    request: &NodeEnvelope,
    method: &str,
    local_node_id: &str,
    downstream_node_id: &str,
    session_id: &str,
    result: Value,
) {
    let mut response = NodeEnvelope::response_ok(
        request.id.clone(),
        method,
        Some(local_node_id.to_owned()),
        Some(downstream_node_id.to_owned()),
        Some(session_id.to_owned()),
        result,
    );
    response.trace_id = request.trace_id.clone();
    response.hop_count = request.hop_count;
    response.deadline_ms = request.deadline_ms;
    let _ = tx.send(response);
}

fn handle_route_update_request(
    state: &AppState,
    downstream_node_id: &str,
    local_node_id: &str,
    session_id: &str,
    tx: &mpsc::UnboundedSender<NodeEnvelope>,
    envelope: NodeEnvelope,
) -> Result<(), AppError> {
    let updates: NodeRouteUpdateParams = envelope.parse_params()?;
    let mut applied = Vec::with_capacity(updates.updates.len());

    for update in updates.updates {
        match update.route_node.as_deref() {
            Some(route_node) if route_node != downstream_node_id => {
                return Err(AppError::InvalidRequestConfig(format!(
                    "node `{downstream_node_id}` cannot assign platform `{}` to `{route_node}`",
                    update.platform
                )));
            }
            Some(route_node) => {
                state.set_runtime_platform_route(update.platform, route_node.to_owned());
                applied.push(format!("{}=node:{route_node}", update.platform));
            }
            None => {
                let cleared =
                    state.clear_runtime_platform_route(update.platform, downstream_node_id);
                if cleared {
                    applied.push(format!("{}=cleared", update.platform));
                }
            }
        }
    }

    info!(
        node_id = %downstream_node_id,
        applied = ?applied,
        "downstream node route update applied"
    );
    state.client.events().emit_log(
        EventLogLevel::Info,
        format!("node route update {downstream_node_id}"),
        applied,
    );

    send_request_ok_response(
        tx,
        &envelope,
        "route.update",
        local_node_id,
        downstream_node_id,
        session_id,
        json!({ "updated": true }),
    );

    Ok(())
}

fn handle_node_drain_request(
    state: &AppState,
    downstream_node_id: &str,
    local_node_id: &str,
    session_id: &str,
    tx: &mpsc::UnboundedSender<NodeEnvelope>,
    envelope: NodeEnvelope,
) -> Result<(), AppError> {
    let drain: NodeDrainParams = envelope.parse_params()?;
    let updated = state
        .node_registry()
        .set_availability(downstream_node_id, NodeAvailability::Draining);
    if !updated {
        return Err(AppError::InvalidRequestConfig(format!(
            "cannot drain offline node `{downstream_node_id}`"
        )));
    }

    info!(
        node_id = %downstream_node_id,
        reason = ?drain.reason,
        "downstream node entered draining state"
    );
    state.client.events().emit_log(
        EventLogLevel::Info,
        format!("node drain {downstream_node_id}"),
        [format!(
            "reason={}",
            drain.reason.as_deref().unwrap_or_default()
        )],
    );

    send_request_ok_response(
        tx,
        &envelope,
        "node.drain",
        local_node_id,
        downstream_node_id,
        session_id,
        json!({ "draining": true }),
    );

    Ok(())
}

fn handle_node_ready_request(
    state: &AppState,
    downstream_node_id: &str,
    local_node_id: &str,
    session_id: &str,
    tx: &mpsc::UnboundedSender<NodeEnvelope>,
    envelope: NodeEnvelope,
) -> Result<(), AppError> {
    let ready: NodeReadyParams = envelope.parse_params()?;
    let updated = state
        .node_registry()
        .set_availability(downstream_node_id, NodeAvailability::Ready);
    if !updated {
        return Err(AppError::InvalidRequestConfig(format!(
            "cannot mark offline node `{downstream_node_id}` ready"
        )));
    }

    info!(
        node_id = %downstream_node_id,
        reason = ?ready.reason,
        "downstream node exited restricted state"
    );
    state.client.events().emit_log(
        EventLogLevel::Info,
        format!("node ready {downstream_node_id}"),
        [format!(
            "reason={}",
            ready.reason.as_deref().unwrap_or_default()
        )],
    );

    send_request_ok_response(
        tx,
        &envelope,
        "node.ready",
        local_node_id,
        downstream_node_id,
        session_id,
        json!({ "ready": true }),
    );

    Ok(())
}

fn handle_node_isolate_request(
    state: &AppState,
    downstream_node_id: &str,
    local_node_id: &str,
    session_id: &str,
    tx: &mpsc::UnboundedSender<NodeEnvelope>,
    envelope: NodeEnvelope,
) -> Result<(), AppError> {
    let isolate: NodeIsolateParams = envelope.parse_params()?;
    let updated = state
        .node_registry()
        .set_availability(downstream_node_id, NodeAvailability::Isolated);
    if !updated {
        return Err(AppError::InvalidRequestConfig(format!(
            "cannot isolate offline node `{downstream_node_id}`"
        )));
    }

    info!(
        node_id = %downstream_node_id,
        reason = ?isolate.reason,
        "downstream node entered isolation state"
    );
    state.client.events().emit_log(
        EventLogLevel::Info,
        format!("node isolate {downstream_node_id}"),
        [format!(
            "reason={}",
            isolate.reason.as_deref().unwrap_or_default()
        )],
    );

    send_request_ok_response(
        tx,
        &envelope,
        "node.isolate",
        local_node_id,
        downstream_node_id,
        session_id,
        json!({ "isolated": true }),
    );

    Ok(())
}

fn handle_node_capacity_request(
    state: &AppState,
    downstream_node_id: &str,
    local_node_id: &str,
    session_id: &str,
    tx: &mpsc::UnboundedSender<NodeEnvelope>,
    envelope: NodeEnvelope,
) -> Result<(), AppError> {
    let capacity: NodeCapacityParams = envelope.parse_params()?;
    let updated = state
        .node_registry()
        .set_capacity(downstream_node_id, capacity.max_concurrent_tasks);
    if !updated {
        return Err(AppError::InvalidRequestConfig(format!(
            "cannot update capacity for offline node `{downstream_node_id}`"
        )));
    }

    info!(
        node_id = %downstream_node_id,
        max_concurrent_tasks = ?capacity.max_concurrent_tasks,
        reason = ?capacity.reason,
        "downstream node capacity updated"
    );
    state.client.events().emit_log(
        EventLogLevel::Info,
        format!("node capacity {downstream_node_id}"),
        [
            format!(
                "max_concurrent_tasks={}",
                capacity.max_concurrent_tasks.unwrap_or_default()
            ),
            format!("reason={}", capacity.reason.as_deref().unwrap_or_default()),
        ],
    );

    send_request_ok_response(
        tx,
        &envelope,
        "node.capacity",
        local_node_id,
        downstream_node_id,
        session_id,
        json!({ "max_concurrent_tasks": capacity.max_concurrent_tasks }),
    );

    Ok(())
}

fn handle_node_advertise_event(
    state: &AppState,
    downstream_node_id: &str,
    local_node_id: &str,
    session_id: &str,
    tx: &mpsc::UnboundedSender<NodeEnvelope>,
    envelope: NodeEnvelope,
) -> Result<(), AppError> {
    let advertise: NodeAdvertiseParams = envelope.parse_params()?;
    let updated = state.node_registry().update_advertisement(
        downstream_node_id,
        advertise.capabilities.clone(),
        advertise.platforms.clone(),
    );
    if !updated {
        return Err(AppError::InvalidRequestConfig(format!(
            "cannot update advertisement for offline node `{downstream_node_id}`"
        )));
    }
    let _ = state
        .node_registry()
        .set_capacity(downstream_node_id, advertise.max_concurrent_tasks);
    let _ = state.node_registry().set_active_tasks(
        downstream_node_id,
        advertise.active_tasks.unwrap_or_default(),
    );

    info!(
        node_id = %downstream_node_id,
        capabilities = ?advertise.capabilities,
        platforms = ?advertise.platforms,
        max_concurrent_tasks = ?advertise.max_concurrent_tasks,
        active_tasks = ?advertise.active_tasks,
        "downstream node advertisement updated"
    );
    state.client.events().emit_log(
        EventLogLevel::Info,
        format!("node advertise {downstream_node_id}"),
        [
            format!("capabilities={}", advertise.capabilities.join(",")),
            format!("platforms={}", advertise.platforms.join(",")),
            format!(
                "max_concurrent_tasks={}",
                advertise.max_concurrent_tasks.unwrap_or_default()
            ),
            format!(
                "active_tasks={}",
                advertise.active_tasks.unwrap_or_default()
            ),
        ],
    );

    send_request_ok_response(
        tx,
        &envelope,
        "node.advertise",
        local_node_id,
        downstream_node_id,
        session_id,
        json!({ "updated": true }),
    );

    Ok(())
}

fn handle_task_progress_event(
    state: &AppState,
    downstream_node_id: &str,
    envelope: NodeEnvelope,
) -> Result<(), AppError> {
    let progress: NodeTaskProgressParams = envelope.parse_params()?;
    let origin_node_id = envelope
        .from
        .as_deref()
        .unwrap_or(downstream_node_id)
        .to_owned();

    info!(
        node_id = %origin_node_id,
        downstream_session_node_id = %downstream_node_id,
        request_id = %progress.request_id,
        stage = %progress.stage,
        percent = ?progress.percent,
        trace_id = ?envelope.trace_id,
        "downstream node task progress"
    );
    state.client.events().emit_log(
        EventLogLevel::Info,
        format!("node task progress {origin_node_id} {}", progress.stage),
        [
            format!("request_id={}", progress.request_id),
            format!(
                "trace_id={}",
                envelope.trace_id.as_deref().unwrap_or_default()
            ),
            format!(
                "message={}",
                progress.message.as_deref().unwrap_or_default()
            ),
        ],
    );

    let mut forwarded = NodeEnvelope::event(
        "task.progress",
        envelope
            .from
            .clone()
            .or_else(|| Some(downstream_node_id.to_owned())),
        None,
        None,
        serde_json::to_value(&progress)?,
    );
    forwarded.trace_id = envelope.trace_id.clone();
    forwarded.hop_count = envelope.hop_count;
    forwarded.deadline_ms = envelope.deadline_ms;

    if let Err(forwarded) = state.send_upstream_envelope(forwarded) {
        if state.node_connect_upstream().is_some() {
            warn!(
                node_id = %downstream_node_id,
                method = %forwarded.method,
                "failed to forward task progress upstream because no upstream session is ready"
            );
        }
    }

    Ok(())
}

async fn read_text_envelope<S>(source: &mut S) -> Result<NodeEnvelope, AppError>
where
    S: futures_util::Stream<Item = Result<Message, axum::Error>> + Unpin,
{
    let Some(message) = source.next().await else {
        return Err(AppError::Io(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            "node websocket closed before handshake completed",
        )));
    };

    match message {
        Ok(Message::Text(text)) => serde_json::from_str(&text).map_err(AppError::from),
        Ok(other) => Err(AppError::InvalidRequestConfig(format!(
            "expected text node envelope, got websocket frame {other:?}"
        ))),
        Err(error) => Err(AppError::Io(std::io::Error::other(format!(
            "failed to read node websocket frame: {error}"
        )))),
    }
}

fn validate_node_auth(state: &AppState, auth: &NodeAuthParams) -> Result<(), AppError> {
    state.validate_incoming_node_auth(&auth.node_id, &auth.token)?;

    let skew = now_ms().abs_diff(auth.timestamp_ms);
    if skew > AUTH_MAX_SKEW_MS {
        return Err(AppError::InvalidRequestConfig(format!(
            "node auth timestamp skew {skew}ms exceeded the allowed window"
        )));
    }

    state.register_node_auth_nonce(&auth.node_id, &auth.nonce)?;
    Ok(())
}

fn respond_handshake_error(
    tx: &mpsc::UnboundedSender<NodeEnvelope>,
    request_id: String,
    method: &str,
    local_node_id: &str,
    remote_node_id: Option<&str>,
    session_id: &str,
    code: &str,
    message: String,
) {
    let _ = tx.send(NodeEnvelope::response_error(
        request_id,
        method,
        Some(local_node_id.to_owned()),
        remote_node_id.map(str::to_owned),
        Some(session_id.to_owned()),
        code,
        message,
    ));
}
