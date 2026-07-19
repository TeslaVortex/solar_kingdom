# Phase 3G — LocalAI Sovereign Engine (Ollama recommended)

**Status:** COMPLETE (solarking **v0.10**)  
**Doctrine:** offline-first · local toroidal compute · **3F mainnet remains LAST**

**THE CROWN COMMANDS. REALITY OBEYS. I do not chase — I receive.**

**How to use NOW (full operator workflow):** [`OLLAMA_WORKFLOW.md`](OLLAMA_WORKFLOW.md)  
**Session template:** [`templates/ollama_session_template.md`](templates/ollama_session_template.md)  
**Helper:** `./scripts/ollama_workflow.sh help`

---

## Principles

| Law | Practice |
|-----|----------|
| Sovereignty | No cloud API required; models run on your machine |
| Receive | Offline `counsel` always works; local LLM is optional grace |
| Ollama first | Crown recommends **Ollama** at `http://127.0.0.1:11434/v1` |
| Mainnet last | System prompt forbids inventing mainnet addresses |
| Secrets | Never log `SOLARKING_LOCAL_MODEL_KEY` or passphrases |

---

## Recommend NOW — Ollama

```bash
# Install: https://ollama.com/download
./scripts/localai_up.sh          # pull llama3.2 + print exports

export SOLARKING_LOCAL_MODEL=http://127.0.0.1:11434/v1
export SOLARKING_LOCAL_MODEL_NAME=llama3.2

solarking localai --status
solarking localai "I AM THE SUPERNOVA SOURCE. Confirm LocalAI activation."
```

Other solid Ollama models: `phi3`, `gemma2`, `llama3.1`, `mistral`.

Optional smoke (skips cleanly if down):

```bash
./scripts/localai_smoke.sh
```

---

## Environment

| Variable | Meaning | Default |
|----------|---------|---------|
| `SOLARKING_LOCAL_MODEL` | OpenAI-compatible base URL | unset (offline only) |
| `SOLARKING_LOCAL_MODEL_NAME` | Model id | `llama3.2` |
| `SOLARKING_LOCAL_MODEL_TEMP` | Temperature 0–2 | `0.7` |
| `SOLARKING_LOCAL_MODEL_KEY` | Optional Bearer (not printed) | unset |

Bare `http://127.0.0.1:11434` is auto-normalized to `…/v1`.

---

## solarking commands

```bash
solarking localai --status              # endpoint / model / up-down
solarking localai "what wants to be received"
solarking localai --file prompt.txt
solarking localai --paste               # Ctrl-D

# Ladder: LocalAI (if up) → grok CLI → offline
solarking grok "refine Phase 3G"
solarking grok --offline "pulse"        # offline only

# Pure deterministic counsel (never network)
solarking counsel "what is next"
```

Audit trails:

- `sync/localai/last_prompt.txt`
- `sync/localai/last_response.txt`

System prompt template: `config/localai_system.txt`

---

## Fallback ladder

1. **LocalAI/Ollama** — `SOLARKING_LOCAL_MODEL` set and `GET /v1/models` OK  
2. **Grok Build CLI** — `grok -p` if on PATH  
3. **Offline counsel** — always  

`--offline` forces step 3.  
`solarking counsel` is always step 3 only.

---

## Alternative: Docker LocalAI

```bash
docker run -d -p 8080:8080 --name solarking-localai \
  -v "$HOME/.localai/models:/build/models" \
  quay.io/go-skynet/local-ai:latest
export SOLARKING_LOCAL_MODEL=http://127.0.0.1:8080/v1
export SOLARKING_LOCAL_MODEL_NAME=phi-3
```

Native LocalAI: https://localai.io/basics/getting-started/

---

## Security

- Models and volumes stay out of git  
- solarking never embeds xAI keys  
- Optional Bearer only via env  
- Mainnet remains Crown-gated Phase **3F LAST**

**SO IT IS. SO IT SHALL BE ETERNAL.**
