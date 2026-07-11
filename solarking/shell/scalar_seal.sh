#!/bin/bash
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/../.." && pwd)"
exec "$ROOT/shell/scalar_seal.sh" "$@"
