#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

export CROSS_SWIFTUI_PATH=${CROSS_SWIFTUI_PATH:-"$PWD/../../../../../../libnativeapi/crossswift"}
port=$(cd -- "$CROSS_SWIFTUI_PATH" && pwd)
export CROSS_SWIFTUI_PATH="$port"

case "${1:-release}" in
  debug|Debug) configuration=Debug ;;
  release|Release) configuration=Release ;;
  *)
    printf 'Usage: %s [debug|release]\n' "$0" >&2
    exit 1
    ;;
esac

derived=.build/wasm
swift run --package-path "$port" xcodebuild \
  -project Example.xcodeproj \
  -scheme Example \
  -configuration "$configuration" \
  -destination generic/platform=Wasm \
  -derivedDataPath "$derived" \
  CODE_SIGNING_ALLOWED=NO \
  build

site=.build/web
staged="$derived/Build/Products/$configuration/Example"
rm -rf -- "$site"
cp -R "$staged" "$site"
printf 'DazzUI Gallery web build: %s\n' "$PWD/$site"
