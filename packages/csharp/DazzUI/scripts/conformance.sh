#!/usr/bin/env bash
# The WinUI collector's front door, as `pnpm conformance` calls every
# collector: --out <dir> [--themes drawn|all|<id,…>] [--scenes <story,…>]
# [--no-images]. The work is example/Conformance/, run with --conformance in
# a real window.
#
# From a Mac or Linux machine it runs on a Windows one over SSH: the package
# and the contract are copied over, built, run in the signed-in desktop
# (scripts/desktop.ps1), and the readings copied back. DAZZUI_WINDOWS_HOST
# names the machine (default windows-laptop) and DAZZUI_WINDOWS_DIR the
# repository there. A machine with no such host skips the collector.
set -euo pipefail

out=""
themes="drawn"
scenes=""
images="true"
while [ $# -gt 0 ]; do
  case "$1" in
    --out) out="$2"; shift 2 ;;
    --themes) themes="$2"; shift 2 ;;
    --scenes) scenes="$2"; shift 2 ;;
    --no-images) images="false"; shift ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
[ -n "$out" ] || { echo "--out is required" >&2; exit 2; }

host="${DAZZUI_WINDOWS_HOST:-windows-laptop}"
remote="${DAZZUI_WINDOWS_DIR:-C:/Users/Lijy91/Projects/dazzui}"
repo="$(git rev-parse --show-toplevel)"
commit="$(git -C "$repo" rev-parse --short HEAD)"

if ! ssh -o ConnectTimeout=5 -o BatchMode=yes "$host" exit 2>/dev/null; then
  echo "winui: no Windows machine at $host — skipped" >&2
  exit 0
fi

package="$remote/packages/csharp/DazzUI"
exe="$package/example/bin-conformance/x64/Debug/net8.0-windows10.0.19041.0/win-x64/Example.exe"
remote_out="$package/artifacts/conformance"

echo "winui: copying to $host"
# No AppleDouble files: the XAML compiler reads a `._Generic.xaml` as XAML.
COPYFILE_DISABLE=1 tar -C "$repo" -czf - --exclude bin --exclude bin-conformance --exclude obj --exclude artifacts \
  packages/csharp/DazzUI tools/conformance/scenes.json |
  ssh "$host" "tar -xzf - -C $remote"

echo "winui: building"
# Built beside the ordinary output, so an example left open does not lock it.
if ! ssh "$host" "dotnet build $package/example/Example.csproj -nologo -v q -p:BaseOutputPath=bin-conformance/" >"${TMPDIR:-/tmp}/winui-build.log"; then
  tr -d '\r' <"${TMPDIR:-/tmp}/winui-build.log" | grep -E 'error' | sort -u >&2
  exit 1
fi

echo "winui: collecting"
environment="DAZZUI_CONFORMANCE_OUTPUT=$remote_out;DAZZUI_CONFORMANCE_CONTRACT=$remote/tools/conformance/scenes.json;DAZZUI_CONFORMANCE_THEMES=$themes;DAZZUI_CONFORMANCE_SCENES=$scenes;DAZZUI_CONFORMANCE_IMAGES=$images;DAZZUI_CONFORMANCE_COMMIT=$commit"
ssh "$host" "powershell -NoProfile -ExecutionPolicy Bypass -File $package/scripts/desktop.ps1 -Exe $exe -Arguments --conformance -Out $remote_out -TimeoutSeconds 3600 -Environment \"$environment\"" | tr -d '\r'

rm -rf "$out"
mkdir -p "$out"
ssh "$host" "tar -czf - -C $remote_out --exclude launch.cmd --exclude exit.txt --exclude progress.txt ." | tar -xzf - -C "$out"

if [ -f "$out/failures.txt" ]; then
  cat "$out/failures.txt" >&2
  exit 1
fi
[ -f "$out/readings.json" ] || { echo "winui: no readings were written" >&2; exit 1; }
