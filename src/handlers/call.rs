use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use std::sync::Arc;
use crate::{
    models::{InitiateCallRequest, BatchCallsRequest, CallResponse, StatsResponse, ErrorResponse},
    services::AppState,
};

pub async fn initiate_call(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<InitiateCallRequest>,
) -> Result<(StatusCode, Json<CallResponse>), (StatusCode, Json<ErrorResponse>)> {
    // Por defecto usar WebSocket Media Streams para latencia óptima
    let use_websocket = std::env::var("USE_MEDIA_STREAMS")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    let result = if use_websocket {
        state.telnyx_service
            .initiate_call_with_stream(
                &payload.telefono,
                &payload.nombre,
                &payload.telefono,
                payload.contexto.as_deref(),
            )
            .await
    } else {
        state.telnyx_service
            .initiate_call(
                &payload.telefono,
                &payload.nombre,
                &payload.telefono,
                payload.contexto.as_deref(),
            )
            .await
    };

    match result {
        Ok(response) => {
            state.total_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Ok((StatusCode::CREATED, Json(response)))
        }
        Err(e) => {
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: "Failed to initiate call".to_string(),
                    message: Some(e.to_string()),
                }),
            ))
        }
    }
}

pub async fn batch_calls(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<BatchCallsRequest>,
) -> (StatusCode, Json<serde_json::Value>) {
    let mut responses = Vec::new();

    for call_req in payload.calls {
        match state.telnyx_service
            .initiate_call(
                &call_req.telefono,
                &call_req.nombre,
                &call_req.telefono,
                call_req.contexto.as_deref(),
            )
            .await
        {
            Ok(response) => {
                state.total_calls.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                responses.push(serde_json::json!({
                    "status": "success",
                    "call_control_id": response.call_control_id,
                    "telefono": call_req.telefono
                }));
            }
            Err(e) => {
                responses.push(serde_json::json!({
                    "status": "error",
                    "telefono": call_req.telefono,
                    "error": e.to_string()
                }));
            }
        }
    }

    (
        StatusCode::OK,
        Json(serde_json::json!({
            "total": responses.len(),
            "results": responses
        })),
    )
}

pub async fn session_stats(
    State(state): State<Arc<AppState>>,
) -> Json<StatsResponse> {
    let uptime = chrono::Utc::now()
        .signed_duration_since(state.start_time)
        .num_seconds() as u64;

    Json(StatsResponse {
        active_sessions: state.sessions.len(),
        total_calls: state.total_calls.load(std::sync::atomic::Ordering::SeqCst),
        uptime_seconds: uptime,
    })
}
