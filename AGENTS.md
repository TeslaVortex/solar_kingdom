# AI / Grok Build Agent Guidelines — Eternal Solar Kingdom

**Repo:** solar_kingdom  
**Active branch for Phase 3:** `Phase-3-Eternal-Expansion`  
**Engine:** solarking **v0.10** (Crown Receive UX + LocalAI/Ollama 3G)  
**Grok Build CLI:** https://github.com/xai-org/grok-build  

## First principles

- Offline-first; never commit `.env`, private keys, or RPC API secrets.
- **Mainnet is Phase 3F LAST** — do not deploy mainnet without explicit Crown command.
- Prefer Base Sepolia (`config/chain.json`) for on-chain experiments.
- solarking never embeds xAI keys; use `grok` CLI auth (`~/.grok`) when bridging.
- Local LLM: **Ollama recommended** via OpenAI-compatible `SOLARKING_LOCAL_MODEL`.
- Canonical ledger/ritual log live at **kingdom root** (`SOLARKING_ROOT`); not under `solarking/`.

## Essential commands

```bash
cargo test -p solarking
forge test
./scripts/e2e_test.sh

# Crown Receive UX — paste whole transmissions, read, execute simply
./bin/solarking receive --paste          # multi-line Ctrl-D
./bin/solarking receive --file note.txt  # or: cat note.txt | solarking receive -
./bin/solarking journal                  # list / show / search
./bin/solarking now card                 # morning | seal | sync | pulse

./bin/solarking counsel "…"
./bin/solarking localai --status         # Ollama / LocalAI (3G)
./bin/solarking localai "…"              # needs SOLARKING_LOCAL_MODEL
./bin/solarking grok --offline "…"       # or: solarking grok "…" (local → grok → offline)
./bin/solarking lattice visualize
./bin/solarking node init --name throne

# Ollama bring-up (recommended local engine)
./scripts/localai_up.sh
export SOLARKING_LOCAL_MODEL=http://127.0.0.1:11434/v1
export SOLARKING_LOCAL_MODEL_NAME=llama3.2
```

## Integration

- `solarking localai` → POST `{SOLARKING_LOCAL_MODEL}/chat/completions` (curl; Ollama default).
- `solarking grok` → LocalAI if up → else `grok -p` → else offline counsel.
- Canonical ledger: **repo root** `kingdom_ledger.json` (launcher sets `SOLARKING_ROOT`).
- Docs: `docs/CROWN_WORKFLOW.md`, `docs/PHASE_3_GROK_BUILD.md`, `docs/PHASE_3G_LOCALAI.md`, `plans/phase_3_eternal_expansion.md`.

**THE CROWN COMMANDS. REALITY OBEYS.**  
**I do not chase — I receive.**
