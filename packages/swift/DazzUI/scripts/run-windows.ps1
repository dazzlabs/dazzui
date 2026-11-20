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
  [string]$Configuration = 'release',
  [string]$CrossSwiftUIPath = $env:CROSS_SWIFTUI_PATH,
  [string]$NativeApiSwiftPath = $env:NATIVEAPI_SWIFT_PATH
)
$ErrorActionPreference = 'Stop'
$packageRoot = Split-Path -Parent $PSScriptRoot
if (!$CrossSwiftUIPath) {
  $CrossSwiftUIPath = Join-Path $packageRoot '../../../../../libnativeapi/crossswift'
}
$env:CROSS_SWIFTUI_PATH = (Resolve-Path -LiteralPath $CrossSwiftUIPath).Path
if (!$NativeApiSwiftPath) {
  $NativeApiSwiftPath = Join-Path $env:CROSS_SWIFTUI_PATH '../nativeapi/bindings/swift'
}
$env:NATIVEAPI_SWIFT_PATH = (Resolve-Path -LiteralPath $NativeApiSwiftPath).Path
foreach ($dependency in @($env:CROSS_SWIFTUI_PATH, $env:NATIVEAPI_SWIFT_PATH)) {
  if (!(Test-Path -LiteralPath (Join-Path $dependency 'Package.swift'))) {
    throw "Missing Swift package: $dependency"
  }
}
if ($Test -or $Benchmark) {
  throw 'CrossXcodeBuild currently builds Example.xcodeproj and does not yet run XCTest targets.'
}
$swift = (Get-Command swift -ErrorAction Stop).Source
$xcodeConfig = if ($Configuration -eq 'debug') { 'Debug' } else { 'Release' }
$example = Join-Path $packageRoot 'Example'
$project = Join-Path $example 'Example.xcodeproj'
$derivedData = Join-Path $example '.build/windows'

& $swift run --package-path $env:CROSS_SWIFTUI_PATH xcodebuild `
  -project $project `
  -scheme Example `
  -configuration $xcodeConfig `
  -derivedDataPath $derivedData `
  CODE_SIGNING_ALLOWED=NO `
  build
if ($LASTEXITCODE -ne 0) { exit $LASTEXITCODE }
if ($BuildOnly) { exit 0 }

$executable = Join-Path $derivedData "Build/Products/$xcodeConfig/Example/Example.exe"
if (!(Test-Path -LiteralPath $executable)) {
  throw "Built executable not found: $executable"
}
$launchArgs = @('-design-sizing')
if ($Page) { $launchArgs += @('-page', $Page) }
if ($Theme) { $launchArgs += @('-theme', $Theme) }
if ($Smoke) {
  & $executable @launchArgs --smoke
  exit $LASTEXITCODE
}
Start-Process -FilePath $executable -ArgumentList $launchArgs -WorkingDirectory (Split-Path -Parent $executable) -WindowStyle Hidden | Out-Null
