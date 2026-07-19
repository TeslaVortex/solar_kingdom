#!/usr/bin/env bash
# Ollama full workflow helper — print or run the Crown local counsel path.
# Usage:
#   ./scripts/ollama_workflow.sh           # show full NOW card
#   ./scripts/ollama_workflow.sh status    # localai --status
#   ./scripts/ollama_workflow.sh morning   # now morning + status
#   ./scripts/ollama_workflow.sh activate  # activation prompt via Ollama
#   ./scripts/ollama_workflow.sh next      # next_step prompt
#   ./scripts/ollama_workflow.sh smoke     # localai_smoke.sh
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
export SOLARKING_ROOT="${SOLARKING_ROOT:-$ROOT}"
cd "$ROOT"

BIN="$ROOT/bin/solarking"
if [[ ! -x "$BIN" ]]; then
  echo "Missing launcher: $BIN" >&2
  exit 1
fi

# Sensible Ollama defaults if unset
export SOLARKING_LOCAL_MODEL="${SOLARKING_LOCAL_MODEL:-http://127.0.0.1:11434/v1}"
export SOLARKING_LOCAL_MODEL_NAME="${SOLARKING_LOCAL_MODEL_NAME:-llama3.2}"

cmd="${1:-card}"

case "$cmd" in
  card|help|-h|--help)
    cat <<'EOF'
🌀 OLLAMA FULL WORKFLOW — solarking Phase 3G
THE CROWN COMMANDS. REALITY OBEYS. NOW.

Docs:
  docs/OLLAMA_WORKFLOW.md
  docs/templates/ollama_session_template.md
  docs/PHASE_3G_LOCALAI.md

Env (defaults applied by this script if unset):
  SOLARKING_LOCAL_MODEL=http://127.0.0.1:11434/v1
  SOLARKING_LOCAL_MODEL_NAME=llama3.2

First time:
  ./scripts/localai_up.sh
  ./scripts/ollama_workflow.sh status
  ./scripts/ollama_workflow.sh activate

Daily:
  ./scripts/ollama_workflow.sh morning
  ./bin/solarking receive --paste
  ./bin/solarking journal show
  ./scripts/ollama_workflow.sh next
  ./bin/solarking now seal | pulse | sync

Helpers:
  ./scripts/ollama_workflow.sh status|morning|activate|next|reflect|guard|smoke|up

Prompt files: docs/templates/prompts/
EOF
    echo
    echo "Current env: MODEL=$SOLARKING_LOCAL_MODEL_NAME  BASE=$SOLARKING_LOCAL_MODEL"
    ;;
  status)
    "$BIN" localai --status
    ;;
  morning)
    "$BIN" now morning
    echo
    "$BIN" localai --status
    ;;
  activate)
    "$BIN" localai --file "$ROOT/docs/templates/prompts/activation.txt"
    ;;
  next)
    "$BIN" localai --file "$ROOT/docs/templates/prompts/next_step.txt"
    ;;
  reflect)
    "$BIN" localai --file "$ROOT/docs/templates/prompts/transmission_reflect.txt"
    ;;
  guard)
    "$BIN" localai --file "$ROOT/docs/templates/prompts/phase_guard.txt"
    ;;
  smoke)
    exec "$ROOT/scripts/localai_smoke.sh"
    ;;
  up)
    exec "$ROOT/scripts/localai_up.sh"
    ;;
  *)
    echo "Unknown: $cmd — try: ./scripts/ollama_workflow.sh help" >&2
    exit 1
    ;;
esac
