# Runs a program in the signed-in user's desktop session and waits for it to
# exit, printing what it wrote to $Out. A WinUI window started from an SSH
# session (session 0) has no desktop and exits silently; a scheduled task
# with an interactive logon runs it where the signed-in user sees it.
param(
  [Parameter(Mandatory)][string]$Exe,
  [string]$Arguments = '',
  [Parameter(Mandatory)][string]$Out,
  [int]$TimeoutSeconds = 600,
  [string]$Environment = ''
)
[Console]::OutputEncoding = [Text.Encoding]::UTF8
Remove-Item -Recurse -Force $Out -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force $Out | Out-Null
$wrapper = Join-Path $Out 'launch.cmd'
$lines = @('@echo off', "set DAZZUI_SMOKE_OUTPUT=$Out")
foreach ($pair in ($Environment -split ';' | Where-Object { $_ })) { $lines += "set $pair" }
$lines += "`"$Exe`" $Arguments"
$lines += "echo %ERRORLEVEL% > `"$Out\exit.txt`""
Set-Content -Path $wrapper -Value $lines -Encoding ASCII
$name = 'DazzUIDesktopRun'
Unregister-ScheduledTask -TaskName $name -Confirm:$false -ErrorAction SilentlyContinue
$action = New-ScheduledTaskAction -Execute 'cmd.exe' -Argument "/c `"$wrapper`""
$ErrorActionPreference = 'Stop'
$user = (Get-CimInstance Win32_ComputerSystem).UserName
if (-not $user) { throw 'Nobody is signed in on this machine' }
$principal = New-ScheduledTaskPrincipal -UserId $user -LogonType Interactive
Register-ScheduledTask -TaskName $name -Action $action -Principal $principal | Out-Null
Start-ScheduledTask -TaskName $name
$ErrorActionPreference = 'Continue'
$deadline = (Get-Date).AddSeconds($TimeoutSeconds)
while (-not (Test-Path "$Out\exit.txt") -and (Get-Date) -lt $deadline) { Start-Sleep 2 }
Unregister-ScheduledTask -TaskName $name -Confirm:$false -ErrorAction SilentlyContinue
if (-not (Test-Path "$Out\exit.txt")) { "TIMED OUT"; Get-Process ([IO.Path]::GetFileNameWithoutExtension($Exe)) -ErrorAction SilentlyContinue | Stop-Process -Force }
else { "exit: $((Get-Content "$Out\exit.txt").Trim())" }
foreach ($f in 'result.txt', 'crash.txt', 'failures.txt', 'progress.txt') {
  $path = Join-Path $Out $f
  if (Test-Path $path) { "--- $f"; Get-Content $path -TotalCount 40 }
}
Get-ChildItem $Out -File | Select-Object Name, Length | Format-Table | Out-String
