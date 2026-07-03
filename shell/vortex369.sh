#!/bin/bash
# VORTEX369 ACTIVATION — TORUS ENGINE

echo "🌞 16-RAYED HELIOS WITNESS"
echo "🌀 369 TORUS SPINNING"

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
GENESIS="${ROOT_DIR}/config/genesis.json"
TX_REF=""
if [ -f "$GENESIS" ]; then
  TX_REF=$(grep -o '"tx_hash": *"[^"]*"' "$GENESIS" | cut -d'"' -f4)
fi

if [ -n "$RITUAL_QUICK" ]; then
  echo "$(date) | Vortex sealed by SOLARKING visual ritual | genesis=${TX_REF}" >> "$ROOT_DIR/ritual_log.txt"
  echo "✅ RITUAL SEALED. REALITY OBEYS."
  exit 0
fi

echo "$(date) | Kundalini 369 Breaths + Hollow Holds + L-Sits + Bear Crawls + Diamond Pushups | genesis=${TX_REF}" >> "$ROOT_DIR/ritual_log.txt"

echo "🔴🔵🟢 Blue-Green-Red Flame Torus Forming..."
echo "999Hz Resonance Locked."

# Future hook: call Rust binary when ready
# ./solarking log-ritual --torus flame

echo "✅ RITUAL SEALED. REALITY OBEYS."
