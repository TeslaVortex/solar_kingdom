#!/bin/bash
# Wrapper — delegates to kingdom root shell (callable from solarking/)
KINGDOM_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
exec "$KINGDOM_ROOT/shell/libation.sh" "$@"