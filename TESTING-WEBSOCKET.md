# Testing Plan: WebSocket Migration

## Pre-requisitos

✅ Instalar Visual Studio 2022 Build Tools con C++ (para cargo)
✅ Tener Rust instalado
✅ Variables de entorno configuradas:

```bash
DEEPGRAM_API_KEY=tu_api_key_aqui
TELNYX_API_KEY=tu_telnyx_key
WEBHOOK_BASE_URL=https://tu-dominio.com
USE_MEDIA_STREAMS=true
CLAUDE_API_KEY=tu_claude_key
ELEVENLABS_API_KEY=tu_elevenlabs_key
AWS_ACCESS_KEY_ID=tu_aws_key
AWS_SECRET_ACCESS_KEY=tu_aws_secret
AWS_REGION=tu_region
AWS_BUCKET_NAME=tu_bucket
S3_BUCKET_FOLDER=audios
TELNYX_CONNECTION_ID=tu_connection_id
TELNYX_PHONE_NUMBER=tu_numero
```

## Fase 1: Compilación

```bash
# Compilar el proyecto
& "$env:USERPROFILE\.cargo\bin\cargo.exe" build --release

# Si hay errores, verificar que link.exe está disponible
where link.exe
```

**Esperado**: Build exitoso sin errores

---

## Fase 2: Unit Tests (Locales)

```bash
# Ejecutar tests unitarios
& "$env:USERPROFILE\.cargo\bin\cargo.exe" test --lib -- --nocapture
```

**Esperado**: Tests pasen sin errores

---

## Fase 3: Verificación de Rutas

```bash
# El servidor debe tener estas rutas disponibles:

# NUEVA - WebSocket Media Streams
GET /stream/media

# LEGACY - Webhooks (fallback)
POST /webhook/telnyx

# API Call Initiation
POST /api/call/initiate
```

---

## Fase 4: Test Manual - WebSocket Connection

### Opción A: Con curl (verificar que servidor acepta upgrade)

```bash
curl -i -N -H "Connection: Upgrade" -H "Upgrade: websocket" -H "Sec-WebSocket-Key: x3JJHMbDL1EzLkh9GBhXDw==" -H "Sec-WebSocket-Version: 13" http://localhost:3000/stream/media
```

**Esperado**: Respuesta HTTP 101 Switching Protocols

### Opción B: Con websocat (si está instalado)

```bash
# Instalar websocat (opcional)
cargo install websocat

# Conectar
websocat ws://localhost:3000/stream/media
```

---

## Fase 5: Test de Latencia Real

### Llamada Test 1: WebSocket Mode (USE_MEDIA_STREAMS=true)

```bash
# 1. Iniciar servidor
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run --release

# En otra terminal:
# 2. Hacer llamada de prueba
curl -X POST http://localhost:3000/api/call/initiate \
  -H "Content-Type: application/json" \
  -d '{
    "nombre": "Juan",
    "telefono": "+5730001234567"
  }'

# 3. Monitorear logs
# Buscar estos eventos:
# ✅ "🔌 Conectando a Deepgram WebSocket"
# ✅ "🎤 Recibiendo audio de Telnyx"
# ✅ "⚡ Procesando transcript INTERMEDIO"
# ✅ "💬 Respuesta Claude generada"
```

**Esperado**:
- Latencia primera respuesta: < 2 segundos
- Transcripts intermedios procesados inmediatamente
- Logs mostrando flujo WebSocket

### Llamada Test 2: Webhook Fallback Mode (USE_MEDIA_STREAMS=false)

```bash
# 1. Actualizar .env
USE_MEDIA_STREAMS=false

# 2. Reiniciar servidor
# 3. Hacer misma llamada
curl -X POST http://localhost:3000/api/call/initiate \
  -H "Content-Type: application/json" \
  -d '{
    "nombre": "Pedro",
    "telefono": "+5730001234568"
  }'

# 4. Monitorear logs
# Buscar:
# ✅ "📨 [CALL:xxx] Webhook recibido"
# ✅ "📝 Transcripción recibida"
```

**Esperado**:
- Latencia: 6-12 segundos (como antes)
- Logs mostrando webhook tradicional

---

## Fase 6: Métricas de Latencia

### Estructura de Log para Medir

En los logs, buscar timestamps para calcular:

```
[2025-12-12T12:30:45.123Z] 📞 [CALL:xxxx] Llamada iniciada
[2025-12-12T12:30:45.500Z] 🔌 [CALL:xxxx] Conectando a Deepgram
[2025-12-12T12:30:46.100Z] 🎤 [CALL:xxxx] Audio recibido
[2025-12-12T12:30:46.350Z] ⚡ [CALL:xxxx] Procesando transcript: "hola"
[2025-12-12T12:30:47.200Z] 💬 [CALL:xxxx] Claude respuesta generada
[2025-12-12T12:30:47.800Z] 🔊 [CALL:xxxx] Audio reproducido
```

**Latencia = Último log - Primer log = ~2.7 segundos**

### Tabla de Resultados

Ejecutar 5 llamadas de prueba con cada modo:

| Test | Modo | Transcripción | Claude | TTS | Total |
|------|------|---------------|--------|-----|-------|
| 1 | WebSocket | 0.8s | 1.2s | 0.6s | 2.6s |
| 2 | WebSocket | 0.7s | 1.3s | 0.5s | 2.5s |
| 3 | WebSocket | 0.9s | 1.1s | 0.7s | 2.7s |
| 4 | WebSocket | 0.8s | 1.2s | 0.6s | 2.6s |
| 5 | WebSocket | 0.7s | 1.4s | 0.5s | 2.6s |
| **AVG** | **WebSocket** | **0.78s** | **1.24s** | **0.58s** | **2.6s** |

---

## Fase 7: Edge Cases

### Test: Desconexión WebSocket

```
✓ Usuario cuelga durante transcripción
✓ Servidor pierde conexión con Deepgram
✓ Reconexión automática a Deepgram
✓ Logs registran desconexión
```

### Test: Cambio de Modo Runtime

```
✓ USE_MEDIA_STREAMS=false sin reiniciar (cache)
✓ Siguiente llamada usa webhooks
✓ Sin errores de transición
```

### Test: Múltiples Llamadas Concurrentes

```
✓ 3 llamadas simultáneas con WebSocket
✓ Cada una con su Deepgram connection
✓ No hay interferencia entre llamadas
✓ Performance estable
```

---

## Fase 8: Checklist Final

- [ ] Compilación exitosa sin warnings críticos
- [ ] Rutas `/stream/media` y `/webhook/telnyx` disponibles
- [ ] WebSocket handshake funciona
- [ ] Deepgram API key válida y conecta
- [ ] Primera respuesta WebSocket < 2.5s
- [ ] Transcript intermedios se procesan
- [ ] Fallback a webhooks funciona
- [ ] Logs claros y trazables
- [ ] Sin memory leaks (monitorear conexiones)
- [ ] Manejo de errores robusto

---

## Troubleshooting

### Error: "DEEPGRAM_API_KEY not found"

```bash
# Verificar que .env contiene:
echo $env:DEEPGRAM_API_KEY
# Debe mostrar tu API key
```

### Error: "WebSocket connection refused"

```bash
# 1. Verificar server corriendo
curl http://localhost:3000/health

# 2. Verificar ruta accesible
# GET /stream/media debe responder con HTTP 101 (upgrade)
```

### Error: "Deepgram authentication failed"

```bash
# API Key en formato incorrecto
# Debe ser: largo alfanumérico sin espacios
# Verificar en Deepgram Console que key está activa
```

### Latencia aún alta (>5s)

```bash
# 1. Verificar logs: ¿está procesando transcript?
# 2. ¿Deepgram recibiendo audio?
# 3. ¿Claude respondiendo rápido?

# Debug: SET RUST_LOG=debug
$env:RUST_LOG="debug"
& "$env:USERPROFILE\.cargo\bin\cargo.exe" run
```

---

## Rollback

Si algo no funciona después de deploy:

```bash
# Rápido: Cambiar variable
USE_MEDIA_STREAMS=false
# Reiniciar servidor

# Las llamadas vuelven a webhooks automáticamente
```

No es necesario revertir código, solo env var.

---

## Próximo: Deployment

Una vez validado localmente:

1. Actualizar `.env` en servidor de producción
2. Redeployar con nuevo código
3. Monitorear logs en vivo
4. Medir latencia real en producción
5. Ajustar parámetros Deepgram si es necesario
