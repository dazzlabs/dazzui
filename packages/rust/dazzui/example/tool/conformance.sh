#!/usr/bin/env bash
# The GPUI collector's front door, as `pnpm conformance` calls every
# collector: --out <dir> [--themes drawn|all|<id,…>] [--scenes <story,…>]
# [--no-images]. The work is src/conformance.rs, run by the example itself
# with --conformance, in a window of its own on this Mac's display.
set -euo pipefail
cd "$(dirname "$0")/.."

out=""
passed=()
while [ $# -gt 0 ]; do
  case "$1" in
    --out) out="$2"; shift 2 ;;
    --themes|--scenes|--motion-ms|--modal|--modal-index|--caret-focus|--caret-input|--caret-keys) passed+=("$1" "$2"); shift 2 ;;
    --no-images|--reduced-motion|--positions) passed+=("$1"); shift ;;
    *) echo "unknown argument: $1" >&2; exit 2 ;;
  esac
done
[ -n "$out" ] || { echo "--out is required" >&2; exit 2; }
mkdir -p "$out"
out="$(cd "$out" && pwd)"
repo="$(cd ../../../.. && pwd)"
log="${TMPDIR:-/tmp}/gpui-conformance.log"

# The Metal toolchain is optional on a Mac; without it GPUI compiles its
# shaders at launch, which draws the same thing.
if ! cargo build -p dazzui-example --features runtime-shaders >"$log" 2>&1; then
  grep -E "^error" -A5 "$log" | head -20 >&2
  echo "gpui: cargo build failed — $log" >&2
  exit 1
fi

cargo run --quiet -p dazzui-example --features runtime-shaders -- --conformance --out "$out" \
  --contract "$repo/tools/conformance/scenes.json" \
  --commit "${DAZZUI_CONFORMANCE_COMMIT:-$(git rev-parse --short HEAD 2>/dev/null || echo unknown)}" "${passed[@]+"${passed[@]}"}"

if [ -f "$out/failures.txt" ]; then
  cat "$out/failures.txt" >&2
  exit 1
fi
[ -f "$out/readings.json" ] || { echo "gpui: no readings were written" >&2; exit 1; }
