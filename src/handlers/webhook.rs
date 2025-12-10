use axum::{
    extract::{State, Json},
    http::StatusCode,
};
use std::sync::Arc;
use tracing::{info, error, debug};
use base64::{engine::general_purpose::STANDARD, Engine};
use serde_json::json;
use crate::{
    models::ClientState, // Limpié los imports no usados para que no salgan warnings
    services::{AppState, SessionManager},
};
use chrono::Timelike; // ✅ Necesario para .hour()

pub async fn handle_telnyx_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    let event_type = payload["data"]["event_type"]
        .as_str()
        .or_else(|| payload["meta"]["event_type"].as_str())
        .unwrap_or("unknown");

    // Extraer call_control_id para tracking consistente
    let call_id = payload["data"]["call_control_id"]
        .as_str()
        .or_else(|| payload["data"]["payload"]["call_control_id"].as_str())
        .unwrap_or("unknown");

    info!("📨 [CALL:{}] Webhook recibido: {}", call_id, event_type);

    match event_type {
        "call.answered" => handle_call_answered(state, payload).await,
        "call.speak.ended" => handle_speak_ended(state, payload).await,
        "call.playback.started" => handle_playback_started(state, payload).await,
        "call.playback.ended" => handle_playback_ended(state, payload).await,
        "call.transcription.transcript_received" => handle_transcription(state, payload).await,
        "call.transcription.transcribed" => handle_transcription(state, payload).await,
        "call.transcription.partial" => handle_transcription_partial(state, payload).await,
        "call.hangup" => handle_hangup(state, payload).await,
        _ => {
            // Log completo del payload para diagnóstico (cuando no encontramos `meta.event_type`)
            debug!("⏭️ Evento no manejado: {} - payload: {}", event_type, payload);
            (StatusCode::OK, Json(json!({"status": "received"})))
        }
    }
}

async fn handle_call_answered(
    state: Arc<AppState>,
    payload: serde_json::Value,
) -> (StatusCode, Json<serde_json::Value>) {
    let call_control_id = match payload["data"]["call_control_id"].as_str()
        .or_else(|| payload["data"]["payload"]["call_control_id"].as_str())
    {
        Some(id) => id.to_string(),
        None => return (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing call_control_id"}))),
    };

    // Separador visual por llamada para facilitar lectura de logs
    info!("---------------------------------------------------------------------- [CALL:{}] Inicio de llamada", call_control_id);

    let client_state_base64 = payload["data"]["client_state"].as_str()
        .or_else(|| payload["data"]["payload"]["client_state"].as_str());
    
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
    let greeting_key = match hour {
        5..=11 => "morning",
        12..=18 => "afternoon",
        _ => "evening",
    };

    info!("🔊 Obteniendo saludo para: {}. ID: {}", greeting_key, call_control_id);

    // Obtener o generar audio bajo demanda
    if let Some(url) = state.get_or_generate_greeting(greeting_key).await {
        if let Err(e) = state.telnyx_service.play_audio(&call_control_id, &url).await {
            error!("❌ Error reproduciendo audio: {}", e);
        }
    } else {
        error!("⚠️ No se pudo obtener saludo para: {}", greeting_key);
    }

    // Iniciar transcripción inmediatamente tras lanzar el saludo (para “escuchar” al usuario antes de que termine el audio)
    if let Err(e) = state.telnyx_service.start_transcription(&call_control_id).await {
        error!("❌ Error iniciando transcripción temprana: {}", e);
    } else {
        debug!("🎤 Transcripción solicitada en call.answered para {}", call_control_id);
    }

    // ✅ Log corregido
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
    let call_control_id = match payload["data"]["call_control_id"].as_str()
        .or_else(|| payload["data"]["payload"]["call_control_id"].as_str())
    {
        Some(id) => id.to_string(),
        None => return (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing call_control_id"}))),
    };

    // 📝 Transcripción ya se inicia en call.answer, aquí solo registramos el evento
    info!("🎤 [CALL:{}] Evento speak_ended recibido", call_control_id);

    (StatusCode::OK, Json(json!({"status": "handled"})))
}

async fn handle_playback_started(
    state: Arc<AppState>,
    payload: serde_json::Value,
) -> (StatusCode, Json<serde_json::Value>) {
    let call_control_id = match payload["data"]["call_control_id"].as_str()
        .or_else(|| payload["data"]["payload"]["call_control_id"].as_str())
    {
        Some(id) => id.to_string(),
        None => return (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing call_control_id"}))),
    };

    // 📝 Ya iniciamos transcripción en handle_call_answered, así que solo registramos que playback comenzó
    info!("▶️ [CALL:{}] Playback iniciado", call_control_id);

    (StatusCode::OK, Json(json!({"status": "handled"})))
}

async fn handle_playback_ended(
    state: Arc<AppState>,
    payload: serde_json::Value,
) -> (StatusCode, Json<serde_json::Value>) {
    let call_control_id = match payload["data"]["call_control_id"].as_str()
        .or_else(|| payload["data"]["payload"]["call_control_id"].as_str())
    {
        Some(id) => id.to_string(),
        None => return (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing call_control_id"}))),
    };

    // 📝 Playback terminó; transcripción ya debería estar activa
    info!("⏸️ [CALL:{}] Playback finalizado", call_control_id);

    (StatusCode::OK, Json(json!({"status": "handled"})))
}

async fn handle_transcription(
    state: Arc<AppState>,
    payload: serde_json::Value,
) -> (StatusCode, Json<serde_json::Value>) {
    let call_control_id = match payload["data"]["call_control_id"].as_str()
        .or_else(|| payload["data"]["payload"]["call_control_id"].as_str())
    {
        Some(id) => id.to_string(),
        None => return (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing call_control_id"}))),
    };

    let transcript = payload["data"]["transcript"].as_str()
        .or_else(|| payload["data"]["payload"]["transcript"].as_str())
        .unwrap_or("");
    let is_final = payload["data"]["is_final"].as_bool()
        .or_else(|| payload["data"]["payload"]["is_final"].as_bool())
        .unwrap_or(false);

    if !is_final || transcript.is_empty() {
        return (StatusCode::OK, Json(json!({"status": "buffering"})));
    }

    // ✅ Log corregido
    info!("📝 [CALL:{}] Transcripción recibida: '{}'", call_control_id, transcript);

    // Obtener sesión y generar respuesta
    if let Some(mut session_ref) = state.sessions.get_mut(&call_control_id) {
        // Reproducir respuesta corta bajo demanda mientras se prepara la respuesta larga
        if let Some(url) = state.get_or_generate_quick_reply("processing").await {
            if let Err(e) = state.telnyx_service.play_audio(&call_control_id, &url).await {
                error!("❌ [CALL:{}] Error reproduciendo quick-reply: {}", call_control_id, e);
            }
        }

        let context = SessionManager::get_conversation_context(&session_ref);

        if let Ok(response) = state.claude_service
            .generate_response(
                transcript,
                &session_ref.nombre,
                if context.is_empty() { None } else { Some(&context) },
            )
            .await
        {
            SessionManager::add_to_history(&mut session_ref, response.clone());

            // ✅ Log corregido
            info!("🤖 Respuesta Claude generada: {}", response);

            // Generar audio con ElevenLabs, subir a S3 y reproducir
            match state.elevenlabs_service.text_to_speech(&response).await {
                Ok(audio_bytes) => {
                    let audio_key = format!("audio/response_{}_{}.mp3", 
                        call_control_id, 
                        chrono::Utc::now().timestamp()
                    );
                    match state.s3_service.upload_audio(&audio_key, audio_bytes).await {
                        Ok(audio_url) => {
                            if let Err(e) = state.telnyx_service.play_audio(&call_control_id, &audio_url).await {
                                error!("❌ [CALL:{}] Error reproduciendo audio: {}", call_control_id, e);
                            }
                        }
                        Err(e) => error!("❌ [CALL:{}] Error subiendo audio a S3: {}", call_control_id, e),
                    }
                }
                Err(e) => error!("❌ [CALL:{}] Error generando audio con ElevenLabs: {}", call_control_id, e),
            }
        }
    } else {
        error!("⚠️ [CALL:{}] Sesión no encontrada", call_control_id);
    }

    (StatusCode::OK, Json(json!({"status": "handled"})))
}

async fn handle_transcription_partial(
    _state: Arc<AppState>,
    payload: serde_json::Value,
) -> (StatusCode, Json<serde_json::Value>) {
    let call_control_id = payload["data"]["call_control_id"].as_str()
        .or_else(|| payload["data"]["payload"]["call_control_id"].as_str())
        .unwrap_or("(sin_id)");
    let transcript = payload["data"]["transcript"].as_str()
        .or_else(|| payload["data"]["payload"]["transcript"].as_str())
        .unwrap_or("");
    debug!("🟡 [CALL:{}] Parcial recibido: {}", call_control_id, transcript);
    (StatusCode::OK, Json(json!({"status": "partial"})))
}

async fn handle_hangup(
    state: Arc<AppState>,
    payload: serde_json::Value,
) -> (StatusCode, Json<serde_json::Value>) {
    let call_control_id = match payload["data"]["call_control_id"].as_str() {
        Some(id) => id.to_string(),
        None => return (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing call_control_id"}))),
    };

    state.sessions.remove(&call_control_id);

    // ✅ Log corregido
    info!("☎️ [CALL:{}] Llamada finalizada", call_control_id);

    (StatusCode::OK, Json(json!({"status": "handled"})))
} 