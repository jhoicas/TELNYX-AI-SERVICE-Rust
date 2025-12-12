# ✅ WebSocket Migration - Checklist de Validación

## Fase 1: Setup (Antes de Compilar)

- [ ] Verificar que tienes Windows con PowerShell 5.1+
- [ ] Instalar Visual Studio 2022 Build Tools con C++
  - [ ] Descargar desde https://aka.ms/vs/17/release/vs_BuildTools.exe
  - [ ] Ejecutar instalador
  - [ ] Seleccionar "Desktop development with C++"
  - [ ] Esperar instalación (~15 min)
  - [ ] Abrir nueva terminal PowerShell
- [ ] Verificar Rust instalado
  ```powershell
  rustc --version  # Debe mostrar versión
  cargo --version  # Debe mostrar versión
  ```
- [ ] Clonar/actualizar repo
  ```powershell
  git clone https://github.com/jhoicas/TELNYX-AI-SERVICE-Rust.git
  cd TELNYX-AI-SERVICE-Rust
  ```

## Fase 2: Configuración

- [ ] Crear/actualizar `.env` con variables requeridas:
  ```bash
  DEEPGRAM_API_KEY=xxxxxxxxxxxxxxxxxxxxx
  TELNYX_API_KEY=xxxxxxxxxxxxxxxxxxxxx
  CLAUDE_API_KEY=xxxxxxxxxxxxxxxxxxxxx
  ELEVENLABS_API_KEY=xxxxxxxxxxxxxxxxxxxxx
  AWS_ACCESS_KEY_ID=xxxxxxxxxxxxxxxxxxxxx
  AWS_SECRET_ACCESS_KEY=xxxxxxxxxxxxxxxxxxxxx
  AWS_REGION=us-east-1
  AWS_BUCKET_NAME=xxxxxxxxxxxxxxxxxxxxx
  S3_BUCKET_FOLDER=audios
  TELNYX_CONNECTION_ID=xxxxxxxxxxxxxxxxxxxxx
  TELNYX_PHONE_NUMBER=+xxxxxxxxxxxx
  WEBHOOK_BASE_URL=https://tu-dominio.com
  USE_MEDIA_STREAMS=true
  TELNYX_TRANSCRIPTION_ENGINE=deepgram
  TELNYX_TRANSCRIPTION_LANG=es
  ```
- [ ] Verificar DEEPGRAM_API_KEY es válida
  ```powershell
  echo $env:DEEPGRAM_API_KEY
  # Debe mostrar tu key (ej: xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx)
  ```

## Fase 3: Compilación

- [ ] Ejecutar cargo check
  ```powershell
  cargo check
  ```
  - [ ] Sin errores críticos
  - [ ] Warnings aceptables

- [ ] Ejecutar cargo build
  ```powershell
  cargo build
  ```
  - [ ] Completado sin errores
  - [ ] Binario generado en target/debug/

- [ ] Ejecutar cargo build --release (opcional pero recomendado)
  ```powershell
  cargo build --release
  ```
  - [ ] Completado sin errores (~3-5 min)
  - [ ] Binario optimizado en target/release/

## Fase 4: Testing Local - WebSocket Mode

### 4.1 Iniciar Servidor

- [ ] Iniciar servidor en Terminal 1
  ```powershell
  cargo run
  ```
  - [ ] Logs mostrar "Server listening on 0.0.0.0:3000"
  - [ ] Sin errores al startup
  - [ ] Logs claros y legibles

### 4.2 Verificar Conectividad

- [ ] En Terminal 2, verificar servidor vivo
  ```powershell
  curl http://localhost:3000/health
  ```
  - [ ] Responde sin error

- [ ] Verificar WebSocket endpoint accesible
  ```powershell
  curl -i -N `
    -H "Connection: Upgrade" `
    -H "Upgrade: websocket" `
    -H "Sec-WebSocket-Key: x3JJHMbDL1EzLkh9GBhXDw==" `
    -H "Sec-WebSocket-Version: 13" `
    http://localhost:3000/stream/media
  ```
  - [ ] Responde con HTTP 101 (Switching Protocols)

### 4.3 Hacer Llamada Test

- [ ] Configurar USE_MEDIA_STREAMS=true en .env
- [ ] Hacer llamada de prueba
  ```powershell
  $body = @{nombre="TestUser"; telefono="+5730001234567"} | ConvertTo-Json
  curl -X POST http://localhost:3000/api/call/initiate `
    -H "Content-Type: application/json" -d $body
  ```
- [ ] Observar logs en Terminal 1:
  - [ ] Log "🔌 Conectando a Deepgram WebSocket"
  - [ ] Log "✅ Conectado a Deepgram WebSocket"
  - [ ] Log "🎤 Recibiendo audio de Telnyx"
  - [ ] Log "⚡ Procesando transcript INTERMEDIO" (si hay audio simulado)
  
### 4.4 Medir Latencia

- [ ] Capturar timestamps de logs
  - [ ] Primer log: [HH:MM:SS.000]
  - [ ] Último log: [HH:MM:SS.000]
  - [ ] Calcular diferencia (debe ser 1-3 segundos para WebSocket)

- [ ] Registrar resultado:
  ```
  Test 1 WebSocket: __ segundos
  Test 2 WebSocket: __ segundos
  Test 3 WebSocket: __ segundos
  Promedio WebSocket: __ segundos
  ```

## Fase 5: Testing Fallback - Webhook Mode

- [ ] Cambiar configuración
  ```powershell
  # En .env: USE_MEDIA_STREAMS=false
  # Reiniciar servidor (Ctrl+C, cargo run)
  ```

- [ ] Hacer misma llamada de prueba
  - [ ] Observar logs:
    - [ ] Log "📨 Webhook recibido: call.transcription"
    - [ ] Log "📝 Transcripción recibida"
  - [ ] Latencia debe ser 6-12 segundos (más lenta que WebSocket)
  
- [ ] Registrar resultado:
  ```
  Test 1 Webhook: __ segundos
  Test 2 Webhook: __ segundos
  Test 3 Webhook: __ segundos
  Promedio Webhook: __ segundos
  ```

## Fase 6: Tests Avanzados

### 6.1 Debug Logging

- [ ] Activar verbose logging
  ```powershell
  $env:RUST_LOG = "debug"
  cargo run
  ```
  - [ ] Logs detallados aparecen
  - [ ] Puedes rastrear cada paso

### 6.2 Múltiples Llamadas Concurrentes

- [ ] En Terminal 2, hacer 3 llamadas casi simultáneamente
  ```powershell
  for ($i=1; $i -le 3; $i++) {
    $body = @{nombre="Test$i"; telefono="+573000123456$i"} | ConvertTo-Json
    curl -X POST http://localhost:3000/api/call/initiate `
      -H "Content-Type: application/json" -d $body &
  }
  ```
  - [ ] Todos se procesan sin error
  - [ ] Logs no se mezclan
  - [ ] Cada llamada tiene su call_control_id

### 6.3 Estabilidad Memory

- [ ] Dejar servidor corriendo 10 minutos
  ```powershell
  # Terminal 1: cargo run (ya corriendo)
  
  # Terminal 2: Monitorear memory cada 5 segundos
  while ($true) {
    Get-Process | Where-Object {$_.ProcessName -eq "cargo"} | `
      Select-Object ProcessName, WorkingSet -First 1
    Start-Sleep -Seconds 5
  }
  ```
  - [ ] WorkingSet se mantiene estable
  - [ ] Sin crecimiento de memoria continuo
  - [ ] Rango esperado: 100-200 MB

### 6.4 Manejo de Errores

- [ ] Desconectar red WiFi mientras está conectado
  - [ ] Logs registran desconexión
  - [ ] Sistema intenta reconectar

- [ ] Cambiar DEEPGRAM_API_KEY a valor inválido
  - [ ] Logs muestran error de auth
  - [ ] Sin crash del servidor

- [ ] Cambiar WEBHOOK_BASE_URL a puerto no accesible
  - [ ] Error capturado gracefully
  - [ ] Sin bloqueo de llamadas

## Fase 7: Documentación & Validación

- [ ] Revisar archivos de documentación creados:
  - [ ] ✅ WEBSOCKET-MIGRATION.md
  - [ ] ✅ TESTING-WEBSOCKET.md
  - [ ] ✅ QUICK-START-TESTING.md
  - [ ] ✅ MIGRATION-SUMMARY.md
  - [ ] ✅ build-and-test.ps1

- [ ] Verificar estructura de código:
  - [ ] ✅ src/services/deepgram_ws.rs existe
  - [ ] ✅ src/handlers/media_stream.rs existe
  - [ ] ✅ src/main.rs tiene ruta /stream/media
  - [ ] ✅ src/handlers/mod.rs exporta media_stream
  - [ ] ✅ src/services/mod.rs exporta deepgram_ws

- [ ] Verificar git commits:
  ```powershell
  git log --oneline | head -5
  ```
  - [ ] ✅ Muestra commits de WebSocket migration
  - [ ] ✅ Todos los cambios están commiteados
  - [ ] ✅ Remoto está actualizado (git push)

## Fase 8: Pre-Deployment

- [ ] Crear backup de versión anterior (si aplica)
- [ ] Documentar cambios para team
- [ ] Crear plan de rollback:
  ```bash
  USE_MEDIA_STREAMS=false  # Volver a webhooks
  ```
- [ ] Verificar que webhooks aún funcionan (test 5.4)
- [ ] Preparar script de deployment
  ```powershell
  cargo build --release
  # Copiar target/release/telnyx_ai_service.exe a servidor
  ```

## Fase 9: Deployment a Staging (Cuando sea Momento)

- [ ] Copiar binario compilado a servidor staging
- [ ] Actualizar .env con valores reales
- [ ] Iniciar servicio/aplicación
- [ ] Monitorear logs por 5 minutos
- [ ] Hacer llamadas de prueba reales (teléfono real)
- [ ] Medir latencia en condiciones reales
- [ ] Registrar resultados:
  ```
  Staging WebSocket Latency: __ segundos
  Transcription Accuracy: __ %
  Success Rate: __ %
  ```

## Fase 10: Deployment a Producción

- [ ] Comunicar cambio a stakeholders
- [ ] Programar ventana de cambio
- [ ] Ejecutar deployment
- [ ] Monitorear métricas por 24 horas
- [ ] Validar SLAs se cumplen
- [ ] Documentar lecciones aprendidas

## Notas & Observaciones

```
Observación 1: ___________________
Observación 2: ___________________
Observación 3: ___________________

Problemas Encontrados:
- ___________________
- ___________________

Soluciones Aplicadas:
- ___________________
- ___________________

Performance Real:
- Latencia WebSocket: __ segundos
- Latencia Webhook: __ segundos
- Mejora: __ % (objetivo >70%)
```

## Firma & Fecha

- [ ] Todas las fases completadas exitosamente
- [ ] Testing validado
- [ ] Documentación actualizada
- [ ] Código commiteado y pushed

**Fecha de Inicio**: _______________
**Fecha de Completación**: _______________
**Validado por**: _______________
**Notas Finales**: _______________

---

## 📋 Resumen Rápido de Comandos

```powershell
# Compilar
cargo check
cargo build --release

# Ejecutar
cargo run

# Tests
cargo test

# Logs detallados
$env:RUST_LOG = "debug"

# Verificar puerto
netstat -ano | Select-String 3000

# Ver procesos Rust
Get-Process | Where-Object {$_.Name -like "*cargo*" -or $_.Name -like "*telnyx*"}

# Matar proceso (si falta)
# Stop-Process -Name "cargo" -Force
```

---

## 🎯 Objetivo Final

✅ Sistema respondiendo en **< 2.5 segundos** (vs 6-12s antes)
✅ Fallback a webhooks funcional
✅ Monitoreo activo implementado
✅ Team capacitado en nueva arquitectura
✅ Documentación completa para onboarding futuro

---

**¡Buena suerte! Estás a pocos pasos de lograr "casi en tiempo real" 🚀**
