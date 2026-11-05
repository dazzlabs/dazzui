# SSH runs in session 0. Start the gallery in the signed-in user's desktop.
param([string]$Exe = '', [switch]$DesktopSession, [string]$ResultPath = '')
$ErrorActionPreference = 'Stop'
[Console]::OutputEncoding = [Text.Encoding]::UTF8
$package = (Resolve-Path (Join-Path $PSScriptRoot '..\..')).Path
if (-not $Exe) { $Exe = Join-Path $package 'target\debug\dazzui-nsh-example.exe' }
$Exe = (Resolve-Path $Exe).Path
# Window enumeration must run in the desktop session too: session 0 cannot
# see another session's HWNDs, even when Get-Process can see its process.
if ($DesktopSession) {
  try {
    $app = Get-Process dazzui-nsh-example -ErrorAction SilentlyContinue |
      Where-Object { $_.Path -eq $Exe -and $_.SessionId -eq (Get-Process -Id $PID).SessionId } |
      Select-Object -First 1
    if (-not $app) {
      if (-not $env:RUST_LOG) { $env:RUST_LOG = 'info' }
      $app = Start-Process -FilePath $Exe -WorkingDirectory $package -PassThru `
        -RedirectStandardOutput (Join-Path $package 'target\desktop.stdout.log') `
        -RedirectStandardError (Join-Path $package 'target\desktop.stderr.log')
    }
    $deadline = (Get-Date).AddSeconds(40)
    do {
      Start-Sleep -Milliseconds 500
      $app.Refresh()
      if ($app.HasExited) { throw "Gallery exited with code $($app.ExitCode)." }
    } until ($app.MainWindowHandle -ne 0 -or (Get-Date) -gt $deadline)
    if ($app.MainWindowHandle -eq 0) { throw 'Gallery did not create a desktop window.' }
    $app | Select-Object Id, SessionId, MainWindowHandle, MainWindowTitle, Responding |
      ConvertTo-Json | Set-Content -Path $ResultPath -Encoding UTF8
  } catch {
    @{ Error = $_.Exception.Message } | ConvertTo-Json |
      Set-Content -Path $ResultPath -Encoding UTF8
    exit 1
  }
  exit 0
}
$user = (Get-CimInstance Win32_ComputerSystem).UserName
if (-not $user) { throw 'Sign in to the Windows desktop before launching the gallery.' }
$name = 'DazzUI-NSH-' + [Guid]::NewGuid().ToString('N')
$resultPath = Join-Path $package "target\$name.json"
$arguments = '-NoProfile -ExecutionPolicy Bypass -File "{0}" -DesktopSession -Exe "{1}" -ResultPath "{2}"' -f $PSCommandPath, $Exe, $resultPath
$action = New-ScheduledTaskAction -Execute 'powershell.exe' -Argument $arguments -WorkingDirectory $package
$principal = New-ScheduledTaskPrincipal -UserId $user -LogonType Interactive -RunLevel Limited
$settings = New-ScheduledTaskSettingsSet -ExecutionTimeLimit ([TimeSpan]::Zero) -AllowStartIfOnBatteries -DontStopIfGoingOnBatteries
try {
  Register-ScheduledTask -TaskName $name -Action $action -Principal $principal -Settings $settings | Out-Null
  Start-ScheduledTask -TaskName $name
  $deadline = (Get-Date).AddSeconds(45)
  do {
    Start-Sleep -Seconds 1
  } until ((Test-Path $resultPath) -or (Get-Date) -gt $deadline)
  if (-not (Test-Path $resultPath)) {
    $result = (Get-ScheduledTaskInfo -TaskName $name).LastTaskResult
    throw "Gallery did not create a desktop window. Scheduled task result: $result"
  }
  $result = Get-Content $resultPath -Raw | ConvertFrom-Json
  if ($result.Error) { throw $result.Error }
  $result
} finally {
  # Unregistering the one-shot launcher leaves its already-running app open.
  Unregister-ScheduledTask -TaskName $name -Confirm:$false -ErrorAction SilentlyContinue
}
