# Migración a WebSocket Media Streams

## 🚀 Beneficios

**Latencia reducida de 6-12s → 1-2s**

- **Webhooks tradicionales**: 6-12 segundos (espera silencio)
- **WebSocket Media Streams**: 1-2 segundos (streaming tiempo real)

## Arquitectura

### ANTES (Webhooks):
```
Usuario → Telnyx → Deepgram → HTTP POST → Servidor
                       ↓ (6-12s delay)
```

### AHORA (WebSockets):
```
Usuario → Telnyx WS ⟷ Servidor ⟷ Deepgram WS
              ↓ (streaming < 1s)
```

## Configuración

### 1. Variables de Entorno

Agregar a `.env`:

```bash
# Deepgram API Key (necesario para WebSocket directo)
DEEPGRAM_API_KEY=tu_deepgram_api_key

# Habilitar Media Streams (true por defecto)
USE_MEDIA_STREAMS=true

# URL base con protocolo WebSocket
WEBHOOK_BASE_URL=https://tu-dominio.com  # Se convierte a wss:// automáticamente
```

### 2. Obtener Deepgram API Key

1. Ir a [Deepgram Console](https://console.deepgram.com/)
2. Crear proyecto
3. Generar API Key
4. Copiar en `.env`

### 3. Deploy

El servidor ahora expone dos endpoints:

**Webhooks tradicionales** (fallback):
```
POST https://tu-dominio.com/webhook/telnyx
```

**WebSocket Media Streams** (por defecto):
```
WSS wss://tu-dominio.com/stream/media
```

## Uso

### Iniciar Llamada con Media Streams

```bash
curl -X POST https://tu-dominio.com/api/call/initiate \
  -H "Content-Type: application/json" \
  -d '{
    "nombre": "Juan",
    "telefono": "+573001234567"
  }'
```

El sistema automáticamente usa WebSocket si `USE_MEDIA_STREAMS=true`.

### Alternar entre Webhooks y WebSocket

**Usar WebSocket (recomendado)**:
```bash
USE_MEDIA_STREAMS=true
```

**Volver a Webhooks (si hay problemas)**:
```bash
USE_MEDIA_STREAMS=false
```

## Pipeline en Tiempo Real

1. **Audio streaming**: Telnyx envía audio mulaw 8kHz
2. **Transcripción inmediata**: Deepgram procesa en < 500ms
3. **IA responde**: Claude genera respuesta en ~1s
4. **TTS instantáneo**: ElevenLabs Turbo en ~0.5s
5. **Reproducción**: Audio vía S3 en ~0.3s

**Total: 1-2 segundos** ⚡

## Parámetros de Optimización

En `src/services/deepgram_ws.rs`:

```rust
DeepgramConfig {
    encoding: "mulaw",        // Formato Telnyx
    sample_rate: 8000,        // Telefonía estándar
    channels: 1,              // Mono
    language: "es",           // Español
    model: "nova-2",          // Modelo más rápido
    interim_results: true,    // Resultados parciales
    endpointing: 200,         // 200ms silencio para finalizar
    utterance_end_ms: 500,    // Detectar fin de frase rápido
    vad_turnoff: 300,         // VAD sensible
}
```

## Troubleshooting

### Error: WebSocket connection refused

**Causa**: Firewall o proxy bloqueando WebSockets

**Solución**:
```bash
# Verificar que puerto soporta WSS
curl -i -N -H "Connection: Upgrade" -H "Upgrade: websocket" \
  https://tu-dominio.com/stream/media
```

### Error: Deepgram authentication failed

**Causa**: API Key incorrecta

**Solución**:
```bash
# Verificar API Key
echo $DEEPGRAM_API_KEY
# Debe ser formato: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx
```

### Latencia sigue alta

**Causa**: Procesando transcripts solo cuando son finales

**Solución**: Ya configurado para procesar transcripts intermedios (≥3 palabras)

```rust
// En media_stream.rs
let should_process = transcript.is_final || (word_count >= 3 && text.len() >= 10);
```

## Comparación de Latencias

| Componente | Webhooks | WebSocket |
|------------|----------|-----------|
| Transcripción | 6-12s | 0.5-1s |
| Claude | 1.5s | 1.5s |
| TTS | 0.5s | 0.5s |
| S3 + Playback | 0.5s | 0.5s |
| **TOTAL** | **8-14s** | **2-3s** |

## Rollback

Para volver a webhooks tradicionales:

1. Set `USE_MEDIA_STREAMS=false` en `.env`
2. Reiniciar servidor
3. Sistema usa webhooks automáticamente

No es necesario cambiar código ni redeployar.

## Monitoreo

Logs indican qué modo está activo:

**WebSocket**:
```
🔌 [CALL:xxx] Conectando a Deepgram WebSocket
✅ [CALL:xxx] Conectado a Deepgram WebSocket
⚡ [CALL:xxx] Procesando transcript INTERMEDIO
```

**Webhooks**:
```
📨 [CALL:xxx] Webhook recibido: call.transcription
📝 [CALL:xxx] Transcripción recibida
```

## Referencias

- [Telnyx Media Streams Docs](https://developers.telnyx.com/docs/api/v2/call-control/Media-Streams)
- [Deepgram WebSocket API](https://developers.deepgram.com/docs/streaming)
- [tokio-tungstenite](https://docs.rs/tokio-tungstenite/)
