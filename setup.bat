@echo off
REM Script de setup para Telnyx AI Service - Rust (Windows PowerShell)

echo 🚀 Setup de Telnyx AI Service (Rust)
echo ====================================
echo.

REM Verificar si cargo está disponible
where cargo >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo ❌ Rust no está instalado
    echo 📥 Descargando Rust desde https://rustup.rs/
    echo.
    echo Para Windows:
    echo   1. Descarga rustup-init.exe de https://rustup.rs/
    echo   2. Ejecuta el instalador
    echo   3. Sigue las instrucciones en pantalla
    echo.
    exit /b 1
)

for /f "tokens=*" %%i in ('rustc --version') do set RUST_VERSION=%%i
echo ✅ Rust detectado: %RUST_VERSION%
echo.

REM Crear archivo .env si no existe
if not exist .env (
    echo 📝 Creando archivo .env desde .env.example...
    copy .env.example .env
    echo ⚠️  Edita .env con tus credenciales antes de ejecutar
    echo.
)

REM Instalar dependencias Rust
echo 📦 Descargando dependencias...
call cargo fetch

REM Build
echo.
echo 🔨 Compilando proyecto (esto puede tomar 2-3 minutos)...
call cargo build

if %ERRORLEVEL% EQU 0 (
    echo.
    echo ✅ Setup completado exitosamente!
    echo.
    echo 📋 Próximos pasos:
    echo   1. Edita .env con tus credenciales de Telnyx y Claude
    echo   2. Asegúrate que WEBHOOK_BASE_URL sea accesible públicamente
    echo   3. Configura el webhook en Telnyx dashboard
    echo   4. Ejecuta: cargo run
    echo.
) else (
    echo ❌ Error durante la compilación
    exit /b 1
)
