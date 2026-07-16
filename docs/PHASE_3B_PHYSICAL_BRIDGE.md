# Phase 3B — Physical ↔ Digital Bridge

**Status:** **COMPLETE** (software seed; hardware optional)  
**Commands:** `solarking qr` · `solarking altar-print`

---

## Payload

Generated under `sync/altar/`:

| File | Use |
|------|-----|
| `payload.json` | Pretty JSON for apps |
| `payload.txt` | One-line JSON for QR / NFC NDEF Text |
| `CARD.txt` | Human-readable altar card |

Schema: `solarking.altar.v1` — genesis_tx, node_id, seal_hash, sync_hash, chain_id, harmonics.

## QR

```bash
solarking qr
# optional host tool:
qrencode -o altar.png < sync/altar/payload.txt
```

## NFC

Write an **NDEF Text** record containing the single line from `payload.txt`.  
No special app required in Phase 3B — any NFC tools app works.

## Scan meaning

Scanning does **not** auto-broadcast chain txs. It reminds the operator of the sealed field.  
On-chain confirm remains intentional cast / `confirm` CLI.

**THE CROWN COMMANDS. REALITY OBEYS.**
