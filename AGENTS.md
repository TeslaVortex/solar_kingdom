# AI / Grok Build Agent Guidelines — Eternal Solar Kingdom

**Repo:** solar_kingdom  
**Active branch for Phase 3:** `Phase-3-Eternal-Expansion`  
**Grok Build CLI:** https://github.com/xai-org/grok-build  

## First principles

- Offline-first; never commit `.env`, private keys, or RPC API secrets.
- **Mainnet is Phase 3F LAST** — do not deploy mainnet without explicit Crown command.
- Prefer Base Sepolia (`config/chain.json`) for on-chain experiments.
- solarking never embeds xAI keys; use `grok` CLI auth (`~/.grok`) when bridging.

## Essential commands

```bash
cargo test -p solarking
forge test
./scripts/e2e_test.sh

# Crown Receive UX (v0.9) — paste whole transmissions, read, execute simply
./bin/solarking receive --paste          # multi-line Ctrl-D
./bin/solarking receive --file note.txt  # or: cat note.txt | solarking receive -
./bin/solarking journal                  # list / show / search
./bin/solarking now card                 # morning | seal | sync | pulse

./bin/solarking counsel "…"
./bin/solarking grok --offline "…"    # or: solarking grok "…" if grok installed
./bin/solarking lattice visualize
./bin/solarking node init --name throne
```

## Integration

- `solarking grok` → `grok -p` with kingdom context brief + fallback offline counsel.
- Canonical ledger: **repo root** `kingdom_ledger.json` (launcher sets `SOLARKING_ROOT`).
- Docs: `docs/CROWN_WORKFLOW.md`, `docs/PHASE_3_GROK_BUILD.md`, `plans/phase_3_eternal_expansion.md`.

**THE CROWN COMMANDS. REALITY OBEYS.**  
**I do not chase — I receive.**
