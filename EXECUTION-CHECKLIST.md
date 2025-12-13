# ✅ CHECKLIST DE EJECUCIÓN - WebSocket Migration

## 📋 Completar en Este Orden

### FASE 1: Setup Build Tools (15-25 min)

- [ ] **PASO 1A**: Abre PowerShell **como administrador**
  - Click derecho en PowerShell → "Run as Administrator"

- [ ] **PASO 1B**: Ejecuta este comando:
  ```powershell
  winget install --id Microsoft.VisualStudio.2022.BuildTools -e
  ```
  - Espera a que se inicie el instalador
  - Cuando se abre la ventana del instalador:
    - [ ] Marca: **"Desktop development with C++"**
    - [ ] Click: **"Install"**
  - Espera a que termine (10-15 min)

- [ ] **PASO 1C**: Cierra **TODAS** las terminales PowerShell

- [ ] **PASO 1D**: Abre **NUEVA** terminal PowerShell (regular, no admin)

- [ ] **PASO 1E**: Verifica que funcionó:
  ```powershell
  where link.exe
  ```
  Debe mostrar: `C:\Program Files\Microsoft Visual Studio\...`
  
  Si no lo muestra, lee: [INSTALL-BUILD-TOOLS.md](./INSTALL-BUILD-TOOLS.md)

---

### FASE 2: Compilación (5-10 min)

- [ ] **PASO 2A**: En la terminal abierta, navega al repo:
  ```powershell
  cd C:\Users\yoiner.castillo\source\repos\TELNYX-AI-SERVICE-Rust
  ```

- [ ] **PASO 2B**: Verifica compilación:
  ```powershell
  cargo check
  ```
  - Espera a que termine (puede tardar 2-3 min en primera vez)
  - Debe terminar con: `Finished 'check'...` **SIN ERRORES**
  
  Si hay error de `link.exe not found`: 
  - [ ] Cierra terminal
  - [ ] Abre "x64 Native Tools Command Prompt for VS 2022"
  - [ ] Intenta de nuevo

- [ ] **PASO 2C**: Compilación completa:
  ```powershell
  cargo build --release
  ```
  - Espera a que termine (tarda 3-5 min en primera vez)
  - Debe terminar con: `Finished 'release'...` **SIN ERRORES**

- [ ] **PASO 2D**: Verifica que el binario existe:
  ```powershell
  ls target\release\telnyx_ai_service.exe
  ```
  Debe mostrar el archivo (4-5 MB)

---

### FASE 3: Configuration (2 min)

- [ ] **PASO 3A**: Abre archivo `.env` en tu editor

- [ ] **PASO 3B**: Verifica que tiene:
  ```bash
  DEEPGRAM_API_KEY=tu_api_key_aqui
  USE_MEDIA_STREAMS=true
  WEBHOOK_BASE_URL=https://tu-dominio.com
  WS_STREAM_URL=wss://tu-dominio.com/stream/media
  ```
  
  Si falta alguno, agrégalo

---

### FASE 4: Testing Local (30 min)

- [ ] **PASO 4A**: En la misma terminal, ejecuta:
  ```powershell
  cargo run
  ```
  - Espera a que inicie
  - Debe mostrar: `Server listening on 0.0.0.0:3000`
  - **NO cierres esta terminal** (déjala corriendo)

- [ ] **PASO 4B**: Abre **NUEVA terminal** (terminal 2)
  ```powershell
  cd C:\Users\yoiner.castillo\source\repos\TELNYX-AI-SERVICE-Rust
  ```

- [ ] **PASO 4C**: Verifica que servidor responde:
  ```powershell
  curl http://localhost:3000/health
  ```
  - Debe responder (OK o JSON)

- [ ] **PASO 4D**: Verifica WebSocket endpoint:
  ```powershell
  curl -i -N `
    -H "Connection: Upgrade" `
    -H "Upgrade: websocket" `
    -H "Sec-WebSocket-Key: x3JJHMbDL1EzLkh9GBhXDw==" `
    -H "Sec-WebSocket-Version: 13" `
    http://localhost:3000/stream/media
  ```
  - Debe responder con: `HTTP/1.1 101 Switching Protocols`

- [ ] **PASO 4D.1**: Confirma inicio de streaming programático (Telnyx):
  - En logs, después de `call.answered`, debe verse `Streaming Start requested` y `Media stream started`.
  - Recuerda: Telnyx Media Streams no se habilita por UI; se inicia con Call Command `streaming_start`.

- [ ] **PASO 4E**: Haz llamada de prueba:
  ```powershell
  $body = @{nombre="TestUser"; telefono="+5730001234567"} | ConvertTo-Json
  curl -X POST http://localhost:3000/api/call/initiate `
    -H "Content-Type: application/json" `
    -d $body
  ```
  - Debe responder sin error

- [ ] **PASO 4F**: En terminal 1, verifica logs:
  - Busca mensajes con: 🔌 🎤 ⚡ 💬 🔊
  - Si ves estos emojis, todo funciona

- [ ] **PASO 4G**: Mide latencia (busca en los logs):
  - Primer timestamp: [HH:MM:SS.000]
  - Último timestamp: [HH:MM:SS.000]
  - Diferencia: debe ser < 3 segundos
  
  Si es < 2.5s: ✅ **EXCELENTE**

---

### FASE 5: Validación (30 min)

- [ ] **PASO 5A**: Lee [TESTING-WEBSOCKET.md](./TESTING-WEBSOCKET.md)

- [ ] **PASO 5B**: Ejecuta testing manual (Phase 4 en ese doc)

- [ ] **PASO 5C**: Prueba fallback a webhooks:
  - [ ] Cierra terminal 1 (Ctrl+C)
  - [ ] En `.env`, cambia: `USE_MEDIA_STREAMS=false`
  - [ ] En terminal 1: `cargo run` de nuevo
  - [ ] Haz misma llamada en terminal 2
  - [ ] Logs deben mostrar: 📨 Webhook recibido

- [ ] **PASO 5D**: Vuelve a WebSocket:
  - [ ] `USE_MEDIA_STREAMS=true` en `.env`
  - [ ] `WS_STREAM_URL` apunta al WS público (`wss://<host>/stream/media`)
  - [ ] Reinicia servidor
  - [ ] Verifica logs muestren: 🔌 WebSocket operations

---

### TELNYX: Media Streams (Importante)

Telnyx confirmó por soporte que Media Streams para Voice API NO se activa en el Portal UI. Debe iniciarse programáticamente usando Call Commands `Streaming Start` durante una llamada activa.

Referencia: https://developers.telnyx.com/api-reference/call-commands/streaming-start

Checklist rápido:
- Webhook HTTP: `https://<host>/webhook/telnyx` (para eventos).
- WebSocket Stream: `wss://<host>/stream/media` (audio en tiempo real).
- En `call.answered`, el backend envía `streaming_start` si `USE_MEDIA_STREAMS=true`.
- Parámetros clave: `stream_url`, `audio.track` (inbound), `audio.codec` (mulaw), `sample_rate` (8000).

Ejemplo de payload (conceptual):
```
POST /calls/{call_control_id}/actions/streaming_start
{
  "stream_url": "wss://<host>/stream/media",
  "audio": {
    "track": "inbound",
    "codec": "mulaw",
    "sample_rate": 8000
  }
}
```

Notas:
- No confundas `Webhook URL` (HTTP) con `Stream URL` (WS); son distintos.
- El audio fluye al WS solo tras `streaming_start` exitoso.

---

### FASE 6: Documentación (10 min)

- [ ] **PASO 6A**: Lee: [FINAL-REPORT.md](./FINAL-REPORT.md)

- [ ] **PASO 6B**: Lee: [ARCHITECTURE-DIAGRAMS.md](./ARCHITECTURE-DIAGRAMS.md)

- [ ] **PASO 6C**: Guarda para referencia: [DOCUMENTATION-INDEX.md](./DOCUMENTATION-INDEX.md)

---

### FASE 7: Preparar para Producción (20 min)

- [ ] **PASO 7A**: Crea build optimizado:
  ```powershell
  cargo build --release
  ```

- [ ] **PASO 7B**: Binario está en:
  ```
  C:\Users\yoiner.castillo\source\repos\TELNYX-AI-SERVICE-Rust\target\release\telnyx_ai_service.exe
  ```

- [ ] **PASO 7C**: Copia este archivo a tu servidor

- [ ] **PASO 7D**: En servidor, crea `.env` con:
  ```bash
  DEEPGRAM_API_KEY=xxx
  TELNYX_API_KEY=xxx
  CLAUDE_API_KEY=xxx
  USE_MEDIA_STREAMS=true
  WEBHOOK_BASE_URL=https://tu-dominio.com
  (... otras variables ...)
  ```

- [ ] **PASO 7E**: Ejecuta binario en servidor:
  ```bash
  ./telnyx_ai_service  # en Linux/Mac
  # o
  telnyx_ai_service.exe  # en Windows
  ```

- [ ] **PASO 7F**: Monitorea logs en vivo
  - Busca: 🔌 Conectado a Deepgram WebSocket
  - Busca: Server listening on 0.0.0.0:3000

---

## 📊 Resumen de Progreso

| Fase | Descripción | Tiempo | Status |
|------|-------------|--------|--------|
| 1 | Instalar Build Tools | 15-25 min | ⏳ TODO |
| 2 | Compilación | 5-10 min | ⏳ TODO |
| 3 | Configuración | 2 min | ⏳ TODO |
| 4 | Testing Local | 30 min | ⏳ TODO |
| 5 | Validación | 30 min | ⏳ TODO |
| 6 | Documentación | 10 min | ⏳ TODO |
| 7 | Production Ready | 20 min | ⏳ TODO |
| **TOTAL** | | **~2 horas** | ⏳ TODO |

---

## 🎯 Criterios de Éxito

### Compilación ✅
- [x] cargo check pasa sin errores
- [x] cargo build --release termina exitosamente
- [x] Binario creado en target/release/

### Testing ✅
- [x] Servidor inicia en puerto 3000
- [x] GET /health responde
- [x] WebSocket upgrade responde HTTP 101
- [x] Logs muestran operaciones WebSocket
- [x] Latencia < 2.5 segundos

### Validación ✅
- [x] Fallback a webhooks funciona
- [x] Sin memory leaks (memoria estable)
- [x] Sin errores en logs
- [x] Múltiples llamadas funciona

### Documentación ✅
- [x] Leíste FINAL-REPORT.md
- [x] Entiendes ARCHITECTURE-DIAGRAMS.md
- [x] Tienes DOCUMENTATION-INDEX.md guardado

---

## 💡 Troubleshooting Rápido

| Problema | Solución |
|----------|----------|
| link.exe no encontrado | Instalar VS Build Tools con C++ |
| cargo check falla | Abrir nueva terminal y reintentar |
| Servidor no inicia | Verificar puerto 3000 no está en uso |
| WebSocket no responde | Verificar DEEPGRAM_API_KEY válida |
| Latencia alta (>5s) | Verificar logs con debug: `$env:RUST_LOG="debug"` |

---

## 🚀 Cuando Termines

Una vez completados TODOS los checkboxes:

✅ Sistema WebSocket funcionando localmente
✅ Latencia < 2.5 segundos
✅ Fallback a webhooks probado
✅ Documentación leída
✅ Binario listo para producción

**ESTÁS LISTO PARA DEPLOYMENT** 🎉

---

## 📝 Notas de Ejecución

```
Fecha de Inicio: _______________
Fecha de Finalización: _______________

Problemas Encontrados:
- ___________________
- ___________________

Soluciones Aplicadas:
- ___________________
- ___________________

Observaciones:
- ___________________
- ___________________

Latencia Promedio: _____ segundos
Success Rate: _____%
```

---

**EMPIEZA AHORA CON FASE 1: PASO 1A** ✨

Instrucciones claras y paso-a-paso. Completa cada checkbox cuando termines.

¡Adelante! 🚀
