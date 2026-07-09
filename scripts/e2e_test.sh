#!/bin/bash
# ETERNAL SOLAR KINGDOM — End-to-End Test Suite (Phase 0 + Phase 1 + v0.3 Rust core)
# THE CROWN COMMANDS. REALITY OBEYS.

set -e
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN="${ROOT}/bin/solarking"
PASS=0
FAIL=0

pass() { echo "  ✅ $1"; PASS=$((PASS + 1)); }
fail() { echo "  ❌ $1"; FAIL=$((FAIL + 1)); }

assert_file() {
  if [ -f "$1" ]; then pass "File exists: $2"; else fail "Missing: $2 ($1)"; fi
}

assert_cmd() {
  if eval "$1" >/dev/null 2>&1; then pass "$2"; else fail "$2"; fi
}

assert_output() {
  local cmd="$1" pattern="$2" label="$3"
  if eval "$cmd" 2>&1 | grep -q "$pattern"; then pass "$label"; else fail "$label (expected: $pattern)"; fi
}

echo "👑 SOLAR KINGDOM E2E TEST — Phase 0 + Phase 1 + v0.3"
echo "Root: $ROOT"
echo ""

# ── Build ──
echo "── BUILD ──"
cd "$ROOT"
if cargo build --release -p solarking 2>&1 | tail -5 | grep -q "Finished"; then
  pass "cargo build --release"
else
  fail "cargo build --release"
fi

echo ""
echo "── UNIT TESTS ──"
if cargo test -p solarking --quiet 2>&1 | tail -3 | grep -q "passed"; then
  pass "cargo test -p solarking"
else
  # show summary line either way
  if cargo test -p solarking 2>&1 | grep -q "test result: ok"; then
    pass "cargo test -p solarking"
  else
    fail "cargo test -p solarking"
  fi
fi

# ── Phase 0: Genesis + Shell + Contracts ──
echo ""
echo "── PHASE 0: Genesis + Shell + Contracts ──"
assert_file "$ROOT/config/genesis.json" "config/genesis.json"
assert_output "grep tx_hash '$ROOT/config/genesis.json'" "0x87bb61f99066460a7df4438c39084fd77c2f1f6287b97261ff7034ddd3503f6c" "Genesis tx hash anchored"
assert_file "$ROOT/shell/crown_command.sh" "shell/crown_command.sh"
assert_file "$ROOT/shell/vortex369.sh" "shell/vortex369.sh"
assert_file "$ROOT/shell/libation.sh" "shell/libation.sh"
assert_file "$ROOT/contracts/Vortex369.sol" "contracts/Vortex369.sol"
assert_file "$ROOT/solarking/shell/libation.sh" "solarking/shell/ wrapper (libation)"

cd "$ROOT"
if forge test 2>&1 | grep -q "2 passed"; then
  pass "forge test (Vortex369 — 2 tests)"
else
  fail "forge test"
fi

RITUAL_QUICK=1 "$ROOT/shell/libation.sh" ancestors >/dev/null 2>&1 && pass "shell/libation.sh ancestors" || fail "shell/libation.sh"
RITUAL_QUICK=1 "$ROOT/shell/crown_command.sh" legacy_99 >/dev/null 2>&1 && pass "shell/crown_command.sh legacy_99" || fail "shell/crown_command.sh legacy_99"

cd "$ROOT/solarking"
RITUAL_QUICK=1 ./shell/libation.sh ancestors >/dev/null 2>&1 && pass "./shell/libation.sh from solarking/" || fail "solarking/shell wrapper"

# ── Phase 1: solarking CLI ──
echo ""
echo "── PHASE 1: solarking CLI ──"
assert_output "$BIN help" "query" "solarking help"
assert_output "$BIN --version" "solarking" "solarking --version"
assert_output "$BIN genesis" "GENESIS SACRIFICE" "solarking genesis"
assert_output "$BIN genesis" "369disclosure.eth" "genesis ENS sender"
assert_output "$BIN status" "369 Cycles" "solarking status"
assert_output "$BIN status" "Field" "status shows field snapshot"

$BIN log "E2E test vision — crown commands reality obeys" >/dev/null 2>&1 && pass "solarking log" || fail "solarking log"
assert_output "$BIN status" "E2E test vision" "vision persisted in status"

assert_output "$BIN query 'What is the heart equation?'" "Heart equation" "solarking query"
assert_output "$BIN query 'ZERO randomness truth'" "ZERO RANDOMNESS" "query — randomness principle"

$BIN sync >/dev/null 2>&1 && pass "solarking sync" || fail "solarking sync"
assert_file "$ROOT/sync/manifest.json" "sync/manifest.json"
assert_file "$ROOT/sync/latest/kingdom_export.json" "sync/latest/kingdom_export.json"
assert_output "$BIN status" "Sync hash" "sync hash in status"
assert_output "$BIN verify-sync" "SYNC VERIFIED" "solarking verify-sync"

$BIN confirm rainbow "e2e rainbow confirm" >/dev/null 2>&1 && pass "solarking confirm rainbow" || fail "solarking confirm rainbow"
assert_output "$BIN field" "FIELD STATE" "solarking field"
assert_output "$BIN seal --dry-run" "DRY RUN" "solarking seal --dry-run"
assert_output "$BIN seal --dry-run" "sealRitual" "seal calldata mentions sealRitual"

RITUAL_QUICK=1 $BIN libation ancestors >/dev/null 2>&1 && pass "solarking libation" || fail "solarking libation"
RITUAL_QUICK=1 $BIN legacy_99 >/dev/null 2>&1 && pass "solarking legacy_99" || fail "solarking legacy_99"

assert_file "$ROOT/systemd/solarking-ritual.timer" "systemd timer"
assert_file "$ROOT/implementation_plan.md" "implementation_plan.md"
assert_file "$ROOT/shell/seal.sh" "shell/seal.sh"
assert_file "$ROOT/config/chain.json" "config/chain.json"

# ── Summary ──
echo ""
echo "══════════════════════════════════════"
echo "  PASSED: $PASS   FAILED: $FAIL"
if [ "$FAIL" -eq 0 ]; then
  echo "  👑 ALL E2E TESTS PASSED — REALITY OBEYS"
  echo ""
  echo "  Manual test (optional, ~60s):"
  echo "    cd $ROOT/solarking && $BIN ritual"
  echo "    cd $ROOT/solarking && $BIN torus"
  exit 0
else
  echo "  ⚠️  SOME TESTS FAILED"
  exit 1
fi