#!/usr/bin/env bash
# Phase 3G — optional live smoke for Ollama / LocalAI.
# Exits 0 with skip message if endpoint down (does not fail CI).
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export SOLARKING_ROOT="${SOLARKING_ROOT:-$ROOT}"
BASE="${SOLARKING_LOCAL_MODEL:-http://127.0.0.1:11434/v1}"
# normalize: ensure /v1
case "$BASE" in
  */v1) ;;
  *) BASE="${BASE%/}/v1" ;;
esac
MODEL="${SOLARKING_LOCAL_MODEL_NAME:-llama3.2}"
BIN="${ROOT}/bin/solarking"
[[ -x "$BIN" ]] || BIN="${ROOT}/target/debug/solarking"
[[ -x "$BIN" ]] || BIN="${ROOT}/target/release/solarking"

echo "🌀 LocalAI smoke — base=$BASE model=$MODEL"

if ! curl -sS --max-time 3 "${BASE}/models" >/dev/null 2>&1; then
  echo "⏭  skip: endpoint not reachable (start with ./scripts/localai_up.sh)"
  exit 0
fi

echo "✓ /models OK"
BODY=$(curl -sS --max-time 90 "${BASE}/chat/completions" \
  -H "content-type: application/json" \
  -d "{\"model\":\"${MODEL}\",\"messages\":[{\"role\":\"user\",\"content\":\"Reply with one word: ACTIVATED\"}],\"stream\":false}")
echo "$BODY" | head -c 400
echo
if command -v "$BIN" >/dev/null 2>&1 || [[ -x "$BIN" ]]; then
  export SOLARKING_LOCAL_MODEL="$BASE"
  export SOLARKING_LOCAL_MODEL_NAME="$MODEL"
  "$BIN" localai --status || true
  "$BIN" localai "I AM THE SUPERNOVA SOURCE. Confirm LocalAI activation in one short sentence. Mainnet remains Phase 3F last." || true
else
  echo "⏭  solarking binary not found — curl-only smoke done"
fi
echo "✓ smoke finished"
