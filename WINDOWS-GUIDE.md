# Windows Quick Start Guide 🪟

Guía paso a paso para ejecutar Telnyx AI Service (Rust) en Windows.

## 1️⃣ Instalar Rust

### Opción A: Installer oficial (Recomendado)

1. Visita https://rustup.rs/
2. Descarga `rustup-init.exe`
3. Ejecuta el instalador
4. Acepta opciones por defecto
5. Reinicia PowerShell/CMD

### Opción B: Via Scoop (si tienes scoop instalado)

```powershell
scoop install rustup
rustup init
```

### Verificar instalación

```powershell
rustc --version
cargo --version
```

Deberías ver:
```
rustc 1.75.0 (...
cargo 1.75.0 (...)
```

## 2️⃣ Clonar/Acceder al proyecto

```powershell
cd C:\Users\yoiner.castillo\source\repos\TELNYX-AI-SERVICE-Rust
```

## 3️⃣ Configurar credenciales

### Crear archivo .env

```powershell
# Copiar archivo de ejemplo
Copy-Item .env.example .env

# Abrir en editor (elige uno)
notepad .env
# O con VS Code:
code .env
```

### Agregar tus credenciales

Edita `.env` y completa:

```env
# Telnyx
TELNYX_API_KEY=sk_test_xxxxx
TELNYX_CONNECTION_ID=1234567890123
TELNYX_PHONE_NUMBER=+1234567890

# Claude
ANTHROPIC_API_KEY=sk-ant-xxxxx

# Server
PORT=3000
WEBHOOK_BASE_URL=https://tu-dominio.com

# AWS (opcional)
AWS_REGION=us-east-1
S3_BUCKET=tu-bucket
```

## 4️⃣ Construir el proyecto

### Compilación de desarrollo (más rápido)

```powershell
cargo build
```

Esto toma 2-3 minutos la primera vez.

### Compilación de producción (optimizado)

```powershell
cargo build --release
```

Esto toma 3-5 minutos pero genera un binario más rápido.

## 5️⃣ Ejecutar el servidor

### Desarrollo (con auto-reload)

```powershell
# Instalar cargo-watch (primera vez)
cargo install cargo-watch

# Ejecutar con auto-reload
cargo watch -x run
```

### Directo

```powershell
cargo run
```

### Producción

```powershell
.\target\release\telnyx_ai_service.exe
```

Deberías ver:
```
🚀 Iniciando Telnyx AI Service en Rust
📡 Servidor escuchando
port=3000 environment=development
```

## 6️⃣ Verificar que funciona

En **otra ventana** PowerShell:

```powershell
# Health check
curl http://localhost:3000/

# Debería retornar JSON con endpoints
```

O usa Postman:
- URL: `http://localhost:3000/api/health`
- Método: GET

## 🧪 Prueba una llamada

```powershell
# Guardar en un archivo script.ps1
$body = @{
    telefono = "+521234567890"
    nombre = "Test Usuario"
    contexto = "Test"
} | ConvertTo-Json

$response = Invoke-WebRequest -Uri "http://localhost:3000/api/call/initiate" `
    -Method POST `
    -ContentType "application/json" `
    -Body $body

$response.Content | ConvertFrom-Json
```

## 🐳 Con Docker (Alternativa)

```powershell
# Asegúrate que Docker Desktop está ejecutándose

# Construir imagen
docker build -t telnyx-ai:latest .

# Ejecutar
docker run --env-file .env -p 3000:3000 telnyx-ai:latest

# O con docker-compose
docker-compose up -d
docker-compose logs -f
```

## 🛠️ Usando Makefile (si tienes make)

```powershell
# Instalar make (si no lo tienes)
# Opción 1: via Scoop
scoop install make

# Opción 2: via Chocolatey
choco install make

# Luego usar Makefile
make help          # Ver comandos disponibles
make setup         # Setup inicial
make run           # Ejecutar
make release       # Build para producción
make clean         # Limpiar
```

## 📝 IDE Recommendations

### VS Code (Recomendado)

```powershell
# Instalar extension
code --install-extension rust-lang.rust-analyzer

# Abrir proyecto
code .
```

Luego presiona F5 para ejecutar.

### Visual Studio 2022

1. Instalar extensión "Rust"
2. File → Open Folder → TELNYX-AI-SERVICE-Rust
3. Ctrl+Shift+B → cargo build

## 🆘 Troubleshooting

### Error: "cargo: The term 'cargo' is not recognized"

**Solución**: Rust no está en PATH
```powershell
# Reinicia PowerShell/CMD completamente
# O ejecuta:
$env:PATH = "$env:PATH;$env:USERPROFILE\.cargo\bin"
cargo --version
```

### Error: "TELNYX_API_KEY must be set"

**Solución**: El archivo `.env` no está en el directorio correcto
```powershell
# Verificar que estás en la carpeta correcta
pwd
# Debería mostrar: C:\Users\yoiner.castillo\source\repos\TELNYX-AI-SERVICE-Rust

# Verificar que .env existe
ls .env

# Si no existe
Copy-Item .env.example .env
```

### Compilación lenta

Es normal. La primera compilación:
- **Descarga dependencias**: 30-60s
- **Compila dependencias**: 90-120s
- **Compila proyecto**: 30-60s
- **Total**: 2-3 minutos

Los builds posteriores son mucho más rápidos (10-30s).

Para acelerar:
```powershell
# Usar SSD
# Aumentar cores de compilación
cargo build -j 4
```

### Port 3000 ya está en uso

```powershell
# Encontrar qué está usando el puerto
Get-NetTCPConnection -LocalPort 3000

# Cambiar puerto en .env
# PORT=3001
```

### Memory/CPU errors en compilación

```powershell
# Reducir jobs de compilación
cargo build -j 2

# O aumentar swap en Windows (SI es necesario)
# Settings → Advanced System → Virtual Memory
```

## 📊 Verificar compilación

```powershell
# Quick check (sin compilar)
cargo check

# Ver tamaño del binario
ls target\release\telnyx_ai_service.exe | % { "{0} MB" -f ($_.Length / 1MB) }
```

## 🎯 Next Steps

1. ✅ Verificar logs: `RUST_LOG=debug cargo run`
2. ✅ Configurar webhook en Telnyx dashboard
3. ✅ Hacer prueba de llamada completa
4. ✅ Monitorear rendimiento
5. ✅ Deploy en servidor

## 📚 Recursos adicionales

- [Rust Book](https://doc.rust-lang.org/book/)
- [Axum Documentation](https://github.com/tokio-rs/axum)
- [Telnyx API Docs](https://developers.telnyx.com/)
- [Claude API](https://docs.anthropic.com/)

## ⚡ Alias útiles (opcional)

Agrega a tu perfil PowerShell (`$PROFILE`):

```powershell
Set-Alias rs cargo
Set-Alias rsr 'cargo run'
Set-Alias rsb 'cargo build --release'
Set-Alias rstest 'cargo test'
```

Luego puedes usar:
```powershell
rs build --release
rsr
```

## 🎉 ¡Listo!

Ya tienes Telnyx AI Service (Rust) ejecutándose en Windows.

Para soporte adicional, consulta:
- README.md - Documentación completa
- QUICKSTART.md - Inicio rápido general
- BENCHMARKS.md - Análisis de rendimiento

---

**Last Updated**: Diciembre 2025
**Status**: ✅ Ready to use
