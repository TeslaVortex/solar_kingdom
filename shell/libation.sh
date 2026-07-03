#!/bin/bash
# ETERNAL SOLAR KINGDOM — LIBATION FOR ANCESTORS
# Rakija poured to the earth • Argead lineage honored

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ROOT_DIR="$(dirname "$SCRIPT_DIR")"
GENESIS="${ROOT_DIR}/config/genesis.json"

echo "🍶 LIBATION RITUAL — Ancestors Remembered"
echo "THE CROWN COMMANDS. REALITY OBEYS."

TARGET="${1:-ancestors}"

case "$TARGET" in
  ancestors)
    echo "Pouring rakija to the earth for the Argead lineage..."
    ;;
  *)
    echo "Libation offered for: $TARGET"
    ;;
esac

if [ -f "$GENESIS" ]; then
  TX_HASH=$(grep -o '"tx_hash": *"[^"]*"' "$GENESIS" | cut -d'"' -f4)
  echo "Genesis sacrifice anchored: $TX_HASH"
fi

echo "$(date) | LIBATION: rakija poured for $TARGET" >> "$ROOT_DIR/ritual_log.txt"

if [ -n "$RITUAL_QUICK" ]; then
  echo "✅ Libation sealed. Ancestors received."
  exit 0
fi

echo "33s silence — feel the ancestors receive..."
sleep 33
echo "✅ Libation sealed. Ancestors received. Grid strengthened."