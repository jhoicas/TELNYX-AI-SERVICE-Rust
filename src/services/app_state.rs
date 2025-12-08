use std::sync::Arc;
use std::collections::HashMap;
use dashmap::DashMap;
use tracing::info;
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

        // Pre-generar y cachear saludos y respuestas rápidas para no generar en cada llamada
        let (greeting_urls, quick_reply_urls) = prewarm_audio(&elevenlabs_service, &s3_service).await;

        info!("✅ AppState inicializado con ElevenLabs + S3");

        Self {
            telnyx_service: TelnyxService::new(),
            claude_service: ClaudeService::new(),
            elevenlabs_service,
            s3_service,
            greeting_urls,
            quick_reply_urls,
            sessions: Arc::new(DashMap::new()),
            start_time: chrono::Utc::now(),
            total_calls: std::sync::atomic::AtomicU64::new(0),
        }
    }
}

async fn prewarm_audio(
    eleven: &ElevenLabsService,
    s3: &S3Service,
) -> (HashMap<String, String>, HashMap<String, String>) {
    let greetings = vec![
        ("morning", "Buenos días, bienvenido a Clínica Veterinaria LA WANDA Y MACARENA, hablas con María. ¿Con quién tengo el gusto?"),
        ("afternoon", "Buenas tardes, bienvenido a Clínica Veterinaria LA WANDA Y MACARENA, hablas con María. ¿Con quién tengo el gusto?"),
        ("evening", "Buenas noches, bienvenido a Clínica Veterinaria LA WANDA Y MACARENA, hablas con María. ¿Con quién tengo el gusto?"),
    ];

    let quick_replies = vec![
        ("processing", "Entendido, dame un segundo mientras preparo tu respuesta."),
    ];

    let mut greeting_urls = HashMap::new();
    let mut quick_reply_urls = HashMap::new();

    for (key, text) in greetings {
        if let Ok(bytes) = eleven.text_to_speech(text).await {
            let s3_key = format!("audio/greeting_{}.mp3", key);
            if let Ok(url) = s3.upload_audio(&s3_key, bytes).await {
                greeting_urls.insert(key.to_string(), url);
            }
        }
    }

    for (key, text) in quick_replies {
        if let Ok(bytes) = eleven.text_to_speech(text).await {
            let s3_key = format!("audio/quick_{}.mp3", key);
            if let Ok(url) = s3.upload_audio(&s3_key, bytes).await {
                quick_reply_urls.insert(key.to_string(), url);
            }
        }
    }

    (greeting_urls, quick_reply_urls)
}
