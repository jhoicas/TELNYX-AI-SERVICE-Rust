use tracing::{info, warn};

pub fn log_startup_info() {
    info!("🚀 Telnyx AI Service iniciado");
    info!("Version: 1.0.0");
    info!("Framework: Axum (Rust)");
    info!("Runtime: Tokio");
    
    // Verificar configuración
    if std::env::var("TELNYX_API_KEY").is_ok() {
        info!("✅ Telnyx configurado");
    } else {
        warn!("⚠️ TELNYX_API_KEY no configurada");
    }

    if std::env::var("ANTHROPIC_API_KEY").is_ok() {
        info!("✅ Claude configurado");
    } else {
        warn!("⚠️ ANTHROPIC_API_KEY no configurada");
    }

    if std::env::var("WEBHOOK_BASE_URL").is_ok() {
        info!("✅ Webhook URL configurada");
    } else {
        warn!("⚠️ WEBHOOK_BASE_URL no configurada");
    }
}
