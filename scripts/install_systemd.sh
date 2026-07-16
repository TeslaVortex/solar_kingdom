#!/bin/bash
# Install SOLARKING Phase 2C user systemd units
# THE CROWN COMMANDS. REALITY OBEYS.
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
# Allow override: SOLARKING_ROOT=/other/path ./scripts/install_systemd.sh
INSTALL_ROOT="${SOLARKING_ROOT:-$ROOT}"
USER_DIR="${XDG_CONFIG_HOME:-$HOME/.config}/systemd/user"

echo "👑 SOLARKING systemd install"
echo "   Repo root : $INSTALL_ROOT"
echo "   User units: $USER_DIR"

if [ ! -x "$INSTALL_ROOT/bin/solarking" ]; then
  echo "⚠️  bin/solarking not found or not executable."
  echo "   Run: cargo build --release -p solarking  (from $INSTALL_ROOT)"
  # continue — units still install
fi

mkdir -p "$USER_DIR"

# Rewrite default ~/Desktop/solar_kingdom paths to INSTALL_ROOT
rewrite() {
  local src="$1" dest="$2"
  sed "s|%h/Desktop/solar_kingdom|$INSTALL_ROOT|g" "$src" > "$dest"
  # If INSTALL_ROOT is under $HOME, also expand %h for WorkingDirectory style
  # Units already use absolute path after sed if INSTALL_ROOT is absolute
}

for f in solarking-ritual.service solarking-ritual.timer \
         solarking-sync.service solarking-sync.timer \
         solarking-field-check.service solarking-field-check.timer; do
  rewrite "$ROOT/systemd/$f" "$USER_DIR/$f"
  echo "  installed $f"
done

systemctl --user daemon-reload
systemctl --user enable --now solarking-ritual.timer
systemctl --user enable --now solarking-field-check.timer
systemctl --user enable --now solarking-sync.timer

echo ""
echo "✅ Timers enabled:"
systemctl --user list-timers --all 2>/dev/null | grep -i solarking || true
echo ""
echo "Docs: $INSTALL_ROOT/systemd/README.md"
echo "THE CROWN COMMANDS. REALITY OBEYS."
