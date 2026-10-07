#!/bin/sh
set -eu
cd "$(dirname "$0")/../.."
if ! pkg-config --atleast-version=4.16 gtk4; then
  echo 'GTK4 >= 4.16 is unavailable; skipping GTK4 conformance.'
  exit 0
fi
cargo run -p dazzui-gtk4-example -- --conformance "$@"
