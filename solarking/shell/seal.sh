#!/bin/bash
# Wrapper — run from solarking/ directory
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
exec "$ROOT/shell/seal.sh" "$@"
