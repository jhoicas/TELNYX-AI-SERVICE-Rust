@echo off
REM Script para configurar Render + GitHub Actions en Windows

echo.
echo 🚀 Configurador de Render + GitHub Actions
echo ===========================================
echo.

REM Verificar que está en repositorio git
if not exist .git (
    echo ❌ No estás en un repositorio Git
    echo Ejecuta desde la raíz del proyecto
    exit /b 1
)

echo 1️⃣  OBTENER DEPLOY HOOK DE RENDER
echo ==================================
echo.
echo Sigue estos pasos en Render:
echo.
echo 1. Ve a https://dashboard.render.com/
echo 2. Selecciona tu servicio 'telnyx-ai-service'
echo 3. Ve a Settings - Deploy Hook
echo 4. Copia la URL
echo.
set /p RENDER_HOOK="Pega el URL del Deploy Hook: "

if "%RENDER_HOOK%"=="" (
    echo ❌ Deploy Hook vacío
    exit /b 1
)

echo.
echo 2️⃣  AGREGAR SECRET A GITHUB
echo ============================
echo.
echo Sigue estos pasos en GitHub:
echo.
echo 1. Ve a tu repositorio en GitHub
echo 2. Settings - Secrets and variables - Actions
echo 3. Click 'New repository secret'
echo 4. Nombre: RENDER_DEPLOY_HOOK
echo 5. Valor: %RENDER_HOOK%
echo 6. Click 'Add secret'
echo.
set /p CONFIRM="¿Ya agregaste el secret en GitHub? (s/n): "

if /i not "%CONFIRM%"=="s" (
    echo Por favor agrega el secret y vuelve a ejecutar
    exit /b 1
)

echo.
echo 3️⃣  VERIFICAR ARCHIVOS
echo ====================
echo.

setlocal enabledelayedexpansion

set "FILES=render.yaml|.github\workflows\deploy.yml|.github\workflows\test.yml|Cargo.toml|src\main.rs"

for %%F in (%FILES%) do (
    if exist "%%F" (
        echo ✅ %%F existe
    ) else (
        echo ❌ %%F NO EXISTE
        exit /b 1
    )
)

echo.
echo 4️⃣  HACER COMMIT
echo ===============
echo.
git add render.yaml .github\
git commit -m "Add: Render deployment configuration"

echo.
echo ✅ Todo listo!
echo.
echo 📋 Próximos pasos:
echo 1. git push origin main
echo 2. GitHub Actions ejecutará test.yml automáticamente
echo 3. Después del merge, deploy.yml notificará a Render
echo 4. Render comenzará el deploy automáticamente
echo.
echo 📊 Monitorea el progreso en:
echo - GitHub: https://github.com/tu-usuario/tu-repo/actions
echo - Render: https://dashboard.render.com/
echo.
