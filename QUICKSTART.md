# Quick Start - Iniciación Rápida

## ⚡ 5 minutos para estar listo

### 1️⃣ Requisitos previos
- Rust 1.70+ (descarga desde https://rustup.rs/)
- Variables de Telnyx (API Key, Connection ID, Número)
- API Key de Claude (Anthropic)

### 2️⃣ Setup inicial (Windows)

```powershell
# Cambiar a directorio
cd "C:\Users\yoiner.castillo\source\repos\TELNYX-AI-SERVICE-Rust"

# Copiar .env
Copy-Item .env.example .env

# Editar .env con tus credenciales
notepad .env
```

### 3️⃣ Build

```powershell
# Compilar (primera vez: 2-3 minutos)
cargo build --release

# El binario estará en: target\release\telnyx_ai_service.exe
```

### 4️⃣ Ejecutar

```powershell
# Desarrollo (con auto-reload)
cargo install cargo-watch
cargo watch -x run

# O directamente
cargo run

# Producción
.\target\release\telnyx_ai_service.exe
```

### 5️⃣ Verificar

```powershell
# En otra terminal
curl http://localhost:3000/

# Deberías ver:
# {
#   "service": "Telnyx AI Service (Rust)",
#   "version": "1.0.0",
#   "status": "running",
#   "endpoints": { ... }
# }
```

## 📋 Variables de .env (Mínimas)

```env
TELNYX_API_KEY=your_key_here
TELNYX_CONNECTION_ID=your_connection_id
TELNYX_PHONE_NUMBER=+1234567890
ANTHROPIC_API_KEY=sk-ant-...
WEBHOOK_BASE_URL=https://tu-dominio.com
PORT=3000
```

## 🧪 Test una llamada

```powershell
# POST /api/call/initiate
$body = @{
    telefono = "+521234567890"
    nombre = "Juan Pérez"
    contexto = "Consulta veterinaria"
} | ConvertTo-Json

Invoke-WebRequest -Uri "http://localhost:3000/api/call/initiate" `
    -Method POST `
    -ContentType "application/json" `
    -Body $body
```

## 🐳 Con Docker

```powershell
# Build imagen
docker build -t telnyx-ai-rust .

# Ejecutar con .env
docker run --env-file .env -p 3000:3000 telnyx-ai-rust

# O con docker-compose
docker-compose up -d
```

## 📊 Status Check

```powershell
# Health check
curl http://localhost:3000/api/health

# Stats
curl http://localhost:3000/api/sessions/stats
```

## 🆘 Si algo falla

### Error: "TELNYX_API_KEY must be set"
→ Asegúrate que `.env` está creado y tiene `TELNYX_API_KEY=...`

### Error: "Connection refused"
→ El servidor no está corriendo. Ejecuta `cargo run`

### Error: "Command not found: cargo"
→ Rust no está instalado. Descarga desde https://rustup.rs/

### Compilación lenta
→ Es normal la primera vez (2-3 min). Los builds posteriores son más rápidos.

## 🚀 Deployment rápido

### Railway.app (RECOMENDADO)

1. Fork/Clone repo en GitHub
2. Conectar a Railway
3. Agregar variables de entorno
4. Deploy automático

### Render.com

1. Conectar GitHub
2. Seleccionar "Rust" como runtime
3. Configurar variables de entorno
4. Deploy

### VPS (DigitalOcean, Linode, etc)

```bash
# SSH a tu servidor
ssh usuario@ip

# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Clonar repo
git clone https://github.com/tu-repo/TELNYX-AI-SERVICE-Rust
cd TELNYX-AI-SERVICE-Rust

# Configurar .env
nano .env

# Build en producción
cargo build --release

# Ejecutar con systemd
sudo nano /etc/systemd/system/telnyx-ai.service
```

systemd file:
```ini
[Unit]
Description=Telnyx AI Service
After=network.target

[Service]
Type=simple
User=tu-usuario
WorkingDirectory=/home/tu-usuario/TELNYX-AI-SERVICE-Rust
ExecStart=/home/tu-usuario/TELNYX-AI-SERVICE-Rust/target/release/telnyx_ai_service
Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
```

```bash
sudo systemctl daemon-reload
sudo systemctl enable telnyx-ai
sudo systemctl start telnyx-ai
sudo systemctl status telnyx-ai
```

## 📈 Next Steps

1. ✅ Configurar en Telnyx dashboard el webhook URL
2. ✅ Realizar primer test de llamada
3. ✅ Monitorear logs: `RUST_LOG=debug cargo run`
4. ✅ Optimizar Cargo.toml con las opciones de Cargo-optimization.toml
5. ✅ Configurar CI/CD (GitHub Actions, GitLab CI, etc)

## 🎯 Resumen

- **5 min setup**: Rust install → build → run
- **10 min productivo**: Primeras llamadas funcionando
- **30 min fully integrated**: Webhook configurado y testeado

---

**Pro Tip**: Si necesitas volver a Node.js, ambas versiones coexisten sin conflicto en directorios separados.
