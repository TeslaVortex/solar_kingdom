#!/bin/bash
# Local libation + offline cast recipe for offerLibation(string) — never broadcasts
# THE CROWN COMMANDS. REALITY OBEYS.
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${ROOT}/bin/solarking"
TARGET="${1:-ancestors}"

run_bin() {
  if [ -x "$BIN" ]; then "$BIN" "$@"; return; fi
  for c in "$ROOT/target/release/solarking" "$ROOT/target/debug/solarking"; do
    if [ -x "$c" ]; then "$c" "$@"; return; fi
  done
  echo "solarking not built" >&2; exit 1
}

export RITUAL_QUICK=1
run_bin libation "$TARGET"
echo ""
echo "🔗 ON-CHAIN offerLibation dry-run recipe (keys stay in your shell):"
CONTRACT=$(jq -r '.contract // empty' "$ROOT/config/chain.json" 2>/dev/null || true)
if [ -n "$CONTRACT" ] && [ "$CONTRACT" != "null" ]; then
  echo "  cast send $CONTRACT 'offerLibation(string)' '$TARGET' --rpc-url \$SOLARKING_RPC_URL --private-key \$PRIVATE_KEY"
else
  echo "  cast send \$SOLARKING_CONTRACT 'offerLibation(string)' '$TARGET' --rpc-url \$SOLARKING_RPC_URL --private-key \$PRIVATE_KEY"
fi
echo "Never auto-broadcast from systemd or this script."
