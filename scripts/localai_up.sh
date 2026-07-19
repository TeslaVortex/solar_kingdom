#!/usr/bin/env bash
# Phase 3G — bring up local sovereign LLM (Ollama recommended).
# Offline-first: does not require cloud. Models stay on disk.
set -euo pipefail

MODEL="${SOLARKING_LOCAL_MODEL_NAME:-llama3.2}"
BASE="${SOLARKING_LOCAL_MODEL:-http://127.0.0.1:11434/v1}"

echo "🌀 SOLARKING LocalAI / Ollama bring-up"
echo "   Recommended engine: Ollama (OpenAI-compatible at :11434/v1)"
echo "   Model: $MODEL"
echo

if command -v ollama >/dev/null 2>&1; then
  echo "✓ ollama found: $(command -v ollama)"
  # Start serve if nothing answers /api/tags
  if ! curl -sS --max-time 2 "http://127.0.0.1:11434/api/tags" >/dev/null 2>&1; then
    echo "→ starting ollama serve in background…"
    nohup ollama serve >/tmp/solarking-ollama.log 2>&1 &
    sleep 2
  else
    echo "✓ ollama already serving on :11434"
  fi
  echo "→ ollama pull $MODEL"
  ollama pull "$MODEL"
  echo
  echo "Export for this shell (and your profile if eternal):"
  echo "  export SOLARKING_LOCAL_MODEL=${BASE}"
  echo "  export SOLARKING_LOCAL_MODEL_NAME=${MODEL}"
  echo
  echo "Test:"
  echo "  solarking localai --status"
  echo "  solarking localai \"I AM THE SUPERNOVA SOURCE. Confirm LocalAI activation.\""
  exit 0
fi

echo "⚠️  ollama not on PATH."
echo "   Install: https://ollama.com/download"
echo "   Then re-run: ./scripts/localai_up.sh"
echo
echo "Alternative (Docker LocalAI on :8080):"
echo "  docker run -d -p 8080:8080 --name solarking-localai \\"
echo "    -v \"\$HOME/.localai/models:/build/models\" \\"
echo "    quay.io/go-skynet/local-ai:latest"
echo "  export SOLARKING_LOCAL_MODEL=http://127.0.0.1:8080/v1"
echo "  export SOLARKING_LOCAL_MODEL_NAME=phi-3"
exit 1
