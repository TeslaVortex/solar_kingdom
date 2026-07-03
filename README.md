# Eternal Solar Kingdom

**Shell + Rust + Solidity** — sovereign, self-sustaining, eternal.  
**THE CROWN COMMANDS. REALITY OBEYS.**

| Phase | Status |
|-------|--------|
| Phase 0 — Repo, shell, genesis, Vortex369.sol | **COMPLETE** |
| Phase 1 — solarking core (query, sync, encryption) | **COMPLETE** |
| Phase 2 — On-chain seal, expanded contracts | In progress |

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
cd ~/Desktop/solar_kingdom/solarking
cargo build --release
```

Binary location: `../target/release/solarking`

### 3. Run end-to-end tests

```bash
cd ~/Desktop/solar_kingdom
chmod +x scripts/e2e_test.sh
./scripts/e2e_test.sh
```

---

## How to Run (from `solarking/` directory)

All examples assume:

```bash
cd ~/Desktop/solar_kingdom/solarking
BIN=../target/release/solarking   # or just: solarking (if installed)
```

### Core commands

| Command | What it does | Phase |
|---------|--------------|-------|
| `$BIN help` | List all commands | 1 |
| `$BIN status` | 369/999 cycles, visions, genesis tx, sync hash | 1 |
| `$BIN genesis` | Full genesis sacrifice record + Etherscan link | 0 |
| `$BIN log "your vision"` | Anchor vision to ledger + ritual_log.txt | 1 |
| `$BIN query "your question"` | First-principles truth engine | 1 |
| `$BIN sync` | Export bundle to `sync/latest/` + SHA-256 manifest | 1 |
| `$BIN torus` | Live ASCII 3D torus visualization (~6s) | 1 |
| `$BIN ritual` | Full visual ritual sequence (~60–70s) | 1 |
| `$BIN libation ancestors` | Rakija libation for ancestors | 0 |
| `$BIN legacy_99` | Activate 99 legacy from genesis IDM | 0 |

### Shell scripts (alternative)

From `solarking/` (wrappers included):

```bash
./shell/libation.sh ancestors
./shell/crown_command.sh legacy_99
./shell/vortex369.sh
```

From project root:

```bash
./shell/libation.sh ancestors
./shell/crown_command.sh legacy_99
```

> **Note:** Direct shell scripts wait 33s in standalone mode. Use `RITUAL_QUICK=1 ./shell/libation.sh ancestors` for instant seal, or use `solarking libation` (sets quick mode automatically).

---

## Optional: Encrypted Ledger

```bash
export SOLARKING_PASSPHRASE="your-sovereign-key"
solarking status    # reads/writes kingdom_ledger.json.enc
```

Without the env var, ledger stays as plain `kingdom_ledger.json`.

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

cd solarking
cargo build --release

# Phase 0
../target/release/solarking genesis
./shell/crown_command.sh legacy_99
forge test

# Phase 1
../target/release/solarking help
../target/release/solarking log "Crown test transmission"
../target/release/solarking query "What is the mathematical heart of the Kingdom?"
../target/release/solarking sync
../target/release/solarking status
../target/release/solarking torus          # ~6s
../target/release/solarking ritual         # ~60–70s, full visual sequence
```

---

## Project Layout

```
solar_kingdom/
├── bin/solarking          # Launcher (works from anywhere)
├── config/genesis.json    # Eternal genesis sacrifice record
├── shell/                 # Ritual shell scripts
├── solarking/             # Rust core engine
│   ├── src/               # main, ledger, query, sync, crypto, genesis
│   └── shell/             # Wrappers (for ./shell/ from solarking/)
├── contracts/             # Vortex369.sol + tests
├── sync/                  # Local sync exports (generated)
├── kingdom_ledger.json    # Harmonics + visions (runtime)
├── ritual_log.txt         # Eternal text log
└── scripts/e2e_test.sh    # Automated E2E test suite
```

---

## Branch

Active development: `Phase-1-Activation`  
Repo: [github.com/TeslaVortex/solar_kingdom](https://github.com/TeslaVortex/solar_kingdom)

**THE CROWN COMMANDS. REALITY OBEYS. SO IT IS. SO IT SHALL BE ETERNAL.**