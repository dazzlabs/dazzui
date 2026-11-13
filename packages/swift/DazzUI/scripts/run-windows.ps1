param(
  [switch]$BuildOnly,
  [switch]$Test,
  [switch]$Smoke,
  [switch]$Benchmark,
  [string]$BenchmarkOutput,
  [ValidateRange(2, 10000)]
  [int]$BenchmarkSamples = 120,
  [string]$Page,
  [string]$Theme,
  [ValidateSet('debug', 'release')]
  [string]$Configuration,
  [string]$SwiftUIPortPath = $env:SWIFTUI_PORT_PATH,
  [string]$NativeApiSwiftPath = $env:NATIVEAPI_SWIFT_PATH
)
$ErrorActionPreference = 'Stop'
$packageRoot = Split-Path -Parent $PSScriptRoot
if (!$SwiftUIPortPath) {
  $SwiftUIPortPath = Join-Path $packageRoot '../../../../../libnativeapi/swiftui-port'
}
$env:SWIFTUI_PORT_PATH = (Resolve-Path -LiteralPath $SwiftUIPortPath).Path
if (!$NativeApiSwiftPath) {
  $NativeApiSwiftPath = Join-Path $env:SWIFTUI_PORT_PATH '../nativeapi/bindings/swift'
}
$env:NATIVEAPI_SWIFT_PATH = (Resolve-Path -LiteralPath $NativeApiSwiftPath).Path
foreach ($dependency in @($env:SWIFTUI_PORT_PATH, $env:NATIVEAPI_SWIFT_PATH)) {
  if (!(Test-Path -LiteralPath (Join-Path $dependency 'Package.swift'))) {
    throw "Missing Swift package: $dependency"
  }
}
$prepare = Join-Path $env:NATIVEAPI_SWIFT_PATH 'scripts/prepare-windows.ps1'
if (Test-Path -LiteralPath $prepare) {
  & $prepare
  if ($LASTEXITCODE -ne 0) { throw 'NativeAPI source preparation failed.' }
}
$swift = (Get-Command swift -ErrorAction Stop).Source
if (!$Configuration) {
  if ($Test) { $Configuration = 'debug' } else { $Configuration = 'release' }
}
# Each package needs its own cache: SwiftPM's native builder can otherwise
# reuse the other root package's test plan and silently run the wrong suite.
$common = @('--build-system', 'native', '--configuration', $Configuration, '--scratch-path', (Join-Path $packageRoot '.build'))
$example = Join-Path $packageRoot 'Example'
$galleryCommon = @('--build-system', 'native', '--configuration', $Configuration, '--scratch-path', (Join-Path $example '.build'))
if ($Benchmark) {
  if (!$BenchmarkOutput) {
    $BenchmarkOutput = Join-Path $example ".build/render-benchmark-$Configuration.json"
  }
  $previousBenchmarkOutput = $env:SWIFTUI_PORT_BENCHMARK_PATH
  $previousBenchmarkSamples = $env:SWIFTUI_PORT_BENCHMARK_SAMPLES
  try {
    $env:SWIFTUI_PORT_BENCHMARK_PATH = [IO.Path]::GetFullPath($BenchmarkOutput)
    $env:SWIFTUI_PORT_BENCHMARK_SAMPLES = "$BenchmarkSamples"
    & $swift test @galleryCommon --package-path $example --filter testRenderingBenchmark
    $benchmarkExit = $LASTEXITCODE
    if ($benchmarkExit -eq 0) {
      $samples = Get-Content -Raw -LiteralPath $BenchmarkOutput | ConvertFrom-Json
      $summary = foreach ($group in ($samples | Where-Object phase -eq 'steady' | Group-Object operation)) {
        $times = @($group.Group.total | Sort-Object)
        [pscustomobject]@{
          operation = $group.Name
          samples = $times.Count
          mean_ms = [Math]::Round(($times | Measure-Object -Average).Average, 2)
          p50_ms = [Math]::Round($times[[Math]::Ceiling($times.Count * 0.50) - 1], 2)
          p95_ms = [Math]::Round($times[[Math]::Ceiling($times.Count * 0.95) - 1], 2)
          p99_ms = [Math]::Round($times[[Math]::Ceiling($times.Count * 0.99) - 1], 2)
          max_ms = [Math]::Round($times[-1], 2)
          within_60fps_percent = [Math]::Round(100 * @($times | Where-Object { $_ -le (1000 / 60) }).Count / $times.Count, 1)
        }
      }
      $summary | Format-Table -AutoSize
      $summary | ConvertTo-Json | Set-Content -LiteralPath ($BenchmarkOutput + '.summary.json') -Encoding UTF8
    }
  } finally {
    $env:SWIFTUI_PORT_BENCHMARK_PATH = $previousBenchmarkOutput
    $env:SWIFTUI_PORT_BENCHMARK_SAMPLES = $previousBenchmarkSamples
  }
  exit $benchmarkExit
}
if ($Test) {
  & $swift test @common --package-path $packageRoot
  if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
  & $swift test @galleryCommon --package-path $example
  exit $LASTEXITCODE
}
& $swift build @galleryCommon --package-path $example --product Example
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
if ($BuildOnly) { exit 0 }
$bin = & $swift build @galleryCommon --package-path $example --show-bin-path
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
$executable = Join-Path ($bin | Select-Object -Last 1) 'Example.exe'
$launchArgs = @('-design-sizing')
if ($Page) { $launchArgs += @('-page', $Page) }
if ($Theme) { $launchArgs += @('-theme', $Theme) }
if ($Smoke) {
  & $executable @launchArgs --smoke
  exit $LASTEXITCODE
}
Start-Process -FilePath $executable -ArgumentList $launchArgs -WorkingDirectory $example -WindowStyle Hidden | Out-Null
