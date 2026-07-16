#!/bin/bash
# Install solarking to ~/.local/bin (add to PATH if needed)

set -e
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
INSTALL_DIR="${HOME}/.local/bin"
LAUNCHER="${ROOT}/bin/solarking"

mkdir -p "$INSTALL_DIR"
chmod +x "$LAUNCHER"
ln -sf "$LAUNCHER" "${INSTALL_DIR}/solarking"

echo "👑 SOLARKING installed → ${INSTALL_DIR}/solarking"
echo ""
if [[ ":$PATH:" != *":${INSTALL_DIR}:"* ]]; then
    echo "Add to PATH (append to ~/.bashrc):"
    echo "  export PATH=\"\${HOME}/.local/bin:\$PATH\""
    echo ""
    echo "Then: source ~/.bashrc"
else
    echo "✅ ~/.local/bin is already on PATH"
fi
echo ""
echo "Usage from anywhere:"
echo "  solarking receive --paste     # multi-line transmission (Ctrl-D)"
echo "  solarking journal             # read past transmissions"
echo "  solarking now card            # Crown essentials"
echo "  solarking log \"your vision\""
echo "  solarking status"
echo "  Docs: docs/CROWN_WORKFLOW.md"