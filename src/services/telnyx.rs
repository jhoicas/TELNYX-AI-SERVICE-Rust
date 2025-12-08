use serde::{Deserialize, Serialize};
use reqwest::Client;
use tracing::{info, error, debug};
use base64::{engine::general_purpose::STANDARD, Engine};
use crate::models::{ClientState, CallResponse};

#[derive(Clone)]
pub struct TelnyxService {
    api_key: String,
    connection_id: String,
    phone_number: String,
    base_url: String,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct InitiateCallPayload {
    connection_id: String,
    to: String,
    from: String,
    webhook_url: String,
    client_state: String,
    answering_machine_detection: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct TelnyxCallResponse {
    data: TelnyxCallData,
}

#[derive(Debug, Serialize, Deserialize)]
struct TelnyxCallData {
    call_control_id: String,
    #[serde(default)]
    call_id: Option<String>,
    #[serde(default)]
    call_leg_id: Option<String>,
    #[serde(default)]
    call_session_id: Option<String>,
    #[serde(default)]
    status: Option<String>,
    #[serde(default)]
    is_alive: Option<bool>,
    #[serde(default)]
    record_type: Option<String>,
}

impl TelnyxService {
    pub fn new() -> Self {
        let api_key = std::env::var("TELNYX_API_KEY")
            .expect("TELNYX_API_KEY must be set");
        let connection_id = std::env::var("TELNYX_CONNECTION_ID")
            .expect("TELNYX_CONNECTION_ID must be set");
        let phone_number = std::env::var("TELNYX_PHONE_NUMBER")
            .expect("TELNYX_PHONE_NUMBER must be set");

        Self {
            api_key,
            connection_id,
            phone_number,
            base_url: "https://api.telnyx.com/v2".to_string(),
            client: Client::new(),
        }
    }

    pub async fn initiate_call(
        &self,
        to: &str,
        nombre: &str,
        telefono: &str,
        contexto: Option<&str>,
    ) -> anyhow::Result<CallResponse> {
        let webhook_url = std::env::var("WEBHOOK_BASE_URL")
            .unwrap_or_else(|_| "https://your-domain.com".to_string());

        let client_state = ClientState {
            nombre: nombre.to_string(),
            telefono: telefono.to_string(),
            contexto: contexto.map(|s| s.to_string()),
            call_control_id: None,
        };

        let client_state_encoded = STANDARD.encode(serde_json::to_string(&client_state)?);

        let payload = InitiateCallPayload {
            connection_id: self.connection_id.clone(),
            to: to.to_string(),
            from: self.phone_number.clone(),
            webhook_url: format!("{}/webhook/telnyx", webhook_url),
            client_state: client_state_encoded,
            answering_machine_detection: "disabled".to_string(),
        };

        let response = self.client
            .post(format!("{}/calls", self.base_url))
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("❌ Error iniciando llamada: {}", error_text);
            return Err(anyhow::anyhow!("Failed to initiate call"));
        }

        let body = response.text().await.unwrap_or_default();
        let telnyx_response: TelnyxCallResponse = serde_json::from_str(&body)
            .map_err(|e| {
                error!("❌ Error decodificando respuesta de Telnyx: {} | body: {}", e, body);
                anyhow::anyhow!("Failed to parse Telnyx response: {}", e)
            })?;
        let data = telnyx_response.data;

        // ✅ Usar call_id si existe, sino usar call_leg_id o call_session_id
        let call_id = data.call_id
            .or_else(|| data.call_leg_id.clone())
            .or_else(|| data.call_session_id.clone())
            .unwrap_or_else(|| "unknown".to_string());

        info!(
            "📞 Llamada iniciada exitosamente. ID: {}, Call Leg ID: {:?}, To: {}",
            data.call_control_id,
            data.call_leg_id,
            to,
        );

        Ok(CallResponse {
            call_control_id: data.call_control_id,
            call_id,
            status: data.status.unwrap_or_else(|| "initiated".to_string()),
            timestamp: chrono::Utc::now(),
        })
    }

    pub async fn speak(
        &self,
        call_control_id: &str,
        message: &str,
    ) -> anyhow::Result<()> {
        #[derive(Serialize)]
        struct SpeakPayload {
            payload: String,
            voice: String,
            language: String,
        }

        let payload = SpeakPayload {
            payload: message.to_string(),
            voice: "female".to_string(),
            language: "es-MX".to_string(),
        };

        let response = self.client
            .post(format!("{}/calls/{}/actions/speak", self.base_url, call_control_id))
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("❌ Error en speak: {}", error_text);
            return Err(anyhow::anyhow!("Failed to speak"));
        }

        // ✅ CORREGIDO
        info!("✅ TTS speak enviado correctamente. ID: {}", call_control_id);
        Ok(())
    }

    pub async fn play_audio(
        &self,
        call_control_id: &str,
        audio_url: &str,
    ) -> anyhow::Result<()> {
        #[derive(Serialize)]
        struct PlaybackPayload {
            audio_url: String,
            #[serde(rename = "loop")]
            r#loop: i32,
            overlay: bool,
            target_legs: String,
            client_state: Option<String>,
        }

        // Preparar client_state para permitir barge-in/interrupción si es necesario
        let client_state_json = serde_json::json!({ "interruptible": true });
        let client_state_b64 = base64::engine::general_purpose::STANDARD.encode(serde_json::to_string(&client_state_json)?);

        let payload = PlaybackPayload {
            audio_url: audio_url.to_string(),
            r#loop: 1,
            overlay: false,
            target_legs: "self".to_string(),
            client_state: Some(client_state_b64),
        };

        let response = self.client
            .post(format!("{}/calls/{}/actions/playback_start", self.base_url, call_control_id))
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("❌ Error en playback: {}", error_text);
            return Err(anyhow::anyhow!("Failed to play audio"));
        }

        // ✅ CORREGIDO - loguear status para diagnóstico
        info!("✅ Playback iniciado. ID: {}, status: {}", call_control_id, response.status());
        Ok(())
    }

    pub async fn start_transcription(&self, call_control_id: &str) -> anyhow::Result<()> {
        let webhook_url = std::env::var("WEBHOOK_BASE_URL")
            .unwrap_or_else(|_| "https://your-domain.com".to_string());

        #[derive(Serialize)]
        struct TranscriptionPayload {
            transcription_engine: String,
            language: String,
            webhook_url: String,
        }

        let payload = TranscriptionPayload {
            // Telnyx docs recommend "ai" or "google"; we use "ai" for better accuracy
            transcription_engine: "ai".to_string(),
            // Use locale for Spanish to improve recognition
            language: "es-ES".to_string(),
            webhook_url: format!("{}/webhook/telnyx", webhook_url),
        };

        let response = self.client
            .post(format!("{}/calls/{}/actions/transcription_start", self.base_url, call_control_id))
            .bearer_auth(&self.api_key)
            .json(&payload)
            .send()
            .await?;

        let body = response.text().await.unwrap_or_default();
        if !response.status().is_success() {
            error!("❌ Error iniciando transcripción: {}", body);
            return Err(anyhow::anyhow!("Failed to start transcription"));
        }

        // Log full body for visibility (Telnyx may return warnings even on 200)
        debug!("🎤 Transcripción iniciada. ID: {} | body: {}", call_control_id, body);
        Ok(())
    }

    pub async fn hangup(&self, call_control_id: &str) -> anyhow::Result<()> {
        let response = self.client
            .post(format!("{}/calls/{}/actions/hangup", self.base_url, call_control_id))
            .bearer_auth(&self.api_key)
            .send()
            .await?;

        if !response.status().is_success() {
            error!("❌ Error colgando llamada");
            return Err(anyhow::anyhow!("Failed to hangup"));
        }

        // ✅ CORREGIDO
        info!("📵 Llamada colgada. ID: {}", call_control_id);
        Ok(())
    }
}