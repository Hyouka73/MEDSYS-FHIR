<#
.SYNOPSIS
    Detiene el contenedor PostgreSQL 16 de MedSys-FHIR.
#>

[CmdletBinding()]
param()

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ComposeFile = Join-Path $ScriptDir "docker-compose.yml"

Write-Host "Deteniendo contenedor PostgreSQL 16..." -ForegroundColor Yellow
& docker compose -f $ComposeFile down

if ($LASTEXITCODE -eq 0) {
    Write-Host "✔ Contenedor PostgreSQL 16 detenido correctamente." -ForegroundColor Green
} else {
    Write-Host "Error al detener el contenedor." -ForegroundColor Red
}
