<#
.SYNOPSIS
    Reinicia y re-ejecuta el script SQL sintético desde cero eliminando el volumen.
#>

[CmdletBinding()]
param()

$ScriptDir = Split-Path -Parent $MyInvocation.MyCommand.Path
$ComposeFile = Join-Path $ScriptDir "docker-compose.yml"

Write-Host "Reiniciando base de datos y eliminando volúmenes existentes..." -ForegroundColor Yellow
& docker compose -f $ComposeFile down -v

Write-Host "Levantando base de datos fresca y ejecutando schema_legado_simulado_nom004.sql..." -ForegroundColor Yellow
& docker compose -f $ComposeFile up -d

Write-Host "✔ Base de datos reiniciada con éxito." -ForegroundColor Green
