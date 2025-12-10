use std::sync::Arc;
use std::collections::HashMap;
use dashmap::DashMap;
use tracing::{info, error};
use crate::models::SessionInfo;
use super::{TelnyxService, ClaudeService, S3Service, ElevenLabsService};

pub struct AppState {
    pub telnyx_service: TelnyxService,
    pub claude_service: ClaudeService,
    pub elevenlabs_service: ElevenLabsService,
    pub s3_service: S3Service,
    pub greeting_urls: HashMap<String, String>,
    pub quick_reply_urls: HashMap<String, String>,
    pub sessions: Arc<DashMap<String, SessionInfo>>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub total_calls: std::sync::atomic::AtomicU64,
}

impl AppState {
    pub async fn new() -> Self {
        let s3_service = S3Service::new().await
            .expect("S3 Service debe estar configurado para usar ElevenLabs");

        let elevenlabs_service = ElevenLabsService::new();

        info!("✅ AppState inicializado con ElevenLabs + S3");

        Self {
            telnyx_service: TelnyxService::new(),
            claude_service: ClaudeService::new(),
            elevenlabs_service,
            s3_service,
            greeting_urls: HashMap::new(),
            quick_reply_urls: HashMap::new(),
            sessions: Arc::new(DashMap::new()),
            start_time: chrono::Utc::now(),
            total_calls: std::sync::atomic::AtomicU64::new(0),
        }
    }

    pub async fn get_or_generate_greeting(&self, greeting_key: &str) -> Option<String> {
        let text = match greeting_key {
            "morning" => "Buenos dias, bienvenido a Clinica Veterinaria LA WANDA Y MACARENA, hablas con Maria. Con quien tengo el gusto?",
            "afternoon" => "Buenas tardes, bienvenido a Clinica Veterinaria LA WANDA Y MACARENA, hablas con Maria. Con quien tengo el gusto?",
            "evening" => "Buenas noches, bienvenido a Clinica Veterinaria LA WANDA Y MACARENA, hablas con Maria. Con quien tengo el gusto?",
            _ => return None,
        };

        let s3_key = format!("audio/greeting_{}.mp3", greeting_key);

        // Verificar si ya existe en S3
        if self.s3_service.object_exists(&s3_key).await {
            let url = self.s3_service.get_url(&s3_key).await;
            info!("♻️ Reutilizando saludo existente: {} -> {}", greeting_key, url);
            return Some(url);
        }

        // Si no existe, generar y subir
        match self.elevenlabs_service.text_to_speech(text).await {
            Ok(bytes) => {
                match self.s3_service.upload_audio(&s3_key, bytes).await {
                    Ok(url) => Some(url),
                    Err(e) => {
                        error!("❌ Error subiendo saludo a S3: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                error!("❌ Error generando saludo con ElevenLabs: {}", e);
                None
            }
        }
    }

    pub async fn get_or_generate_quick_reply(&self, key: &str) -> Option<String> {
        let text = match key {
            "processing" => "Entendido, dame un segundo mientras preparo tu respuesta.",
            _ => return None,
        };

        let s3_key = format!("audio/quick_{}.mp3", key);

        // Verificar si ya existe en S3
        if self.s3_service.object_exists(&s3_key).await {
            let url = self.s3_service.get_url(&s3_key).await;
            info!("♻️ Reutilizando respuesta rapida existente: {} -> {}", key, url);
            return Some(url);
        }

        // Si no existe, generar y subir
        match self.elevenlabs_service.text_to_speech(text).await {
            Ok(bytes) => {
                match self.s3_service.upload_audio(&s3_key, bytes).await {
                    Ok(url) => Some(url),
                    Err(e) => {
                        error!("❌ Error subiendo respuesta rapida a S3: {}", e);
                        None
                    }
                }
            }
            Err(e) => {
                error!("❌ Error generando respuesta rapida con ElevenLabs: {}", e);
                None
            }
        }
    }
}
