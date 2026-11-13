#!/usr/bin/env bash
set -euo pipefail
cd -- "$(dirname -- "${BASH_SOURCE[0]}")/.."

export SWIFTUI_PORT_WEB=1
export SWIFTUI_PORT_PATH=${SWIFTUI_PORT_PATH:-"$PWD/../../../../../../libnativeapi/swiftui-port"}
port=$(cd -- "$SWIFTUI_PORT_PATH" && pwd)
export SWIFTUI_PORT_PATH="$port"
sdk=${SWIFT_WASM_SDK:-swift-6.4.2-RELEASE_wasm}
configuration=${1:-release}

swift package --swift-sdk "$sdk" --scratch-path .build/wasm \
  js --product Example -c "$configuration"

package=.build/wasm/plugins/PackageToJS/outputs/Package
site=.build/web
staging=$(mktemp -d .build/web.XXXXXX)
trap 'rm -rf -- "$staging"' EXIT
mkdir -p "$staging/app" "$staging/fluent"
cp -R "$package"/. "$staging/app/"
npm install --prefix "$staging/app" --ignore-scripts --omit=dev --no-audit --no-fund
cp "$port/Web/index.html" "$staging/index.html"
cp ../Windows/FluentIcons/Resources/* "$staging/fluent/"
rm -rf -- "$site"
mv -- "$staging" "$site"
printf 'DazzUI Gallery web build: %s\n' "$PWD/$site"
