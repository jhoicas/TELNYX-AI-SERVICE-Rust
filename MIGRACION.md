# Guía de Migración de Node.js a Rust

Este documento explica las diferencias y cómo la versión Rust replicar la funcionalidad del servicio Node.js.

## 📊 Comparación de Arquitectura

### Node.js (Original)
```
Express.js (Web Framework)
    ↓
Telnyx API (HTTP Client: axios)
    ↓
Claude API (SDK Anthropic)
    ↓
AWS S3 (AWS SDK)
    ↓
Winston (Logging)
```

### Rust (Nuevo)
```
Axum (Web Framework) + Tokio (Runtime)
    ↓
Telnyx API (HTTP Client: reqwest)
    ↓
Claude API (HTTP directo con reqwest)
    ↓
AWS S3 (aws-sdk-s3)
    ↓
Tracing + Tracing-subscriber (Logging)
```

## 🔄 Mapeo de Componentes

| Node.js | Rust | Archivo |
|---------|------|---------|
| CallController.js | handlers/call.rs | Endpoints de llamadas |
| routes/webhook.js | handlers/webhook.rs | Procesamiento de webhooks |
| TelnyxService.js | services/telnyx.rs | Integración Telnyx |
| GeminiService.js | services/claude.rs | Integración Claude |
| S3Service.js | services/s3.rs | Almacenamiento S3 |
| SessionManager.js | services/session.rs | Gestión de sesiones |
| AudioCacheService.js | - | Implementar si se necesita cache |
| logger.js | utils/logger.rs | Configuración logging |

## 🔀 Cambios en la API

### Estructura de respuestas

**Node.js:**
```javascript
res.json({
  service: 'Telnyx AI Service',
  version: '1.0.0',
  status: 'running'
})
```

**Rust:**
```rust
Json(serde_json::json!({
  "service": "Telnyx AI Service (Rust)",
  "version": "1.0.0",
  "status": "running"
}))
```

### Manejo de errores

**Node.js:**
```javascript
try {
  await telnyx.initiate(...)
} catch (error) {
  logger.error(error)
}
```

**Rust:**
```rust
match telnyx_service.initiate_call(...).await {
  Ok(response) => { /* handle */ },
  Err(e) => { /* handle */ }
}
```

## 🚀 Ventajas de la versión Rust

### 1. Rendimiento
- **Compilación nativa**: Código compilado a machine code, no interpretado
- **Concurrencia**: Tokio proporciona concurrencia ultra-eficiente sin GC
- **Memoria**: Gestión automática sin garbage collector
- **Latencia**: Predictible y baja

### 2. Confiabilidad
- **Seguridad de memoria**: Garantizado por el compilador (sin memory leaks)
- **Type safety**: Sistema de tipos estricto en tiempo de compilación
- **No panics**: En código de producción bien escrito

### 3. Deploys
- **Single binary**: Un archivo ejecutable, sin dependencias de Node
- **Startup rápido**: ~100ms vs 2-3s en Node
- **Memoria baja**: 10-20MB vs 150-200MB en Node
- **CPU eficiente**: Menos overhead

## 📝 Cambios en el flujo

### Iniciación de llamada

**Node.js:**
```javascript
POST /api/call/initiate
body: {
  telefono: "+521234567890",
  nombre: "Juan",
  contexto: "consulta"
}
```

**Rust:** (idéntico)
```bash
POST /api/call/initiate
{
  "telefono": "+521234567890",
  "nombre": "Juan",
  "contexto": "consulta"
}
```

### Manejo de webhooks

**Node.js:**
```javascript
app.post('/webhook/telnyx', async (req, res) => {
  const eventType = req.body.meta.event_type;
  
  if (eventType === 'call.answered') {
    await handleCallAnswered(req.body);
  }
})
```

**Rust:**
```rust
async fn handle_telnyx_webhook(
    State(state): State<Arc<AppState>>,
    Json(payload): Json<serde_json::Value>,
) -> (StatusCode, Json<serde_json::Value>) {
    let event_type = payload["meta"]["event_type"].as_str().unwrap_or("unknown");
    
    match event_type {
        "call.answered" => handle_call_answered(state, payload).await,
        ...
    }
}
```

## 🔌 Migrando servicios personalizados

Si tienes servicios adicionales, aquí está la estructura:

### Agregar nuevo servicio

1. Crear archivo en `src/services/mi_servicio.rs`
2. Implementar struct con métodos
3. Agregar a `src/services/mod.rs`
4. Agregar a `AppState` en `src/services/app_state.rs`

Ejemplo:

```rust
// src/services/custom_service.rs
pub struct CustomService {
    // campos
}

impl CustomService {
    pub fn new() -> Self {
        Self { }
    }

    pub async fn do_something(&self) -> anyhow::Result<String> {
        Ok("Done".to_string())
    }
}
```

```rust
// src/services/mod.rs
pub mod custom_service;

pub use custom_service::CustomService;
```

## 🛠️ Herramientas y dependencias

### Instalación de Rust

```bash
# Windows (PowerShell)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# O descargar desde https://rustup.rs/
```

### Cargo commands útiles

```bash
cargo new project                # Crear nuevo proyecto
cargo add crate_name            # Agregar dependencia
cargo build --release           # Build optimizado
cargo run                        # Ejecutar en desarrollo
cargo test                       # Correr tests
cargo fmt                        # Formatear código
cargo clippy                     # Linting
cargo doc --open                # Documentación
```

## 📦 Dependencias principales

```toml
[dependencies]
# Web
axum = "0.7"                    # Framework web (Express-like)
tokio = { version = "1", features = ["full"] }  # Runtime async
tower = "0.4"                   # Middleware
tower-http = "0.5"              # Middleware HTTP

# HTTP Client
reqwest = "0.12"                # HTTP client (axios-like)

# Serialization
serde = "1.0"                   # Serialization (JSON, etc)
serde_json = "1.0"              # JSON

# Environment
dotenv = "0.15"                 # .env files

# Logging
tracing = "0.1"                 # Structured logging
tracing-subscriber = "0.3"      # Logging backend

# Collections
dashmap = "5.5"                 # Concurrent HashMap

# AWS
aws-sdk-s3 = "1.0"              # AWS S3 client
```

## 🔍 Debugging

### Habilitar logs detallados

```bash
# Development
RUST_LOG=debug cargo run

# Production (requiere recompilación)
RUST_LOG=telnyx_ai_service=debug
```

### Usar rust-gdb

```bash
cargo build --debug
rust-gdb ./target/debug/telnyx_ai_service
```

## 📚 Recursos de aprendizaje

- [The Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://github.com/tokio-rs/axum)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)

## ✅ Checklist de migración

- [ ] Instalar Rust y cargo
- [ ] Configurar variables de entorno (.env)
- [ ] Probar endpoints localmente
- [ ] Configurar webhook en Telnyx dashboard
- [ ] Realizar test de llamada completa
- [ ] Monitorear logs en producción
- [ ] Comparar métricas de rendimiento
- [ ] Deprecar versión Node.js

## 🆘 Troubleshooting

### Error: "TELNYX_API_KEY not found"
```bash
# Asegúrate que .env existe y está en la raíz del proyecto
cp .env.example .env
# Edita .env con tus credenciales
```

### Error: "Failed to build S3 service"
```bash
# Si no necesitas S3, es opcional. El servicio continuará funcionando
# Pero configura las variables si quieres usarlo:
export AWS_REGION=us-east-1
export S3_BUCKET=tu-bucket
```

### Latencia alta en transcripciones
```bash
# Aumenta timeout en models.rs si es necesario
# Default: SILENCE_TIMEOUT_MS = 20000 (20 segundos)
```

---

**Versión**: 1.0.0  
**Última actualización**: Diciembre 2025
