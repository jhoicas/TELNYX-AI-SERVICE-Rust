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
    let event_type = payload["meta"]["event_type"]
        .as_str()
        .unwrap_or("unknown");

    // ✅ Log corregido
    info!("📨 Webhook recibido: {}", event_type);

    match event_type {
        "call.answered" => handle_call_answered(state, payload).await,
        "call.speak.ended" => handle_speak_ended(state, payload).await,
        "call.playback.ended" => handle_playback_ended(state, payload).await,
        "call.transcription.transcript_received" => handle_transcription(state, payload).await,
        "call.hangup" => handle_hangup(state, payload).await,
        _ => {
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

    // ✅ Log corregido
    info!("🔊 Generando saludo con ElevenLabs. ID: {}", call_control_id);

    // Generar audio con ElevenLabs, subir a S3 y reproducir
    match state.elevenlabs_service.text_to_speech(greeting).await {
        Ok(audio_bytes) => {
            let audio_key = format!("audio/greeting_{}.mp3", call_control_id);
            match state.s3_service.upload_audio(&audio_key, audio_bytes).await {
                Ok(audio_url) => {
                    if let Err(e) = state.telnyx_service.play_audio(&call_control_id, &audio_url).await {
                        error!("❌ Error reproduciendo audio: {}", e);
                    }
                }
                Err(e) => error!("❌ Error subiendo audio a S3: {}", e),
            }
        }
        Err(e) => error!("❌ Error generando audio con ElevenLabs: {}", e),
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
    let call_control_id = match payload["data"]["call_control_id"].as_str() {
        Some(id) => id.to_string(),
        None => return (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing call_control_id"}))),
    };

    // ✅ Log corregido
    info!("🎤 Iniciando transcripción después de speak. ID: {}", call_control_id);

    if let Err(e) = state.telnyx_service.start_transcription(&call_control_id).await {
        error!("❌ Error iniciando transcripción: {}", e);
    }

    (StatusCode::OK, Json(json!({"status": "handled"})))
}

async fn handle_playback_ended(
    state: Arc<AppState>,
    payload: serde_json::Value,
) -> (StatusCode, Json<serde_json::Value>) {
    let call_control_id = match payload["data"]["call_control_id"].as_str() {
        Some(id) => id.to_string(),
        None => return (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing call_control_id"}))),
    };

    // ✅ Log corregido
    info!("🔊 Playback finalizado, iniciando transcripción. ID: {}", call_control_id);

    if let Err(e) = state.telnyx_service.start_transcription(&call_control_id).await {
        error!("❌ Error iniciando transcripción: {}", e);
    }

    (StatusCode::OK, Json(json!({"status": "handled"})))
}

async fn handle_transcription(
    state: Arc<AppState>,
    payload: serde_json::Value,
) -> (StatusCode, Json<serde_json::Value>) {
    let call_control_id = match payload["data"]["call_control_id"].as_str() {
        Some(id) => id.to_string(),
        None => return (StatusCode::BAD_REQUEST, Json(json!({"error": "Missing call_control_id"}))),
    };

    let transcript = payload["data"]["transcript"].as_str().unwrap_or("");
    let is_final = payload["data"]["is_final"].as_bool().unwrap_or(false);

    if !is_final || transcript.is_empty() {
        return (StatusCode::OK, Json(json!({"status": "buffering"})));
    }

    // ✅ Log corregido
    info!("📝 Transcripción recibida: '{}' (ID: {})", transcript, call_control_id);

    // Obtener sesión y generar respuesta
    if let Some(mut session_ref) = state.sessions.get_mut(&call_control_id) {
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
                                error!("❌ Error reproduciendo audio: {}", e);
                            }
                        }
                        Err(e) => error!("❌ Error subiendo audio a S3: {}", e),
                    }
                }
                Err(e) => error!("❌ Error generando audio con ElevenLabs: {}", e),
            }
        }
    } else {
        error!("⚠️ Sesión no encontrada para call_control_id: {}", call_control_id);
    }

    (StatusCode::OK, Json(json!({"status": "handled"})))
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
    info!("📵 Llamada finalizada. ID: {}", call_control_id);

    (StatusCode::OK, Json(json!({"status": "handled"})))
} 