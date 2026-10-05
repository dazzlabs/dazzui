#!/usr/bin/env bash
# Runs the WinUI host's smoke test on a Windows machine over SSH, from a Mac
# or Linux one: copies the compiler and DazzUI over, builds the Rust core and
# the host there, clicks through the generated page in the signed-in desktop
# (packages/csharp/DazzUI/scripts/desktop.ps1), and copies the result and the
# screenshots back to --out (default ./smoke-output).
#
# The same, on the Windows machine itself, from tools/dazzui-compiler:
#   cargo build -p settings-example
#   dotnet build examples/settings_example/windows/SettingsExample.sln
#   SettingsExample.exe --smoke-test   (in the signed-in desktop session)
#
# DAZZUI_WINDOWS_HOST names the machine (default windows-laptop) and
# DAZZUI_WINDOWS_DIR the repository there.
set -euo pipefail

out="smoke-output"
[ "${1:-}" = "--out" ] && out="$2"

host="${DAZZUI_WINDOWS_HOST:-windows-laptop}"
remote="${DAZZUI_WINDOWS_DIR:-C:/Users/Lijy91/Projects/dazzui}"
repo="$(git rev-parse --show-toplevel)"
win() { echo "${1//\//\\}"; }

# COPYFILE_DISABLE: macOS tar would add `._name` resource forks, which the
# compiler would find among the crate's .rs files. bin-conformance: what the
# conformance collector builds into, whose copy of Generic.xaml a plain build
# would pick up as a second page.
COPYFILE_DISABLE=1 tar -C "$repo" -czf - --exclude target --exclude .build --exclude .swiftpm \
  --exclude bin --exclude bin-conformance --exclude obj --exclude artifacts --exclude smoke-output \
  tools/dazzui-compiler packages/csharp/DazzUI | ssh "$host" "tar -xzf - -C $remote"

# cargo.exe is a symlink to rustup.exe, which Windows will not run over SSH;
# the toolchain's own cargo runs.
toolchain='%USERPROFILE%\.rustup\toolchains\stable-x86_64-pc-windows-msvc\bin'
compiler="$(win "$remote/tools/dazzui-compiler")"
ssh "$host" "cd /d $compiler && set PATH=$toolchain;%PATH% && cargo build -q -p settings-example" \
  | tr -d '\r'
ssh "$host" "cd /d $compiler\\examples\\settings_example\\windows && dotnet build SettingsExample.sln -nologo -v q" \
  | tr -d '\r'

exe="$compiler\\examples\\settings_example\\windows\\SettingsExample\\bin\\x64\\Debug\\net8.0-windows10.0.19041.0\\win-x64\\SettingsExample.exe"
result="$(win "$remote")\\settings-smoke"
ssh "$host" "powershell -NoProfile -ExecutionPolicy Bypass -File $(win "$remote/packages/csharp/DazzUI/scripts/desktop.ps1") -Exe $exe -Arguments --smoke-test -Out $result -TimeoutSeconds 120" \
  | tr -d '\r' | tee /dev/stderr | grep -q '^PASS' && passed=1 || passed=0

mkdir -p "$out"
scp -q "$host:${remote}/settings-smoke/*.png" "$host:${remote}/settings-smoke/result.txt" "$out/" 2>/dev/null || true
echo "copied to $out" >&2
[ "$passed" = 1 ]
