# ==============================================================================
# MedSys-FHIR: Pipeline de Benchmarking con Telemetría docker stats (PowerShell)
# Tesis: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero
# Trazabilidad: AUD-002 / CON-004 / CVD-004
# ==============================================================================

param(
    [string]$BaseUrl = "http://localhost:8080",
    [string]$ContainerName = "medsys-server",
    [int]$SamplingIntervalSec = 5,
    [int]$MaxWaitSec = 90
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = (Resolve-Path "$scriptDir\..\..").Path
$composeFile = Join-Path $projectRoot "docker\docker-compose.yml"
$csvOutput = Join-Path $scriptDir "benchmark_stats.csv"
$k6Script = Join-Path $scriptDir "load_test.js"

Write-Host "`n================================================================================" -ForegroundColor Cyan
Write-Host " MedSys-FHIR — Orquestación Docker y Pipeline de Benchmarking Reproducible" -ForegroundColor Cyan
Write-Host " Límite cgroups: 1.0 vCPU | 256 MB RAM | Muestreo docker stats cada ${SamplingIntervalSec}s" -ForegroundColor Cyan
Write-Host "================================================================================`n" -ForegroundColor Cyan

# 1. Verificar binarios requeridos (docker y k6)
$foundDocker = Get-Command docker -ErrorAction SilentlyContinue
if (-not $foundDocker) {
    Write-Error "No se encontró el comando 'docker' en el PATH del sistema."
    exit 1
}

$k6Cmd = Get-Command k6 -ErrorAction SilentlyContinue
if (-not $k6Cmd) {
    $fallbackK6 = "C:\Program Files\k6\k6.exe"
    if (Test-Path $fallbackK6) {
        $k6Path = $fallbackK6
    } else {
        Write-Error "No se encontró el ejecutable de 'k6' en PATH ni en '$fallbackK6'."
        exit 1
    }
} else {
    $k6Path = $k6Cmd.Source
}
Write-Host "[✓] Binarios detectados: Docker ($($foundDocker.Source)) | k6 ($k6Path)" -ForegroundColor Green

# 2. Levantar servicios en segundo plano con docker compose
Write-Host "[*] Desplegando servicios contenerizados con directivas cgroups v2..." -ForegroundColor Yellow
& docker compose -f "$composeFile" up -d
if ($LASTEXITCODE -ne 0) {
    Write-Error "Fallo al ejecutar 'docker compose up -d'."
    exit 1
}

# 3. Esperar a que el endpoint de salud responda HTTP 200
Write-Host "[*] Esperando disponibilidad de $BaseUrl/health (Timeout: ${MaxWaitSec}s)..." -ForegroundColor Yellow
$elapsed = 0
$serverReady = $false

while ($elapsed -lt $MaxWaitSec) {
    try {
        $healthResponse = Invoke-RestMethod -Uri "$BaseUrl/health" -Method Get -TimeoutSec 3 -ErrorAction Stop
        if ($healthResponse.status -eq "ok" -or $healthResponse.status -eq "pass") {
            $serverReady = $true
            break
        }
    } catch {
        # Continúa esperando
    }
    Start-Sleep -Seconds 2
    $elapsed += 2
    Write-Host "    ... esperando respuesta de /health (${elapsed}s transcurridos)" -ForegroundColor DarkGray
}

if (-not $serverReady) {
    Write-Host "[!] ERROR: El middleware no respondió HTTP 200 en $BaseUrl/health tras ${MaxWaitSec}s." -ForegroundColor Red
    Write-Host "[*] Volcado de logs de $ContainerName :" -ForegroundColor DarkYellow
    & docker compose -f "$composeFile" logs --tail 30 $ContainerName
    exit 1
}

Write-Host "[✓] Middleware '$ContainerName' en línea y respondiendo HTTP 200." -ForegroundColor Green

# 4. Inicializar archivo CSV de telemetría
"timestamp,cpu_percentage,mem_usage_raw" | Out-File -FilePath $csvOutput -Encoding utf8

Write-Host "[*] Iniciando muestreo de docker stats sobre '$ContainerName' cada ${SamplingIntervalSec}s -> $csvOutput..." -ForegroundColor Yellow

# Script block para el recolector en segundo plano
$statsScriptBlock = {
    param($container, $interval, $outFile)
    while ($true) {
        $ts = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
        $raw = docker stats $container --no-stream --format "{{.CPUPerc}},{{.MemUsage}}" 2>$null
        if ($raw) {
            "$ts,$raw" | Out-File -FilePath $outFile -Append -Encoding utf8
        }
        Start-Sleep -Seconds $interval
    }
}

$statsJob = Start-Job -ScriptBlock $statsScriptBlock -ArgumentList $ContainerName, $SamplingIntervalSec, $csvOutput

# 5. Ejecutar la prueba de carga con k6
Write-Host "`n================================================================================" -ForegroundColor Cyan
Write-Host " [*] Ejecutando k6 con escenario de meseta sostenida a 50 VUs (load_test.js)..." -ForegroundColor Cyan
Write-Host "================================================================================`n" -ForegroundColor Cyan

$k6ExitCode = 0
try {
    & $k6Path run --env "BASE_URL=$BaseUrl" "$k6Script"
    $k6ExitCode = $LASTEXITCODE
} finally {
    # 6. Detener recolección de telemetría de forma garantizada
    Write-Host "`n[*] Deteniendo proceso de muestreo de telemetría..." -ForegroundColor Yellow
    Stop-Job -Job $statsJob -ErrorAction SilentlyContinue
    Remove-Job -Job $statsJob -ErrorAction SilentlyContinue
}

# 7. Consolidar y presentar estadísticas de recursos
Write-Host "`n================================================================================" -ForegroundColor Cyan
Write-Host " Consolidación de Telemetría de Recursos (docker stats) — MedSys-FHIR" -ForegroundColor Cyan
Write-Host "================================================================================" -ForegroundColor Cyan

if (Test-Path $csvOutput) {
    $rows = Import-Csv -Path $csvOutput -ErrorAction SilentlyContinue
    if ($rows -and $rows.Count -gt 0) {
        $cpuValues = @()
        $memValuesMib = @()

        foreach ($row in $rows) {
            if ($row.cpu_percentage) {
                $cpuClean = $row.cpu_percentage.Replace("%", "").Trim()
                $cpuParsed = 0.0
                if ([double]::TryParse($cpuClean, [System.Globalization.NumberStyles]::Any, [System.Globalization.CultureInfo]::InvariantCulture, [ref]$cpuParsed)) {
                    $cpuValues += $cpuParsed
                }
            }

            if ($row.mem_usage_raw) {
                $memPart = ($row.mem_usage_raw -split "/")[0].Trim()
                if ($memPart -match "([0-9.]+)\s*([A-Za-z]+)") {
                    $val = [double]::Parse($Matches[1], [System.Globalization.CultureInfo]::InvariantCulture)
                    $unit = $Matches[2].ToLower()
                    if ($unit.Contains("k")) {
                        $memValuesMib += ($val / 1024.0)
                    } elseif ($unit.Contains("g")) {
                        $memValuesMib += ($val * 1024.0)
                    } else {
                        $memValuesMib += $val
                    }
                }
            }
        }

        function Get-Percentile($list, [double]$p) {
            if (-not $list -or $list.Count -eq 0) { return 0.0 }
            $sorted = $list | Sort-Object
            $k = ($sorted.Count - 1) * ($p / 100.0)
            $f = [Math]::Floor($k)
            $c = [Math]::Min($f + 1, $sorted.Count - 1)
            $d = $k - $f
            return $sorted[$f] + ($sorted[$c] - $sorted[$f]) * $d
        }

        $sampleCount = $cpuValues.Count
        if ($sampleCount -gt 0) {
            $cpuAvg = ($cpuValues | Measure-Object -Average).Average
            $cpuMax = ($cpuValues | Measure-Object -Maximum).Maximum
            $cpuP95 = Get-Percentile $cpuValues 95

            $memAvg = ($memValuesMib | Measure-Object -Average).Average
            $memMax = ($memValuesMib | Measure-Object -Maximum).Maximum
            $memP95 = Get-Percentile $memValuesMib 95

            Write-Host ("Total de Muestras Colectadas: {0}" -f $sampleCount) -ForegroundColor White
            Write-Host "--------------------------------------------------------------------------------" -ForegroundColor DarkGray
            Write-Host ("Consumo de CPU:       Promedio: {0:N2}% | p95: {1:N2}% | Max: {2:N2}% (Criterio: <= 50.0%)" -f $cpuAvg, $cpuP95, $cpuMax) -ForegroundColor White
            Write-Host ("Memoria Residente:    Promedio: {0:N2} MiB | p95: {1:N2} MiB | Max: {2:N2} MiB (Criterio: <= 150 MiB)" -f $memAvg, $memP95, $memMax) -ForegroundColor White
            Write-Host "--------------------------------------------------------------------------------" -ForegroundColor DarkGray

            $cpuStatus = if ($cpuP95 -le 50.0) { "[ APROBADO ]" } else { "[ RECHAZADO ]" }
            $memStatus = if ($memP95 -le 150.0) { "[ APROBADO ]" } else { "[ RECHAZADO ]" }
            Write-Host "Evaluación de Criterios: CPU $cpuStatus | Memoria RSS $memStatus" -ForegroundColor $(if ($cpuP95 -le 50.0 -and $memP95 -le 150.0) { "Green" } else { "Yellow" })
        } else {
            Write-Warning "No se pudieron calcular métricas numéricas a partir de las filas en $csvOutput."
        }
    } else {
        Write-Warning "No se encontraron muestras en $csvOutput."
    }
}

Write-Host "================================================================================" -ForegroundColor Cyan
Write-Host "[✓] Telemetría exportada a: $csvOutput" -ForegroundColor Green
if ($k6ExitCode -ne 0) {
    Write-Warning "k6 finalizó con código de salida: $k6ExitCode"
    exit $k6ExitCode
}
Write-Host "[✓] Benchmark finalizado exitosamente.`n" -ForegroundColor Green
