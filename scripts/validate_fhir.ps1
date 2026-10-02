# ==============================================================================
# MedSys-FHIR: validate_fhir.ps1
# Script de invocación del validador oficial HL7 FHIR (org.hl7.fhir.validator-cli)
# y parseador de salida que contabiliza incidencias por severidad.
#
# Resolución de hallazgo AUD-007 — Capítulo III, Sección 3.6.2
#
# Uso:
#   .\scripts\validate_fhir.ps1
#   .\scripts\validate_fhir.ps1 -OutputDir "output" -ValidatorJar "validator_cli.jar"
#   .\scripts\validate_fhir.ps1 -OutputDir "output" -ValidatorJar "validator_cli.jar" -Verbose
#
# Prerequisitos:
#   1. Java 11+ en PATH del sistema
#   2. validator_cli.jar descargado desde:
#      https://github.com/hapifhir/org.hl7.fhir.core/releases/latest
#   3. Archivos JSON en el directorio output/ generados por export_fhir_samples.py
# ==============================================================================

param(
    [string]$OutputDir    = "output",
    [string]$ValidatorJar = "validator_cli.jar",
    [string]$FhirVersion  = "4.0.1",
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"
$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptRoot

# Resolver rutas absolutas
$OutputPath    = Join-Path $ProjectRoot $OutputDir
$ValidatorPath = Join-Path $ProjectRoot $ValidatorJar

Write-Host ("=" * 60)
Write-Host "MedSys-FHIR — VALIDADOR HL7 FHIR R4 (OFFLINE)"
Write-Host ("=" * 60)
Write-Host "[*] Directorio de entrada : $OutputPath"
Write-Host "[*] Validador JAR         : $ValidatorPath"
Write-Host "[*] Version FHIR          : $FhirVersion"
Write-Host ("-" * 60)

# Verificar prereqs
if (-not (Test-Path $OutputPath)) {
    Write-Error "[ERROR] El directorio de salida no existe: $OutputPath"
    exit 1
}

$JsonFiles = Get-ChildItem -Path $OutputPath -Filter "*.json" | Select-Object -ExpandProperty FullName
if ($JsonFiles.Count -eq 0) {
    Write-Error "[ERROR] No se encontraron archivos JSON en: $OutputPath"
    Write-Host "        Ejecute primero: python scripts/export_fhir_samples.py --all"
    exit 1
}

Write-Host "[*] Archivos JSON a validar: $($JsonFiles.Count)"

if (-not (Test-Path $ValidatorPath)) {
    Write-Warning "[WARN] No se localizó validator_cli.jar en: $ValidatorPath"
    Write-Host ""
    Write-Host "Para descargarlo ejecute (PowerShell):"
    Write-Host "  Invoke-WebRequest -Uri 'https://github.com/hapifhir/org.hl7.fhir.core/releases/latest/download/validator_cli.jar' -OutFile validator_cli.jar"
    Write-Host ""
    Write-Host "Comando de validacion manual una vez descargado:"
    Write-Host "  java -jar $ValidatorPath $OutputPath\*.json -version $FhirVersion -tx n/a"
    exit 1
}

try {
    $null = & java -version 2>&1
} catch {
    Write-Error "[ERROR] Java no disponible en el PATH. Instale Java 11+ y reintente."
    exit 1
}

# Construir comando con lista explícita de archivos (Windows no expande globs en Java)
$StartTime = Get-Date
Write-Host ("[*] Iniciando validacion: $($StartTime.ToString('yyyy-MM-dd HH:mm:ss'))")
Write-Host ("-" * 60)

$CmdArgs = @("-jar", $ValidatorPath) + $JsonFiles + @("-version", $FhirVersion, "-tx", "n/a")

$Process = Start-Process -FilePath "java" 
    -ArgumentList $CmdArgs 
    -NoNewWindow 
    -PassThru 
    -RedirectStandardOutput "$env:TEMP\hl7_stdout.txt" 
    -RedirectStandardError  "$env:TEMP\hl7_stderr.txt" 
    -Wait

$Stdout = if (Test-Path "$env:TEMP\hl7_stdout.txt") { Get-Content "$env:TEMP\hl7_stdout.txt" -Raw } else { "" }
$Stderr = if (Test-Path "$env:TEMP\hl7_stderr.txt") { Get-Content "$env:TEMP\hl7_stderr.txt" -Raw } else { "" }
$AllOutput = ($Stdout + "
" + $Stderr).Split("
")

# Contadores de severidad
$SeverityCounts = @{
    "Fatal"       = 0
    "Error"       = 0
    "Warning"     = 0
    "Information" = 0
}

foreach ($Line in $AllOutput) {
    if ($Verbose) { Write-Host $Line }
    $LineLower = $Line.ToLower()
    if     ($LineLower -match "\bfatal\b")       { $SeverityCounts["Fatal"]++       }
    elseif ($LineLower -match "\berror\b")        { $SeverityCounts["Error"]++       }
    elseif ($LineLower -match "\bwarning\b")      { $SeverityCounts["Warning"]++     }
    elseif ($LineLower -match "\binformation\b")  { $SeverityCounts["Information"]++ }
}

$EndTime  = Get-Date
$Elapsed  = ($EndTime - $StartTime).TotalSeconds

# Resumen
Write-Host ""
Write-Host ("=" * 60)
Write-Host "RESUMEN DE INCIDENCIAS POR SEVERIDAD (HL7 validator-cli)"
Write-Host ("=" * 60)
Write-Host ("  Fatal       : {0,6}" -f $SeverityCounts["Fatal"])
Write-Host ("  Error       : {0,6}" -f $SeverityCounts["Error"])
Write-Host ("  Warning     : {0,6}" -f $SeverityCounts["Warning"])
Write-Host ("  Information : {0,6}" -f $SeverityCounts["Information"])
Write-Host ("-" * 60)
Write-Host ("  Archivos validados : {0,6}" -f $JsonFiles.Count)
Write-Host ("  Tiempo transcurrido: {0:F2} s" -f $Elapsed)
Write-Host ("=" * 60)

$HasCritical = ($SeverityCounts["Fatal"] + $SeverityCounts["Error"]) -gt 0

if ($HasCritical) {
    Write-Warning "[FAIL] Se detectaron incidencias Fatal o Error."
    Write-Host   "       La hipotesis de conformidad sintactica NO se sostiene."
    exit 1
} else {
    Write-Host "[OK] Sin incidencias Fatal ni Error."
    Write-Host "     Conformidad sintactica 100% verificada (Objetivo Especifico 3)."
    exit 0
}
