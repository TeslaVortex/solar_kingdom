#!/bin/bash
# Field confirmation: oracle
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${ROOT}/bin/solarking"
NOTE="${*:-oracle confirmation}"
if [ -x "$BIN" ]; then exec "$BIN" confirm oracle "$NOTE"; fi
for c in "$ROOT/target/release/solarking" "$ROOT/target/debug/solarking"; do
  [ -x "$c" ] && exec "$c" confirm oracle "$NOTE"
done
echo "solarking not built" >&2; exit 1
