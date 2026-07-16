# SOLARKING Systemd Automation (Phase 2C)

Living automation for the Eternal Solar Kingdom.  
**Offline-first:** no private keys, no auto-broadcast. Rituals stay local.

**THE CROWN COMMANDS. REALITY OBEYS.**

---

## Units

| Unit | When | What |
|------|------|------|
| `solarking-ritual.timer` | Daily **06:00** | `solarking ritual` (`RITUAL_QUICK=1`) |
| `solarking-field-check.timer` | Daily **12:00** | Append `field` snapshot to `ritual_log.txt` |
| `solarking-sync.timer` | **Sunday 18:00** | `solarking sync` + `verify-sync` |

Default paths assume the repo lives at:

```text
~/Desktop/solar_kingdom
```

---

## Install (user systemd)

```bash
cd ~/Desktop/solar_kingdom   # or your clone path
./scripts/install_systemd.sh
```

Manual install:

```bash
mkdir -p ~/.config/systemd/user
cp systemd/solarking-*.service systemd/solarking-*.timer ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now solarking-ritual.timer
systemctl --user enable --now solarking-field-check.timer
systemctl --user enable --now solarking-sync.timer
systemctl --user list-timers --all | grep solarking
```

---

## Non-Desktop / custom path

If the repo is not at `~/Desktop/solar_kingdom`:

```bash
# Option A — install script
SOLARKING_ROOT=/path/to/solar_kingdom ./scripts/install_systemd.sh

# Option B — user drop-in
systemctl --user edit solarking-ritual.service
```

Drop-in example:

```ini
[Service]
Environment=SOLARKING_ROOT=/home/YOU/code/solar_kingdom
WorkingDirectory=/home/YOU/code/solar_kingdom
ExecStart=
ExecStart=/home/YOU/code/solar_kingdom/bin/solarking ritual
```

Apply the same idea for sync/field services.

---

## Verify

```bash
systemctl --user status solarking-ritual.timer
systemctl --user list-timers | grep solarking
# Dry-run oneshot
systemctl --user start solarking-field-check.service
```

---

## Uninstall

```bash
systemctl --user disable --now solarking-ritual.timer solarking-field-check.timer solarking-sync.timer
rm -f ~/.config/systemd/user/solarking-*.{service,timer}
systemctl --user daemon-reload
```

**Never** put `PRIVATE_KEY` or RPC secrets into unit files.
