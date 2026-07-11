# Plan: Phase 2B — On-Chain Resonance + Scalar Node Anchoring

**Repo:** [TeslaVortex/solar_kingdom](https://github.com/TeslaVortex/solar_kingdom)  
**Branch:** `Phase-1-Activation`  
**Plan path:** [`plans/phase_2b_onchain_resonance.md`](https://github.com/TeslaVortex/solar_kingdom/blob/Phase-1-Activation/plans/phase_2b_onchain_resonance.md)  
**Doctrine:** First principles • 0 marginal cost • Shell + Rust + Solidity resonance  
**Seal:** THE CROWN COMMANDS. REALITY OBEYS. SO IT IS. SO IT SHALL BE FOREVER.

**Status of this document:** PLAN ONLY — contract implementation not executed until Crown commands code.  
**Last plan update:** 2026-07-10 — Scalar Node woven into 2B on-chain architecture.

---

## Context — Where the Kingdom Stands

| Layer | Status |
|-------|--------|
| Phase 0 — Repo, shell, genesis, Vortex369 scaffold | **COMPLETE** |
| Phase 1 — solarking CLI, query, sync, encryption, systemd | **COMPLETE** |
| Phase 2 Rust track (v0.3) — field, confirm, ledger v2, seal dry-run, Argon2, verify-sync | **COMPLETE** (`d962c74`) |
| Phase 2B Rust geometry — scalar node lattice (Tesla 369 in 3D) | **COMPLETE** (`bd1750d`, solarking **v0.4.0**) |
| Phase 2B On-chain contracts — expanded Solidity + scalar anchoring | **COMPLETE** (v0.5 code; deploy when keys ready) |
| Phase 2C — Living automation | Waiting on 2B on-chain loop |
| Phase 3A — Eternal expansion seeds | Waiting on sealed chain loop |

### Already live in Rust (v0.4 — do not re-implement)

| Capability | Location |
|------------|----------|
| Nested cubocta × 6, phase 1–9, period 1296 | `solarking/src/scalar.rs` |
| Timeline frequency 44 228 Hz optional multiplier | `scalar sync --hz` |
| `scalar node [--obj]` / `scalar sync` / `scalar seal` | CLI + ledger `scalar: ScalarNodeState` |
| SHA-256 seal hash + offline cast payload | `scalar seal` → `sealRitual(string)` recipe |
| Kagome ASCII + OBJ export | `sync/scalar/scalar_node.obj` |
| Torus lattice overlay | `torus` command |

### Still open (on-chain debt)

1. `Vortex369.sol` is genesis-minimal (`sealRitual` + `getGenesis` only) — no scalar structs, libation, rainbow, or 999-rich events.
2. `CrownCommand.sol` and `SolarKingdom.sol` do not exist yet.
3. `config/chain.json` has `contract: null` — no deploy write-back.
4. No `chain-status` / `seal-record` bridge; no on-chain scalar `nodeId` in ledger.
5. No shell `scalar-seal` wrapper for activate/advance cast recipes.
6. Full lattice geometry stays off-chain (by design); chain stores hashes + phase counters only.

**Principle:** Close **Shell → Rust scalar → Solidity scalar → ledger write-back** before Phase 3. Additive ABI only. Offline-first. Gas-aware.

---

## Recommended Approach — Stacked Stages (updated)

```
Phase 2B-R  Scalar geometry in Rust          v0.4.0   COMPLETE
Phase 2B-C  On-chain contracts + scalar      v0.5.x   ← DO NEXT (this plan)
Phase 2C    Living automation                v0.5.x   after 2B-C
Phase 3A    Eternal expansion seeds          v0.6.x   after sealed loop
```

Stay offline-first: core rituals never require RPC. Chain features opt-in via `config/chain.json` / env.

---

## Phase 2B-C — On-Chain Contracts with Scalar Node (PRIMARY NEXT)

**Goal:** Eternalize field + **scalar lattice** seals on-chain; reconcile harmonics and node phase with the local ledger.

### Integration principles (Crown law)

| Rule | Detail |
|------|--------|
| **Additive only** | Never break existing `sealRitual(string)` / `getGenesis()` |
| **Dry-run compatible** | Rust encodes scalar parameters offline; cast remains external |
| **Offline-first** | Contracts opt-in; no mandatory network in default binary |
| **Gas-aware** | On-chain: phase, counters, hashes, nodeId — **not** full 3D vertex arrays |
| **Geometry home** | Full nested cubocta lattice remains in Rust ledger for verification |
| **Resonance key** | 44 228 Hz used as optional off-chain / encoding multiplier when sealing nodes (not stored as continuous audio on-chain) |

---

### 2B-C.1 Vortex369.sol upgrades (Core + Scalar)

**Keep:** `sealRitual(string)`, `getGenesis()`, genesis immutables, existing harmonic 369/999 counters.

**Add — ScalarNode struct (minimal on-chain geometry state):**

```solidity
struct ScalarNode {
    address sealer;           // who activated
    uint8 phase;              // 1–9 cycle
    uint16 shellCoherence;    // 0–6 nested shells “lit” (mirrors Rust)
    uint32 harmonicIndex;     // position in 1296 lattice [0, 1295]
    bytes32 sealHash;         // SHA-256 of off-chain node snapshot (from solarking scalar seal)
    uint64 activatedAt;       // block.timestamp
    bool active;
}
```

**New events:**

```solidity
event ScalarNodeActivated(
    address indexed sealer,
    uint256 indexed nodeId,
    uint8 phase,
    bytes32 sealHash
);
event LatticeHarmonic(
    uint256 indexed nodeId,
    uint256 shift,            // harmonicIndex or delta
    uint8 phase,
    uint256 timestamp
);
// Retain / add non-scalar resonance events:
event LibationOffered(address indexed offerer, string target, uint256 h369);
event RainbowVortex(address indexed sealer, uint8 intensity, uint256 h999);
event FieldConfirm(address indexed sealer, string kind, bytes32 noteHash);
```

**New functions:**

| Function | Behavior |
|----------|----------|
| `activateScalarNode(uint8 initialPhase, bytes32 sealHash, uint16 shellCoherence, uint32 harmonicIndex)` | Mints/activates node tied to ritual seal; emits `ScalarNodeActivated`; phase clamped 1–9 |
| `advancePhase(uint256 nodeId)` | Advances 369 phase cycle (phase = phase % 9 + 1); bumps harmonicIndex mod 1296; emits `LatticeHarmonic` |
| `queryNodeState(uint256 nodeId)` | View: returns full `ScalarNode` for off-chain verification vs Rust ledger |
| `offerLibation(string target)` | Ancestor libation counter/event (gas-light) |
| `sealRainbow(uint8 intensity)` | intensity 1–9; rainbow vortex event |
| `sealConfirm(string kind, bytes32 noteHash)` | Field confirm hash on-chain |

**Constants (mirror Rust):**

```solidity
uint8  public constant PHASE_MAX = 9;
uint16 public constant HARMONIC_PERIOD = 1296;
uint8  public constant MAX_SHELLS = 6;
// 44228 is off-chain resonance key — optional encoded into sealHash payload, not a required storage field
```

- Foundry tests for every new function + event + phase wrap 9→1 + harmonicIndex mod 1296.
- **Do not** break `sealRitual` ABI — Rust `solarking seal --dry-run` and `scalar seal` remain valid.

---

### 2B-C.2 CrownCommand.sol (new)

Immutable / append-only command ledger bound to scalar nodes.

```solidity
event DecreeSealed(
    address indexed commander,
    uint256 indexed decreeId,
    uint256 indexed nodeId,
    bytes32 decreeHash,
    uint256 timestamp
);

function sealWithScalar(uint256 nodeId, string calldata decree) external;
// Stores: decreeHash = keccak256(bytes(decree)), nodeId reference, commander, timestamp
// Requires node active on Vortex369 (interface call or same-deploy coordination)
function getDecree(uint256 decreeId) external view returns (...);
```

- Crown decrees (“THE CROWN COMMANDS…”) bound to a specific lattice `nodeId`.
- Full decree text may stay off-chain; on-chain stores hash + nodeId for integrity.

---

### 2B-C.3 SolarKingdom.sol — SBT layer (P1; may ship with 2B-C or slip to 2C)

Soul-bound (non-transferable) tokens representing activated scalar nodes or completed lattice phases.

| Badge concept | Mint trigger |
|---------------|--------------|
| **369 Node Guardian** | Successful `activateScalarNode` |
| **Lattice Phase Complete** | Phase cycle returns to 1 after full 1–9 advance set (or harmonicIndex milestone) |
| **Double-Edged Rainbow Lattice** | `sealRainbow` + scalar node active |
| **Legacy 99→999** | Off-chain legacy tier 999 + on-chain harmonic999 threshold |

- ERC-721 (or minimal custom) with `transfer` disabled / soul-bound pattern.
- Minted only upon successful on-chain seal with scalar data (not freestanding mint).
- Rust later: `solarking badge-status` dry-run only.

---

### 2B-C.4 Config + deploy write-back

- After deploy (Sepolia first): write `contract`, `rpc_url`, `chain_id` into `config/chain.json`.
- Optional `config/deployments.json` history (network, addresses for Vortex369 / CrownCommand / SolarKingdom, tx, timestamp).
- Extend `.env.example`: `SOLARKING_CONTRACT`, `SOLARKING_CROWN_COMMAND`, `SOLARKING_SBT`, RPC, chain id.
- Deploy scripts under `script/` (Foundry): `DeployVortex369.s.sol` update + `DeployCrownCommand.s.sol` (+ SBT when ready).

---

### 2B-C.5 Rust chain bridge v2 (`chain.rs` + optional `chain` feature)

| Command | Behavior |
|---------|----------|
| `solarking seal --dry-run` | Keep (existing) |
| `solarking scalar seal` | Keep — emit sealHash + payload; **extend** dry-run to print `activateScalarNode` cast calldata when ABI ready |
| `solarking chain-status` | Offline: config; Online: `eth_call getGenesis()` + optional `queryNodeState(nodeId)` vs local scalar |
| `solarking seal-record <tx_hash>` | Record tx into `ledger.chain.last_seal_tx` + ritual log |
| `solarking scalar-record <nodeId> [tx]` | Persist on-chain `nodeId` + optional tx on `ledger.scalar` / `ledger.chain` |

**Ledger extensions (additive, serde default):**

```rust
// ChainState / ScalarNodeState additions (illustrative)
pub onchain_node_id: Option<u64>,
pub last_scalar_tx: Option<String>,
```

- Encode helpers: `activateScalarNode(...)`, `advancePhase(nodeId)` selectors + ABI (mirror existing `encode_seal_ritual`).
- Optional Cargo feature `chain` for JSON-RPC; default binary stays network-free.
- **44 228 Hz:** continue as offline multiplier in `scalar sync --hz`; fold into sealHash payload string for on-chain resonance (already pattern in v0.4).

---

### 2B-C.6 Shell resonance

| Script | Role |
|--------|------|
| `shell/seal.sh` | Existing dry-run; document cast after deploy |
| `shell/scalar_seal.sh` **(new)** | `solarking scalar seal` → print activate/advance cast recipes |
| `shell/libation_onchain.sh` (optional) | Local libation + cast for `offerLibation` |

- systemd remains ritual-only; **never** auto-broadcast.

---

### 2B-C.7 Acceptance (Phase 2B-C complete when)

- [ ] `forge test` green for Vortex369 scalar + libation/rainbow/confirm (+ CrownCommand)
- [ ] `activateScalarNode` / `advancePhase` / `queryNodeState` covered
- [ ] Deployed address recorded (testnet OK) in `config/chain.json` or deployments log
- [ ] `solarking seal --dry-run` still matches live `sealRitual` ABI
- [ ] `solarking scalar seal` prints compatible activate calldata / recipe
- [ ] `solarking chain-status` offline path works; online path optional
- [ ] `solarking seal-record` / scalar nodeId write-back works
- [ ] `shell/scalar_seal.sh` present
- [ ] README + `implementation_plan.md` mark Phase 2B on-chain track COMPLETE (or “testnet sealed”)
- [ ] E2E extended without mandatory RPC

**Critical files (when implementing — not in this docs commit):**  
`contracts/Vortex369.sol`, `contracts/CrownCommand.sol`, `contracts/SolarKingdom.sol` (P1), `contracts/test/*`, `script/*`, `solarking/src/chain.rs`, `solarking/src/scalar.rs`, `solarking/src/cli.rs`, `solarking/src/ledger.rs`, `config/chain.json`, `shell/scalar_seal.sh`, `scripts/e2e_test.sh`, docs.

**Reuse:** `encode_seal_ritual`, `selector`, `keccak256`, `load_chain_config`, `ScalarNodeState`, `seal_hash`, `field::seal_ready`, genesis loaders, Foundry layout, existing e2e harness.

---

## Phase 2C — Living Automation (after 2B-C)

**Goal:** The kingdom runs itself daily without friction.

### 2C.1 systemd / scheduling

- Keep `solarking-ritual.timer` (06:00).
- Optional: weekly `solarking sync` + `verify-sync`; midday field check prompt.
- Document non-`Desktop` install paths.

### 2C.2 Field confirmation UX

- Shell aliases (`confirm_rainbow.sh`).
- Manual confirm remains default (privacy + 0 cost).

### 2C.3 SolarKingdom SBT polish (if deferred)

- Badge mint paths + `solarking badge-status` dry-run.

### 2C.4 Sync durability

- Optional encrypted sync export; cold storage / USB docs (no IPFS required yet).

### 2C.5 Acceptance

- [ ] Timers documented and smoke-tested  
- [ ] Shell aliases work  
- [ ] E2E green; no new mandatory network  

---

## Phase 3A — Eternal Expansion Seeds (only after 2B-C loop is real)

| Seed | Deliverable |
|------|-------------|
| **Export hook** | `solarking export-cid` / optional `ipfs add` if CLI present |
| **AI co-pilot stub** | `solarking counsel` wraps query + optional local model path |
| **Altar bridge** | `solarking qr` — genesis tx + last sync / scalar seal hash |

Multi-node marketplace remains **out of scope** until a single sovereign node is fully sealed on-chain.

---

## Explicit Non-Goals (this plan cycle)

- Embedding private keys in solarking  
- Mandatory mainnet before testnet validation  
- Storing full 3D vertex lattices on-chain  
- Full local LLM stack  
- Multi-user auth / hosted backend  
- Breaking Phase 1–2 CLI names or ledger schema without migration  
- **Implementing contracts in this plan-update commit** (docs only)

---

## Suggested Execution Order (when Crown commands code)

1. **Solidity:** expand `Vortex369` with `ScalarNode` + events/functions + Foundry tests  
2. **Solidity:** add `CrownCommand` (`sealWithScalar`) + tests  
3. **Solidity (P1):** `SolarKingdom` SBT minimal soul-bound mint  
4. **Deploy scripts** + config write path (Sepolia first)  
5. **Rust:** extend `chain.rs` / `scalar seal` dry-run calldata; `chain-status`; `seal-record` + nodeId  
6. **Shell:** `scalar_seal.sh` + docs/E2E  
7. Bump solarking toward **v0.5.0** when 2B-C acceptance met  
8. Then 2C automation → 3A seeds  

---

## Verification Strategy

```bash
# Contracts (after implementation)
cd ~/Desktop/solar_kingdom && forge test && forge build

# Rust (already green on v0.4 scalar)
cargo test -p solarking
cargo build --release -p solarking
./scripts/e2e_test.sh
./bin/solarking scalar node
./bin/solarking scalar sync --hz
./bin/solarking scalar seal
./bin/solarking seal --dry-run

# After chain bridge
./bin/solarking chain-status
./bin/solarking seal-record 0x…
```

**Manual crown loop (once deployed):**

1. `solarking ritual` / `confirm rainbow` / `scalar node` until seal-ready  
2. `solarking scalar seal` → copy hash/payload  
3. `cast send … activateScalarNode(...)` (keys stay in shell)  
4. `solarking seal-record <tx>` + record `nodeId`  
5. `advancePhase` on-chain as rituals continue  
6. `queryNodeState` / `chain-status` → Rust ledger reconciles  

---

## Success Metrics (Eternal)

- On-chain scalar nodes + lattice harmonics exist and grow (testnet → mainnet when ready).  
- Local ledger proves last seal tx, sealHash, and on-chain `nodeId` / phase.  
- Daily ritual path still works with **zero network**.  
- 0 marginal cost preserved: open source, single binary, optional RPC only.  
- Geometry remains mathematically coherent: Rust 1296 lattice ↔ chain phase/index counters.

---

## Risk Notes

| Risk | Mitigation |
|------|------------|
| ABI break of `sealRitual` | Additive only; Foundry + e2e dry-run guards |
| Gas cost of rich geometry | Hashes + uint phase/index only; vertices stay in Rust |
| Scalar phase drift off/on-chain | `queryNodeState` + `scalar sync` reconcile rules |
| Scope creep into Phase 3 | Hard gate: no IPFS/AI/nodes until 2B-C acceptance |
| Mainnet keys | Cast-only; never load `PRIVATE_KEY` in Rust |

---

## Summary — Crown Directive Snapshot

| Track | Status | Next action |
|-------|--------|-------------|
| Rust scalar lattice | **COMPLETE** v0.4 | Maintain; extend dry-run when ABI lands |
| On-chain Vortex369 + ScalarNode | **PLANNED** | Implement when Crown commands code |
| CrownCommand + SBT | **PLANNED** | After/with Vortex369 scalar |
| chain-status / seal-record / scalar-seal shell | **PLANNED** | After deployable ABI |

**The scalar node is no longer only geometric or energetic — this plan makes it immutable on the chain path.**  
Every ritual, every breath, every 369 cycle can anchor into the living lattice.

**THE CROWN COMMANDS THE ON-CHAIN LATTICE.**  
**REALITY OBEYS.**  
**THE TIME IS NOW.**  
**SO IT IS — FOREVER.**  
**♡ × 9**
