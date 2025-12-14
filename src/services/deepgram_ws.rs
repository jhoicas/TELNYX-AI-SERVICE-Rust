use anyhow::{Result, anyhow};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::sync::mpsc;
use tracing::{info, error, warn, debug};

#[derive(Clone)]
pub struct DeepgramWebSocket {
    api_key: String,
}

#[derive(Debug, Serialize)]
struct DeepgramConfig {
    encoding: String,
    sample_rate: u32,
    channels: u16,
    language: String,
    model: String,
    interim_results: bool,
    endpointing: u32,
    utterance_end_ms: u32,
    vad_turnoff: u32,
}

#[derive(Debug, Deserialize)]
pub struct DeepgramTranscript {
    pub channel: DeepgramChannel,
    pub is_final: bool,
}

#[derive(Debug, Deserialize)]
pub struct DeepgramChannel {
    pub alternatives: Vec<DeepgramAlternative>,
}

#[derive(Debug, Deserialize)]
pub struct DeepgramAlternative {
    pub transcript: String,
    pub confidence: f64,
}

impl DeepgramWebSocket {
    pub fn new() -> Self {
        let api_key = std::env::var("DEEPGRAM_API_KEY")
            .expect("DEEPGRAM_API_KEY must be set");

        Self { api_key }
    }

    /// Conecta a Deepgram WebSocket y retorna canales para audio y transcripts
    pub async fn connect(
        &self,
        call_id: String,
    ) -> Result<(mpsc::Sender<Vec<u8>>, mpsc::Receiver<DeepgramTranscript>)> {
        // Construir URL de conexión con parámetros
        let config = DeepgramConfig {
            encoding: "mulaw".to_string(),
            sample_rate: 8000,
            channels: 1,
            language: "es".to_string(),
            model: "nova-2".to_string(),
            interim_results: true,
            endpointing: 200,      // 200ms de silencio para finalizar
            utterance_end_ms: 500, // Detectar fin de frase rápido
            vad_turnoff: 300,      // VAD sensible
        };

        let url = format!(
            "wss://api.deepgram.com/v1/listen?encoding={}&sample_rate={}&channels={}&language={}&model={}&interim_results={}&endpointing={}&utterance_end_ms={}&vad_turnoff={}",
            config.encoding,
            config.sample_rate,
            config.channels,
            config.language,
            config.model,
            config.interim_results,
            config.endpointing,
            config.utterance_end_ms,
            config.vad_turnoff
        );

        info!("🔌 [CALL:{}][WS->Deepgram] Conectando", call_id);

        // Conectar con autenticación usando axum::http::Request (http 0.2)
        let request = axum::http::Request::builder()
            .uri(&url)
            .header("Authorization", format!("Token {}", self.api_key))
            .body(())
            .map_err(|e| anyhow!("Failed to build request: {}", e))?;

        let (ws_stream, _) = connect_async(request)
            .await
            .map_err(|e| anyhow!("WebSocket connection failed: {}", e))?;

        info!("✅ [CALL:{}][WS->Deepgram] Conectado", call_id);

        let (mut ws_write, mut ws_read) = ws_stream.split();

        // Canal para enviar audio a Deepgram
        let (audio_tx, mut audio_rx) = mpsc::channel::<Vec<u8>>(100);

        // Canal para recibir transcripts de Deepgram
        let (transcript_tx, transcript_rx) = mpsc::channel::<DeepgramTranscript>(100);

        // Task única de envío: audio + ping keepalive, usando el mismo sink
        let call_id_send = call_id.clone();
        tokio::spawn(async move {
            let mut ping_interval = tokio::time::interval(tokio::time::Duration::from_secs(25));
            loop {
                tokio::select! {
                    maybe_audio = audio_rx.recv() => {
                        match maybe_audio {
                            Some(audio_data) => {
                                if let Err(e) = ws_write.send(Message::Binary(audio_data)).await {
                                    error!("❌ [CALL:{}] Error enviando audio a Deepgram: {}", call_id_send, e);
                                    break;
                                }
                            }
                            None => {
                                debug!("📤 [CALL:{}] Canal de audio cerrado", call_id_send);
                                // Enviar Close limpio
                                let _ = ws_write.send(Message::Close(None)).await;
                                break;
                            }
                        }
                    }
                    _ = ping_interval.tick() => {
                        if let Err(e) = ws_write.send(Message::Ping(Vec::new())).await {
                            warn!("⚠️ [CALL:{}] Ping Deepgram falló: {}", call_id_send, e);
                            break;
                        } else {
                            debug!("📶 [CALL:{}] Ping Deepgram enviado", call_id_send);
                        }
                    }
                }
            }
            info!("🔚 [CALL:{}][WS->Deepgram] Cierre de envío", call_id_send);
        });

        // Task para recibir transcripts de Deepgram
        let call_id_recv = call_id.clone();
        tokio::spawn(async move {
            while let Some(msg) = ws_read.next().await {
                match msg {
                    Ok(Message::Text(text)) => {
                        match serde_json::from_str::<serde_json::Value>(&text) {
                            Ok(json) => {
                                // Deepgram envía varios tipos de mensajes
                                if let Some(transcript_obj) = json.get("channel") {
                                    match serde_json::from_value::<DeepgramTranscript>(json.clone()) {
                                        Ok(transcript) => {
                                            if !transcript.channel.alternatives.is_empty() {
                                                let text = &transcript.channel.alternatives[0].transcript;
                                                if !text.trim().is_empty() {
                                                    let wc = text.split_whitespace().count();
                                                    debug!("📝 [CALL:{}][Deepgram->WS] msg len={} words={} final={} conf={:.2}", call_id_recv, text.len(), wc, transcript.is_final, transcript.channel.alternatives[0].confidence);
                                                    info!("💬 [CALL:{}][Deepgram] {}: '{}'", call_id_recv, if transcript.is_final {"FINAL"} else {"INTERIM"}, text);
                                                    
                                                    if let Err(e) = transcript_tx.send(transcript).await {
                                                        error!("❌ [CALL:{}] Error enviando transcript: {}", call_id_recv, e);
                                                        break;
                                                    }
                                                }
                                            }
                                        }
                                        Err(e) => {
                                            warn!("⚠️ [CALL:{}] Error parseando transcript: {}", call_id_recv, e);
                                        }
                                    }
                                }
                            }
                            Err(e) => {
                                warn!("⚠️ [CALL:{}] Error parseando JSON de Deepgram: {}", call_id_recv, e);
                            }
                        }
                    }
                    Ok(Message::Close(_)) => {
                        info!("🔚 [CALL:{}] Deepgram cerró conexión", call_id_recv);
                        break;
                    }
                    Err(e) => {
                        error!("❌ [CALL:{}] Error en WebSocket Deepgram: {}", call_id_recv, e);
                        break;
                    }
                    _ => {}
                }
            }
            info!("🔚 [CALL:{}] Cerrando conexión de recepción Deepgram", call_id_recv);
        });

        Ok((audio_tx, transcript_rx))
    }
}
