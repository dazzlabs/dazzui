#!/usr/bin/env bash
# The Flutter collector's front door, as `pnpm conformance` calls every
# collector: --out <dir> [--themes drawn|all|<id,…>] [--scenes <story,…>]
# [--no-images]. The work is conformance/collect_test.dart.
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

flutter test conformance/collect_test.dart \
  --dart-define=CONFORMANCE_OUT="$out" \
  --dart-define=CONFORMANCE_THEMES="$themes" \
  --dart-define=CONFORMANCE_SCENES="$scenes" \
  --dart-define=CONFORMANCE_IMAGES="$images" \
  --dart-define=CONFORMANCE_COMMIT="$(git rev-parse --short HEAD)"

if [ -f "$out/failures.txt" ]; then
  cat "$out/failures.txt" >&2
  exit 1
fi
