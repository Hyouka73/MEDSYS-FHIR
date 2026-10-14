# ============================================================================
# MedSys-FHIR: Ejecutor Automatizado de Benchmarks k6
# Tesis: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero (UNACH 2026)
# ============================================================================

param(
    [string]$BaseUrl = "",
    [string]$K6Path = "C:\Program Files\k6\k6.exe",
    [int]$Replicas = 1
)

if (-not $BaseUrl) {
    try {
        $null = Invoke-RestMethod -Uri "http://localhost:8080/health" -Method Get -TimeoutSec 1 -ErrorAction Stop
        $BaseUrl = "http://localhost:8080"
    } catch {
        $BaseUrl = "http://localhost:3000"
    }
}

Write-Host "`n========================================================" -ForegroundColor Cyan
Write-Host " MedSys-FHIR -- Suite de Pruebas de Rendimiento k6 (UNACH 2026)" -ForegroundColor Cyan
Write-Host (" Replicas de carga: {0}" -f $Replicas) -ForegroundColor Cyan
Write-Host "========================================================`n" -ForegroundColor Cyan

# 1. Verificar ejecutable de k6
if (-not (Test-Path $K6Path)) {
    $foundK6 = Get-Command k6 -ErrorAction SilentlyContinue
    if ($foundK6) {
        $K6Path = $foundK6.Source
    } else {
        Write-Error "No se encontro el binario de k6 en '$K6Path'. Asegurese de tener k6 instalado."
        exit 1
    }
}
Write-Host ("[OK] Binario de k6 detectado: {0}" -f $K6Path) -ForegroundColor Green

# 2. Verificar conectividad con el servidor backend
Write-Host ("[*] Comprobando conectividad con el middleware en {0}/health ..." -f $BaseUrl) -ForegroundColor Yellow
try {
    $healthResponse = Invoke-RestMethod -Uri "$BaseUrl/health" -Method Get -TimeoutSec 3 -ErrorAction Stop
    Write-Host ("[OK] Servidor MedSys-FHIR en linea: Version {0}, FHIR {1}" -f $healthResponse.server_version, $healthResponse.fhir_version) -ForegroundColor Green
} catch {
    Write-Warning ("El servidor en {0} no respondio de forma inmediata. Asegurese de tener el servidor ejecutandose." -f $BaseUrl)
}

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path

# 3. Ejecutar Smoke Test
Write-Host "`n--------------------------------------------------------" -ForegroundColor Cyan
Write-Host " 1/3: Ejecutando Smoke Test (Disponibilidad y Negociacion FHIR)" -ForegroundColor Cyan
Write-Host "--------------------------------------------------------" -ForegroundColor Cyan
$smokeScript = Join-Path $scriptDir "smoke_test.js"
& $K6Path run --env "BASE_URL=$BaseUrl" "$smokeScript"

# 4. Ejecutar Load Test (Concurrencia hasta 50 VUs)
Write-Host "`n--------------------------------------------------------" -ForegroundColor Cyan
Write-Host (" 2/3: Ejecutando Load Test (Concurrencia hasta 50 VUs, {0} replicas)" -f $Replicas) -ForegroundColor Cyan
Write-Host "--------------------------------------------------------" -ForegroundColor Cyan
$loadScript = Join-Path $scriptDir "load_test.js"
for ($r = 1; $r -le $Replicas; $r++) {
    $resultsLoadJson = Join-Path $scriptDir ("results_load_replica_{0}.json" -f $r)
    Write-Host ("[*] Corriendo replica {0} de {1}..." -f $r, $Replicas) -ForegroundColor DarkCyan
    & $K6Path run --env "BASE_URL=$BaseUrl" --out "json=$resultsLoadJson" "$loadScript"
    Write-Host ("[OK] Resultados guardados en: {0}" -f $resultsLoadJson) -ForegroundColor Green
}

# 5. Ejecutar Resilience & Errors Test
Write-Host "`n--------------------------------------------------------" -ForegroundColor Cyan
Write-Host " 3/3: Ejecutando Resilience Test (OperationOutcome bajo estres)" -ForegroundColor Cyan
Write-Host "--------------------------------------------------------" -ForegroundColor Cyan
$resilienceScript = Join-Path $scriptDir "resilience_and_errors_test.js"
& $K6Path run --env "BASE_URL=$BaseUrl" "$resilienceScript"

Write-Host "`n========================================================" -ForegroundColor Green
Write-Host " [OK] Suite de Pruebas k6 finalizada con exito." -ForegroundColor Green
Write-Host "========================================================`n" -ForegroundColor Green
