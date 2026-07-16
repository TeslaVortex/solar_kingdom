# Phase 2C — Living Automation

**Status:** **COMPLETE**  
**solarking:** **v0.6.0**  
**Seal:** THE CROWN COMMANDS. REALITY OBEYS.

---

## Deliverables

### 1. Systemd scheduling

| Timer | Schedule | Action |
|-------|----------|--------|
| `solarking-ritual.timer` | Daily 06:00 | Full ritual (`RITUAL_QUICK=1`) |
| `solarking-field-check.timer` | Daily 12:00 | Field snapshot → `ritual_log.txt` |
| `solarking-sync.timer` | Sunday 18:00 | `sync` + `verify-sync` |

Install:

```bash
./scripts/install_systemd.sh
# custom path:
SOLARKING_ROOT=/path/to/solar_kingdom ./scripts/install_systemd.sh
```

Docs: [`systemd/README.md`](../systemd/README.md)

### 2. Field confirmation aliases

```bash
./shell/confirm_rainbow.sh [note]
./shell/confirm_oracle.sh [note]
./shell/confirm_grid.sh [note]
./shell/confirm_sneeze.sh [note]
./shell/confirm_highpitch.sh [note]
```

Manual only — no mic/camera (privacy + 0 cost).

### 3. Badge status (SBT dry-run)

```bash
./bin/solarking badge-status
./bin/solarking badge-status --json
```

Eligibility for SolarKingdom kinds (NodeGuardian, LatticePhase, RainbowLattice, Legacy999).  
Mint remains external `cast send` with deployer key.

### 4. Sync durability (cold export)

```bash
./bin/solarking cold-export /media/usb/kingdom_backup
SOLARKING_PASSPHRASE=… ./bin/solarking cold-export /media/usb/vault --encrypt
```

Copies latest sync bundle + manifest + `COLD_EXPORT_README.txt` to offline media.  
Restore: `solarking import-sync <kingdom_export_*.json>`.

### 5. On-chain libation recipe (optional shell)

```bash
./shell/libation_onchain.sh ancestors
```

Local libation + printed `offerLibation` cast recipe (never auto-broadcast).

---

## Acceptance

- [x] Timers unit files + install script + non-Desktop path docs  
- [x] Confirm shell aliases  
- [x] `badge-status`  
- [x] `cold-export` (+ optional encrypt)  
- [x] E2E coverage for Phase 2C artifacts  
- [x] Documentation updated  

**No private keys in units or scripts. Offline-first preserved.**

**SO IT IS. SO IT SHALL BE ETERNAL.**
