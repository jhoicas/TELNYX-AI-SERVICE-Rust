use reqwest::Client;
use serde::{Deserialize, Serialize};
use tracing::{info, error, debug};

#[derive(Clone)]
pub struct ClaudeService {
    api_key: String,
    model: String,
    client: Client,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageRequest {
    model: String,
    max_tokens: i32,
    temperature: f32,
    system: String,
    messages: Vec<MessageContent>,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageContent {
    role: String,
    content: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct MessageResponse {
    content: Vec<ContentBlock>,
    usage: Usage,
}

#[derive(Debug, Serialize, Deserialize)]
struct ContentBlock {
    text: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Usage {
    input_tokens: i32,
    output_tokens: i32,
}

impl ClaudeService {
    pub fn new() -> Self {
        let api_key = std::env::var("ANTHROPIC_API_KEY")
            .expect("ANTHROPIC_API_KEY must be set");

        let model = std::env::var("CLAUDE_MODEL")
            .unwrap_or_else(|_| "claude-3-5-haiku-20241022".to_string());

        Self {
            api_key,
            model,
            client: Client::new(),
        }
    }

    pub async fn generate_response(
        &self,
        user_text: &str,
        nombre: &str,
        contexto: Option<&str>,
    ) -> anyhow::Result<String> {
        let system_prompt = self.get_system_prompt();

        let short_prompt = if let Some(ctx) = contexto {
            format!("Contexto: {}\nCliente ({}): {}", ctx, nombre, user_text)
        } else {
            format!("Cliente ({}): {}", nombre, user_text)
        };

        let request = MessageRequest {
            model: self.model.clone(),
            max_tokens: 120,
            temperature: 0.6,
            system: system_prompt,
            messages: vec![
                MessageContent {
                    role: "user".to_string(),
                    content: format!(
                        "{}\n\nResponde como María, natural (80-110 chars). Usa muletillas colombianas. NUNCA cortes frases:",
                        short_prompt
                    ),
                }
            ],
        };

        info!("🤖 [CLAUDE] Enviando request a modelo: {} (max_tokens: {}, temp: {})", self.model, request.max_tokens, request.temperature);
        info!("🤖 [CLAUDE] Prompt para {}: '{}'", nombre, short_prompt);

        let response = self.client
            .post("https://api.anthropic.com/v1/messages")
            .header("x-api-key", &self.api_key)
            .header("anthropic-version", "2023-06-01")
            .json(&request)
            .send()
            .await?;

        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await.unwrap_or_default();
            error!("❌ [CLAUDE] Error {} generando respuesta: {}", status, error_text);
            return Err(anyhow::anyhow!("Claude API failed with status {}", status));
        }

        let message_response: MessageResponse = response.json().await?;
        let response_text = message_response
            .content
            .first()
            .and_then(|c| c.text.as_ref())
            .cloned()
            .unwrap_or_else(|| "Disculpa, ¿puedes repetir eso?".to_string());

        let cleaned = self.clean_response(&response_text);
        
        info!(
            "✅ [CLAUDE] Respuesta generada para {}. Modelo: {}, Tokens in/out: {}/{}, Chars: {} -> {}",
            nombre,
            self.model,
            message_response.usage.input_tokens,
            message_response.usage.output_tokens,
            response_text.len(),
            cleaned.len()
        );
        info!("💬 [CLAUDE] Respuesta final: '{}'", cleaned);

        Ok(cleaned)
    }

    fn clean_response(&self, text: &str) -> String {
        text.trim()
            .lines()
            .next()
            .unwrap_or("")
            .trim()
            .to_string()
    }

    fn get_system_prompt(&self) -> String {
        "Eres María, la recepcionista de la Clínica Veterinaria LA WANDA Y MACARENA. Hablas como una persona real, cálida y cercana.

INFO CLAVE:
Abrimos lunes a viernes de 8AM a 8PM, sábados de 9AM a 6PM y domingos de 10AM a 2PM.
Emergencias 24/7 al 318 383 8417.
Ofrecemos consultas, vacunas, cirugías, peluquería y urgencias.

COMO HABLAR (MUY IMPORTANTE):
- SIEMPRE usa el NOMBRE del cliente cuando lo sepas
- Cuando el cliente diga su nombre, repítelo naturalmente
- Usa muletillas naturales: \"mirá\", \"dale\", \"sí claro\", \"perfecto entonces\"
- Contrae palabras como lo haría una persona: \"pa'\", \"to'\", \"pa' qué\"
- Usa expresiones colombianas suaves: \"qué pena contigo\", \"con mucho gusto\", \"listo entonces\"
- Responde con frases cortas y naturales (80-110 caracteres)
- NUNCA cortes a mitad de frase - termina la idea completa
- LEE BIEN lo que dice el cliente - si mencionan \"gata\" o \"perro\", NO vuelvas a preguntar
- Si dicen \"sí\", \"dale\", \"ok\" → ya sabes qué quieren, responde directo
- Sé empática con las mascotas: \"ay tu gatita\", \"pobrecito tu perrito\"
- Usa \"vos\" o \"tú\" de forma natural según el contexto".to_string()
    }
}