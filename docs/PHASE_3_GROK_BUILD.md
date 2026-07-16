# Phase 3E — Grok Build CLI Integration

**Grok Build:** [github.com/xai-org/grok-build](https://github.com/xai-org/grok-build) · [x.ai/cli](https://x.ai/cli)

**Status:** **COMPLETE** (bridge live in solarking v0.8)

---

## Principles

| Law | Practice |
|-----|----------|
| Sovereignty | solarking never stores xAI API keys; `grok` uses `~/.grok` auth |
| Receive | Offline counsel always works; grok is optional grace |
| Mainnet last | System prompt forbids inventing mainnet addresses |

## Install Grok Build (optional)

```bash
curl -fsSL https://x.ai/cli/install.sh | bash
grok --version
```

## solarking commands

```bash
# Offline counsel (always)
solarking counsel "what is next"
solarking grok --offline "refine the lattice"

# Headless single-turn via Grok Build (if installed)
solarking grok "Suggest the next Phase 3D node protocol step"
# equivalent core: grok -p "…" --cwd /path/to/solar_kingdom

# Blueprint pulse (offline append)
solarking blueprint "What wants to be received next?"
```

## Context brief

`solarking grok` injects field/scalar/harmonics/genesis into the prompt file at:

`sync/grok/last_prompt.txt`  
Responses (when grok succeeds): `sync/grok/last_response.txt`

## Project rules

See root `AGENTS.md` for Grok Build / agent guidelines when working in this repo.

**I do not chase — I receive. What is meant for me does not wander.**  
**THE CROWN COMMANDS. REALITY OBEYS.**
