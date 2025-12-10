mod handlers;
mod services;
mod models;
mod utils;
mod middleware;

use axum::{
    routing::{get, post},
    Router,
    http::StatusCode,
    Json,
};
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber;

use crate::handlers::{call, webhook, test};
use crate::services::AppState;

#[tokio::main]
async fn main() {
    // Load environment variables
    dotenv::dotenv().ok();
    
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env()
                .add_directive("telnyx_ai_service=debug".parse().unwrap())
        )
        .json()
        .init();

    tracing::info!("🚀 Iniciando Telnyx AI Service en Rust");

    // Create app state
    let state = Arc::new(AppState::new().await);

    // Define routes
    let app = Router::new()
        // Health check
        .route("/", get(root_handler))
        
        // API routes
        .route("/api/call/initiate", post(call::initiate_call))
        .route("/api/call/batch", post(call::batch_calls))
        .route("/api/sessions/stats", get(call::session_stats))
        .route("/api/health", get(health_check))
        
        // Test routes
        .route("/api/test/claude", post(test::test_claude))
        
        // Webhook routes
        .route("/webhook/telnyx", post(webhook::handle_telnyx_webhook))
        
        .layer(CorsLayer::permissive())
        .with_state(state);

    // Get port from env or use default
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .unwrap_or(3000);

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("Failed to bind to port");

    tracing::info!(
        port = port,
        environment = std::env::var("NODE_ENV").unwrap_or_else(|_| "development".to_string()),
        "📡 Servidor escuchando"
    );

    axum::serve(listener, app)
        .await
        .expect("Server error");
}

async fn root_handler() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "service": "Telnyx AI Service (Rust)",
        "version": "1.0.0",
        "status": "running",
        "endpoints": {
            "webhook": "/webhook/telnyx",
            "initiateCall": "POST /api/call/initiate",
            "batchCalls": "POST /api/call/batch",
            "sessionStats": "GET /api/sessions/stats",
            "health": "GET /api/health"
        }
    }))
}

async fn health_check() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::OK,
        Json(serde_json::json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339()
        }))
    )
}
