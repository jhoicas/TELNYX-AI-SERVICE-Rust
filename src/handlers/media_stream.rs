use axum::{
    extract::{State, ws::{WebSocket, WebSocketUpgrade, Message}},
    response::IntoResponse,
};
use std::sync::Arc;
use tokio::sync::mpsc;
use tracing::{info, error, warn, debug};
use futures_util::StreamExt;
use base64::{engine::general_purpose::STANDARD, Engine};
use chrono::Timelike;

use crate::services::{AppState, SessionManager, DeepgramWebSocket};

/// Handler para conexión WebSocket de Telnyx Media Streams
pub async fn handle_media_stream(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

async fn handle_socket(socket: WebSocket, state: Arc<AppState>) {
    info!("🔌 [MediaStream][Telnyx->WS] Nueva conexión establecida");

    let (_, mut ws_receiver) = socket.split();
    // Canal para coalescer audio y reducir overhead de frames pequeños
    let (coalesce_tx, mut coalesce_rx) = tokio::sync::mpsc::channel::<Vec<u8>>(256);
    
    // Esperamos el mensaje "start" que trae call_control_id; ignoramos handshakes como "connected".
    let call_id = loop {
        match ws_receiver.next().await {
            Some(Ok(Message::Text(text))) => {
                match serde_json::from_str::<serde_json::Value>(&text) {
                    Ok(json) => {
                        if let Some(event) = json.get("event").and_then(|e| e.as_str()) {
                            match event {
                                "start" => {
                                    let call_id = json.get("start")
                                        .and_then(|s| s.get("call_control_id"))
                                        .and_then(|c| c.as_str())
                                        .unwrap_or("unknown")
                                        .to_string();
                                    let stream_id = json.get("start")
                                        .and_then(|s| s.get("stream_id"))
                                        .and_then(|c| c.as_str())
                                        .unwrap_or("n/a");
                                    info!("📞 [CALL:{}][MediaStream] START recibido (stream_id={})", call_id, stream_id);
                                    break call_id;
                                }
                                other => {
                                    warn!("⚠️ [MediaStream] Mensaje inicial ignorado (event={}): {}", other, text);
                                    continue;
                                }
                            }
                        } else {
                            warn!("⚠️ [MediaStream] Mensaje sin evento: {}", text);
                            continue;
                        }
                    }
                    Err(e) => {
                        error!("❌ [MediaStream] Error parseando mensaje inicial: {}", e);
                        return;
                    }
                }
            }
            Some(Ok(_)) => {
                // Binario en handshake: seguir leyendo
                continue;
            }
            Some(Err(e)) => {
                error!("❌ [MediaStream] Error recibiendo mensaje inicial: {}", e);
                return;
            }
            None => {
                error!("❌ [MediaStream] No se recibió mensaje inicial");
                return;
            }
        }
    };

    // Crear sesión
    let session = SessionManager::create_session(
        call_id.clone(),
        "Cliente".to_string(),
        "desconocido".to_string(),
    );
    state.sessions.insert(call_id.clone(), session);

    // Conectar a Deepgram WebSocket
    let deepgram = DeepgramWebSocket::new();
    let (audio_tx, mut transcript_rx) = match deepgram.connect(call_id.clone()).await {
        Ok(channels) => channels,
        Err(e) => {
            error!("❌ [CALL:{}] Error conectando a Deepgram: {}", call_id, e);
            return;
        }
    };

    // Reproducir saludo
    tokio::spawn({
        let call_id = call_id.clone();
        let state = state.clone();
        async move {
            tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
            
            let hour = chrono::Utc::now().hour();
            let greeting_key = match hour {
                5..=11 => "morning",
                12..=18 => "afternoon",
                _ => "evening",
            };

            info!("🔊 [CALL:{}][TTS] Reproduciendo saludo: {}", call_id, greeting_key);
            
            if let Some(url) = state.get_or_generate_greeting(greeting_key).await {
                if let Err(e) = state.telnyx_service.play_audio(&call_id, &url).await {
                    error!("❌ [CALL:{}] Error reproduciendo saludo: {}", call_id, e);
                }
            }
        }
    });

    // Task para procesar audio de Telnyx → buffer de coalescing
    let call_id_audio = call_id.clone();
    tokio::spawn(async move {
        let mut frame_count: u64 = 0;
        while let Some(msg) = ws_receiver.next().await {
            match msg {
                Ok(Message::Text(text)) => {
                    // Parsear mensaje de Telnyx Media Stream
                    if let Ok(json) = serde_json::from_str::<serde_json::Value>(&text) {
                        if let Some(event) = json.get("event").and_then(|e| e.as_str()) {
                            match event {
                                "media" => {
                                    // Audio payload en base64 (mulaw)
                                    if let Some(payload) = json.get("media")
                                        .and_then(|m| m.get("payload"))
                                        .and_then(|p| p.as_str())
                                    {
                                        if let Ok(audio_data) = STANDARD.decode(payload) {
                                            frame_count += 1;
                                            if frame_count <= 5 {
                                                info!("🎤 [CALL:{}][Telnyx->Deepgram] frame#{} bytes={} b64_len={}", call_id_audio, frame_count, audio_data.len(), payload.len());
                                            } else {
                                                debug!("🎤 [CALL:{}][Telnyx->Deepgram] frame#{} bytes={} b64_len={}", call_id_audio, frame_count, audio_data.len(), payload.len());
                                            }
                                            // Enviar al buffer de coalescing
                                            if let Err(e) = coalesce_tx.send(audio_data).await {
                                                error!("❌ [CALL:{}] Error en buffer de coalescing: {}", call_id_audio, e);
                                                break;
                                            }
                                        }
                                    }
                                }
                                "stop" => {
                                    info!("🔚 [CALL:{}][MediaStream] STOP recibido", call_id_audio);
                                    break;
                                }
                                _ => {}
                            }
                        }
                    }
                }
                Ok(Message::Close(_)) => {
                    info!("🔚 [CALL:{}][MediaStream] WebSocket cerrado por cliente", call_id_audio);
                    break;
                }
                Err(e) => {
                    error!("❌ [CALL:{}][MediaStream] Error en WebSocket: {}", call_id_audio, e);
                    break;
                }
                _ => {}
            }
        }
    });

    // Task de coalescing: agrupa frames y los envía periódicamente a Deepgram
    let call_id_flush = call_id.clone();
    tokio::spawn(async move {
        let mut buffer: Vec<u8> = Vec::with_capacity(4096);
        let mut last_flush = tokio::time::Instant::now();
        let flush_interval = tokio::time::Duration::from_millis(40);
        loop {
            tokio::select! {
                maybe_chunk = coalesce_rx.recv() => {
                    match maybe_chunk {
                        Some(chunk) => {
                            buffer.extend_from_slice(&chunk);
                            if buffer.len() >= 2048 {
                                if let Err(e) = audio_tx.send(std::mem::take(&mut buffer)).await {
                                    error!("❌ [CALL:{}] Error enviando audio coalesced: {}", call_id_flush, e);
                                    break;
                                }
                                last_flush = tokio::time::Instant::now();
                            }
                        }
                        None => {
                            // canal cerrado
                            if !buffer.is_empty() {
                                let _ = audio_tx.send(std::mem::take(&mut buffer)).await;
                            }
                            info!("🔚 [CALL:{}] Coalescing finalizado", call_id_flush);
                            break;
                        }
                    }
                }
                _ = tokio::time::sleep(flush_interval) => {
                    if !buffer.is_empty() && last_flush.elapsed() >= flush_interval {
                        if let Err(e) = audio_tx.send(std::mem::take(&mut buffer)).await {
                            error!("❌ [CALL:{}] Error enviando audio coalesced (timer): {}", call_id_flush, e);
                            break;
                        }
                        last_flush = tokio::time::Instant::now();
                    }
                }
            }
        }
    });

    // Cola FIFO para reproducir respuestas TTS sin solaparse
    let (tts_tx, mut tts_rx) = tokio::sync::mpsc::channel::<String>(100);

    // Worker de reproducción: toma respuestas de la cola y las reproduce en orden
    let call_id_tts_worker = call_id.clone();
    let state_tts_worker = state.clone();
    tokio::spawn(async move {
        while let Some(response_text) = tts_rx.recv().await {
            // Generar audio con ElevenLabs
            match state_tts_worker.elevenlabs_service.text_to_speech(&response_text).await {
                Ok(audio_bytes) => {
                    let audio_key = format!("audio/response_{}_{}.mp3", 
                        call_id_tts_worker, 
                        chrono::Utc::now().timestamp()
                    );

                    // Subir a S3
                    match state_tts_worker.s3_service.upload_audio(&audio_key, audio_bytes).await {
                        Ok(url) => {
                            info!("🔊 [CALL:{}][TTS] Audio generado y subido: {}", call_id_tts_worker, url);
                            // Reproducir audio
                            if let Err(e) = state_tts_worker.telnyx_service.play_audio(&call_id_tts_worker, &url).await {
                                error!("❌ [CALL:{}] Error reproduciendo audio: {}", call_id_tts_worker, e);
                            }
                        }
                        Err(e) => {
                            error!("❌ [CALL:{}] Error subiendo audio a S3: {}", call_id_tts_worker, e);
                        }
                    }
                }
                Err(e) => {
                    error!("❌ [CALL:{}] Error generando audio: {}", call_id_tts_worker, e);
                }
            }
        }
        info!("🔚 [CALL:{}] TTS worker finalizado", call_id_tts_worker);
    });

    // Task para procesar transcripts de Deepgram → Claude → push a cola TTS
    let call_id_transcript = call_id.clone();
    let state_transcript = state.clone();
    tokio::spawn(async move {
        while let Some(transcript) = transcript_rx.recv().await {
            if transcript.channel.alternatives.is_empty() {
                continue;
            }

            let text = &transcript.channel.alternatives[0].transcript;
            let confidence = transcript.channel.alternatives[0].confidence;

            // Filtrar por confianza mínima
            if confidence < 0.6 {
                warn!("⚠️ [CALL:{}] Confianza baja: {} ({})", call_id_transcript, confidence, text);
                continue;
            }

            // Procesar solo si tiene suficiente contenido
            let word_count = text.split_whitespace().count();
            let ends_sentence = text.trim_end().ends_with('.') || text.trim_end().ends_with('?') || text.trim_end().ends_with('!');
            let should_process = transcript.is_final || (word_count >= 4 && (text.len() >= 14 || ends_sentence));

            if !should_process {
                continue;
            }

            let marker = if transcript.is_final {"FINAL"} else {"INTERIM"};
            let wc = word_count;
            let conf = confidence;
            info!("💬 [CALL:{}][Deepgram->App] {} (conf {:.2}, words {}): '{}'", call_id_transcript, marker, conf, wc, text);

            // Obtener sesión y contexto
            if let Some(mut session_ref) = state_transcript.sessions.get_mut(&call_id_transcript) {
                let context = SessionManager::get_conversation_context(&session_ref);

                // Generar respuesta con Claude
                match state_transcript.claude_service
                    .generate_response(
                        text,
                        &session_ref.nombre,
                        if context.is_empty() { None } else { Some(&context) },
                    )
                    .await
                {
                    Ok(response) => {
                        info!("🤖 [CALL:{}][Claude] Respuesta: '{}'", call_id_transcript, response);

                        // Agregar a historial
                        SessionManager::add_to_history(&mut session_ref, response.clone());
                        // Empujar respuesta a la cola TTS para reproducción ordenada
                        if let Err(e) = tts_tx.send(response).await {
                            error!("❌ [CALL:{}] Error encolar respuesta TTS: {}", call_id_transcript, e);
                        }
                    }
                    Err(e) => {
                        error!("❌ [CALL:{}] Error generando respuesta Claude: {}", call_id_transcript, e);
                    }
                }
            }
        }

        info!("🔚 [CALL:{}] Finalizando procesamiento de transcripts", call_id_transcript);
    });

    info!("✅ [CALL:{}] Pipeline WebSocket completo configurado", call_id);
}
