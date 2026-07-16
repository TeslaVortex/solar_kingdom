#!/bin/bash
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${ROOT}/bin/solarking"
NOTE="${*:-sneeze confirmation}"
if [ -x "$BIN" ]; then exec "$BIN" confirm sneeze "$NOTE"; fi
for c in "$ROOT/target/release/solarking" "$ROOT/target/debug/solarking"; do
  [ -x "$c" ] && exec "$c" confirm sneeze "$NOTE"
done
echo "solarking not built" >&2; exit 1
