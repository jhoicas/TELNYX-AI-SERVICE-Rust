#!/bin/bash
# Script para generar el Deploy Hook de Render y agregarlo a GitHub

set -e

echo "🚀 Configurador de Render + GitHub Actions"
echo "=========================================="
echo ""

# Verificar que está en repositorio git
if [ ! -d ".git" ]; then
    echo "❌ No estás en un repositorio Git"
    echo "Ejecuta desde la raíz del proyecto: C:\...\TELNYX-AI-SERVICE-Rust"
    exit 1
fi

echo "1️⃣  OBTENER DEPLOY HOOK DE RENDER"
echo "=================================="
echo ""
echo "Sigue estos pasos en Render:"
echo ""
echo "1. Ve a https://dashboard.render.com/"
echo "2. Selecciona tu servicio 'telnyx-ai-service'"
echo "3. Ve a Settings → Deploy Hook"
echo "4. Copia la URL (parecerá a):"
echo "   https://api.render.com/deploy/srv-xxxxxxxx?key=xxxxxxxx"
echo ""
read -p "Pega el URL del Deploy Hook: " RENDER_HOOK

if [ -z "$RENDER_HOOK" ]; then
    echo "❌ Deploy Hook vacío"
    exit 1
fi

echo ""
echo "2️⃣  AGREGAR SECRET A GITHUB"
echo "============================"
echo ""
echo "Sigue estos pasos en GitHub:"
echo ""
echo "1. Ve a tu repositorio en GitHub"
echo "2. Settings → Secrets and variables → Actions"
echo "3. Click 'New repository secret'"
echo "4. Nombre: RENDER_DEPLOY_HOOK"
echo "5. Valor: (pega el URL que acabas de copiar)"
echo "6. Click 'Add secret'"
echo ""
read -p "¿Ya agregaste el secret en GitHub? (s/n): " CONFIRM

if [ "$CONFIRM" != "s" ] && [ "$CONFIRM" != "S" ]; then
    echo "Por favor agrega el secret y vuelve a ejecutar"
    exit 1
fi

echo ""
echo "3️⃣  VERIFICAR ARCHIVOS"
echo "===================="
echo ""

# Verificar archivos necesarios
FILES=(
    "render.yaml"
    ".github/workflows/deploy.yml"
    ".github/workflows/test.yml"
    "Cargo.toml"
    "src/main.rs"
)

for file in "${FILES[@]}"; do
    if [ -f "$file" ]; then
        echo "✅ $file existe"
    else
        echo "❌ $file NO EXISTE"
        exit 1
    fi
done

echo ""
echo "4️⃣  HACER COMMIT"
echo "==============="
echo ""
git add render.yaml .github/
git commit -m "Add: Render deployment configuration"

echo ""
echo "✅ Todo listo!"
echo ""
echo "📋 Próximos pasos:"
echo "1. git push origin main"
echo "2. GitHub Actions ejecutará test.yml automáticamente"
echo "3. Después del merge, deploy.yml notificará a Render"
echo "4. Render comenzará el deploy automáticamente"
echo ""
echo "📊 Monitorea el progreso en:"
echo "- GitHub: https://github.com/tu-usuario/tu-repo/actions"
echo "- Render: https://dashboard.render.com/"
echo ""
