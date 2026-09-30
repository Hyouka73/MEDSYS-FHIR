# ============================================================================
# MedSys-FHIR: Ejecutor Automatizado de Benchmarks k6
# Tesis: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero (UNACH 2026)
# ============================================================================

param(
    [string]$BaseUrl = "http://localhost:3000",
    [string]$K6Path = "C:\Program Files\k6\k6.exe"
)

Write-Host "`n========================================================" -ForegroundColor Cyan
Write-Host " MedSys-FHIR — Suite de Pruebas de Rendimiento k6 (UNACH 2026)" -ForegroundColor Cyan
Write-Host "========================================================`n" -ForegroundColor Cyan

# 1. Verificar ejecutable de k6
if (-not (Test-Path $K6Path)) {
    $foundK6 = Get-Command k6 -ErrorAction SilentlyContinue
    if ($foundK6) {
        $K6Path = $foundK6.Source
    } else {
        Write-Error "No se encontró el binario de k6 en '$K6Path'. Asegúrese de tener k6 instalado."
        exit 1
    }
}
Write-Host "[✓] Binario de k6 detectado: $K6Path" -ForegroundColor Green

# 2. Verificar conectividad con el servidor backend
Write-Host "[*] Comprobando conectividad con el middleware en $BaseUrl/health ..." -ForegroundColor Yellow
try {
    $healthResponse = Invoke-RestMethod -Uri "$BaseUrl/health" -Method Get -TimeoutSec 3 -ErrorAction Stop
    Write-Host "[✓] Servidor MedSys-FHIR en línea: Versión $($healthResponse.server_version), FHIR $($healthResponse.fhir_version)" -ForegroundColor Green
} catch {
    Write-Warning "El servidor en $BaseUrl no respondió de forma inmediata. Asegúrese de tener 'cargo run -p medsys-server' o el binario release ejecutándose."
    Write-Host "[?] Desea continuar ejecutando las pruebas de todas formas? (s/N): " -NoNewline
    $resp = Read-Host
    if ($resp -ne "s" -and $resp -ne "S") {
        Write-Host "Cancelando ejecución de pruebas k6." -ForegroundColor Red
        exit 0
    }
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# 3. Ejecutar Smoke Test
Write-Host "`n--------------------------------------------------------" -ForegroundColor Cyan
Write-Host " 1/3: Ejecutando Smoke Test (Disponibilidad y Negociación FHIR)" -ForegroundColor Cyan
Write-Host "--------------------------------------------------------" -ForegroundColor Cyan
& $K6Path run --env BASE_URL=$BaseUrl "$scriptDir\smoke_test.js"

# 4. Ejecutar Load Test (Concurrencia)
Write-Host "`n--------------------------------------------------------" -ForegroundColor Cyan
Write-Host " 2/3: Ejecutando Load Test (Concurrencia hasta 50 VUs)" -ForegroundColor Cyan
Write-Host "--------------------------------------------------------" -ForegroundColor Cyan
$resultsLoadJson = Join-Path $scriptDir "results_load.json"
& $K6Path run --env BASE_URL=$BaseUrl --out json="$resultsLoadJson" "$scriptDir\load_test.js"
Write-Host "[✓] Resultados guardados en: $resultsLoadJson" -ForegroundColor Green

# 5. Ejecutar Resilience & Errors Test
Write-Host "`n--------------------------------------------------------" -ForegroundColor Cyan
Write-Host " 3/3: Ejecutando Resilience Test (OperationOutcome bajo estrés)" -ForegroundColor Cyan
Write-Host "--------------------------------------------------------" -ForegroundColor Cyan
& $K6Path run --env BASE_URL=$BaseUrl "$scriptDir\resilience_and_errors_test.js"

Write-Host "`n========================================================" -ForegroundColor Green
Write-Host " [✓] Suite de Pruebas k6 finalizada con éxito." -ForegroundColor Green
Write-Host "========================================================`n" -ForegroundColor Green
