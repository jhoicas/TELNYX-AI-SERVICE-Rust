#!/usr/bin/env pwsh
# Script para probar Claude API directamente

$API_URL = if ($env:API_URL) { $env:API_URL } else { "http://localhost:3000" }

Write-Host "🧪 Probando Claude API en: $API_URL/api/test/claude" -ForegroundColor Cyan
Write-Host ""

# Test 1: Pregunta simple
Write-Host "📝 Test 1: Pregunta sobre horarios" -ForegroundColor Yellow
$body1 = @{
    nombre = "Carlos"
    mensaje = "Hola, a qué hora abren?"
} | ConvertTo-Json

Write-Host "Request: $body1" -ForegroundColor Gray
$response1 = Invoke-RestMethod -Uri "$API_URL/api/test/claude" -Method POST -Body $body1 -ContentType "application/json" -ErrorAction SilentlyContinue
if ($response1) {
    Write-Host "✅ Modelo: $($response1.model)" -ForegroundColor Green
    Write-Host "✅ Respuesta: $($response1.response)" -ForegroundColor Green
} else {
    Write-Host "❌ Error en request" -ForegroundColor Red
}
Write-Host ""

# Test 2: Con contexto
Write-Host "📝 Test 2: Pregunta con contexto" -ForegroundColor Yellow
$body2 = @{
    nombre = "María"
    mensaje = "Necesito agendar una cita para mi gata"
    contexto = "La cliente llamó anteriormente preguntando por vacunas"
} | ConvertTo-Json

Write-Host "Request: $body2" -ForegroundColor Gray
$response2 = Invoke-RestMethod -Uri "$API_URL/api/test/claude" -Method POST -Body $body2 -ContentType "application/json" -ErrorAction SilentlyContinue
if ($response2) {
    Write-Host "✅ Modelo: $($response2.model)" -ForegroundColor Green
    Write-Host "✅ Respuesta: $($response2.response)" -ForegroundColor Green
} else {
    Write-Host "❌ Error en request" -ForegroundColor Red
}
Write-Host ""

# Test 3: Emergencia
Write-Host "📝 Test 3: Consulta de emergencia" -ForegroundColor Yellow
$body3 = @{
    nombre = "Pedro"
    mensaje = "Mi perro se comió chocolate, es urgente!"
} | ConvertTo-Json

Write-Host "Request: $body3" -ForegroundColor Gray
$response3 = Invoke-RestMethod -Uri "$API_URL/api/test/claude" -Method POST -Body $body3 -ContentType "application/json" -ErrorAction SilentlyContinue
if ($response3) {
    Write-Host "✅ Modelo: $($response3.model)" -ForegroundColor Green
    Write-Host "✅ Respuesta: $($response3.response)" -ForegroundColor Green
} else {
    Write-Host "❌ Error en request" -ForegroundColor Red
}
Write-Host ""

Write-Host "✅ Tests completados. Revisa los logs del servidor para ver detalles de Claude." -ForegroundColor Cyan
