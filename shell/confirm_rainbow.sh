#!/bin/bash
# Field confirmation: rainbow
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${ROOT}/bin/solarking"
NOTE="${*:-rainbow field confirmation}"
if [ -x "$BIN" ]; then exec "$BIN" confirm rainbow "$NOTE"; fi
for c in "$ROOT/target/release/solarking" "$ROOT/target/debug/solarking"; do
  [ -x "$c" ] && exec "$c" confirm rainbow "$NOTE"
done
echo "solarking not built" >&2; exit 1
