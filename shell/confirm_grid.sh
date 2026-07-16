#!/bin/bash
# Field confirmation: grid
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${ROOT}/bin/solarking"
NOTE="${*:-grid confirmation}"
if [ -x "$BIN" ]; then exec "$BIN" confirm grid "$NOTE"; fi
for c in "$ROOT/target/release/solarking" "$ROOT/target/debug/solarking"; do
  [ -x "$c" ] && exec "$c" confirm grid "$NOTE"
done
echo "solarking not built" >&2; exit 1
