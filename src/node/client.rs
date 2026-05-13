use std::collections::HashMap;
use std::sync::Arc;

use futures_util::StreamExt;
use serde::de::DeserializeOwned;
use serde_json::json;
use tokio::sync::{Mutex, mpsc, oneshot};
use tokio::task::AbortHandle;
use tokio::time::{Duration, interval, sleep};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tracing::{info, warn};

use crate::error::AppError;
use crate::events::EventLogLevel;
use crate::node::protocol::{
    NODE_CALLER_HEADER, NODE_DEADLINE_MS_HEADER, NODE_HANDSHAKE_NODE_ID_HEADER,
    NODE_REQUEST_ID_HEADER, NODE_REQUESTED_AT_MS_HEADER, NODE_TRACE_ID_HEADER, NodeAdvertiseParams,
    NodeAuthParams, NodeCapacityParams, NodeDrainParams, NodeEnvelope, NodeEnvelopeKind,
    NodeHeartbeatParams, NodeHelloAck, NodeHelloParams, NodeIsolateParams, NodeReadyParams,
    NodeShutdownNoticeParams, NodeTaskCancelParams, NodeTaskDispatchParams, NodeTaskProgressParams,
    NodeTaskResult, PROXY_HOP_HEADER, new_message_id, now_ms,
};
use crate::node::session::NodeSessionState;
use crate::node::upstream::UpstreamPeerInfo;
use crate::server::state::AppState;

type ActiveTaskRegistry = Arc<Mutex<HashMap<String, ActiveNodeTask>>>;

#[derive(Debug, Clone)]
struct ActiveNodeTask {
    request_id: String,
    from: Option<String>,
    session_id: Option<String>,
    trace_id: Option<String>,
    hop_count: Option<u32>,
    deadline_ms: Option<u64>,
    abort_handle: AbortHandle,
}

pub(crate) fn spawn_upstream_connector(state: AppState) {
    let Some(upstream_url) = state.node_connect_upstream().map(str::to_owned) else {
        return;
    };

    tokio::spawn(async move {
        loop {
            state.set_upstream_connection_state(NodeSessionState::Connecting);
            let reconnect_delay_ms = match connect_once(state.clone(), &upstream_url).await {
                Ok(()) => {
                    info!(upstream = %upstream_url, "node upstream session ended");
                    3_000
                }
                Err(AppError::UpstreamReconnect { delay_ms, message }) => {
                    warn!(
                        upstream = %upstream_url,
                        delay_ms,
                        reason = %message,
                        "node upstream requested delayed reconnect"
                    );
                    delay_ms
                }
                Err(error) => {
                    warn!(upstream = %upstream_url, error = %error, "node upstream session failed");
                    3_000
                }
            };

            sleep(Duration::from_millis(reconnect_delay_ms)).await;
        }
    });
}

async fn connect_once(state: AppState, upstream_url: &str) -> Result<(), AppError> {
    let Some(node_id) = state.node_id().map(str::to_owned) else {
        return Ok(());
    };
    let Some(role) = state.node_role() else {
        return Ok(());
    };
    let Some(auth_token) = state.node_auth_token().map(str::to_owned) else {
        return Ok(());
    };

    let mut request = upstream_url.into_client_request().map_err(|error| {
        AppError::Io(std::io::Error::other(format!(
            "failed to build upstream websocket request: {error}"
        )))
    })?;
    request.headers_mut().insert(
        tokio_tungstenite::tungstenite::http::header::AUTHORIZATION,
        tokio_tungstenite::tungstenite::http::HeaderValue::from_str(&format!(
            "Bearer {auth_token}"
        ))
        .map_err(|error| {
            AppError::InvalidRequestConfig(format!(
                "invalid node auth token for Authorization header: {error}"
            ))
        })?,
    );
    request.headers_mut().insert(
        tokio_tungstenite::tungstenite::http::HeaderName::from_static(
            NODE_HANDSHAKE_NODE_ID_HEADER,
        ),
        tokio_tungstenite::tungstenite::http::HeaderValue::from_str(&node_id).map_err(|error| {
            AppError::InvalidRequestConfig(format!(
                "invalid node id for websocket handshake header: {error}"
            ))
        })?,
    );

    let (stream, _) = connect_async(request).await.map_err(|error| {
        AppError::Io(std::io::Error::other(format!(
            "failed to connect upstream websocket: {error}"
        )))
    })?;
    state.set_upstream_connection_state(NodeSessionState::Authenticating);
    let (mut sink, mut source) = stream.split();
    let active_tasks: ActiveTaskRegistry = Arc::new(Mutex::new(HashMap::new()));
    let (tx, mut rx) = mpsc::unbounded_channel::<NodeEnvelope>();
    let writer = tokio::spawn(async move {
        while let Some(envelope) = rx.recv().await {
            let payload = match serde_json::to_string(&envelope) {
                Ok(payload) => payload,
                Err(_) => break,
            };
            if futures_util::SinkExt::send(&mut sink, Message::Text(payload.into()))
                .await
                .is_err()
            {
                break;
            }
        }
    });

    let result = async {
        let auth = NodeEnvelope::request(
            "node.auth",
            json!(NodeAuthParams {
                node_id: node_id.clone(),
                token: auth_token,
                timestamp_ms: now_ms(),
                nonce: new_message_id("nonce"),
            }),
        );
        send_envelope(&tx, auth)?;
        expect_ok_response(&mut source, "node.auth").await?;

        let advertisement = current_advertisement(&state);
        let hello = NodeEnvelope::request(
            "node.hello",
            json!(NodeHelloParams {
                node_id: node_id.clone(),
                role,
                version: state.version.to_owned(),
                capabilities: advertisement.capabilities.clone(),
                platforms: advertisement.platforms.clone(),
            }),
        );
        send_envelope(&tx, hello)?;
        let hello_ack: NodeHelloAck = expect_ok_response_result(&mut source, "node.hello").await?;

        let advertise = NodeEnvelope::request("node.advertise", json!(advertisement));
        send_envelope(&tx, advertise)?;
        expect_ok_response(&mut source, "node.advertise").await?;

        state.set_upstream_connection_ready(
            tx.clone(),
            UpstreamPeerInfo {
                session_id: Some(hello_ack.session_id),
                node_id: Some(hello_ack.node_id),
                role: Some(hello_ack.role),
                version: Some(hello_ack.version),
                capabilities: hello_ack.capabilities,
                platforms: hello_ack.platforms,
            },
        );
        if state.node_auto_claim_published_routes() {
            state.claim_published_upstream_routes().map_err(|envelope| {
                AppError::Io(std::io::Error::other(format!(
                    "upstream websocket write failed: {}",
                    envelope.method
                )))
            })?;
            expect_ok_response(&mut source, "route.update").await?;
        }

        info!(node_id = %node_id, upstream = %upstream_url, "node upstream session ready");

        let heartbeat_ms = state.node_heartbeat_ms().unwrap_or(10_000);
        let mut ticker = interval(Duration::from_millis(heartbeat_ms));
        ticker.tick().await;

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    let heartbeat = NodeEnvelope::event(
                        "node.heartbeat",
                        Some(node_id.clone()),
                        None,
                        None,
                        json!(NodeHeartbeatParams { timestamp_ms: now_ms() }),
                    );
                    send_envelope(&tx, heartbeat)?;
                }
                message = source.next() => {
                    match message {
                        Some(Ok(Message::Text(text))) => {
                            state.touch_upstream_connection();
                            let envelope: NodeEnvelope = serde_json::from_str(&text).map_err(AppError::from)?;
                            handle_runtime_envelope(&state, &node_id, &tx, &active_tasks, envelope).await?;
                        }
                        Some(Ok(Message::Ping(_))) | Some(Ok(Message::Pong(_))) => {}
                        Some(Ok(Message::Binary(_))) => {}
                        Some(Ok(Message::Frame(_))) => {}
                        Some(Ok(Message::Close(_))) | None => break Ok(()),
                        Some(Err(error)) => {
                            break Err(AppError::Io(std::io::Error::other(format!(
                                "upstream websocket read failed: {error}"
                            ))));
                        }
                    }
                }
            }
        }
    }
    .await;

    let disconnect_error = result.as_ref().err().map(ToString::to_string);
    state.record_upstream_disconnect(NodeSessionState::Reconnecting, disconnect_error);
    drop(tx);
    let _ = writer.await;
    result
}

async fn handle_runtime_envelope(
    state: &AppState,
    node_id: &str,
    sender: &mpsc::UnboundedSender<NodeEnvelope>,
    active_tasks: &ActiveTaskRegistry,
    envelope: NodeEnvelope,
) -> Result<(), AppError> {
    match envelope.kind {
        NodeEnvelopeKind::Response => {
            if let Some(error) = envelope.error {
                return Err(AppError::InvalidRequestConfig(format!(
                    "node upstream returned {} for `{}`: {}",
                    error.code, envelope.method, error.message
                )));
            }
        }
        NodeEnvelopeKind::Event => {
            if envelope.method == "node.shutdown_notice" {
                let notice: NodeShutdownNoticeParams = envelope.parse_params()?;
                info!(
                    node_id = %node_id,
                    reason = ?notice.reason,
                    reconnect_after_ms = ?notice.reconnect_after_ms,
                    "received node shutdown notice from upstream"
                );
                return Err(AppError::UpstreamReconnect {
                    delay_ms: notice.reconnect_after_ms.unwrap_or(3_000),
                    message: notice
                        .reason
                        .unwrap_or_else(|| "upstream requested session shutdown".to_owned()),
                });
            }
        }
        NodeEnvelopeKind::Request => match envelope.method.as_str() {
            "task.dispatch" => {
                let params: NodeTaskDispatchParams = match envelope.parse_params() {
                    Ok(params) => params,
                    Err(error) => {
                        let mut response = NodeEnvelope::response_error(
                            envelope.id,
                            "task.error",
                            Some(node_id.to_owned()),
                            envelope.from.clone(),
                            envelope.session_id.clone(),
                            "node_protocol_invalid",
                            error.to_string(),
                        );
                        response.trace_id = envelope.trace_id.clone();
                        response.hop_count = envelope.hop_count;
                        send_envelope(sender, response)?;
                        return Ok(());
                    }
                };
                let request_timeout_ms = match task_request_timeout_ms(state, &envelope) {
                    Ok(timeout_ms) => timeout_ms,
                    Err((code, message)) => {
                        let mut response = NodeEnvelope::response_error(
                            envelope.id,
                            "task.error",
                            Some(node_id.to_owned()),
                            envelope.from.clone(),
                            envelope.session_id.clone(),
                            code,
                            message,
                        );
                        response.trace_id = envelope.trace_id.clone();
                        response.hop_count = envelope.hop_count;
                        send_envelope(sender, response)?;
                        return Ok(());
                    }
                };
                if !state.is_local_node_ready_for_tasks() {
                    let (code, detail) = match state.local_node_availability() {
                        crate::node::registry::NodeAvailability::Draining => (
                            "node_draining",
                            format!("node `{node_id}` is draining and not accepting new tasks"),
                        ),
                        crate::node::registry::NodeAvailability::Isolated => (
                            "node_isolated",
                            format!("node `{node_id}` is isolated and not accepting new tasks"),
                        ),
                        crate::node::registry::NodeAvailability::Ready => (
                            "node_unavailable",
                            format!("node `{node_id}` is not ready for new tasks"),
                        ),
                    };
                    let mut response = NodeEnvelope::response_error(
                        envelope.id,
                        "task.error",
                        Some(node_id.to_owned()),
                        envelope.from.clone(),
                        envelope.session_id.clone(),
                        code,
                        detail,
                    );
                    response.trace_id = envelope.trace_id.clone();
                    response.hop_count = envelope.hop_count;
                    send_envelope(sender, response)?;
                    return Ok(());
                }
                let max_concurrent_tasks = state.node_max_concurrent_tasks().unwrap_or(u32::MAX);
                let current_active_tasks = {
                    let guard = active_tasks.lock().await;
                    guard.len() as u32
                };
                if current_active_tasks >= max_concurrent_tasks {
                    let mut response = NodeEnvelope::response_error(
                        envelope.id,
                        "task.error",
                        Some(node_id.to_owned()),
                        envelope.from.clone(),
                        envelope.session_id.clone(),
                        "node_capacity_exceeded",
                        format!(
                            "node `{node_id}` is at capacity ({current_active_tasks}/{max_concurrent_tasks})"
                        ),
                    );
                    response.trace_id = envelope.trace_id.clone();
                    response.hop_count = envelope.hop_count;
                    send_envelope(sender, response)?;
                    return Ok(());
                }

                emit_task_progress(
                    sender,
                    node_id,
                    &envelope,
                    "running",
                    Some("executing local task"),
                )?;

                let request_id = envelope.id.clone();
                let worker_state = state.clone();
                let worker_node_id = node_id.to_owned();
                let worker_sender = sender.clone();
                let worker_registry = active_tasks.clone();
                let request_envelope = envelope.clone();
                let (start_tx, start_rx) = oneshot::channel();
                let task = tokio::spawn(async move {
                    let _ = start_rx.await;
                    let result = execute_local_task(
                        &worker_state,
                        &request_envelope,
                        params,
                        request_timeout_ms,
                    )
                    .await;

                    let removed = {
                        let mut guard = worker_registry.lock().await;
                        let removed = guard.remove(&request_id);
                        let active_tasks_len = guard.len() as u32;
                        drop(guard);
                        let _ = send_current_advertisement(
                            &worker_sender,
                            &worker_state,
                            &worker_node_id,
                            active_tasks_len,
                        );
                        removed
                    };
                    if removed.is_none() {
                        return;
                    }

                    if result.is_ok() {
                        let _ = emit_task_progress(
                            &worker_sender,
                            &worker_node_id,
                            &request_envelope,
                            "completed",
                            Some("local task finished"),
                        );
                    }

                    let mut response = match result {
                        Ok(result) => NodeEnvelope::response_ok(
                            request_envelope.id.clone(),
                            "task.result",
                            Some(worker_node_id),
                            request_envelope.from.clone(),
                            request_envelope.session_id.clone(),
                            json!(result),
                        ),
                        Err(error) => NodeEnvelope::response_error(
                            request_envelope.id.clone(),
                            "task.error",
                            Some(worker_node_id),
                            request_envelope.from.clone(),
                            request_envelope.session_id.clone(),
                            "task_failed",
                            error.to_string(),
                        ),
                    };
                    response.trace_id = request_envelope.trace_id.clone();
                    response.hop_count = request_envelope.hop_count;
                    let _ = send_envelope(&worker_sender, response);
                });

                let mut guard = active_tasks.lock().await;
                guard.insert(
                    envelope.id.clone(),
                    ActiveNodeTask {
                        request_id: envelope.id,
                        from: envelope.from,
                        session_id: envelope.session_id,
                        trace_id: envelope.trace_id,
                        hop_count: envelope.hop_count,
                        deadline_ms: envelope.deadline_ms,
                        abort_handle: task.abort_handle(),
                    },
                );
                let active_tasks_len = guard.len() as u32;
                drop(guard);
                let _ = send_current_advertisement(sender, state, node_id, active_tasks_len);
                let _ = start_tx.send(());
            }
            "task.cancel" => {
                let params: NodeTaskCancelParams = match envelope.parse_params() {
                    Ok(params) => params,
                    Err(error) => {
                        let mut response = NodeEnvelope::response_error(
                            envelope.id,
                            "task.cancel",
                            Some(node_id.to_owned()),
                            envelope.from.clone(),
                            envelope.session_id.clone(),
                            "node_protocol_invalid",
                            error.to_string(),
                        );
                        response.trace_id = envelope.trace_id.clone();
                        response.hop_count = envelope.hop_count;
                        send_envelope(sender, response)?;
                        return Ok(());
                    }
                };

                let active_task = {
                    let mut guard = active_tasks.lock().await;
                    let removed = guard.remove(&params.request_id);
                    let active_tasks_len = guard.len() as u32;
                    drop(guard);
                    let _ = send_current_advertisement(sender, state, node_id, active_tasks_len);
                    removed
                };
                let Some(active_task) = active_task else {
                    let mut response = NodeEnvelope::response_error(
                        envelope.id,
                        "task.cancel",
                        Some(node_id.to_owned()),
                        envelope.from.clone(),
                        envelope.session_id.clone(),
                        "task_not_found",
                        format!(
                            "task `{}` is not running on node `{node_id}`",
                            params.request_id
                        ),
                    );
                    response.trace_id = envelope.trace_id.clone();
                    response.hop_count = envelope.hop_count;
                    send_envelope(sender, response)?;
                    return Ok(());
                };

                active_task.abort_handle.abort();
                let downstream_target = state
                    .node_registry()
                    .pending_target_for(&active_task.request_id);
                if let Some((target, downstream_sender)) = downstream_target {
                    let mut cancel = NodeEnvelope::request(
                        "task.cancel",
                        json!(NodeTaskCancelParams {
                            request_id: active_task.request_id.clone(),
                            reason: params.reason.clone(),
                        }),
                    );
                    cancel.from = Some(node_id.to_owned());
                    cancel.to = Some(target.node_id.clone());
                    cancel.session_id = Some(target.session_id.clone());
                    cancel.trace_id = active_task.trace_id.clone();
                    cancel.hop_count = active_task.hop_count;
                    cancel.deadline_ms = active_task.deadline_ms;
                    let _ = send_envelope(&downstream_sender, cancel);
                }

                emit_task_progress_from_active(
                    sender,
                    node_id,
                    &active_task,
                    "cancelled",
                    params
                        .reason
                        .as_deref()
                        .or(Some("task cancellation requested")),
                )?;
                send_task_cancelled_error(
                    sender,
                    node_id,
                    &active_task,
                    params
                        .reason
                        .as_deref()
                        .unwrap_or("task cancellation requested"),
                )?;

                let mut response = NodeEnvelope::response_ok(
                    envelope.id,
                    "task.cancel",
                    Some(node_id.to_owned()),
                    envelope.from.clone(),
                    envelope.session_id.clone(),
                    json!({
                        "cancelled": true,
                        "request_id": active_task.request_id,
                    }),
                );
                response.trace_id = envelope.trace_id.clone();
                response.hop_count = envelope.hop_count;
                send_envelope(sender, response)?;
            }
            "node.drain" => {
                let params: NodeDrainParams = match envelope.parse_params() {
                    Ok(params) => params,
                    Err(error) => {
                        let mut response = NodeEnvelope::response_error(
                            envelope.id,
                            "node.drain",
                            Some(node_id.to_owned()),
                            envelope.from.clone(),
                            envelope.session_id.clone(),
                            "node_protocol_invalid",
                            error.to_string(),
                        );
                        response.trace_id = envelope.trace_id.clone();
                        response.hop_count = envelope.hop_count;
                        send_envelope(sender, response)?;
                        return Ok(());
                    }
                };

                let notes = state.enter_local_node_drain(params.reason.as_deref());
                let detail = notes.join("; ");
                info!(
                    node_id = %node_id,
                    reason = ?params.reason,
                    "entered draining state because upstream requested node drain"
                );
                state.client.events().emit_log(
                    EventLogLevel::Info,
                    format!("node drain {node_id}"),
                    [
                        format!("reason={}", params.reason.as_deref().unwrap_or_default()),
                        detail.clone(),
                    ],
                );

                let mut response = NodeEnvelope::response_ok(
                    envelope.id,
                    "node.drain",
                    Some(node_id.to_owned()),
                    envelope.from.clone(),
                    envelope.session_id.clone(),
                    json!({
                        "draining": true,
                        "detail": detail,
                    }),
                );
                response.trace_id = envelope.trace_id.clone();
                response.hop_count = envelope.hop_count;
                send_envelope(sender, response)?;
            }
            "node.ready" => {
                let params: NodeReadyParams = match envelope.parse_params() {
                    Ok(params) => params,
                    Err(error) => {
                        let mut response = NodeEnvelope::response_error(
                            envelope.id,
                            "node.ready",
                            Some(node_id.to_owned()),
                            envelope.from.clone(),
                            envelope.session_id.clone(),
                            "node_protocol_invalid",
                            error.to_string(),
                        );
                        response.trace_id = envelope.trace_id.clone();
                        response.hop_count = envelope.hop_count;
                        send_envelope(sender, response)?;
                        return Ok(());
                    }
                };

                let notes = state.exit_local_node_drain();
                let detail = notes.join("; ");
                info!(
                    node_id = %node_id,
                    reason = ?params.reason,
                    "exited draining state because upstream requested node ready"
                );
                state.client.events().emit_log(
                    EventLogLevel::Info,
                    format!("node ready {node_id}"),
                    [
                        format!("reason={}", params.reason.as_deref().unwrap_or_default()),
                        detail.clone(),
                    ],
                );

                let mut response = NodeEnvelope::response_ok(
                    envelope.id,
                    "node.ready",
                    Some(node_id.to_owned()),
                    envelope.from.clone(),
                    envelope.session_id.clone(),
                    json!({
                        "ready": true,
                        "detail": detail,
                    }),
                );
                response.trace_id = envelope.trace_id.clone();
                response.hop_count = envelope.hop_count;
                send_envelope(sender, response)?;
            }
            "node.isolate" => {
                let params: NodeIsolateParams = match envelope.parse_params() {
                    Ok(params) => params,
                    Err(error) => {
                        let mut response = NodeEnvelope::response_error(
                            envelope.id,
                            "node.isolate",
                            Some(node_id.to_owned()),
                            envelope.from.clone(),
                            envelope.session_id.clone(),
                            "node_protocol_invalid",
                            error.to_string(),
                        );
                        response.trace_id = envelope.trace_id.clone();
                        response.hop_count = envelope.hop_count;
                        send_envelope(sender, response)?;
                        return Ok(());
                    }
                };

                let notes = state.enter_local_node_isolation(params.reason.as_deref());
                let detail = notes.join("; ");
                info!(
                    node_id = %node_id,
                    reason = ?params.reason,
                    "entered isolation state because upstream requested node isolation"
                );
                state.client.events().emit_log(
                    EventLogLevel::Info,
                    format!("node isolate {node_id}"),
                    [
                        format!("reason={}", params.reason.as_deref().unwrap_or_default()),
                        detail.clone(),
                    ],
                );

                let mut response = NodeEnvelope::response_ok(
                    envelope.id,
                    "node.isolate",
                    Some(node_id.to_owned()),
                    envelope.from.clone(),
                    envelope.session_id.clone(),
                    json!({
                        "isolated": true,
                        "detail": detail,
                    }),
                );
                response.trace_id = envelope.trace_id.clone();
                response.hop_count = envelope.hop_count;
                send_envelope(sender, response)?;
            }
            "node.capacity" => {
                let params: NodeCapacityParams = match envelope.parse_params() {
                    Ok(params) => params,
                    Err(error) => {
                        let mut response = NodeEnvelope::response_error(
                            envelope.id,
                            "node.capacity",
                            Some(node_id.to_owned()),
                            envelope.from.clone(),
                            envelope.session_id.clone(),
                            "node_protocol_invalid",
                            error.to_string(),
                        );
                        response.trace_id = envelope.trace_id.clone();
                        response.hop_count = envelope.hop_count;
                        send_envelope(sender, response)?;
                        return Ok(());
                    }
                };

                state.set_local_node_capacity_override(params.max_concurrent_tasks);
                let detail = match params.max_concurrent_tasks {
                    Some(limit) => format!("local capacity override set to {limit}"),
                    None => "local capacity override cleared".to_owned(),
                };
                info!(
                    node_id = %node_id,
                    max_concurrent_tasks = ?params.max_concurrent_tasks,
                    reason = ?params.reason,
                    "updated local capacity because upstream requested node capacity change"
                );
                state.client.events().emit_log(
                    EventLogLevel::Info,
                    format!("node capacity {node_id}"),
                    [
                        format!(
                            "max_concurrent_tasks={}",
                            params.max_concurrent_tasks.unwrap_or_default()
                        ),
                        format!("reason={}", params.reason.as_deref().unwrap_or_default()),
                    ],
                );

                let active_tasks_len = {
                    let guard = active_tasks.lock().await;
                    guard.len() as u32
                };
                let _ = send_current_advertisement(sender, state, node_id, active_tasks_len);

                let mut response = NodeEnvelope::response_ok(
                    envelope.id,
                    "node.capacity",
                    Some(node_id.to_owned()),
                    envelope.from.clone(),
                    envelope.session_id.clone(),
                    json!({
                        "max_concurrent_tasks": params.max_concurrent_tasks,
                        "detail": detail,
                    }),
                );
                response.trace_id = envelope.trace_id.clone();
                response.hop_count = envelope.hop_count;
                send_envelope(sender, response)?;
            }
            _ => {
                let mut response = NodeEnvelope::response_error(
                    envelope.id,
                    "task.error",
                    Some(node_id.to_owned()),
                    envelope.from.clone(),
                    envelope.session_id.clone(),
                    "node_method_unsupported",
                    format!("unsupported node request method `{}`", envelope.method),
                );
                response.trace_id = envelope.trace_id.clone();
                send_envelope(sender, response)?;
            }
        },
    }

    Ok(())
}

async fn execute_local_task(
    state: &AppState,
    envelope: &NodeEnvelope,
    params: NodeTaskDispatchParams,
    request_timeout_ms: u64,
) -> Result<NodeTaskResult, AppError> {
    let method = reqwest::Method::from_bytes(params.http_method.as_bytes()).map_err(|error| {
        AppError::InvalidRequestConfig(format!(
            "invalid node task HTTP method `{}`: {error}",
            params.http_method
        ))
    })?;
    let local_url = format!(
        "{}{}",
        local_server_origin(state),
        local_task_path_and_query(&params)
    );

    let mut headers = params.headers;
    if let Some(hop_count) = envelope.hop_count {
        headers.insert(PROXY_HOP_HEADER.to_owned(), hop_count.to_string());
    }
    if let Some(trace_id) = &envelope.trace_id {
        headers.insert(NODE_TRACE_ID_HEADER.to_owned(), trace_id.clone());
    }
    if let Some(deadline_ms) = envelope.deadline_ms {
        headers.insert(NODE_DEADLINE_MS_HEADER.to_owned(), deadline_ms.to_string());
    }
    headers.insert(NODE_REQUEST_ID_HEADER.to_owned(), envelope.id.clone());
    if let Some(requested_at_ms) = params.requested_at_ms {
        headers.insert(
            NODE_REQUESTED_AT_MS_HEADER.to_owned(),
            requested_at_ms.to_string(),
        );
    }
    if let Some(caller) = &params.caller {
        headers.insert(NODE_CALLER_HEADER.to_owned(), caller.clone());
    }

    let mut request = state.proxy_client.request(method, local_url);
    request = request.timeout(Duration::from_millis(request_timeout_ms));

    for (name, value) in headers {
        request = request.header(name, value);
    }

    let response = request.body(params.body).send().await?;
    let status = response.status().as_u16();
    let content_type = response
        .headers()
        .get(reqwest::header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);
    let response_headers = collect_node_result_headers(response.headers());
    let body = response.bytes().await?.to_vec();

    Ok(NodeTaskResult {
        status,
        content_type,
        headers: response_headers,
        body,
    })
}

fn task_request_timeout_ms(
    state: &AppState,
    envelope: &NodeEnvelope,
) -> Result<u64, (&'static str, String)> {
    let max_hops = state.node_max_hops().unwrap_or(u32::MAX);
    if envelope
        .hop_count
        .is_some_and(|hop_count| hop_count > max_hops)
    {
        return Err((
            "node_loop_detected",
            format!(
                "node hop {} exceeded configured maximum {max_hops}",
                envelope.hop_count.unwrap_or_default()
            ),
        ));
    }

    let default_timeout_ms = state.node_request_timeout_ms().unwrap_or(15_000);
    match envelope.deadline_ms {
        Some(deadline_ms) => {
            let remaining_ms = deadline_ms.saturating_sub(now_ms());
            if remaining_ms == 0 {
                return Err((
                    "node_timeout",
                    format!("node deadline {deadline_ms} has already expired"),
                ));
            }
            Ok(default_timeout_ms.min(remaining_ms))
        }
        None => Ok(default_timeout_ms),
    }
}

fn current_advertisement(state: &AppState) -> NodeAdvertiseParams {
    current_advertisement_with_active_tasks(state, 0)
}

fn local_task_path_and_query(params: &NodeTaskDispatchParams) -> String {
    let path = if params.path.is_empty() {
        "/".to_owned()
    } else if params.path.starts_with('/') {
        params.path.clone()
    } else {
        format!("/{}", params.path)
    };
    let mut full_path = params.platform.api_base_path().to_owned();
    if path != "/" {
        full_path.push_str(&path);
    }
    if params.query.is_empty() {
        return full_path;
    }

    full_path.push('?');
    for (index, (key, value)) in params.query.iter().enumerate() {
        if index > 0 {
            full_path.push('&');
        }
        full_path.push_str(key);
        if !value.is_empty() {
            full_path.push('=');
            full_path.push_str(value);
        }
    }
    full_path
}

fn collect_node_result_headers(
    headers: &reqwest::header::HeaderMap,
) -> std::collections::BTreeMap<String, String> {
    const FORWARDED_RESPONSE_HEADERS: &[&str] = &[
        "cache-control",
        "etag",
        "last-modified",
        "content-disposition",
    ];

    let mut forwarded = std::collections::BTreeMap::new();
    for header_name in FORWARDED_RESPONSE_HEADERS {
        if let Some(value) = headers
            .get(*header_name)
            .and_then(|value| value.to_str().ok())
        {
            forwarded.insert((*header_name).to_owned(), value.to_owned());
        }
    }
    for (name, value) in headers {
        let Some(name) = name.as_str().strip_prefix("x-amagi-") else {
            continue;
        };
        let Some(value) = value.to_str().ok() else {
            continue;
        };
        forwarded.insert(format!("x-amagi-{name}"), value.to_owned());
    }
    forwarded
}

fn current_advertisement_with_active_tasks(
    state: &AppState,
    active_tasks: u32,
) -> NodeAdvertiseParams {
    state.current_node_advertisement(active_tasks)
}

fn send_current_advertisement(
    sender: &mpsc::UnboundedSender<NodeEnvelope>,
    state: &AppState,
    node_id: &str,
    active_tasks: u32,
) -> Result<(), AppError> {
    state.set_local_node_active_tasks(active_tasks);
    if !state.is_local_node_ready_for_tasks() {
        return Ok(());
    }
    let mut event = NodeEnvelope::request(
        "node.advertise",
        json!(current_advertisement_with_active_tasks(state, active_tasks)),
    );
    event.from = Some(node_id.to_owned());
    send_envelope(sender, event)
}

fn emit_task_progress(
    sender: &mpsc::UnboundedSender<NodeEnvelope>,
    node_id: &str,
    request: &NodeEnvelope,
    stage: &str,
    message: Option<&str>,
) -> Result<(), AppError> {
    let mut event = NodeEnvelope::event(
        "task.progress",
        Some(node_id.to_owned()),
        request.from.clone(),
        request.session_id.clone(),
        json!(NodeTaskProgressParams {
            request_id: request.id.clone(),
            stage: stage.to_owned(),
            message: message.map(str::to_owned),
            percent: None,
        }),
    );
    event.trace_id = request.trace_id.clone();
    event.hop_count = request.hop_count;
    event.deadline_ms = request.deadline_ms;
    send_envelope(sender, event)
}

fn emit_task_progress_from_active(
    sender: &mpsc::UnboundedSender<NodeEnvelope>,
    node_id: &str,
    request: &ActiveNodeTask,
    stage: &str,
    message: Option<&str>,
) -> Result<(), AppError> {
    let mut event = NodeEnvelope::event(
        "task.progress",
        Some(node_id.to_owned()),
        request.from.clone(),
        request.session_id.clone(),
        json!(NodeTaskProgressParams {
            request_id: request.request_id.clone(),
            stage: stage.to_owned(),
            message: message.map(str::to_owned),
            percent: None,
        }),
    );
    event.trace_id = request.trace_id.clone();
    event.hop_count = request.hop_count;
    event.deadline_ms = request.deadline_ms;
    send_envelope(sender, event)
}

fn send_task_cancelled_error(
    sender: &mpsc::UnboundedSender<NodeEnvelope>,
    node_id: &str,
    request: &ActiveNodeTask,
    reason: &str,
) -> Result<(), AppError> {
    let mut response = NodeEnvelope::response_error(
        request.request_id.clone(),
        "task.error",
        Some(node_id.to_owned()),
        request.from.clone(),
        request.session_id.clone(),
        "task_cancelled",
        reason,
    );
    response.trace_id = request.trace_id.clone();
    response.hop_count = request.hop_count;
    send_envelope(sender, response)
}

fn send_envelope(
    sender: &mpsc::UnboundedSender<NodeEnvelope>,
    envelope: NodeEnvelope,
) -> Result<(), AppError> {
    sender.send(envelope).map_err(|error| {
        AppError::Io(std::io::Error::other(format!(
            "upstream websocket write failed: {}",
            error.0.method
        )))
    })
}

async fn expect_ok_response<S>(source: &mut S, expected_method: &str) -> Result<(), AppError>
where
    S: futures_util::Stream<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    expect_ok_response_envelope(source, expected_method)
        .await
        .map(|_| ())
}

async fn expect_ok_response_result<S, T>(
    source: &mut S,
    expected_method: &str,
) -> Result<T, AppError>
where
    S: futures_util::Stream<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
    T: DeserializeOwned,
{
    let envelope = expect_ok_response_envelope(source, expected_method).await?;
    let Some(result) = envelope.result else {
        return Err(AppError::InvalidRequestConfig(format!(
            "node upstream returned no result for `{expected_method}`"
        )));
    };

    serde_json::from_value(result).map_err(AppError::from)
}

async fn expect_ok_response_envelope<S>(
    source: &mut S,
    expected_method: &str,
) -> Result<NodeEnvelope, AppError>
where
    S: futures_util::Stream<Item = Result<Message, tokio_tungstenite::tungstenite::Error>> + Unpin,
{
    let Some(message) = source.next().await else {
        return Err(AppError::Io(std::io::Error::new(
            std::io::ErrorKind::UnexpectedEof,
            format!("upstream websocket closed before `{expected_method}` completed"),
        )));
    };

    match message {
        Ok(Message::Text(text)) => {
            let envelope: NodeEnvelope = serde_json::from_str(&text).map_err(AppError::from)?;
            if envelope.method != expected_method {
                return Err(AppError::InvalidRequestConfig(format!(
                    "expected `{expected_method}` response, got `{}`",
                    envelope.method
                )));
            }
            if let Some(error) = envelope.error.clone() {
                return Err(AppError::InvalidRequestConfig(format!(
                    "node upstream returned {} for `{expected_method}`: {}",
                    error.code, error.message
                )));
            }
            Ok(envelope)
        }
        Ok(other) => Err(AppError::InvalidRequestConfig(format!(
            "expected text response for `{expected_method}`, got websocket frame {other:?}"
        ))),
        Err(error) => Err(AppError::Io(std::io::Error::other(format!(
            "upstream websocket read failed: {error}"
        )))),
    }
}

fn local_server_origin(state: &AppState) -> String {
    let host = match state.serve.host.as_str() {
        "0.0.0.0" | "::" | "[::]" => "127.0.0.1",
        other => other,
    };

    if host.contains(':') && !host.starts_with('[') {
        format!("http://[{host}]:{}", state.serve.port)
    } else {
        format!("http://{host}:{}", state.serve.port)
    }
}
