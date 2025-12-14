use dashmap::DashMap;
use std::sync::Arc;
use chrono::Utc;
use crate::models::SessionInfo;

pub type Sessions = Arc<DashMap<String, SessionInfo>>;

pub struct SessionManager;

impl SessionManager {
    pub fn create_session(call_control_id: String, nombre: String, telefono: String) -> SessionInfo {
        SessionInfo {
            call_control_id: call_control_id.clone(),
            nombre,
            telefono,
            contexto: None,
            created_at: Utc::now(),
            conversation_history: Vec::new(),
            transcription_started: false,
        }
    }

    pub fn add_to_history(session: &mut SessionInfo, message: String) {
        session.conversation_history.push(message);
        // Mantener solo el último mensaje (contexto muy corto = respuestas rápidas)
        if session.conversation_history.len() > 1 {
            session.conversation_history.remove(0);
        }
    }

    pub fn get_conversation_context(session: &SessionInfo) -> String {
        session.conversation_history.join(" | ")
    }
}
