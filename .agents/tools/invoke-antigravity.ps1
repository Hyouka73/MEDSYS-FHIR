param(
    [Parameter(Mandatory=$true)][string]$PromptFile,
    [Parameter(Mandatory=$true)][string]$OutputDirectory,
    [string]$Model = 'gemini-3.8-flash-high',
    [int]$TimeoutSeconds = 600
)
$ErrorActionPreference = 'Stop'
$repoPath = (Resolve-Path (Join-Path $PSScriptRoot '../..')).Path
$contractPath = (Resolve-Path -LiteralPath $PromptFile).Path
$promptText = Get-Content -LiteralPath $contractPath -Raw
$agyPath = Join-Path $env:LOCALAPPDATA 'agy/bin/agy.exe'
if (-not (Test-Path -LiteralPath $agyPath)) { throw 'Antigravity CLI no instalado.' }
if (Test-Path -LiteralPath $OutputDirectory) { throw 'Usa un directorio nuevo para conservar evidencia anterior.' }
$outputPath = (New-Item -ItemType Directory -Path $OutputDirectory).FullName
Push-Location $repoPath
try {
    $started = [DateTime]::UtcNow.ToString('o')
    & $agyPath -p $promptText --model $Model --dangerously-skip-permissions --output-format stream-json --print-timeout "$($TimeoutSeconds)s" 1> (Join-Path $outputPath 'events.jsonl') 2> (Join-Path $outputPath 'stderr.log')
    $runExit = $LASTEXITCODE
    $lastResult = Get-Content -LiteralPath (Join-Path $outputPath 'events.jsonl') | ForEach-Object { try { $_ | ConvertFrom-Json } catch {} } | Where-Object event -EQ 'result' | Select-Object -Last 1
    if ($lastResult) { $lastResult.result | ConvertTo-Json -Depth 30 | Set-Content -LiteralPath (Join-Path $outputPath 'result.json') }
    @{ model=$Model; exit_code=$runExit; started_utc=$started; finished_utc=[DateTime]::UtcNow.ToString('o'); prompt_sha256=(Get-FileHash -LiteralPath $contractPath -Algorithm SHA256).Hash; cli_sha256=(Get-FileHash -LiteralPath $agyPath -Algorithm SHA256).Hash; permissions='--dangerously-skip-permissions'; workspace=$repoPath } | ConvertTo-Json | Set-Content -LiteralPath (Join-Path $outputPath 'execution.json')
    Write-Output "Antigravity: exit=$runExit; resultado=$outputPath"
} finally { Pop-Location }
exit $runExit
