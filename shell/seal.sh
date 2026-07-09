#!/bin/bash
# ETERNAL SOLAR KINGDOM — On-chain seal dry-run (no broadcast)
# THE CROWN COMMANDS. REALITY OBEYS.

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${ROOT}/bin/solarking"

if [ -x "$BIN" ]; then
  exec "$BIN" seal --dry-run "$@"
fi

# Fallback to cargo-built binary
for candidate in \
  "$ROOT/target/release/solarking" \
  "$ROOT/target/debug/solarking"
do
  if [ -x "$candidate" ]; then
    exec "$candidate" seal --dry-run "$@"
  fi
done

echo "⚠️  solarking binary not found. Run: cd $ROOT/solarking && cargo build --release" >&2
exit 1
