# ⚡ Cheat Sheet - Referencia rápida

## 🚀 Comandos más usados

```powershell
# Setup inicial
cargo build                    # Compilar (primera vez: 2-3 min)
cargo run                      # Ejecutar servidor

# Desarrollo
cargo watch -x run            # Auto-reload (requiere cargo-watch)
cargo check                   # Verificar sin compilar
cargo test                    # Ejecutar tests

# Producción
cargo build --release         # Build optimizado (3-5 min)
.\target\release\telnyx_ai_service.exe  # Ejecutar binario

# Herramientas
cargo fmt                     # Formatear código
cargo clippy                  # Linter
cargo doc --open              # Generar documentación
cargo audit                   # Verificar vulnerabilidades
```

## 📁 Rutas importantes

```
Proyecto:     C:\Users\yoiner.castillo\source\repos\TELNYX-AI-SERVICE-Rust
Código:       src/
Configuración: .env (crear desde .env.example)
Binario:      target/release/telnyx_ai_service.exe
Documentación: *.md (README.md, QUICKSTART.md, etc)
```

## 🔧 Variables de entorno (.env)

```env
# CRÍTICO
TELNYX_API_KEY=sk_test_xxxxx
TELNYX_CONNECTION_ID=1234567890
TELNYX_PHONE_NUMBER=+1234567890
ANTHROPIC_API_KEY=sk-ant-xxxxx

# RECOMENDADO
WEBHOOK_BASE_URL=https://tu-dominio.com
PORT=3000

# OPCIONAL
AWS_REGION=us-east-1
S3_BUCKET=tu-bucket
RUST_LOG=debug
```

## 🌐 Endpoints principales

```
GET    /                           # Info del servicio
GET    /api/health                 # Health check
POST   /api/call/initiate          # Iniciar llamada
POST   /api/call/batch             # Batch de llamadas
GET    /api/sessions/stats         # Estadísticas
POST   /webhook/telnyx             # Webhook Telnyx
```

## 🧪 Test rápido

```powershell
# Health check (desde PowerShell)
curl http://localhost:3000/api/health

# Iniciar llamada
$body = @{
    telefono = "+521234567890"
    nombre = "Test"
} | ConvertTo-Json

Invoke-WebRequest -Uri "http://localhost:3000/api/call/initiate" `
    -Method POST `
    -ContentType "application/json" `
    -Body $body
```

## 📊 Estructura de directorios

```
src/
  ├── main.rs                    # Entry point
  ├── models.rs                  # Estructuras de datos
  ├── services/
  │   ├── telnyx.rs             # API Telnyx
  │   ├── claude.rs             # IA Claude
  │   ├── session.rs            # Sesiones
  │   ├── s3.rs                 # AWS S3
  │   └── app_state.rs          # Estado global
  ├── handlers/
  │   ├── call.rs               # Endpoints de llamadas
  │   └── webhook.rs            # Webhooks
  ├── utils/
  │   └── logger.rs             # Logging
  └── middleware/
      └── mod.rs                # Middlewares
```

## 🐳 Docker

```powershell
# Build
docker build -t telnyx-ai:latest .

# Ejecutar
docker run --env-file .env -p 3000:3000 telnyx-ai:latest

# O con docker-compose
docker-compose up -d
docker-compose logs -f
docker-compose down
```

## 🛠️ Troubleshooting rápido

| Problema | Solución |
|----------|----------|
| "cargo not found" | Instalar Rust desde https://rustup.rs/ |
| "TELNYX_API_KEY must be set" | Crear .env y agregar credenciales |
| "Port 3000 in use" | `PORT=3001 cargo run` |
| "Build lento" | Usar `cargo check` para verificación rápida |
| "Memory error" | Compilar con `cargo build -j 2` |

## 📈 Performance

```
Startup:        ~100ms
Memory:         ~12MB base
Max requests:   1000+ req/s
P95 latency:    200-400ms
Connections:    10,000+
```

## 📚 Documentación importante

```
QUICKSTART.md              ← COMIENZA AQUÍ
WINDOWS-GUIDE.md           ← Si usas Windows
README.md                  ← Completa
MIGRACION.md              ← Si vienes de Node.js
BENCHMARKS.md             ← Análisis de rendimiento
```

## 🎯 Desarrollo local

```powershell
# Terminal 1: Servidor
cargo watch -x run

# Terminal 2: Testing
curl -X POST http://localhost:3000/api/call/initiate `
    -H "Content-Type: application/json" `
    -d '{"telefono":"+521234567890","nombre":"Test"}'

# Terminal 3: Logs
$env:RUST_LOG="debug"; cargo run
```

## 🚀 Deploy rápido

```powershell
# Build para producción
cargo build --release

# Binario está en:
# ./target/release/telnyx_ai_service.exe

# Copiar a servidor y ejecutar:
# .\telnyx_ai_service.exe

# O usar Docker:
docker build -t telnyx-ai .
docker run --env-file .env -p 3000:3000 telnyx-ai
```

## 🔍 Verificar estructura

```powershell
# Ver todos los archivos
tree src /F

# Ver estadísticas
python verify_structure.py
```

## 📦 Agregar dependencias

```powershell
# Agregar crate
cargo add nombre_crate

# Con version específica
cargo add nombre_crate@1.2.3

# Con features
cargo add nombre_crate -F "feature1,feature2"
```

## 🧹 Limpieza

```powershell
# Borrar build
cargo clean

# Ver tamaño de proyecto
du -sh target/

# Actualizar dependencias
cargo update
```

## 🔐 Seguridad

```powershell
# Verificar vulnerabilidades
cargo audit

# Actualizar a latest (con cuidado)
cargo update --aggressive
```

## 📊 Análisis

```powershell
# Arbol de dependencias
cargo tree

# Dependencias desactualizadas
cargo outdated

# Tamaño del binario
ls target/release/telnyx_ai_service.exe | % { "{0} MB" -f ($_.Length / 1MB) }
```

## 🎓 Aprende Rust rápido

```
Conceptos clave:
- ownership & borrowing
- Result<T, E> para errores
- async/await con tokio
- serde para serialización

Librerías usadas aquí:
- axum: Web framework
- tokio: Async runtime
- serde_json: JSON
- reqwest: HTTP client
- tracing: Logging
```

## ⚡ Tips de productividad

```powershell
# Alias útiles (agrega a $PROFILE)
Set-Alias rs cargo
Set-Alias rsr 'cargo run'
Set-Alias rsb 'cargo build --release'

# Luego usar:
rs check                  # cargo check
rsr                      # cargo run
rsb                      # cargo build --release
```

## 🆘 SOS

```powershell
# Borrar todo y empezar fresh
cargo clean
cargo build

# Si compiler se bloquea
taskkill /F /IM rustc.exe

# Reinstalar Rust
rustup self uninstall
# Luego descargar desde https://rustup.rs/
```

---

**Última actualización**: Diciembre 2025  
**Imprime este archivo o guárdalo en favoritos** 🔖
