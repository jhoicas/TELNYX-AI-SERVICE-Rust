use axum::{
    extract::State,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::services::AppState;

#[derive(Debug, Deserialize)]
pub struct TestClaudeRequest {
    pub nombre: String,
    pub mensaje: String,
    pub contexto: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct TestClaudeResponse {
    pub success: bool,
    pub model: String,
    pub input_tokens: Option<i32>,
    pub output_tokens: Option<i32>,
    pub response: Option<String>,
    pub error: Option<String>,
}

/// Endpoint para probar Claude directamente sin hacer una llamada
/// POST /api/test/claude
/// Body: { "nombre": "Juan", "mensaje": "Hola, necesito una cita", "contexto": "opcional" }
pub async fn test_claude(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<TestClaudeRequest>,
) -> (StatusCode, Json<TestClaudeResponse>) {
    info!("🧪 [TEST CLAUDE] Iniciando prueba para: {}", payload.nombre);
    info!("🧪 [TEST CLAUDE] Mensaje: '{}'", payload.mensaje);
    
    match state.claude_service.generate_response(
        &payload.mensaje,
        &payload.nombre,
        payload.contexto.as_deref(),
    ).await {
        Ok(response) => {
            info!("✅ [TEST CLAUDE] Prueba exitosa. Respuesta: '{}'", response);
            (
                StatusCode::OK,
                Json(TestClaudeResponse {
                    success: true,
                    model: std::env::var("CLAUDE_MODEL")
                        .unwrap_or_else(|_| "claude-3-5-haiku-20241022".to_string()),
                    input_tokens: None, // Claude service no expone estos datos actualmente
                    output_tokens: None,
                    response: Some(response),
                    error: None,
                })
            )
        }
        Err(e) => {
            info!("❌ [TEST CLAUDE] Error en prueba: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(TestClaudeResponse {
                    success: false,
                    model: std::env::var("CLAUDE_MODEL")
                        .unwrap_or_else(|_| "claude-3-5-haiku-20241022".to_string()),
                    input_tokens: None,
                    output_tokens: None,
                    response: None,
                    error: Some(e.to_string()),
                })
            )
        }
    }
}
