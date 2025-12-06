use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use std::sync::Arc;
use tracing::{info, error, debug};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::json;
use crate::{
    models::{ClientState, SessionInfo, TranscriptionPayload},
    services::{AppState, SessionManager},
};
use chrono::Timelike; // ✅ Necesario para .hour()

pub async fn handle_telnyx_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    let event_type = payload["meta"]["event_type"]
        .as_str()
        .unwrap_or("unknown");

    // ✅ CORREGIDO: Agregado ": {}"
    info!("📨 Webhook recibido: {}", event_type);

    match event_type {
        "call.answered" => handle_call_answered(state, payload).await,
        "call.speak.ended" => handle_speak_ended(state, payload).await,
        "call.playback.ended" => handle_playback_ended(state, payload).await,
        "call.transcription.transcript_received" => handle_transcription(state, payload).await,
        "call.hangup" => handle_hangup(state, payload).await,
        _ => {
            // ✅ CORREGIDO
            debug!("⏭️ Evento no manejado: {}", event_type);
            (StatusCode::OK, Json(json!({"status": "received"})))
        }
    }
}

async fn handle_call_answered(
    state: Arc<AppState>,
    payload: serde_json::Value,
) -> (StatusCode, Json<serde_json::Value>) {
    let call_control_id = match payload["data"]["call_control_id"].as_str() {
        Some(id) => id.to_string(),
        None => return (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing call_control_id"}))),
    };

    let client_state_base64 = payload["data"]["client_state"].as_str();
    
    let mut client_state = ClientState {
        nombre: "Cliente".to_string(),
        telefono: "desconocido".to_string(),
        contexto: None,
        call_control_id: Some(call_control_id.clone()),
    };

    if let Some(b64) = client_state_base64 {
        if let Ok(decoded) = STANDARD.decode(b64) {
            if let Ok(decoded_str) = String::from_utf8(decoded) {
                if let Ok(parsed) = serde_json::from_str::<ClientState>(&decoded_str) {
                    client_state = parsed;
                    client_state.call_control_id = Some(call_control_id.clone());
                }
            }
        }
    }

    // Crear sesión
    let session = SessionManager::create_session(
        call_control_id.clone(),
        client_state.nombre.clone(),
        client_state.telefono.clone(),
    );

    state.sessions.insert(call_control_id.clone(), session);

    // Generar saludo
    let hour = chrono::Local::now().hour();
    let greeting = match hour {
        5..=11 => "Buenos días, bienvenido a Clínica Veterinaria LA WANDA Y MACARENA, hablas con María. ¿Con quién tengo el gusto?",
        12..=18 => "Buenas tardes, bienvenido a Clínica Veterinaria LA WANDA Y MACARENA, hablas con María. ¿Con quién tengo el gusto?",
        _ => "Buenas noches, bienvenido a Clínica Veterinaria LA WANDA Y MACARENA, hablas con María. ¿Con quién tengo el gusto?",
    };

    // ✅ CORREGIDO
    info!("🔊 Reproduciendo saludo personalizado. ID: {}", call_control_id);

    // Enviar saludo
    if let Err(e) = state.telnyx_service.speak(&call_control_id, greeting).await {
        error!("❌ Error reproduciendo saludo: {}", e);
    }

    // ✅ CORREGIDO: Agregados place holders {}
    info!("✅ Llamada contestada y saludo enviado. Nombre: {}, Tel: {}", 
        client_state.nombre,
        client_state.telefono
    );

    (StatusCode::OK, Json(json!({"status": "handled"})))
}

async fn handle_speak_ended(
    state: Arc<AppState>,
    payload: serde_json::Value,
) -> (StatusCode, Json<serde_json::Value>) {
    let call_control_id = match payload["data"]["call_control_id"].as_str() {
        Some(id) => id.to_string(),
        None => return (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing call_control_id"}))),
    };

    // ✅ CORREGIDO
    info!("🎤 Iniciando transcripción después de speak. ID: {}", call_control_id);

    if let Err(e)