#!/bin/bash
# Script de setup para Telnyx AI Service - Rust

echo "🚀 Setup de Telnyx AI Service (Rust)"
echo "===================================="
echo ""

# Verificar si Rust está instalado
if ! command -v cargo &> /dev/null; then
    echo "❌ Rust no está instalado"
    echo "📥 Descargando Rust desde https://rustup.rs/"
    echo ""
    echo "Para Windows:"
    echo "  1. Descarga rustup-init.exe de https://rustup.rs/"
    echo "  2. Ejecuta el instalador"
    echo "  3. Sigue las instrucciones en pantalla"
    echo ""
    echo "Para Linux/macOS:"
    echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust detectado: $(rustc --version)"
echo ""

# Crear archivo .env si no existe
if [ ! -f .env ]; then
    echo "📝 Creando archivo .env desde .env.example..."
    cp .env.example .env
    echo "⚠️  Edita .env con tus credenciales antes de ejecutar"
    echo ""
fi

# Instalar dependencias Rust
echo "📦 Descargando dependencias..."
cargo fetch

# Build en modo debug
echo ""
echo "🔨 Compilando proyecto (esto puede tomar 2-3 minutos)..."
cargo build

if [ $? -eq 0 ]; then
    echo ""
    echo "✅ Setup completado exitosamente!"
    echo ""
    echo "📋 Próximos pasos:"
    echo "  1. Edita .env con tus credenciales de Telnyx y Claude"
    echo "  2. Asegúrate que WEBHOOK_BASE_URL sea accesible públicamente"
    echo "  3. Configura el webhook en Telnyx dashboard"
    echo "  4. Ejecuta: cargo run"
    echo ""
else
    echo "❌ Error durante la compilación"
    exit 1
fi
