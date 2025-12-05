# 🚀 Telnyx AI Service - Versión Rust

Servicio de alta performance para llamadas telefónicas con IA usando Telnyx y Claude.

**Ventajas de la versión Rust:**
- ⚡ Rendimiento ultra-rápido (compilado a código nativo)
- 🔒 Seguridad de memoria garantizada por el compilador
- 📦 Binario único sin dependencias externas
- 🎯 Latencia ultra-baja con Axum + Tokio
- 💾 Bajo consumo de memoria y CPU

## 📋 Requisitos

- Rust 1.70+ ([Instalar Rust](https://rustup.rs/))
- Cuenta de Telnyx con API Key
- API Key de Claude (Anthropic)
- (Opcional) AWS S3 para almacenar audios

## 🛠️ Instalación

### 1. Clonar e instalar

```bash
cd C:\Users\yoiner.castillo\source\repos\TELNYX-AI-SERVICE-Rust
cargo build --release
```

### 2. Configurar variables de entorno

Copia `.env.example` a `.env`:

```bash
cp .env.example .env
```

Edita `.env` con tus credenciales:

```env
# Telnyx Configuration
TELNYX_API_KEY=tu_api_key
TELNYX_CONNECTION_ID=tu_connection_id
TELNYX_PHONE_NUMBER=+1234567890

# Claude Configuration
ANTHROPIC_API_KEY=tu_anthropic_key
CLAUDE_MODEL=claude-3-5-haiku-20241022

# Server Configuration
PORT=3000
WEBHOOK_BASE_URL=https://tu-dominio.com

# AWS S3 (opcional)
AWS_REGION=us-east-1
S3_BUCKET=tu-bucket
```

### 3. Ejecutar

```bash
# Desarrollo (con auto-reload usando cargo-watch)
cargo install cargo-watch
cargo watch -x run

# Producción
cargo build --release
./target/release/telnyx_ai_service
```

## 📊 Endpoints disponibles

### Iniciar una llamada
```bash
POST /api/call/initiate
Content-Type: application/json

{
  "telefono": "+521234567890",
  "nombre": "Juan Pérez",
  "contexto": "Cliente frecuente",
  "saludo": "¡Hola Juan!"
}
```

### Llamadas en lote
```bash
POST /api/call/batch
Content-Type: application/json

{
  "calls": [
    {
      "telefono": "+521234567890",
      "nombre": "Juan Pérez",
      "contexto": "Consulta veterinaria"
    },
    {
      "telefono": "+521987654321",
      "nombre": "María García",
      "contexto": "Seguimiento"
    }
  ]
}
```

### Estadísticas de sesiones
```bash
GET /api/sessions/stats
```

### Health check
```bash
GET /api/health
```

### Webhook de Telnyx
```
POST /webhook/telnyx
```

## 🏗️ Estructura del proyecto

```
src/
├── main.rs                  # Punto de entrada
├── models.rs               # Estructuras de datos
├── services/
│   ├── mod.rs             # Módulos de servicios
│   ├── telnyx.rs          # Integración Telnyx API
│   ├── claude.rs          # Integración Claude API
│   ├── session.rs         # Gestión de sesiones
│   ├── s3.rs              # Almacenamiento en S3
│   └── app_state.rs       # Estado compartido
├── handlers/
│   ├── mod.rs
│   ├── call.rs            # Endpoints de llamadas
│   └── webhook.rs         # Handlers de webhooks
├── utils/
│   ├── mod.rs
│   └── logger.rs          # Configuración de logging
└── middleware/
    └── mod.rs             # Middleware personalizado
```

## 🔄 Flujo de una llamada

1. **Inicio**: POST `/api/call/initiate` → Telnyx inicia llamada saliente
2. **Contestada**: Webhook `call.answered` → Reproducir saludo
3. **Audio completado**: Webhook `call.speak.ended` → Iniciar transcripción
4. **Transcripción**: Webhook `call.transcription.transcript_received` → Claude genera respuesta
5. **Respuesta**: Reproducir audio → Volver a paso 3
6. **Fin**: Webhook `call.hangup` → Limpiar sesión

## 🚀 Optimizaciones implementadas

- ✅ Streaming de Claude para TTFT (Time to First Token) ultra-bajo
- ✅ Sessions en memoria con DashMap para acceso concurrente rápido
- ✅ Pool de conexiones HTTP reutilizables con reqwest
- ✅ Logging asincrónico sin bloqueos
- ✅ Compilación con optimizaciones agresivas
- ✅ Error handling graceful sin panics

## 📈 Benchmarks (estimados)

| Métrica | Node.js | Rust |
|---------|---------|------|
| Startup | 2-3s | 100ms |
| Memoria | 150-200MB | 10-20MB |
| Latencia | 500-800ms | 200-400ms |
| Throughput | 100 req/s | 1000+ req/s |

## 🔐 Variables de entorno

Todas las variables están disponibles en `.env.example`

## 📝 Logging

El proyecto usa `tracing` para logging estructurado:

```bash
# Mostrar logs de debug
RUST_LOG=telnyx_ai_service=debug cargo run

# Mostrar todos los logs
RUST_LOG=debug cargo run
```

## 🛠️ Desarrollo

### Agregar dependencias

```bash
cargo add nombre_crate
```

### Ejecutar tests

```bash
cargo test
```

### Formatear código

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## 🚢 Deployment

### Build para producción

```bash
cargo build --release
```

El binario estará en `target/release/telnyx_ai_service`

### Docker (opcional)

```dockerfile
FROM rust:latest as builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /app/target/release/telnyx_ai_service /usr/local/bin/
CMD ["telnyx_ai_service"]
```

## 📋 Comparación con versión Node.js

| Aspecto | Node.js | Rust |
|---------|---------|------|
| Lenguaje | JavaScript | Systems Programming |
| Tiempo compilación | - | ~2-3 minutos |
| Dependencias | 15+ directas | 12+ directas |
| Tamaño binario | - | ~30-50MB |
| Seguridad de memoria | Runtime checks | Compile-time checks |
| Async/Await | ✅ | ✅ |
| Manejo de errores | Try/Catch | Result<T, E> |

## 🤝 Contribuir

1. Fork del repositorio
2. Crear rama para tu feature (`git checkout -b feature/AmazingFeature`)
3. Commit cambios (`git commit -m 'Add AmazingFeature'`)
4. Push a la rama (`git push origin feature/AmazingFeature`)
5. Abrir Pull Request

## 📄 Licencia

MIT License - ver LICENSE para más detalles

## 📞 Soporte

Para problemas o preguntas:
1. Revisar documentación de [Axum](https://github.com/tokio-rs/axum)
2. Revisar documentación de [Telnyx API](https://developers.telnyx.com/)
3. Revisar documentación de [Claude API](https://docs.anthropic.com/)

---

**Versión Rust**: 1.0.0  
**Última actualización**: Diciembre 2025  
**Status**: ✅ En producción
