use std::sync::Arc;
use dashmap::DashMap;
use tracing::info;
use crate::models::SessionInfo;
use super::{TelnyxService, ClaudeService, S3Service};

pub struct AppState {
    pub telnyx_service: TelnyxService,
    pub claude_service: ClaudeService,
    pub s3_service: Option<S3Service>,
    pub sessions: Arc<DashMap<String, SessionInfo>>,
    pub start_time: chrono::DateTime<chrono::Utc>,
    pub total_calls: std::sync::atomic::AtomicU64,
}

impl AppState {
    pub async fn new() -> Self {
        let s3_service = match S3Service::new().await {
            Ok(svc) => Some(svc),
            Err(e) => {
                tracing::warn!("⚠️ S3 Service no disponible: {}", e);
                None
            }
        };

        info!("✅ AppState inicializado");

        Self {
            telnyx_service: TelnyxService::new(),
            claude_service: ClaudeService::new(),
            s3_service,
            sessions: Arc::new(DashMap::new()),
            start_time: chrono::Utc::now(),
            total_calls: std::sync::atomic::AtomicU64::new(0),
        }
    }
}
