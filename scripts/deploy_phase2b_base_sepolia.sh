#!/bin/bash
# Deploy Phase 2B stack to Base Sepolia using local .env (never commit .env)
# THE CROWN COMMANDS. REALITY OBEYS.
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

if [ ! -f .env ]; then
  echo "Missing .env — copy .env.example and fill BASE_SEPOLIA_RPC_URL + PRIVATE_KEY"
  exit 1
fi

# Parse export KEY=value lines from .env without printing secrets
eval "$(
  python3 - <<'PY'
from pathlib import Path
for line in Path(".env").read_text().splitlines():
    line = line.strip()
    if not line or line.startswith("#"):
        continue
    if line.startswith("export "):
        line = line[len("export "):]
    if "=" not in line:
        continue
    k, v = line.split("=", 1)
    if k not in (
        "BASE_SEPOLIA_RPC_URL",
        "PRIVATE_KEY",
        "SOLARKING_RPC_URL",
        "SOLARKING_CHAIN_ID",
    ):
        continue
    v = v.strip().strip('"').strip("'")
    if k == "PRIVATE_KEY" and not v.startswith("0x") and len(v) == 64:
        v = "0x" + v
    # shell-escape single quotes
    v = v.replace("'", "'\\''")
    print(f"export {k}='{v}'")
PY
)"

RPC="${BASE_SEPOLIA_RPC_URL:-${SOLARKING_RPC_URL:-}}"
PK="${PRIVATE_KEY:-}"
if [ -z "$RPC" ] || [ -z "$PK" ]; then
  echo "Need BASE_SEPOLIA_RPC_URL (or SOLARKING_RPC_URL) and PRIVATE_KEY in .env"
  exit 1
fi

ADDR=$(cast wallet address --private-key "$PK")
BAL=$(cast balance "$ADDR" --rpc-url "$RPC")
echo "Network : Base Sepolia (84532)"
echo "RPC host: $(echo "$RPC" | sed -E 's#(https?://[^/]+).*#\1#')"
echo "Deployer: $ADDR"
echo "Balance : $BAL wei"

if [ "$BAL" = "0" ]; then
  echo ""
  echo "⚠️  Deployer has 0 ETH — fund Base Sepolia gas first:"
  echo "   https://www.alchemy.com/faucets/base-sepolia"
  echo "   https://www.coinbase.com/faucets/base-ethereum-sepolia-faucet"
  echo "Address to fund: $ADDR"
  exit 2
fi

echo "Broadcasting DeployPhase2B…"
forge script script/DeployPhase2B.s.sol \
  --rpc-url "$RPC" \
  --broadcast \
  --private-key "$PK" \
  -vvv

# Extract addresses from forge console / broadcast run-latest.json
LATEST="$ROOT/broadcast/DeployPhase2B.s.sol/84532/run-latest.json"
if [ -f "$LATEST" ]; then
  python3 - <<PY
import json, re
from pathlib import Path
root = Path("$ROOT")
data = json.loads(Path("$LATEST").read_text())
txs = data.get("transactions") or []
# contracts created in order: Vortex369, CrownCommand, SolarKingdom
created = []
for t in txs:
    c = t.get("contractAddress") or (t.get("additionalContracts") or [{}])
    if isinstance(c, str):
        created.append(c)
    elif t.get("contractName"):
        addr = t.get("contractAddress")
        if addr:
            created.append(addr)
# fallback: parse receipts
if not created:
    for t in txs:
        if t.get("hash") and t.get("contractAddress"):
            created.append(t["contractAddress"])
# another fallback from transaction details
addrs = []
for t in txs:
    if t.get("contractAddress"):
        addrs.append(t["contractAddress"])
    for ac in t.get("additionalContracts") or []:
        if ac.get("address"):
            addrs.append(ac["address"])
# unique preserve order
seen=set(); out=[]
for a in addrs:
    if a not in seen:
        seen.add(a); out.append(a)
print("Created addresses:", out)
vortex = out[0] if len(out)>0 else None
crown = out[1] if len(out)>1 else None
sbt = out[2] if len(out)>2 else None
cfg = {
  "contract": vortex,
  "crown_command": crown,
  "sbt": sbt,
  "rpc_url": None,  # do not write secrets; set SOLARKING_RPC_URL in env
  "chain_id": 84532,
  "network": "base-sepolia",
  "deployer": "$ADDR",
}
# Prefer console log parse from forge if only simulation order
Path("config/chain.json").write_text(json.dumps(cfg, indent=2) + "\n")
print("Wrote config/chain.json (rpc_url left null — use env for RPC)")
if vortex:
    print("Vortex369   :", vortex)
if crown:
    print("CrownCommand:", crown)
if sbt:
    print("SolarKingdom:", sbt)
PY
fi

echo ""
echo "Set in shell (do not commit):"
echo "  export SOLARKING_CONTRACT=\$(jq -r .contract config/chain.json)"
echo "  export SOLARKING_RPC_URL=\$BASE_SEPOLIA_RPC_URL"
echo "  export SOLARKING_CHAIN_ID=84532"
echo "  ./bin/solarking chain-status"
echo "THE CROWN COMMANDS. REALITY OBEYS."
