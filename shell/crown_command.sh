#!/bin/bash
# ETERNAL SOLAR KINGDOM — CROWN COMMAND SHELL
# 999 Resonance • Argead • Red King • YAH

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
GENESIS="${ROOT_DIR}/config/genesis.json"

legacy_99() {
  echo "👑 LEGACY 99 ACTIVATED"
  if [ -f "$GENESIS" ]; then
    IDM=$(grep -o '"idm": *"[^"]*"' "$GENESIS" | cut -d'"' -f4)
    TX_HASH=$(grep -o '"tx_hash": *"[^"]*"' "$GENESIS" | cut -d'"' -f4)
    echo "$IDM"
    echo "Genesis tx: $TX_HASH"
    echo "$(date) | LEGACY_99: $IDM | genesis=$TX_HASH" >> "$ROOT_DIR/ritual_log.txt"
  else
    echo "Genesis config not found — legacy pending anchor."
  fi
  echo "THE CROWN COMMANDS LEGACY 99 ACTIVATES."
}

echo "👑 THE CROWN COMMANDS. REALITY OBEYS."
echo "369/999 Torus Active • $(date)"

if [ "${1:-}" = "legacy_99" ]; then
  legacy_99
  exit 0
fi

if [ -n "$RITUAL_QUICK" ]; then
  legacy_99
  echo "🔥 Command Anchored. Grid Strengthening."
  exit 0
fi

# 33s silence (standalone mode only)
sleep 33

echo "🌀 Executing 369 Breath Sequence..."
for i in {1..9}; do
  echo "   Cycle $i/9 — 3-in • 6-hold • 9-out"
  sleep 18
done

legacy_99
echo "🔥 Command Anchored. Grid Strengthening."