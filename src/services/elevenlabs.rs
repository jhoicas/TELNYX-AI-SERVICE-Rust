use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, error};

#[derive(Clone)]
pub struct ElevenLabsService {
    api_key: String,
    voice_id: String,
    base_url: String,
    client: Client,
}

#[derive(Debug, Serialize)]
struct TextToSpeechRequest {
    text: String,
    model_id: String,
    voice_settings: VoiceSettings,
}

#[derive(Debug, Serialize)]
struct VoiceSettings {
    stability: f32,
    similarity_boost: f32,
    style: f32,
    use_speaker_boost: bool,
}

impl ElevenLabsService {
    pub fn new() -> Self {
        let api_key = std::env::var("ELEVENLABS_API_KEY")
            .expect("ELEVENLABS_API_KEY must be set");
        
        // Voice ID por defecto (Rachel - voz femenina en español)
        // Puedes cambiarla por otra voz de ElevenLabs
        let voice_id = std::env::var("ELEVENLABS_VOICE_ID")
            .unwrap_or_else(|_| "21m00Tcm4TlvDq8ikWAM".to_string());

        info!("✅ ElevenLabs Service inicializado con voice_id: {}", voice_id);

        Self {
            api_key,
            voice_id,
            base_url: "https://api.elevenlabs.io/v1".to_string(),
            client: Client::new(),
        }
    }

    /// Genera audio desde texto usando ElevenLabs
    /// Retorna los bytes del audio en formato MP3
    pub async fn text_to_speech(&self, text: &str) -> anyhow::Result<Vec<u8>> {
        info!("🎤 Generando audio con ElevenLabs: '{}'", text);

        let request = TextToSpeechRequest {
            text: text.to_string(),
            model_id: "eleven_multilingual_v2".to_string(),
            voice_settings: VoiceSettings {
                stability: 0.5,
                similarity_boost: 0.75,
                style: 0.0,
                use_speaker_boost: true,
            },
        };

        let url = format!(
            "{}/text-to-speech/{}",
            self.base_url, self.voice_id
        );

        let response = self.client
            .post(&url)
            .header("xi-api-key", &self.api_key)
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await.unwrap_or_default();
            error!("❌ Error generando audio con ElevenLabs: {}", error_text);
            return Err(anyhow::anyhow!("Failed to generate audio"));
        }

        let audio_bytes = response.bytes().await?.to_vec();
        
        info!("✅ Audio generado exitosamente ({} bytes)", audio_bytes.len());

        Ok(audio_bytes)
    }

    /// Genera audio y retorna en base64 (útil para debugging o APIs)
    pub async fn text_to_speech_base64(&self, text: &str) -> anyhow::Result<String> {
        let audio_bytes = self.text_to_speech(text).await?;
        let base64_audio = base64::engine::general_purpose::STANDARD.encode(&audio_bytes);
        Ok(base64_audio)
    }
}
