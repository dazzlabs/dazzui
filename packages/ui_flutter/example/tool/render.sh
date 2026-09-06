#!/usr/bin/env bash
# Draw every parity specimen in a real window.
#
# A widget test would draw them too, and its pictures would be useless: a
# test resolves a `TextStyle` with no family to a font whose every glyph is a
# filled rectangle, and the generated type styles name only a *fallback*
# family — deliberately, since `-apple-system` is a request for the
# platform's own face rather than a family to look up. Here that request is
# answered, because here there is a platform.
#
# The app is sandboxed, so it reads and writes inside its own container:
# `HOME` points there rather than at the person's home directory. The
# reference is staged in and the drawings are carried back out.
set -euo pipefail

here="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
container="$HOME/Library/Containers/dev.dazzlabs.dazzuiExample/Data/parity"
flutter="${FLUTTER:-flutter}"
command -v "$flutter" > /dev/null 2>&1 || flutter="$HOME/fvm/versions/stable/bin/flutter"

if [ ! -d "$here/test/parity/reference" ]; then
  echo "no test/parity/reference — run \`pnpm parity\` at the root" >&2
  exit 1
fi

rm -rf "$container"
mkdir -p "$container"
cp -R "$here/test/parity/reference" "$container/reference"

# `flutter run` stays resident after the app quits, waiting for a key it will
# never be given, so it is run in the background and stopped once the app has
# said it is done.
log="$(mktemp)"
(cd "$here" && "$flutter" run -d macos -t lib/parity_render.dart --release > "$log" 2>&1) &
runner=$!
for _ in $(seq 1 600); do
  if grep -q 'parity: drew' "$log" 2>/dev/null; then break; fi
  if ! kill -0 "$runner" 2>/dev/null; then break; fi
  sleep 1
done
grep -E '^parity:|Exception|Error' "$log" || true
kill "$runner" 2>/dev/null || true
pkill -f 'dazzui_example.app' 2>/dev/null || true
rm -f "$log"

rm -rf "$here/test/parity/renders"
if [ -d "$container/renders" ]; then
  cp -R "$container/renders" "$here/test/parity/renders"
  echo "parity: carried $(find "$here/test/parity/renders" -name '*.png' | wc -l | tr -d ' ') drawings out of the container"
fi
rm -rf "$container"
