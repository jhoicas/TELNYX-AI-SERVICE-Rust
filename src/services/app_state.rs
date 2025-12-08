use std::sync::Arc;
use dashmap::DashMap;
use tracing::info;
use crate::models::SessionInfo;
use super::{TelnyxService, ClaudeService, S3Service, ElevenLabsService};

pub struct AppState {
    pub telnyx_service: TelnyxService,
    pub claude_service: ClaudeService,
    pub elevenlabs_service: ElevenLabsService,
    pub s3_service: S3Service,
    pub sessions: Arc<DashMap<String, SessionInfo>>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub total_calls: std::sync::atomic::AtomicU64,
}

impl AppState {
    pub async fn new() -> Self {
        let s3_service = S3Service::new().await
            .expect("S3 Service debe estar configurado para usar ElevenLabs");

        info!("✅ AppState inicializado con ElevenLabs + S3");

        Self {
            telnyx_service: TelnyxService::new(),
            claude_service: ClaudeService::new(),
            elevenlabs_service: ElevenLabsService::new(),
            s3_service,
            sessions: Arc::new(DashMap::new()),
            start_time: chrono::Utc::now(),
            total_calls: std::sync::atomic::AtomicU64::new(0),
        }
    }
}
