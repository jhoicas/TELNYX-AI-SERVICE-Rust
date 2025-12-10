use serde::{Deserialize, Serialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InitiateCallRequest {
    pub telefono: String,
    pub nombre: String,
    pub contexto: Option<String>,
    pub saludo: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchCallsRequest {
    pub calls: Vec<InitiateCallRequest>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallResponse {
    pub call_control_id: String,
    pub call_id: String,
    pub status: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookPayload {
    pub data: serde_json::Value,
    pub meta: WebhookMeta,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebhookMeta {
    pub attempt: i32,
    pub delivered_at: String,
    pub event_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallAnsweredPayload {
    pub call_control_id: String,
    pub client_state: Option<String>,
    pub direction: String,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TranscriptionPayload {
    pub call_control_id: String,
    pub transcript: String,
    pub confidence: f32,
    pub is_final: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientState {
    pub nombre: String,
    pub telefono: String,
    pub contexto: Option<String>,
    pub call_control_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionInfo {
    pub call_control_id: String,
    pub nombre: String,
    pub telefono: String,
    pub contexto: Option<String>,
    pub created_at: DateTime<Utc>,
    pub conversation_history: Vec<String>,
    pub transcription_started: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatsResponse {
    pub active_sessions: usize,
    pub total_calls: u64,
    pub uptime_seconds: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub message: Option<String>,
}
