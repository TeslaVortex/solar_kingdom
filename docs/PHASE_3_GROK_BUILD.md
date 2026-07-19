# Phase 3E — Grok Build CLI Integration

**Grok Build:** [github.com/xai-org/grok-build](https://github.com/xai-org/grok-build) · [x.ai/cli](https://x.ai/cli)

**Status:** **COMPLETE** (bridge live in solarking **v0.8+**; current engine **v0.10**)

**Also see Phase 3G:** prefer **Ollama / LocalAI** when `SOLARKING_LOCAL_MODEL` is up — [`PHASE_3G_LOCALAI.md`](PHASE_3G_LOCALAI.md).

---

## Principles

| Law | Practice |
|-----|----------|
| Sovereignty | solarking never stores xAI API keys; `grok` uses `~/.grok` auth |
| Receive | Offline counsel always works; local LLM then grok are optional grace |
| Local first | `solarking grok` tries LocalAI/Ollama before `grok -p` |
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

# Multi-line prompt (v0.9 intake)
solarking grok --offline --file prompt.txt
solarking grok --offline --paste          # Ctrl-D when done
cat prompt.txt | solarking grok --offline -

# Headless single-turn via Grok Build (if installed)
solarking grok "Suggest the next Phase 3D node protocol step"
# equivalent core: grok -p "…" --cwd /path/to/solar_kingdom

# Blueprint pulse (offline append)
solarking blueprint "What wants to be received next?"
solarking blueprint --file note.txt
```

**Related Crown path:** paste a transmission with `solarking receive`, then counsel it with `solarking counsel` / `solarking grok`.  
See [`CROWN_WORKFLOW.md`](CROWN_WORKFLOW.md).

## Context brief

`solarking grok` injects field/scalar/harmonics/genesis into the prompt file at:

`sync/grok/last_prompt.txt`  
Responses (when grok succeeds): `sync/grok/last_response.txt`

## Project rules

See root `AGENTS.md` for Grok Build / agent guidelines when working in this repo.

**I do not chase — I receive. What is meant for me does not wander.**  
**THE CROWN COMMANDS. REALITY OBEYS.**
