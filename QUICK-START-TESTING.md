# Guía Rápida: Testing WebSocket Migration

## 1️⃣ Setup Inicial (Una sola vez)

### Instalar Build Tools (si no lo hiciste)

```powershell
# Opción 1: Descargar instalador gráfico
# https://aka.ms/vs/17/release/vs_BuildTools.exe
# Marca: "Desktop development with C++"

# Opción 2: Con winget (automático)
winget install --id Microsoft.VisualStudio.2022.BuildTools -e
# Selecciona "Desktop development with C++" en el instalador
```

### Verificar Rust

```powershell
# Debe mostrar versión
rustc --version
cargo --version

# Si no funciona, abre nueva terminal (reinicia PATH)
```

### Variables de Entorno

```powershell
# Agregar a tu .env:
DEEPGRAM_API_KEY=tu_key_de_deepgram
USE_MEDIA_STREAMS=true
WEBHOOK_BASE_URL=https://tu-dominio.com
```

---

## 2️⃣ Compilación Rápida

```powershell
# Verificar sintaxis (rápido, sin build)
cargo check

# Compilar (toma ~2-3 min la primera vez)
cargo build

# Compilar optimizado para producción
cargo build --release

# Si hay errores, ver logs detallados
cargo build 2>&1 | Select-String "error"
```

---

## 3️⃣ Ejecutar Servidor

```powershell
# Terminal 1: Ejecutar servidor
cargo run

# Debe mostrar:
# ✅ "Server listening on 0.0.0.0:3000"
# ✅ "WebSocket endpoint ready at /stream/media"

# En otra terminal (Terminal 2):
# Hacer pruebas (ver sección 4)

# Para detener: Ctrl+C en Terminal 1
```

---

## 4️⃣ Pruebas Rápidas

### Verificar que servidor está vivo

```powershell
# Terminal 2
curl http://localhost:3000/health
# Respuesta esperada: OK o JSON
```

### Verificar WebSocket endpoint (sin audio real)

```powershell
# Intenta conexión WebSocket
curl -i -N `
  -H "Connection: Upgrade" `
  -H "Upgrade: websocket" `
  -H "Sec-WebSocket-Key: x3JJHMbDL1EzLkh9GBhXDw==" `
  -H "Sec-WebSocket-Version: 13" `
  http://localhost:3000/stream/media

# Esperado: HTTP 101 Switching Protocols
```

### Hacer llamada real (WebSocket Mode)

```powershell
# Asegúrate que USE_MEDIA_STREAMS=true en .env

$body = @{
    nombre = "Test User"
    telefono = "+5730001234567"
} | ConvertTo-Json

$response = curl -X POST http://localhost:3000/api/call/initiate `
  -H "Content-Type: application/json" `
  -d $body

$response
```

**Monitorea los logs en Terminal 1:**
```
✅ "🔌 Conectando a Deepgram WebSocket"
✅ "🎤 Recibiendo audio"
✅ "⚡ Procesando transcript INTERMEDIO"
```

### Test: Fallback a Webhooks

```powershell
# Cambiar env
$env:USE_MEDIA_STREAMS = "false"

# O en .env: USE_MEDIA_STREAMS=false
# Reinicia servidor (Ctrl+C, cargo run)

# Hacer misma llamada
# Logs mostrarán: "📨 Webhook recibido"
```

---

## 5️⃣ Medir Latencia

### Con Timestamps en Logs

En Terminal 1, busca estos logs:

```
[12:30:45.100Z] 📞 [CALL:xxx] Llamada iniciada
[12:30:45.500Z] 🔌 [CALL:xxx] Conectado a Deepgram
[12:30:46.200Z] 📝 [CALL:xxx] Transcript: "hola mundo"
[12:30:47.500Z] 💬 [CALL:xxx] Claude respondió
[12:30:48.100Z] 🔊 [CALL:xxx] Audio reproducido
```

**Latencia total = Últimos timestamp - primero ≈ 3 segundos**

### Debug: Activar logs detallados

```powershell
# En PowerShell
$env:RUST_LOG = "debug"
cargo run

# Verás MUCHO más output (útil para troubleshoot)
```

---

## 6️⃣ Troubleshooting

### Cargo no encuentra link.exe

```powershell
# Solución: Abrir "x64 Native Tools Command Prompt for VS 2022"
# En lugar de PowerShell normal

# O instalar Build Tools desde:
# https://visualstudio.microsoft.com/downloads/ 
# → "Build Tools for Visual Studio 2022"
```

### DEEPGRAM_API_KEY no cargada

```powershell
# Verificar:
echo $env:DEEPGRAM_API_KEY
# Debe mostrar tu key

# Si está vacío:
# 1. Agregar a .env
# 2. Cerrar y abrir nueva terminal
# 3. Reiniciar cargo run
```

### WebSocket connection refused

```powershell
# 1. Verificar servidor corriendo
curl http://localhost:3000/health

# 2. Verificar puerto no bloqueado
netstat -ano | Select-String 3000

# 3. Si puerto en uso, matar proceso
# Get-Process | where {$_.Id -eq PID} | Stop-Process -Force
```

### Latencia muy alta (>5s)

```powershell
# Activar debug logs
$env:RUST_LOG = "debug,telnyx_ai_service=trace"
cargo run

# Buscar timestamps exactos para cada paso
# Identifica qué parte es lenta:
# - Deepgram? (conexión WS)
# - Claude? (procesamiento IA)
# - TTS? (síntesis voz)
# - Playback? (reproducción audio)
```

### Memory leak o conexiones no se cierran

```powershell
# Ejecutar por 5 min, luego verificar
cargo run --release

# En otra terminal
while ($true) { 
    Get-Process telnyx_ai_service | Select-Object WorkingSet
    Start-Sleep -Seconds 5
}

# WorkingSet debe mantenerse estable (~50-150 MB)
# Si crece constantemente → memory leak
```

---

## 7️⃣ Deploy a Producción

Una vez validado localmente:

```powershell
# 1. Compilar optimizado
cargo build --release

# 2. Binario está en:
# target\release\telnyx_ai_service.exe

# 3. Copiar a servidor
# 4. Actualizar .env con claves reales
# 5. Ejecutar binario

# Alternativa: Docker
docker build -t telnyx-ai .
docker run -e DEEPGRAM_API_KEY=xxx ... telnyx-ai
```

---

## 8️⃣ Comandos Útiles Rápidos

```powershell
# Ver version del proyecto
cargo --version

# Limpiar build cache (si hay problemas)
cargo clean

# Verificar sin compilar
cargo check

# Ejecutar con output específico
cargo run 2>&1 | Select-String "error|warning|Stream"

# Ver dependencias
cargo tree

# Actualizar dependencias
cargo update

# Generar documentación
cargo doc --open
```

---

## ✅ Checklist Final

Antes de considerar "listo":

- [ ] `cargo check` sin errores
- [ ] `cargo build --release` exitoso
- [ ] Servidor inicia en puerto 3000
- [ ] GET /health responde
- [ ] WebSocket upgrade responde HTTP 101
- [ ] Primera llamada WebSocket < 2.5s
- [ ] Transcript intermedio procesado
- [ ] Fallback a webhook funciona
- [ ] Logs claros con timestamps
- [ ] Sin memory leaks después de 10+ min
- [ ] Múltiples llamadas concurrentes funcionan

---

## 📞 Próximos Pasos

1. **Ahora**: Instalar Build Tools (si falta) → `cargo build`
2. **Testing**: Ejecutar `cargo run` y hacer pruebas
3. **Medición**: Capturar latencias reales
4. **Deployment**: Mover a producción con nuevo `.env`
5. **Monitoreo**: Verificar métricas en vivo
