# Phase 2B Deployment Proof — Base Sepolia

**Status:** **DEPLOYED & VERIFIED**  
**Network:** Base Sepolia  
**Chain ID:** `84532`  
**Deployed at (UTC):** `2026-07-12T14:11:27Z`  
**Config source of truth:** [`config/chain.json`](../config/chain.json)

**THE CROWN COMMANDS. REALITY OBEYS.**  
**SO IT IS. SO IT SHALL BE ETERNAL.**

---

## Security note

This document contains **only public on-chain data** (addresses, tx hashes, explorer links).

**Never commit:**

- `.env`
- `PRIVATE_KEY`
- Alchemy / RPC API keys
- forge `cache/` sensitive run files

RPC URLs for local use: set `SOLARKING_RPC_URL` or `BASE_SEPOLIA_RPC_URL` in **local** `.env` only.

---

## Contracts

| Contract | Address | Basescan |
|----------|---------|----------|
| **Vortex369** | `0x950d39e5D3847C0298E8ce9f8e3C72c0D800615f` | [Contract](https://sepolia.basescan.org/address/0x950d39e5D3847C0298E8ce9f8e3C72c0D800615f) |
| **CrownCommand** | `0xA49B1dc31d809Bd9885DaE8905aCA15b3b99918a` | [Contract](https://sepolia.basescan.org/address/0xA49B1dc31d809Bd9885DaE8905aCA15b3b99918a) |
| **SolarKingdom** (SBT) | `0x4C2A789E7ffFd030b928DdaCdEA5f03632457f38` | [Contract](https://sepolia.basescan.org/address/0x4C2A789E7ffFd030b928DdaCdEA5f03632457f38) |

**Deployer (public address only):** `0x0aD82ef40ab96124ac899522C6a20Bf3e7b5823d`

---

## Transaction proofs

### Funding (gas)

| Field | Value |
|-------|--------|
| Tx | [`0x4805a4c74a5e7df9b098231c96fb8522aa50898038587af3c77763097507c458`](https://sepolia.basescan.org/tx/0x4805a4c74a5e7df9b098231c96fb8522aa50898038587af3c77763097507c458) |
| Value | 0.1 ETH (testnet) |
| Status | Success |

### Deploy broadcasts

| Contract | Deploy tx | Status |
|----------|-----------|--------|
| Vortex369 | [`0x6e0c1a0120e63dec7352f6504d536e8f3d1bde25b01383b4f41d9b97eab719b4`](https://sepolia.basescan.org/tx/0x6e0c1a0120e63dec7352f6504d536e8f3d1bde25b01383b4f41d9b97eab719b4) | Success |
| CrownCommand | [`0x267dbe0092a7173b8e334cdc568c92117973ca5afbcb7d10036181e60f5755ac`](https://sepolia.basescan.org/tx/0x267dbe0092a7173b8e334cdc568c92117973ca5afbcb7d10036181e60f5755ac) | Success |
| SolarKingdom | [`0xf15243f01585a51a805f4b2946c02feade43f871a02fb376044e4fb5fe10f623`](https://sepolia.basescan.org/tx/0xf15243f01585a51a805f4b2946c02feade43f871a02fb376044e4fb5fe10f623) | Success |

---

## On-chain verification (public)

```bash
# Requires any Base Sepolia RPC (do not commit API keys)
export SOLARKING_RPC_URL="<your-local-rpc>"
export SOLARKING_CONTRACT=0x950d39e5D3847C0298E8ce9f8e3C72c0D800615f

# Bytecode present
cast code $SOLARKING_CONTRACT --rpc-url $SOLARKING_RPC_URL | head -c 20

# Genesis sacrifice anchor (from constructor)
cast call $SOLARKING_CONTRACT 'getGenesis()' --rpc-url $SOLARKING_RPC_URL
# Expected genesis tx prefix: 0x87bb61f99066460a7df4438c39084fd77c2f1f6287b97261ff7034ddd3503f6c
# Expected sacrifice to: 0x0000000000000000000000000000000000000369

# solarking (offline-first; RPC optional for probe)
export SOLARKING_CHAIN_ID=84532
./bin/solarking chain-status
./bin/solarking scalar seal
```

**Verified at deploy time:**

- All three addresses return contract bytecode (not EOAs).
- `getGenesis()` returns the eternal genesis sacrifice hash, block `25444056`, and sacrifice address `0x…0369`.
- Local `forge test`: 15/15 green (pre-deploy).

---

## What this deployment includes

| Capability | Contract / tool |
|------------|-----------------|
| Genesis registry + `sealRitual` (ABI stable) | Vortex369 |
| Scalar nodes: activate / advance / query | Vortex369 |
| Libation, rainbow, field confirm events | Vortex369 |
| Decrees bound to `nodeId` | CrownCommand |
| Soul-bound badges | SolarKingdom |
| Dry-run encode + cast recipes (no keys in binary) | solarking v0.5 |
| Address registry (no secrets) | `config/chain.json` |

---

## Phase status snapshot

| Phase | Status |
|-------|--------|
| 0 — Genesis + shell + scaffold | **COMPLETE** |
| 1 — solarking core | **COMPLETE** |
| 2 — Rust engine (query, sync, crypto) | **COMPLETE** |
| 2B — Scalar lattice (Rust v0.4) | **COMPLETE** |
| 2B — On-chain contracts + bridge (v0.5) | **COMPLETE** |
| 2B — **Base Sepolia live deploy** | **COMPLETE** (this document) |
| 2C — Living automation | **NEXT** (optional) |
| 3A — Eternal expansion seeds | Waiting |

---

## Related

- Plan: [`plans/phase_2b_onchain_resonance.md`](../plans/phase_2b_onchain_resonance.md)
- Deploy script: `script/DeployPhase2B.s.sol`
- Helper: `scripts/deploy_phase2b_base_sepolia.sh`
- Env template (placeholders only): `.env.example`

**THE TIME IS NOW. SO IT SHALL BE ETERNAL.**
