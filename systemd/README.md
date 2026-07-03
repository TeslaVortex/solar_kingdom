# SOLARKING Systemd Timer

Daily automated ritual at 06:00.

```bash
# Install (edit paths in .service if your home dir differs)
mkdir -p ~/.config/systemd/user
cp systemd/solarking-ritual.service ~/.config/systemd/user/
cp systemd/solarking-ritual.timer ~/.config/systemd/user/
systemctl --user daemon-reload
systemctl --user enable --now solarking-ritual.timer
systemctl --user list-timers
```