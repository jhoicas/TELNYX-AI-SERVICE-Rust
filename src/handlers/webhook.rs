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
use chrono::{Timelike, FixedOffset, Utc}; // ✅ Necesario para .hour() y zona horaria Bogotá

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

    // Log con full payload para transcripción
    if event_type.contains("transcription") {
        info!("📡 [CALL:{}] Webhook completo (transcription event): {}", call_id, serde_json::to_string(&payload).unwrap_or_default());
    }

    info!("📨 [CALL:{}] Webhook recibido: {}", call_id, event_type);

    match event_type {
        "call.answered" => handle_call_answered(state, payload).await,
        "call.speak.ended" => handle_speak_ended(state, payload).await,
        "call.playback.started" => handle_playback_started(state, payload).await,
        "call.playback.ended" => handle_playback_ended(state, payload).await,
        "call.transcription" => handle_transcription(state, payload).await,
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

    // Generar saludo usando hora de Bogotá (UTC-5)
    let bogota_tz = FixedOffset::west_opt(5 * 3600).unwrap();
    let hour = Utc::now().with_timezone(&bogota_tz).hour();
    let greeting_key = match hour {
        5..=11 => "morning",
        12..=18 => "afternoon",
        _ => "evening",
    };

    info!("🔊 Obteniendo saludo para: {}. ID: {}", greeting_key, call_control_id);

    // Obtener o generar audio bajo demanda y reproducir saludo
    if let Some(url) = state.get_or_generate_greeting(greeting_key).await {
        if let Err(e) = state.telnyx_service.play_audio(&call_control_id, &url).await {
            error!("❌ Error reproduciendo audio: {}", e);
        }
    } else {
        error!("⚠️ No se pudo obtener saludo para: {}", greeting_key);
    }

    let use_media_streams = std::env::var("USE_MEDIA_STREAMS")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    if use_media_streams {
        let call_id_for_stream = call_control_id.clone();
        let telnyx_svc = state.telnyx_service.clone();

        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(300)).await; // pequeño delay para que la llamada esté viva
            if let Err(e) = telnyx_svc.start_media_stream(&call_id_for_stream).await {
                error!("❌ [CALL:{}] Error iniciando Media Stream: {}", call_id_for_stream, e);
            } else {
                info!("✅ [CALL:{}] Media Stream iniciado en paralelo al saludo", call_id_for_stream);
            }
        });

        info!("📡 [CALL:{}] Intentando iniciar Media Stream en paralelo", call_control_id);
    } else {
        // 🎙️ Fallback: transcripción clásica vía webhook de Telnyx
        let call_id_for_transcription = call_control_id.clone();
        let telnyx_svc = state.telnyx_service.clone();
        let session_mgr = state.sessions.clone();
        
        tokio::spawn(async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            
            if let Some(mut sess) = session_mgr.get_mut(&call_id_for_transcription) {
                sess.transcription_started = true;
            }
            
            if let Err(e) = telnyx_svc.start_transcription(&call_id_for_transcription).await {
                error!("❌ [CALL:{}] Error iniciando transcripción paralela: {}", call_id_for_transcription, e);
            } else {
                info!("✅ [CALL:{}] Transcripción iniciada EN PARALELO con saludo", call_id_for_transcription);
            }
        });

        info!("📡 [CALL:{}] Transcripción iniciándose en paralelo con saludo", call_control_id);
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

    // ✅ Iniciar transcripción SOLO en modo webhook (cuando NO usamos Media Streams)
    let use_media_streams = std::env::var("USE_MEDIA_STREAMS")
        .unwrap_or_else(|_| "true".to_string())
        .parse::<bool>()
        .unwrap_or(true);

    if use_media_streams {
        info!("⏸️ [CALL:{}] Playback finalizado - Media Streams activo, sin iniciar transcripción Telnyx", call_control_id);
    } else if let Some(mut session) = state.sessions.get_mut(&call_control_id) {
        if !session.transcription_started {
            info!("🎙️ [CALL:{}] Iniciando transcripción después del saludo", call_control_id);
            if let Err(e) = state.telnyx_service.start_transcription(&call_control_id).await {
                error!("❌ Error iniciando transcripción: {}", e);
            } else {
                session.transcription_started = true;
                info!("✅ [CALL:{}] Transcripción iniciada - esperando audio del usuario", call_control_id);
            }
        } else {
            info!("⏸️ [CALL:{}] Playback finalizado - transcripción sigue activa", call_control_id);
        }
    } else {
        info!("⏸️ [CALL:{}] Playback finalizado (sesión no encontrada)", call_control_id);
    }

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
        .or_else(|| payload["data"]["payload"]["transcription_data"]["transcript"].as_str())
        .unwrap_or("");
    let is_final = payload["data"]["is_final"].as_bool()
        .or_else(|| payload["data"]["payload"]["is_final"].as_bool())
        .or_else(|| payload["data"]["payload"]["transcription_data"]["is_final"].as_bool())
        .unwrap_or(false);

    info!("📝 [CALL:{}] Evento transcripción - final: {}, texto: '{}'", call_control_id, is_final, transcript);

    // Limpieza temprana
    let transcript_clean = sanitize_plain(transcript).trim().to_string();
    
    // 🚀 OPTIMIZACIÓN: Procesar transcripts intermedios si tienen contenido suficiente
    // Esto reduce latencia de 6-12s a 1-3s
    let word_count = transcript_clean.split_whitespace().count();
    let should_process = is_final || (word_count >= 5 && transcript_clean.len() >= 15);

    if transcript_clean.is_empty() || (!should_process && !is_final) {
        info!("⏳ [CALL:{}] Ignorando (vacío o muy corto: {} palabras)", call_control_id, word_count);
        return (StatusCode::OK, Json(json!({"status": "buffering"})));
    }

    if !is_final {
        info!("⚡ [CALL:{}] Procesando transcript INTERMEDIO ({} palabras) - optimización de latencia", call_control_id, word_count);
    }

    // Log de transcripción cruda y limpia
    info!("📝 [CALL:{}] Transcripción recibida: '{}'", call_control_id, transcript);
    info!("🧹 [CALL:{}] Transcripción limpia: '{}'", call_control_id, transcript_clean);

    // Obtener sesión y generar respuesta
    if let Some(mut session_ref) = state.sessions.get_mut(&call_control_id) {
        let context = SessionManager::get_conversation_context(&session_ref);

        // Respuesta rápida opcional mientras se procesa la final
        if is_quick_reply_enabled() {
            if let Some(url) = state.get_or_generate_quick_reply("processing").await {
                if let Err(e) = state.telnyx_service.play_audio(&call_control_id, &url).await {
                    error!("❌ [CALL:{}] Error reproduciendo quick-reply: {}", call_control_id, e);
                }
            }
        }

        if let Ok(response) = state.claude_service
            .generate_response(
                &transcript_clean,
                &session_ref.nombre,
                if context.is_empty() { None } else { Some(&context) },
            )
            .await
        {
            let response_clean = sanitize_plain(&response);

            SessionManager::add_to_history(&mut session_ref, response_clean.clone());

            // Log de respuesta limpia antes de TTS
            info!("💬 [CALL:{}] Respuesta limpia: '{}'", call_control_id, response_clean);

            // Generar audio con ElevenLabs, subir a S3 y reproducir
            match state.elevenlabs_service.text_to_speech(&response_clean).await {
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

// Deja solo caracteres ASCII imprimibles y colapsa espacios
fn sanitize_plain(input: &str) -> String {
    let mut cleaned = String::new();
    let mut last_space = false;
    for c in input.chars() {
        // Permitir todos los caracteres excepto caracteres de control
        if !c.is_control() {
            let ch = if c.is_whitespace() { ' ' } else { c };
            if ch == ' ' {
                if !last_space {
                    cleaned.push(' ');
                    last_space = true;
                }
            } else {
                cleaned.push(ch);
                last_space = false;
            }
        }
    }
    cleaned.trim().to_string()
}

fn is_quick_reply_enabled() -> bool {
    std::env::var("QUICK_REPLY_ENABLED")
        .map(|v| v.eq_ignore_ascii_case("true") || v == "1")
        .unwrap_or(false)
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
    info!("🟡 [CALL:{}] Transcripción parcial: '{}'", call_control_id, transcript);
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