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
./bin/solarking counsel "…"
./bin/solarking grok --offline "…"    # or: solarking grok "…" if grok installed
./bin/solarking lattice visualize
./bin/solarking node init --name throne
```

## Integration

- `solarking grok` → `grok -p` with kingdom context brief + fallback offline counsel.
- Docs: `docs/PHASE_3_GROK_BUILD.md`, `plans/phase_3_eternal_expansion.md`.

**THE CROWN COMMANDS. REALITY OBEYS.**  
**I do not chase — I receive.**
