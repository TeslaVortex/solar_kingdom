# Crown Workflow — Receive · Read · Execute

**Engine:** solarking **v0.9**  
**Canonical state:** kingdom root `kingdom_ledger.json` + `ritual_log.txt`  
(not the stale copies under `solarking/`)

**THE CROWN COMMANDS. REALITY OBEYS. NOW.**  
**I do not chase — I receive.**

---

## 1. Paste whole transmissions

```bash
# Interactive multi-line paste (Ctrl-D when done)
solarking receive --paste

# Pipe (clipboard tools vary by OS)
cat note.txt | solarking receive -
printf 'Line A\nLine B\n' | solarking receive -

# From file
solarking receive --file ~/Desktop/transmission.txt

# Optional: title, field confirm, offline counsel
solarking receive --file note.txt --title "MORNING TX" --confirm oracle --counsel

# One-liner still works
solarking log "short vision — crown commands reality obeys"
```

**What `receive` does**

1. Archives raw body → `sync/transmissions/YYYYMMDD_HHMMSS_<sha8>.txt` (+ `.meta.json`)
2. Anchors vision in the ledger (multi-line preserved)
3. Appends `ritual_log.txt` (block format for multi-line)
4. Prints index, tags, archive path

---

## 2. Read

```bash
solarking journal                 # last 10 visions
solarking journal list --last 5
solarking journal show            # full latest
solarking journal show 2          # second latest
solarking journal search queen
solarking journal log --tail 40   # ritual_log.txt
solarking journal files           # transmission archives
solarking journal open            # print latest archive body
```

---

## 3. Execute simply (`now` recipes)

```bash
solarking now card       # Crown card — 9 essential commands
solarking now morning    # status + field + last 3 visions
solarking now receive    # paste mode
solarking now seal       # seal dry-run + chain-status + badge-status
solarking now sync       # sync + verify-sync
solarking now pulse      # scalar node + scalar sync
```

---

## 4. Daily loop (minimal)

```bash
solarking now morning
# … live the field …
solarking receive --paste          # or: --file / path
solarking journal show
solarking counsel "what wants to be received next"
solarking now sync                 # when ready to export
```

---

## Notes

- **Offline-first.** No keys in solarking. Mainnet is Phase **3F LAST**.
- Multi-line log uses block markers in `ritual_log.txt`:
  `=== TRANSMISSION BEGIN … ===` … `=== TRANSMISSION END sha=… ===`
- Install: `./install_solarking.sh` · launcher sets `SOLARKING_ROOT` to repo root.

**SO IT IS. SO IT SHALL BE ETERNAL.**
