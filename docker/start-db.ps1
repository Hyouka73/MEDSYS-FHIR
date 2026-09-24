<#
.SYNOPSIS
    Inicia el contenedor PostgreSQL 16 con el esquema legado sintético NOM-004.
.DESCRIPTION
    Script de automatización para Windows PowerShell en MedSys-FHIR.
    Verifica la disponibilidad del motor Docker, inicia el servicio mediante
    Docker Compose y valida el estado de salud (healthcheck) de la base de datos.
#>

[CmdletBinding()]
param()

$ErrorActionPreference = "Stop"

Write-Host "==========================================================" -ForegroundColor Cyan
Write-Host "  MedSys-FHIR — Inicialización de PostgreSQL 16 (NOM-004)" -ForegroundColor Cyan
Write-Host "==========================================================" -ForegroundColor Cyan

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ComposeFile = Join-Path $ScriptDir "docker-compose.yml"

if (-not (Test-Path $ComposeFile)) {
    Write-Error "No se encontró el archivo $ComposeFile"
    exit 1
}

# 1. Verificar comando Docker
if (-not (Get-Command docker -ErrorAction SilentlyContinue)) {
    Write-Host "[ERROR] El binario 'docker' no está en el PATH." -ForegroundColor Red
    Write-Host "Por favor instala Docker Desktop y verifica que esté en el PATH." -ForegroundColor Yellow
    exit 1
}

# 2. Verificar que Docker daemon esté en ejecución
Write-Host "[1/3] Verificando estado del motor Docker Desktop..." -ForegroundColor Yellow
$dockerInfo = & docker info 2>&1
if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Docker Desktop no parece estar en ejecución." -ForegroundColor Red
    Write-Host "Inicia la aplicación Docker Desktop en Windows y vuelve a ejecutar este script." -ForegroundColor Yellow
    exit 1
}
Write-Host "  ✔ Motor Docker activo y accesible." -ForegroundColor Green

# 3. Levantar servicio PostgreSQL
Write-Host "[2/3] Levantando contenedor con Docker Compose..." -ForegroundColor Yellow
& docker compose -f $ComposeFile up -d

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Falló la ejecución de 'docker compose up -d'." -ForegroundColor Red
    exit $LASTEXITCODE
}

# 4. Esperar y verificar Healthcheck
Write-Host "[3/3] Esperando que PostgreSQL complete la inicialización del esquema..." -ForegroundColor Yellow
$MaxRetries = 15
$Retry = 0
$ContainerName = "medsys_fhir_postgres"
$Healthy = $false

while ($Retry -lt $MaxRetries) {
    Start-Sleep -Seconds 2
    $Status = (& docker inspect --format '{{if .State.Health}}{{.State.Health.Status}}{{else}}{{.State.Status}}{{end}}' $ContainerName 2>$null)
    if ($Status -eq "healthy") {
        $Healthy = $true
        break
    }
    $Retry++
    Write-Host "  ... esperando verificación de salud ($Retry/$MaxRetries) - Estado actual: $Status" -ForegroundColor DarkGray
}

Write-Host ""
if ($Healthy) {
    Write-Host "==========================================================" -ForegroundColor Green
    Write-Host "  ✔ Base de datos PostgreSQL 16 lista y saludable!" -ForegroundColor Green
    Write-Host "==========================================================" -ForegroundColor Green
} else {
    Write-Host "  ⚠ El contenedor está en ejecución, pero aún no reporta 'healthy'." -ForegroundColor Yellow
    Write-Host "    Puedes verificar los logs con: docker logs $ContainerName" -ForegroundColor Yellow
}

Write-Host ""
Write-Host "Configuración de Acceso:" -ForegroundColor Cyan
Write-Host "  Host:         localhost:5432"
Write-Host "  Base Datos:   medsys_legacy"
Write-Host "  Usuario:      medsys_user"
Write-Host "  Contraseña:   medsys_secure_pass_2026"
Write-Host "  DATABASE_URL: postgres://medsys_user:medsys_secure_pass_2026@localhost:5432/medsys_legacy"
Write-Host ""
Write-Host "Tablas NOM-004 disponibles:" -ForegroundColor Cyan
Write-Host "  - tbl_pacientes (2 registros de prueba cargados)"
Write-Host "  - tbl_consultas (2 registros de prueba cargados)"
Write-Host "  - tbl_signos_vitales (2 registros de prueba cargados)"
Write-Host "  - tbl_diagnosticos (2 registros de prueba cargados)"
Write-Host ""
