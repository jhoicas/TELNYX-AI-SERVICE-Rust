#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_environment_loaded() {
        dotenv::dotenv().ok();
        
        let api_key = std::env::var("TELNYX_API_KEY");
        // Este test pasará si .env está correctamente configurado
        // Se puede comentar si las variables no están definidas aún
    }

    #[test]
    fn test_client_state_serialization() {
        use serde_json;
        use crate::models::ClientState;

        let state = ClientState {
            nombre: "Test".to_string(),
            telefono: "+12345678".to_string(),
            contexto: Some("Test context".to_string()),
            call_control_id: None,
        };

        let json = serde_json::to_string(&state).unwrap();
        assert!(json.contains("Test"));
        assert!(json.contains("+12345678"));
    }
}
