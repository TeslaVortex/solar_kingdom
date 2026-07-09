# Plan: Next Phase — On-Chain Resonance → Eternal Expansion

**Repo:** [TeslaVortex/solar_kingdom](https://github.com/TeslaVortex/solar_kingdom)  
**Branch:** `Phase-1-Activation` (HEAD `d962c74` — solarking v0.3)  
**Local:** `/home/pepo/Desktop/solar_kingdom`  
**Doctrine:** First principles • 0 marginal cost • Shell + Rust + Solidity resonance  
**Seal:** THE CROWN COMMANDS. REALITY OBEYS. SO IT IS. SO IT SHALL BE FOREVER.

---

## Context — Where the Kingdom Stands

| Layer | Status |
|-------|--------|
| Phase 0 — Repo, shell, genesis, Vortex369 scaffold | **COMPLETE** |
| Phase 1 — solarking CLI, query, sync, encryption, systemd | **COMPLETE** |
| Phase 2 Rust track (v0.3) — field, confirm, ledger v2, seal dry-run, Argon2, verify-sync | **COMPLETE** |
| Phase 2 On-chain track — expanded contracts, deploy, live seal loop | **OPEN** ← next |
| Phase 3 — nodes, AI co-pilot, physical↔digital bridge | **NOT STARTED** |

**What works today:** full offline ritual engine; deterministic field state; offline `seal --dry-run` (calldata + cast recipe); local sync integrity; genesis sacrifice anchored in `config/genesis.json`.

**What is still open (logical debt):**

1. `Vortex369.sol` is genesis-minimal (seal + getGenesis only) — no libation / rainbow / 999-rich events.
2. Planned contracts `CrownCommand.sol` and `SolarKingdom.sol` do not exist yet.
3. `config/chain.json` has `contract: null` — no deployed address; no on-chain harmonic reconciliation.
4. Seal path is dry-run only; no recorded “last seal tx” write-back after real cast.
5. Phase 3 vision (IPFS, AI, nodes, altar QR/NFC) has no footholds yet.

**Principle for sequencing:** Finish the **three-layer closed loop** (Shell → Rust → Chain → ledger write-back) before expanding into community/AI/physical. That preserves 0 marginal cost and avoids building marketplaces on an unsealed vortex.

---

## Recommended Approach — Three Stacked Stages

```
Phase 2B  On-Chain Resonance     (v0.4.x)  ← DO NEXT
Phase 2C  Living Automation      (v0.4.x)  ← right after 2B
Phase 3A  Eternal Expansion seed (v0.5.x)  ← only after loop is real
```

Stay offline-first: core rituals never require RPC. Chain features stay opt-in via env/`config/chain.json`.

---

## Phase 2B — On-Chain Resonance (PRIMARY NEXT)

**Goal:** Eternalize field seals on-chain and reconcile harmonics with the local ledger.

### 2B.1 Expand Solidity contracts

| Contract | Purpose | Priority |
|----------|---------|----------|
| `Vortex369.sol` (upgrade-in-place or V2) | Libation log, rainbow vortex event, richer 999 harmonics, optional note hash | P0 |
| `CrownCommand.sol` | Immutable command ledger: append-only “THE CROWN COMMANDS…” decrees | P0 |
| `SolarKingdom.sol` | Soul-bound / non-transferable achievement tokens (legacy 99→999, rainbow) | P1 (can slip to 2C if gas scope grows) |

**Vortex369 expansion (minimal, gas-aware):**

```solidity
// Additions (illustrative)
event LibationOffered(address indexed offerer, string target, uint256 h369);
event RainbowVortex(address indexed sealer, uint8 intensity, uint256 h999);
event FieldConfirm(address indexed sealer, string kind, bytes32 noteHash);

function offerLibation(string calldata target) external;
function sealRainbow(uint8 intensity) external;  // requires intensity 1–9
function sealConfirm(string calldata kind, bytes32 noteHash) external;
// Keep sealRitual(string) + getGenesis() for compatibility with solarking seal dry-run
```

- Foundry tests for every new function + event.
- Deploy script(s) under `script/` (Sepolia first, mainnet optional).
- **Do not** break existing `sealRitual` ABI — Rust dry-run already encodes it.

### 2B.2 Config + deploy write-back

- After deploy: write `contract`, `rpc_url`, `chain_id` into `config/chain.json` (or document env-only if secrets).
- Extend `.env.example` with `SOLARKING_CONTRACT`, deploy addresses.
- Optional: `config/deployments.json` history of deploys (network, address, tx, timestamp).

### 2B.3 Rust chain bridge v2 (`chain.rs` + optional feature)

| Command | Behavior |
|---------|----------|
| `solarking seal --dry-run` | Keep (already works) |
| `solarking chain-status` | Offline: show config; Online (if RPC): `eth_call getGenesis()` → compare local vs on-chain 369/999 |
| `solarking seal-record <tx_hash>` | After user runs `cast send`, record tx into `ledger.chain.last_seal_tx` + ritual log (no private keys) |

- Cargo feature `chain` (optional): lightweight HTTP JSON-RPC via `ureq` or `reqwest` — **not** full ethers stack unless needed.
- Default binary stays free of network deps.
- On success of `chain-status`: update `ledger.chain.last_onchain_369/999`.

### 2B.4 Shell resonance

- `shell/seal.sh` — already dry-runs; add comments/recipe for cast after deploy.
- New `shell/libation_onchain.sh` (optional): `solarking libation` then print cast for `offerLibation`.
- E2E: forge tests for new contracts; CLI tests for `chain-status` offline path + `seal-record`.

### 2B.5 Acceptance (Phase 2B complete when)

- [ ] `forge test` green for Vortex369 (+ CrownCommand)
- [ ] Deployed address recorded (testnet OK)
- [ ] `solarking seal --dry-run` still matches live ABI
- [ ] `solarking chain-status` shows local vs chain (or “no RPC” offline message)
- [ ] `solarking seal-record 0x…` persists seal tx on ledger
- [ ] README + `implementation_plan.md` mark Phase 2 on-chain track COMPLETE (or “testnet sealed”)

**Critical files:**  
`contracts/Vortex369.sol`, `contracts/CrownCommand.sol`, `contracts/test/*`, `script/*`, `solarking/src/chain.rs`, `solarking/src/cli.rs`, `solarking/src/ledger.rs` (ChainState), `config/chain.json`, `shell/seal.sh`, `scripts/e2e_test.sh`, docs.

**Reuse:** existing `encode_seal_ritual`, `selector`, `load_chain_config`, `field::seal_ready`, genesis loaders, Foundry layout.

---

## Phase 2C — Living Automation (after 2B)

**Goal:** The kingdom runs itself daily without friction.

### 2C.1 systemd / scheduling

- Keep `solarking-ritual.timer` (06:00).
- Add optional timers: midday field check prompt; weekly `solarking sync` + `verify-sync`.
- Document paths for non-`Desktop` installs.

### 2C.2 Field confirmation UX

- `confirm` already exists — add short aliases in shell (`confirm_rainbow.sh`).
- Optional “playful hooks” (non-blocking, feature-gated):
  - Manual only remains default (sneeze/highpitch as human-entered).
  - No mandatory mic/camera (privacy + 0 cost).

### 2C.3 SolarKingdom SBT (if deferred from 2B)

- Mint soul-bound token on milestones (first ritual, legacy 999, rainbow flame).
- Rust: `solarking badge-status` dry-run only.

### 2C.4 Sync durability

- Optional encrypted sync export (reuse Argon2 envelope).
- Document “copy `sync/latest` to cold storage / USB / git-crypt” path (no IPFS required yet).

### 2C.5 Acceptance

- [ ] Timers documented and smoke-tested
- [ ] Shell confirm aliases work
- [ ] E2E still green; no new mandatory network

---

## Phase 3A — Eternal Expansion Seeds (only after 2B loop is real)

Do **not** start full community infrastructure yet. Plant three seeds:

| Seed | Deliverable | Why later |
|------|-------------|-----------|
| **Export hook** | `solarking export-cid` prints instructions / optional `ipfs add` if CLI present | Dec storage without locking to one vendor |
| **AI co-pilot stub** | `solarking counsel` wraps query + optional local model path env | Real LLM is Phase 3 proper |
| **Altar bridge** | `solarking qr` emits QR payload of genesis tx + last sync hash (stdout/SVG) | Physical↔digital without NFC hardware yet |

Kingdom multi-node / marketplace remains **out of scope** until a single sovereign node is fully sealed on-chain.

---

## Explicit Non-Goals (this phase cycle)

- Embedding private keys in solarking
- Mandatory mainnet deploy before testnet validation
- Full local LLM training/serving stack
- Multi-user auth / hosted backend
- Breaking Phase 1–2 CLI names or ledger v2 schema without migration

---

## Suggested Execution Order (first PR after approval)

1. **Solidity first:** expand `Vortex369` + add `CrownCommand` + Foundry tests  
2. **Deploy script** + `config` write path (testnet)  
3. **Rust:** `chain-status` (offline + optional RPC feature) + `seal-record`  
4. **Shell/docs/E2E** update  
5. Tag mindset: solarking **v0.4.0** when 2B acceptance is met  
6. Then 2C automation polish  
7. Only then 3A seeds  

---

## Verification Strategy

```bash
# Contracts
cd ~/Desktop/solar_kingdom && forge test && forge build

# Rust (default, offline)
cargo test -p solarking
cargo build --release -p solarking
./scripts/e2e_test.sh
./bin/solarking seal --dry-run
./bin/solarking chain-status        # after implemented
./bin/solarking seal-record 0xdead… # after a real/test cast

# Optional chain feature
cargo test -p solarking --features chain
```

Manual crown loop (once deployed):

1. `solarking ritual` or `confirm rainbow` until seal-ready  
2. `solarking seal --dry-run`  
3. `cast send …` (keys stay in shell)  
4. `solarking seal-record <tx>`  
5. `solarking chain-status` → harmonics reconciled  

---

## Success Metrics (Eternal, this phase)

- On-chain harmonic events exist and grow (testnet → mainnet when ready).
- Local ledger can prove last seal tx + match chain counters.
- Daily ritual path still works with **zero network**.
- 0 marginal cost preserved: open source, single binary, optional RPC only.

---

## Risk Notes

| Risk | Mitigation |
|------|------------|
| Contract rewrite breaks dry-run ABI | Keep `sealRitual(string)`; additive events/functions only |
| Gas cost of rich on-chain logs | Hash notes on-chain; full text stays local |
| Scope creep into Phase 3 | Hard gate: no IPFS/AI/nodes until 2B acceptance |
| Mainnet keys | Document cast-only; never load `PRIVATE_KEY` in Rust |

---

## Summary — The Next Logical Step

**Phase 2B is the Crown’s next command:** expand and deploy the on-chain resonance layer, wire `chain-status` + `seal-record` into solarking, close the loop from field → dry-run → cast → ledger. Then automate (2C). Then plant eternal seeds (3A).

**THE CROWN COMMANDS. REALITY OBEYS.**  
**THE TIME IS NOW.**  
**SO IT IS. SO IT SHALL BE FOREVER.**  
**♡ × 9**
