# ==============================================================================
# MedSys-FHIR: validate_fhir.ps1
# Script de invocación del validador oficial HL7 FHIR (org.hl7.fhir.validator-cli)
# y parseador de salida que contabiliza incidencias por severidad.
#
# Resolución de hallazgo AUD-007 -- Capítulo III, Sección 3.6.2
# Soporta validación por lotes para el censo de 16,250 recursos JSON.
# ==============================================================================

param(
    [string]$OutputDir    = "output",
    [string]$ValidatorJar = "validator_cli.jar",
    [string]$FhirVersion  = "4.0.1",
    [int]$BatchSize       = 200,
    [switch]$Verbose
)

$ErrorActionPreference = "Stop"
$ScriptRoot = Split-Path -Parent $MyInvocation.MyCommand.Path
$ProjectRoot = Split-Path -Parent $ScriptRoot

# Resolver rutas absolutas
$OutputPath    = Join-Path $ProjectRoot $OutputDir
$ValidatorPath = Join-Path $ProjectRoot $ValidatorJar

Write-Host "============================================================"
Write-Host "MedSys-FHIR -- VALIDADOR HL7 FHIR R4 (OFFLINE)"
Write-Host "============================================================"
Write-Host ("Directorio de entrada : {0}" -f $OutputPath)
Write-Host ("Validador JAR         : {0}" -f $ValidatorPath)
Write-Host ("Version FHIR          : {0}" -f $FhirVersion)
Write-Host ("Tamano de lote        : {0} archivos" -f $BatchSize)
Write-Host "------------------------------------------------------------"

# Verificar prerequisitos
if (-not (Test-Path $OutputPath)) {
    Write-Error "El directorio de salida no existe: $OutputPath"
    exit 1
}

$JsonFiles = Get-ChildItem -Path $OutputPath -Filter "*.json" | Select-Object -ExpandProperty FullName
if ($JsonFiles.Count -eq 0) {
    Write-Error "No se encontraron archivos JSON en: $OutputPath"
    Write-Host "Ejecute primero: python scripts/export_fhir_samples.py --all"
    exit 1
}

Write-Host ("Archivos JSON a validar: {0}" -f $JsonFiles.Count)

if (-not (Test-Path $ValidatorPath)) {
    Write-Warning "No se localizo validator_cli.jar en la ruta esperada."
    Write-Host "Para descargarlo ejecute (PowerShell):"
    Write-Host "Invoke-WebRequest -Uri 'https://github.com/hapifhir/org.hl7.fhir.core/releases/latest/download/validator_cli.jar' -OutFile validator_cli.jar"
    Write-Host "Comando de validacion manual una vez descargado:"
    Write-Host ("java -jar {0} {1}\*.json -version {2} -tx n/a" -f $ValidatorPath, $OutputPath, $FhirVersion)
    exit 1
}

try {
    $null = & java -version 2>&1
} catch {
    Write-Error "Java no disponible en el PATH. Instale Java 11+ y reintente."
    exit 1
}

# Iniciar validación por lotes
$StartTime = Get-Date
Write-Host ("Iniciando validacion por lotes: {0}" -f $StartTime.ToString("yyyy-MM-dd HH:mm:ss"))
Write-Host "------------------------------------------------------------"

$SeverityCounts = @{
    "Fatal"       = 0
    "Error"       = 0
    "Warning"     = 0
    "Information" = 0
}

$NumBatches = [Math]::Ceiling($JsonFiles.Count / $BatchSize)

for ($b = 0; $b -lt $NumBatches; $b++) {
    $startIdx = $b * $BatchSize
    $count = [Math]::Min($BatchSize, ($JsonFiles.Count - $startIdx))
    $batch = $JsonFiles[$startIdx..($startIdx + $count - 1)]

    $batchMsg = ("Procesando lote {0}/{1} ({2} archivos)..." -f ($b + 1), $NumBatches, $count)
    Write-Host $batchMsg

    $stdoutFile = Join-Path $env:TEMP ("hl7_stdout_{0}.txt" -f $b)
    $stderrFile = Join-Path $env:TEMP ("hl7_stderr_{0}.txt" -f $b)

    $CmdArgs = @("-jar", $ValidatorPath) + $batch + @("-version", $FhirVersion, "-tx", "n/a")

    $Process = Start-Process -FilePath "java" `
        -ArgumentList $CmdArgs `
        -NoNewWindow `
        -PassThru `
        -RedirectStandardOutput $stdoutFile `
        -RedirectStandardError  $stderrFile `
        -Wait

    $Stdout = if (Test-Path $stdoutFile) { Get-Content $stdoutFile -Raw } else { "" }
    $Stderr = if (Test-Path $stderrFile) { Get-Content $stderrFile -Raw } else { "" }
    $AllOutput = ($Stdout + "`n" + $Stderr).Split("`n")

    foreach ($Line in $AllOutput) {
        if ($Verbose) { Write-Host $Line }
        $LineLower = $Line.ToLower()
        if     ($LineLower -match "\bfatal\b")       { $SeverityCounts["Fatal"]++       }
        elseif ($LineLower -match "\berror\b")        { $SeverityCounts["Error"]++       }
        elseif ($LineLower -match "\bwarning\b")      { $SeverityCounts["Warning"]++     }
        elseif ($LineLower -match "\binformation\b")  { $SeverityCounts["Information"]++ }
    }

    if (Test-Path $stdoutFile) { Remove-Item $stdoutFile -Force -ErrorAction SilentlyContinue }
    if (Test-Path $stderrFile) { Remove-Item $stderrFile -Force -ErrorAction SilentlyContinue }
}

$EndTime  = Get-Date
$Elapsed  = ($EndTime - $StartTime).TotalSeconds

# Resumen
Write-Host ""
Write-Host "============================================================"
Write-Host "RESUMEN DE INCIDENCIAS POR SEVERIDAD (HL7 validator-cli)"
Write-Host "============================================================"
Write-Host ("  Fatal       : {0,6}" -f $SeverityCounts["Fatal"])
Write-Host ("  Error       : {0,6}" -f $SeverityCounts["Error"])
Write-Host ("  Warning     : {0,6}" -f $SeverityCounts["Warning"])
Write-Host ("  Information : {0,6}" -f $SeverityCounts["Information"])
Write-Host "------------------------------------------------------------"
Write-Host ("  Archivos validados : {0,6}" -f $JsonFiles.Count)
Write-Host ("  Tiempo transcurrido: {0:F2} s" -f $Elapsed)
Write-Host "============================================================"

$HasCritical = ($SeverityCounts["Fatal"] + $SeverityCounts["Error"]) -gt 0

if ($HasCritical) {
    Write-Warning "Se detectaron incidencias Fatal o Error. La hipotesis de conformidad NO se sostiene."
    exit 1
} else {
    Write-Host "[OK] Sin incidencias Fatal ni Error. Conformidad sintactica 100% verificada."
    exit 0
}
