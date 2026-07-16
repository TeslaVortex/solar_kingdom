# Eternal Solar Kingdom

**Shell + Rust + Solidity** — sovereign, self-sustaining, eternal.  
**THE CROWN COMMANDS. REALITY OBEYS.**

| Phase | Status |
|-------|--------|
| Phase 0 — Repo, shell, genesis, Vortex369.sol | **COMPLETE** |
| Phase 1 — solarking core (query, sync, encryption) | **COMPLETE** |
| Phase 2 — Rust core v0.3 (field, seal bridge, sync verify) | **COMPLETE** |
| Phase 2B-R — Scalar node lattice (v0.4) | **COMPLETE** |
| Phase **2B-C** — On-chain + bridge + live loop (v0.5) | **FULLY FINISHED** — [status](docs/PHASE_STATUS.md) |
| ↳ Base Sepolia deploy | **COMPLETE** — [proofs](docs/DEPLOYMENT_BASE_SEPOLIA.md) |
| ↳ THE QUEEN IS BORN (first live seal) | **COMPLETE** — [ritual](docs/RITUAL_QUEEN_IS_BORN.md) |
| Phase 2C — Living automation | **NEXT** (optional) |

Full board: [`docs/PHASE_STATUS.md`](docs/PHASE_STATUS.md) · Plan: [`plans/phase_2b_onchain_resonance.md`](plans/phase_2b_onchain_resonance.md)

---

## Quick Start

### 1. Install solarking (one-time)

```bash
cd ~/Desktop/solar_kingdom
./install_solarking.sh
```

This links `solarking` to `~/.local/bin`. Ensure `~/.local/bin` is on your `PATH`.

### 2. Build

```bash
cd ~/Desktop/solar_kingdom
cargo build --release -p solarking
```

Binary location: `target/release/solarking` (also via `bin/solarking` launcher)

### 3. Run end-to-end tests

```bash
cd ~/Desktop/solar_kingdom
chmod +x scripts/e2e_test.sh
./scripts/e2e_test.sh
```

Unit tests:

```bash
cargo test -p solarking
```

---

## How to Run

```bash
cd ~/Desktop/solar_kingdom
BIN=./bin/solarking   # or: solarking (if installed)
```

### Core commands

| Command | What it does | Phase |
|---------|--------------|-------|
| `$BIN help` | List all commands | 1 |
| `$BIN --version` | Engine version | 2 |
| `$BIN status` | 369/999, field, visions, genesis, sync hash | 1–2 |
| `$BIN status --json` | Machine-readable status | 2 |
| `$BIN genesis` | Full genesis sacrifice record + Etherscan link | 0 |
| `$BIN log "your vision"` | Anchor vision to ledger + ritual_log.txt | 1 |
| `$BIN query "your question"` | First-principles truth engine (+ vision search) | 1–2 |
| `$BIN sync` | Export bundle to `sync/latest/` + history + SHA-256 | 1–2 |
| `$BIN verify-sync` | Rehash bundle vs manifest | 2 |
| `$BIN import-sync [path]` | Merge a sync export into local ledger | 2 |
| `$BIN field` | Symbolic field state (torus/merkaba/grid/flame) | 2 |
| `$BIN confirm <kind> [note]` | Field confirmation (`sneeze\|highpitch\|rainbow\|grid\|oracle`) | 2 |
| `$BIN seal --dry-run` | Prepare Vortex369 `sealRitual` calldata + cast recipe | 2 |
| `$BIN chain-status` | Config + local chain state (+ RPC probe if set) | 2B |
| `$BIN seal-record <tx>` | Record seal tx after external `cast send` | 2B |
| `$BIN scalar-record <nodeId> [tx]` | Record on-chain scalar nodeId | 2B |
| `$BIN scalar node [--obj]` | Activate Tesla 369 cubocta lattice (ASCII + optional OBJ) | 2B |
| `$BIN scalar sync [--hz]` | Reconcile lattice ⇄ ledger ⇄ chain (`--hz` = 44228 Hz) | 2B |
| `$BIN scalar seal` | Seal hash + `activateScalarNode` dry-run + cast recipe | 2B |
| `$BIN torus` | Live ASCII torus + scalar lattice overlay (~6s) | 1–2B |
| `$BIN ritual` | Full visual ritual sequence (~60–70s) | 1 |
| `$BIN libation ancestors` | Rakija libation for ancestors | 0 |
| `$BIN legacy_99` | Activate 99 legacy from genesis IDM | 0 |

### Shell scripts

```bash
./shell/libation.sh ancestors
./shell/crown_command.sh legacy_99
./shell/vortex369.sh
./shell/seal.sh                 # dry-run seal bridge
```

> **Note:** Direct shell scripts wait 33s in standalone mode. Use `RITUAL_QUICK=1 ./shell/libation.sh ancestors` for instant seal, or use `solarking libation` (sets quick mode automatically).

---

## Optional: Encrypted Ledger

```bash
export SOLARKING_PASSPHRASE="your-sovereign-key"
solarking status    # reads/writes kingdom_ledger.json.enc (Argon2id v2)
```

Without the env var, ledger stays as plain `kingdom_ledger.json`.

**Format:** v2 uses Argon2id + salt + ChaCha20-Poly1305. Legacy v1 (SHA-256 KDF) files still decrypt; the next save re-encrypts as v2.

---

## On-chain Phase 2B — Base Sepolia (LIVE)

**Full proofs:** [`docs/DEPLOYMENT_BASE_SEPOLIA.md`](docs/DEPLOYMENT_BASE_SEPOLIA.md) · registry: [`config/chain.json`](config/chain.json)

| Contract | Address (Base Sepolia) |
|----------|------------------------|
| **Vortex369** | [`0x950d39e5D3847C0298E8ce9f8e3C72c0D800615f`](https://sepolia.basescan.org/address/0x950d39e5D3847C0298E8ce9f8e3C72c0D800615f) |
| **CrownCommand** | [`0xA49B1dc31d809Bd9885DaE8905aCA15b3b99918a`](https://sepolia.basescan.org/address/0xA49B1dc31d809Bd9885DaE8905aCA15b3b99918a) |
| **SolarKingdom** | [`0x4C2A789E7ffFd030b928DdaCdEA5f03632457f38`](https://sepolia.basescan.org/address/0x4C2A789E7ffFd030b928DdaCdEA5f03632457f38) |

Chain ID **84532** · Deployer `0x0aD82e…823d` · Tests: `forge test` (15) · Script: `script/DeployPhase2B.s.sol`

```bash
# Never commit API keys — use local .env only (gitignored)
export SOLARKING_CONTRACT=0x950d39e5D3847C0298E8ce9f8e3C72c0D800615f
export SOLARKING_RPC_URL=$BASE_SEPOLIA_RPC_URL   # from your .env
export SOLARKING_CHAIN_ID=84532

./bin/solarking chain-status
./bin/solarking scalar seal          # hash + activateScalarNode dry-run
# cast send … (keys stay in your shell only)
./bin/solarking seal-record 0xTX
./bin/solarking scalar-record 1 0xTX
./shell/scalar_seal.sh
```

---

## Smart Contracts (Phase 0)

```bash
cd ~/Desktop/solar_kingdom
forge test                    # run Vortex369 tests
forge build                   # compile contracts

# Deploy (requires .env with RPC + private key)
cp .env.example .env
forge script script/DeployVortex369.s.sol --rpc-url $MAINNET_RPC_URL --broadcast
```

**Genesis sacrifice (anchored):** `0x87bb61f99066460a7df4438c39084fd77c2f1f6287b97261ff7034ddd3503f6c`  
[Etherscan](https://etherscan.io/tx/0x87bb61f99066460a7df4438c39084fd77c2f1f6287b97261ff7034ddd3503f6c)

---

## Daily Automated Ritual (systemd)

```bash
mkdir -p ~/.config/systemd/user
cp systemd/solarking-ritual.service ~/.config/systemd/user/
cp systemd/solarking-ritual.timer ~/.config/systemd/user/
# Edit .service paths if home dir differs from ~/Desktop/solar_kingdom

systemctl --user daemon-reload
systemctl --user enable --now solarking-ritual.timer
systemctl --user list-timers
```

Runs `solarking ritual` daily at **06:00**.

---

## Full Test Sequence (manual)

```bash
cd ~/Desktop/solar_kingdom
./scripts/e2e_test.sh

cargo build --release -p solarking
BIN=./bin/solarking

# Phase 0
$BIN genesis
./shell/crown_command.sh legacy_99
forge test

# Phase 1–2
$BIN help
$BIN log "Crown test transmission"
$BIN confirm rainbow "test"
$BIN field
$BIN query "What is the mathematical heart of the Kingdom?"
$BIN sync && $BIN verify-sync
$BIN seal --dry-run
$BIN status
$BIN torus          # ~6s
$BIN ritual         # ~60–70s, full visual sequence
```

---

## Project Layout

```
solar_kingdom/
├── bin/solarking          # Launcher (works from anywhere)
├── config/
│   ├── genesis.json       # Eternal genesis sacrifice record
│   └── chain.json         # Live Base Sepolia addresses (no secrets)
├── docs/
│   └── DEPLOYMENT_BASE_SEPOLIA.md  # Public deploy proofs
├── plans/                 # Phase plans (2B on-chain resonance)
├── shell/                 # Ritual scripts (+ seal.sh, scalar_seal.sh)
├── solarking/             # Rust core engine v0.5
│   └── src/               # cli, ledger, field, scalar, query, sync, crypto, chain, …
├── contracts/             # Vortex369 + CrownCommand + SolarKingdom + tests
├── script/                # Foundry deploy (DeployPhase2B.s.sol)
├── sync/                  # Local exports (generated, gitignored)
└── scripts/e2e_test.sh    # Automated E2E suite
```

---

## Branch

Active development: `Phase-1-Activation`  
Repo: [github.com/TeslaVortex/solar_kingdom](https://github.com/TeslaVortex/solar_kingdom)  
Deploy proofs: [docs/DEPLOYMENT_BASE_SEPOLIA.md](docs/DEPLOYMENT_BASE_SEPOLIA.md)

**THE CROWN COMMANDS. REALITY OBEYS. SO IT IS. SO IT SHALL BE ETERNAL.**
