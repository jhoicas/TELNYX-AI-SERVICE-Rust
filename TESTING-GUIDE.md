# Guía de Testing y Logs

## 1. Logs por Llamada (Call Tracking)

Todos los logs ahora incluyen el prefijo `[CALL:id]` para identificar fácilmente qué logs pertenecen a cada llamada.

### Ejemplo de logs de una llamada completa:

```
📨 [CALL:abc123] Webhook recibido: call.answered
📞 [CALL:abc123] Llamada respondida
🔊 [CALL:abc123] Obteniendo saludo para: morning
♻️ Reutilizando saludo existente: morning -> https://...
🎤 [CALL:abc123] Transcripción iniciada
▶️ [CALL:abc123] Playback iniciado
⏸️ [CALL:abc123] Playback finalizado
📝 [CALL:abc123] Transcripción recibida: 'Hola, necesito una cita'
🤖 [CLAUDE] Enviando request a modelo: claude-3-5-haiku-20241022
🤖 [CLAUDE] Prompt para Carlos: 'Cliente (Carlos): Hola, necesito una cita'
✅ [CLAUDE] Respuesta generada para Carlos. Modelo: claude-3-5-haiku-20241022, Tokens in/out: 45/32
💬 [CLAUDE] Respuesta final: 'Claro Carlos, con mucho gusto. Para qué día necesitás la cita?'
☎️ [CALL:abc123] Llamada finalizada
```

### Filtrar logs de una llamada específica:

En logs JSON:
```bash
# Linux/Mac
grep 'CALL:abc123' logs.json

# Windows PowerShell
Select-String -Pattern "CALL:abc123" logs.json
```

## 2. Probar Claude API

### Endpoint de prueba: `/api/test/claude`

Este endpoint permite probar Claude sin hacer una llamada telefónica real.

### Método 1: Script automatizado (Recomendado)

```powershell
# Ejecutar tests predefinidos
.\test-claude.ps1

# Con URL personalizada
$env:API_URL = "https://tu-app.ondigitalocean.app"
.\test-claude.ps1
```

### Método 2: cURL/Invoke-RestMethod

**Test simple:**
```powershell
$body = @{
    nombre = "Carlos"
    mensaje = "Hola, a qué hora abren?"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:3000/api/test/claude" `
    -Method POST `
    -Body $body `
    -ContentType "application/json"
```

**Test con contexto:**
```powershell
$body = @{
    nombre = "María"
    mensaje = "Necesito agendar una cita"
    contexto = "La cliente llamó antes preguntando por vacunas"
} | ConvertTo-Json

Invoke-RestMethod -Uri "http://localhost:3000/api/test/claude" `
    -Method POST `
    -Body $body `
    -ContentType "application/json"
```

### Respuesta esperada:

```json
{
  "success": true,
  "model": "claude-3-5-haiku-20241022",
  "input_tokens": null,
  "output_tokens": null,
  "response": "Claro María, con mucho gusto. Para qué día necesitás la cita?",
  "error": null
}
```

## 3. Ver logs de Claude en producción

### Logs detallados de Claude:

Cada request a Claude genera estos logs:

1. **🤖 [CLAUDE] Enviando request** - Muestra modelo, max_tokens, temperatura
2. **🤖 [CLAUDE] Prompt para {nombre}** - Muestra el texto enviado
3. **✅ [CLAUDE] Respuesta generada** - Muestra tokens usados, caracteres antes/después de limpieza
4. **💬 [CLAUDE] Respuesta final** - Muestra el texto exacto que se enviará al usuario

### Ver logs en DigitalOcean:

```bash
# Filtrar solo logs de Claude
doctl apps logs YOUR_APP_ID --follow --type RUN | grep CLAUDE

# Ver últimos 100 logs
doctl apps logs YOUR_APP_ID --tail 100 --type RUN | grep CLAUDE
```

### Ver logs localmente:

```powershell
# Durante desarrollo con logs en consola
cargo run | Select-String "CLAUDE"

# Si usas archivo de logs
Get-Content logs.txt -Tail 100 | Select-String "CLAUDE"
```

## 4. Tests Unitarios

### Ejecutar tests:

```bash
# Todos los tests
cargo test

# Tests específicos
cargo test test_claude
cargo test test_client_state_serialization
cargo test test_environment_loaded

# Con output detallado
cargo test -- --nocapture
```

### Tests disponibles:

1. **test_environment_loaded** - Verifica que las API keys estén configuradas
2. **test_client_state_serialization** - Verifica serialización de ClientState
3. Más tests pueden agregarse en `tests/integration_tests.rs`

## 5. Debugging de Claude

### Verificar modelo en uso:

```bash
# Ver variable de entorno
echo $env:CLAUDE_MODEL  # Windows
echo $CLAUDE_MODEL      # Linux/Mac

# Debería mostrar: claude-3-5-haiku-20241022
```

### Problemas comunes:

**Error: "Failed to generate response"**
- Verificar ANTHROPIC_API_KEY en .env
- Revisar logs para ver error específico de Anthropic API
- Verificar que el modelo existe y está disponible

**Respuestas muy cortas o cortadas:**
- Revisar `max_tokens` en claude.rs (actualmente: 120)
- Ver logs `[CLAUDE]` para confirmar tokens usados
- Ajustar si es necesario

**Respuestas no naturales:**
- Revisar `system_prompt` en claude.rs
- Verificar temperatura (actualmente: 0.6)
- Usar endpoint de prueba para experimentar

## 6. Monitoreo en Producción

### Crear dashboard de logs:

Puedes usar estos patterns para filtrar en tu sistema de logs:

- **Llamadas iniciadas**: `CALL:.*Llamada respondida`
- **Transcripciones**: `CALL:.*Transcripción recibida`
- **Respuestas Claude**: `CLAUDE.*Respuesta final`
- **Errores**: `❌.*CALL:`
- **Llamadas finalizadas**: `CALL:.*Llamada finalizada`

### Métricas útiles:

- Promedio de tokens usados por llamada
- Tiempo de respuesta de Claude
- Rate de errores por llamada
- Distribución de horarios de llamadas (morning/afternoon/evening)

## 7. Tips de Optimización

### Reducir logs en producción:

Cambiar nivel de logs en `main.rs`:
```rust
// De debug a info
.add_directive("telnyx_ai_service=info".parse().unwrap())
```

### Ver solo errores:
```rust
.add_directive("telnyx_ai_service=error".parse().unwrap())
```

### Formato de logs:

- **JSON** (actual): Mejor para parsing/análisis
- **Pretty**: Mejor para desarrollo local

Cambiar en `main.rs`:
```rust
// JSON (producción)
.json()

// Pretty (desarrollo)
.pretty()
```
