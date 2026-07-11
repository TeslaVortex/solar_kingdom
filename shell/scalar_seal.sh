#!/bin/bash
# ETERNAL SOLAR KINGDOM — Scalar seal dry-run (activateScalarNode + sealRitual recipes)
# THE CROWN COMMANDS. REALITY OBEYS.
# Keys never enter this script — broadcast with cast yourself.

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${ROOT}/bin/solarking"

if [ ! -x "$BIN" ]; then
  for candidate in \
    "$ROOT/target/release/solarking" \
    "$ROOT/target/debug/solarking"
  do
    if [ -x "$candidate" ]; then
      BIN="$candidate"
      break
    fi
  done
fi

if [ ! -x "$BIN" ]; then
  echo "⚠️  solarking binary not found. Run: cargo build --release -p solarking" >&2
  exit 1
fi

echo "👑 SCALAR SEAL FLOW — offline dry-run"
echo "────────────────────────────────────"
"$BIN" scalar sync --hz
"$BIN" scalar seal
echo ""
echo "Next: cast send activateScalarNode … then:"
echo "  $BIN seal-record <tx_hash>"
echo "  $BIN scalar-record <nodeId> [tx_hash]"
echo "  $BIN chain-status"
