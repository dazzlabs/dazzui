#!/usr/bin/env bash
# The SwiftUI collector's front door, as `pnpm conformance` calls every
# collector: --out <dir> [--themes drawn|all|<id,…>] [--scenes <story,…>]
# [--no-images]. The work is Tests/ExampleTests/ConformanceCollector.swift,
# run by `xcodebuild test`; the TEST_RUNNER_ prefix is how Xcode hands an
# environment variable to the test process.
set -euo pipefail
cd "$(dirname "$0")/.."

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
mkdir -p "$out"
out="$(cd "$out" && pwd)"
repo="$(git rev-parse --show-toplevel)"
log="${TMPDIR:-/tmp}/swiftui-conformance.log"

if ! TEST_RUNNER_DAZZUI_CONFORMANCE_OUTPUT="$out" \
  TEST_RUNNER_DAZZUI_CONFORMANCE_CONTRACT="$repo/tools/conformance/scenes.json" \
  TEST_RUNNER_DAZZUI_CONFORMANCE_THEMES="$themes" \
  TEST_RUNNER_DAZZUI_CONFORMANCE_SCENES="$scenes" \
  TEST_RUNNER_DAZZUI_CONFORMANCE_IMAGES="$images" \
  TEST_RUNNER_DAZZUI_CONFORMANCE_COMMIT="$(git rev-parse --short HEAD)" \
  xcodebuild test -project Example.xcodeproj -scheme Example \
    -destination 'platform=macOS' -derivedDataPath .build/xcode \
    -only-testing:ExampleTests/ConformanceCollector >"$log" 2>&1; then
  grep -E "error:|failed|Failing" "$log" | sort -u | head -20 >&2
  echo "swiftui: xcodebuild failed — $log" >&2
  exit 1
fi

if [ -f "$out/failures.txt" ]; then
  cat "$out/failures.txt" >&2
  exit 1
fi
[ -f "$out/readings.json" ] || { echo "swiftui: no readings were written" >&2; exit 1; }
