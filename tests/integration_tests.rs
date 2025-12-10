#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_environment_loaded() {
        dotenv::dotenv().ok();
        
        // Verificar que al menos una variable clave esté presente
        let has_telnyx = std::env::var("TELNYX_API_KEY").is_ok();
        let has_anthropic = std::env::var("ANTHROPIC_API_KEY").is_ok();
        
        // El test pasa si alguna variable está configurada
        assert!(
            has_telnyx || has_anthropic,
            "Al menos una API key debe estar configurada (TELNYX_API_KEY o ANTHROPIC_API_KEY)"
        );
    }

    #[test]
    fn test_client_state_serialization() {
        use serde_json;
        use crate::models::ClientState;

        let state = ClientState {
            nombre: "Test".to_string(),
            telefono: "+12345678".to_string(),
            contexto: Some("Test context".to_string()),
            call_control_id: Some("test_call_id".to_string()),
        };

        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("Test"));
        assert!(json.contains("+12345678"));
        assert!(json.contains("test_call_id"));
    }
}
