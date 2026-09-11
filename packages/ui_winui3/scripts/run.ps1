param(
    [ValidateSet('Debug', 'Release')][string]$Configuration = 'Debug',
    [ValidateSet('x64', 'ARM64')][string]$Platform = 'x64',
    [switch]$NoBuild,
    [switch]$SmokeTest
)

$ErrorActionPreference = 'Stop'
$packageRoot = Split-Path $PSScriptRoot -Parent
$projectPath = Join-Path $packageRoot 'example/Playground.csproj'
if (-not $NoBuild) {
    & dotnet build $projectPath -c $Configuration "-p:Platform=$Platform"
    if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
}

$rid = if ($Platform -eq 'ARM64') { 'win-arm64' } else { 'win-x64' }
$executable = Join-Path $packageRoot "example/bin/$Platform/$Configuration/net8.0-windows10.0.19041.0/$rid/Playground.exe"
if (-not (Test-Path -LiteralPath $executable)) { throw "Build output not found: $executable" }

if ($SmokeTest) {
    # A separate output directory on each run avoids reading stale results.
    $previousOutput = $env:DAZZUI_SMOKE_OUTPUT
    $outputDirectory = Join-Path $packageRoot ('artifacts/smoke-' + [Guid]::NewGuid().ToString('N'))
    $env:DAZZUI_SMOKE_OUTPUT = $outputDirectory
    try {
        $appProcess = Start-Process -FilePath $executable -ArgumentList '--smoke-test' -WindowStyle Hidden -PassThru
        $deadline = (Get-Date).AddMinutes(15)
        $lastProgress = ''
        while (-not $appProcess.WaitForExit(1000)) {
            if ((Get-Date) -gt $deadline) { Stop-Process -Id $appProcess.Id; throw 'WinUI smoke test timed out after 15 minutes.' }
            $progressPath = Join-Path $outputDirectory 'progress.txt'
            if (Test-Path -LiteralPath $progressPath) {
                $progressText = Get-Content -LiteralPath $progressPath -Raw -Encoding UTF8
                if ($progressText -ne $lastProgress) { Write-Output $progressText; $lastProgress = $progressText }
            }
        }
        $appProcess.Refresh()
        $resultPath = Join-Path $outputDirectory 'result.txt'
        if (Test-Path -LiteralPath $resultPath) { Get-Content -LiteralPath $resultPath -Encoding UTF8 }
        if ($appProcess.ExitCode -ne 0) { throw "WinUI smoke test failed. See $outputDirectory" }
        if (-not (Test-Path -LiteralPath $resultPath)) { throw "Smoke test produced no result: $outputDirectory" }
        if ((Get-Command node -ErrorAction SilentlyContinue) -and (Test-Path (Join-Path $packageRoot '../ui_swiftui/Example'))) {
            & node (Join-Path $PSScriptRoot 'sync-stories.mjs') --check (Join-Path $outputDirectory 'stories.json')
            if ($LASTEXITCODE -ne 0) { throw 'The runtime story registry does not match the native playground catalog.' }
        }
        Write-Output "Screenshots and results: $outputDirectory"
    }
    finally { $env:DAZZUI_SMOKE_OUTPUT = $previousOutput }
} else {
    # This is the interactive playground the caller requested to open.
    Start-Process -FilePath $executable
}
