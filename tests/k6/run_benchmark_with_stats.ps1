# ==============================================================================
# MedSys-FHIR: Pipeline de Benchmarking con Telemetría docker stats y VmRSS (PowerShell)
# Tesis: Carlos Eduardo Iglesias de la Cruz & Alexis Andrey Gálvez Roblero
# Trazabilidad: AUD-002 / CON-004 / CVD-004 (Límites cgroups 1.0 vCPU, 256MB RAM)
# ==============================================================================

param(
    [string]$BaseUrl = "http://localhost:8080",
    [string]$ContainerName = "medsys-server",
    [string]$DbContainerName = "medsys_fhir_postgres",
    [int]$SamplingIntervalSec = 5,
    [int]$MaxWaitSec = 90,
    [int]$Replicas = 1
)

$ErrorActionPreference = "Stop"

$scriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$projectRoot = (Resolve-Path "$scriptDir\..\..").Path
$composeFile = Join-Path $projectRoot "docker\docker-compose.yml"
$csvOutput = Join-Path $scriptDir "benchmark_stats.csv"
$k6Script = Join-Path $scriptDir "load_test.js"

Write-Host "`n================================================================================" -ForegroundColor Cyan
Write-Host " MedSys-FHIR -- Pipeline de Benchmarking y Telemetria Conjunta (cgroups v2)" -ForegroundColor Cyan
Write-Host (" Replicas oficiales: {0} -- Muestreo de Telemetria cada {1}s" -f $Replicas, $SamplingIntervalSec) -ForegroundColor Cyan
Write-Host "================================================================================`n" -ForegroundColor Cyan

# 1. Verificar binarios requeridos (docker y k6)
$foundDocker = Get-Command docker -ErrorAction SilentlyContinue
if (-not $foundDocker) {
    Write-Error "No se encontro el comando 'docker' en el PATH del sistema."
    exit 1
}

$k6Cmd = Get-Command k6 -ErrorAction SilentlyContinue
if (-not $k6Cmd) {
    $fallbackK6 = "C:\Program Files\k6\k6.exe"
    if (Test-Path $fallbackK6) {
        $k6Path = $fallbackK6
    } else {
        Write-Error "No se encontro el ejecutable de 'k6' en PATH ni en '$fallbackK6'."
        exit 1
    }
} else {
    $k6Path = $k6Cmd.Source
}
$dockerSource = $foundDocker.Source
Write-Host ("[OK] Binarios detectados: Docker ({0}) -- k6 ({1})" -f $dockerSource, $k6Path) -ForegroundColor Green

# 2. Desplegar servicios contenerizados
Write-Host "[*] Desplegando servicios con limites cgroups v2 (medsys-server 256M, postgres 512M)..." -ForegroundColor Yellow
& docker compose -f "$composeFile" up -d
if ($LASTEXITCODE -ne 0) {
    Write-Error "Fallo al ejecutar 'docker compose up -d'."
    exit 1
}

# 3. Esperar disponibilidad del endpoint de salud
Write-Host ("[*] Esperando disponibilidad de {0}/health (Timeout: {1}s)..." -f $BaseUrl, $MaxWaitSec) -ForegroundColor Yellow
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
        # Esperando
    }
    Start-Sleep -Seconds 2
    $elapsed += 2
    Write-Host ("    ... esperando respuesta de /health ({0}s transcurridos)" -f $elapsed) -ForegroundColor DarkGray
}

if (-not $serverReady) {
    Write-Host ("[!] ERROR: El middleware no respondio HTTP 200 en {0}/health tras {1}s." -f $BaseUrl, $MaxWaitSec) -ForegroundColor Red
    Write-Host ("[*] Volcado de logs de {0}:" -f $ContainerName) -ForegroundColor DarkYellow
    & docker compose -f "$composeFile" logs --tail 30 $ContainerName
    exit 1
}

Write-Host ("[OK] Middleware '{0}' en linea y respondiendo HTTP 200." -f $ContainerName) -ForegroundColor Green

# 4. Inicializar archivo CSV de telemetría completa
"timestamp,server_cpu,server_mem,postgres_cpu,postgres_mem,vm_rss_kb,cgroup_memory_bytes" | Out-File -FilePath $csvOutput -Encoding utf8

# Script block para el recolector en segundo plano (captura docker stats + VmRSS + cgroup)
$statsScriptBlock = {
    param($srvContainer, $dbContainer, $interval, $outFile)
    while ($true) {
        $ts = (Get-Date).ToUniversalTime().ToString("yyyy-MM-ddTHH:mm:ssZ")
        
        # 1. Telemetría de medsys-server
        $rawSrv = docker stats $srvContainer --no-stream --format "{{.CPUPerc}},{{.MemUsage}}" 2>$null
        $srvCpu = "0.00%"
        $srvMem = "0B / 256MB"
        if ($rawSrv) {
            $parts = $rawSrv.Split(",")
            if ($parts.Length -ge 2) {
                $srvCpu = $parts[0].Trim()
                $srvMem = $parts[1].Trim()
            }
        }

        # 2. Telemetría de postgres
        $rawDb = docker stats $dbContainer --no-stream --format "{{.CPUPerc}},{{.MemUsage}}" 2>$null
        $dbCpu = "0.00%"
        $dbMem = "0B / 512MB"
        if ($rawDb) {
            $parts = $rawDb.Split(",")
            if ($parts.Length -ge 2) {
                $dbCpu = $parts[0].Trim()
                $dbMem = $parts[1].Trim()
            }
        }

        # 3. VmRSS real desde /proc/1/status
        $vmRss = docker exec $srvContainer sh -c "grep VmRSS /proc/1/status 2>/dev/null" 2>$null
        if ($vmRss -match "(\d+)") {
            $vmRssVal = $Matches[1]
        } else {
            $vmRssVal = "0"
        }

        # 4. cgroups v2 memory.current
        $cgroupBytes = docker exec $srvContainer sh -c "cat /sys/fs/cgroup/memory.current 2>/dev/null" 2>$null
        if (-not $cgroupBytes) {
            $cgroupBytes = "0"
        }

        $rowText = "{0},{1},{2},{3},{4},{5},{6}" -f $ts, $srvCpu, $srvMem, $dbCpu, $dbMem, $vmRssVal, $cgroupBytes
        Add-Content -Path $outFile -Value $rowText -Encoding utf8
        Start-Sleep -Seconds $interval
    }
}

$statsJob = Start-Job -ScriptBlock $statsScriptBlock -ArgumentList $ContainerName, $DbContainerName, $SamplingIntervalSec, $csvOutput

# 5. Ejecutar la(s) réplica(s) de benchmarking con k6
$overallExitCode = 0
try {
    for ($rep = 1; $rep -le $Replicas; $rep++) {
        Write-Host "`n================================================================================" -ForegroundColor Cyan
        Write-Host (" [*] Ejecutando Replica {0} de {1} de k6 (load_test.js a 50 VUs)..." -f $rep, $Replicas) -ForegroundColor Cyan
        Write-Host "================================================================================`n" -ForegroundColor Cyan
        
        $replicaJson = Join-Path $scriptDir ("k6_replica_{0}.json" -f $rep)
        & $k6Path run --env "BASE_URL=$BaseUrl" --out "json=$replicaJson" "$k6Script"
        $code = $LASTEXITCODE
        if ($code -ne 0) {
            $overallExitCode = $code
            Write-Warning ("Replica {0} finalizo con codigo de salida no cero: {1}" -f $rep, $code)
        }
    }
} finally {
    # 6. Detener recolección de telemetría de forma garantizada
    Write-Host "`n[*] Deteniendo proceso de muestreo de telemetria..." -ForegroundColor Yellow
    Stop-Job -Job $statsJob -ErrorAction SilentlyContinue
    Remove-Job -Job $statsJob -ErrorAction SilentlyContinue
}

# 7. Consolidar y presentar estadísticas
Write-Host "`n================================================================================" -ForegroundColor Cyan
Write-Host " Consolidacion de Telemetria (docker stats + VmRSS + cgroup) -- MedSys-FHIR" -ForegroundColor Cyan
Write-Host "================================================================================" -ForegroundColor Cyan

if (Test-Path $csvOutput) {
    $rows = Import-Csv -Path $csvOutput -ErrorAction SilentlyContinue
    if ($rows -and $rows.Count -gt 0) {
        $cpuValues = @()
        $memValuesMib = @()
        $vmRssMib = @()

        foreach ($row in $rows) {
            if ($row.server_cpu) {
                $cpuClean = $row.server_cpu.Replace("%", "").Trim()
                $cpuParsed = 0.0
                if ([double]::TryParse($cpuClean, [System.Globalization.NumberStyles]::Any, [System.Globalization.CultureInfo]::InvariantCulture, [ref]$cpuParsed)) {
                    $cpuValues += $cpuParsed
                }
            }

            if ($row.server_mem) {
                $memPart = ($row.server_mem -split "/")[0].Trim()
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

            if ($row.vm_rss_kb) {
                $rssKb = 0.0
                if ([double]::TryParse($row.vm_rss_kb.Trim(), [System.Globalization.NumberStyles]::Any, [System.Globalization.CultureInfo]::InvariantCulture, [ref]$rssKb)) {
                    $vmRssMib += ($rssKb / 1024.0)
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

            $rssAvg = if ($vmRssMib.Count -gt 0) { ($vmRssMib | Measure-Object -Average).Average } else { 0.0 }
            $rssP95 = if ($vmRssMib.Count -gt 0) { Get-Percentile $vmRssMib 95 } else { 0.0 }

            Write-Host ("Total de Muestras de Telemetria: {0}" -f $sampleCount) -ForegroundColor White
            Write-Host "--------------------------------------------------------------------------------" -ForegroundColor DarkGray
            Write-Host ("Consumo CPU Servidor: Promedio: {0:N2}% -- p95: {1:N2}% -- Max: {2:N2}% (Umbral: <= 50.0%)" -f $cpuAvg, $cpuP95, $cpuMax) -ForegroundColor White
            Write-Host ("Memoria docker stats: Promedio: {0:N2} MiB -- p95: {1:N2} MiB -- Max: {2:N2} MiB (Umbral: <= 150 MiB)" -f $memAvg, $memP95, $memMax) -ForegroundColor White
            Write-Host ("Memoria VmRSS Kernel: Promedio: {0:N2} MiB -- p95: {1:N2} MiB" -f $rssAvg, $rssP95) -ForegroundColor White
            Write-Host "--------------------------------------------------------------------------------" -ForegroundColor DarkGray

            $cpuStatus = if ($cpuP95 -le 50.0) { "[ APROBADO ]" } else { "[ RECHAZADO ]" }
            $memStatus = if ($memP95 -le 150.0) { "[ APROBADO ]" } else { "[ RECHAZADO ]" }
            Write-Host ("Evaluacion de Hipotesis: CPU {0} -- Memoria {1}" -f $cpuStatus, $memStatus) -ForegroundColor $(if ($cpuP95 -le 50.0 -and $memP95 -le 150.0) { "Green" } else { "Yellow" })
        }
    }
}

Write-Host "================================================================================" -ForegroundColor Cyan
Write-Host ("[OK] Telemetria exportada a: {0}" -f $csvOutput) -ForegroundColor Green
if ($overallExitCode -ne 0) {
    Write-Warning "Una o mas replicas finalizaron con error."
    exit $overallExitCode
}
Write-Host "[OK] Pipeline de benchmarking completado con exito.`n" -ForegroundColor Green
