#!/usr/bin/env pwsh

# Script de compilación y testing para WebSocket Migration
# Uso: .\build-and-test.ps1

param(
    [ValidateSet('check', 'build', 'run', 'test', 'release')]
    [string]$Task = 'build',
    
    [string]$CargoPath = "$env:USERPROFILE\.cargo\bin\cargo.exe"
)

$ErrorActionPreference = 'Stop'

# Colores para output
function Write-Success { Write-Host "✅ $args" -ForegroundColor Green }
function Write-Info { Write-Host "ℹ️  $args" -ForegroundColor Cyan }
function Write-Error { Write-Host "❌ $args" -ForegroundColor Red }
function Write-Warn { Write-Host "⚠️  $args" -ForegroundColor Yellow }

# Verificar que cargo existe
if (-not (Test-Path $CargoPath)) {
    Write-Error "Cargo no encontrado en: $CargoPath"
    Write-Info "Instala Rust desde https://rustup.rs/ o usa el instalador oficial"
    exit 1
}

Write-Info "Usando cargo: $CargoPath"
Write-Info "Tarea: $Task"

# Verificar variables de entorno críticas
$required_vars = @('DEEPGRAM_API_KEY', 'TELNYX_API_KEY', 'CLAUDE_API_KEY')
$missing_vars = @()

foreach ($var in $required_vars) {
    if (-not [Environment]::GetEnvironmentVariable($var)) {
        $missing_vars += $var
    }
}

if ($missing_vars.Count -gt 0) {
    Write-Warn "Variables de entorno faltantes: $($missing_vars -join ', ')"
    Write-Info "Verifica que .env contiene todas las keys necesarias"
}

switch ($Task) {
    'check' {
        Write-Info "Ejecutando cargo check..."
        & $CargoPath check
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Verificación completada sin errores"
        } else {
            Write-Error "Errores detectados"
            exit 1
        }
    }
    
    'build' {
        Write-Info "Compilando debug..."
        & $CargoPath build
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Build debug completado"
        } else {
            Write-Error "Error en compilación"
            exit 1
        }
    }
    
    'release' {
        Write-Info "Compilando release (optimizado)..."
        & $CargoPath build --release
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Build release completado"
            Write-Info "Binario en: target\release\telnyx_ai_service.exe"
        } else {
            Write-Error "Error en compilación"
            exit 1
        }
    }
    
    'test' {
        Write-Info "Ejecutando tests..."
        & $CargoPath test -- --nocapture
        if ($LASTEXITCODE -eq 0) {
            Write-Success "Tests pasados"
        } else {
            Write-Error "Tests fallaron"
            exit 1
        }
    }
    
    'run' {
        Write-Info "Compilando y ejecutando (debug)..."
        Write-Warn "Presiona Ctrl+C para detener"
        
        # Verificar puerto disponible
        $port = 3000
        $listening = Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue
        if ($listening) {
            Write-Warn "Puerto $port ya en uso"
            $newPort = $port + 1
            Write-Info "Intenta matar el proceso anterior o usar puerto diferente"
            exit 1
        }
        
        Write-Info "Iniciando servidor en puerto $port..."
        & $CargoPath run
    }
}

Write-Success "Listo! ✨"
Write-Info "Para las próximas tareas:"
Write-Info "  Verificación rápida: .\build-and-test.ps1 -Task check"
Write-Info "  Compilar debug:      .\build-and-test.ps1 -Task build"
Write-Info "  Compilar release:    .\build-and-test.ps1 -Task release"
Write-Info "  Ejecutar tests:      .\build-and-test.ps1 -Task test"
Write-Info "  Ejecutar servidor:   .\build-and-test.ps1 -Task run"
