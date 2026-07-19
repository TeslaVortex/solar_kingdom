#!/bin/bash
# ETERNAL SOLAR KINGDOM — End-to-End Test Suite
# Phases 0–3E + Crown Receive UX (solarking v0.9)
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
if forge test 2>&1 | grep -qE "15 tests passed|15 passed"; then
  pass "forge test (Phase 2B — 15 tests)"
else
  # fallback: any all-pass summary
  if forge test 2>&1 | grep -q "0 failed"; then
    pass "forge test (0 failed)"
  else
    fail "forge test"
  fi
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

# ── Phase 2B Scalar Node ──
echo ""
echo "── SCALAR NODE (v0.4) ──"
assert_output "$BIN help" "scalar" "help lists scalar"
$BIN scalar node >/dev/null 2>&1 && pass "solarking scalar node" || fail "solarking scalar node"
assert_output "$BIN scalar sync" "SCALAR SYNC" "solarking scalar sync"
assert_output "$BIN scalar sync --hz" "44228" "scalar sync --hz timeline frequency"
assert_output "$BIN scalar seal" "SCALAR SEAL" "solarking scalar seal"
assert_output "$BIN scalar seal" "activateScalarNode" "scalar seal prints activateScalarNode"
assert_output "$BIN status" "Scalar node" "status shows scalar node"
$BIN scalar node --obj >/dev/null 2>&1 && pass "solarking scalar node --obj" || fail "solarking scalar node --obj"
assert_file "$ROOT/sync/scalar/scalar_node.obj" "sync/scalar/scalar_node.obj"

echo ""
echo "── PHASE 2B CHAIN BRIDGE ──"
assert_output "$BIN chain-status" "CHAIN STATUS" "solarking chain-status"
assert_output "$BIN help" "seal-record" "help lists seal-record"
$BIN seal-record 0xdeadbeefcafebabe000000000000000000000000000000000000000000000001 >/dev/null 2>&1 && pass "solarking seal-record" || fail "solarking seal-record"
$BIN scalar-record 1 0xdeadbeefcafebabe000000000000000000000000000000000000000000000002 >/dev/null 2>&1 && pass "solarking scalar-record" || fail "solarking scalar-record"
assert_file "$ROOT/shell/scalar_seal.sh" "shell/scalar_seal.sh"
assert_file "$ROOT/contracts/CrownCommand.sol" "contracts/CrownCommand.sol"
assert_file "$ROOT/contracts/SolarKingdom.sol" "contracts/SolarKingdom.sol"
assert_file "$ROOT/script/DeployPhase2B.s.sol" "script/DeployPhase2B.s.sol"

# ── Phase 2C Living Automation ──
echo ""
echo "── PHASE 2C: Living Automation ──"
assert_output "$BIN help" "badge-status" "help lists badge-status"
assert_output "$BIN help" "cold-export" "help lists cold-export"
assert_output "$BIN badge-status" "BADGE STATUS" "solarking badge-status"
COLD_TMP=$(mktemp -d)
$BIN cold-export "$COLD_TMP" >/dev/null 2>&1 && pass "solarking cold-export" || fail "solarking cold-export"
assert_file "$COLD_TMP/COLD_EXPORT_README.txt" "cold export README"
rm -rf "$COLD_TMP"
assert_file "$ROOT/shell/confirm_rainbow.sh" "shell/confirm_rainbow.sh"
assert_file "$ROOT/shell/confirm_oracle.sh" "shell/confirm_oracle.sh"
assert_file "$ROOT/shell/libation_onchain.sh" "shell/libation_onchain.sh"
assert_file "$ROOT/scripts/install_systemd.sh" "scripts/install_systemd.sh"
assert_file "$ROOT/systemd/solarking-sync.timer" "systemd sync timer"
assert_file "$ROOT/systemd/solarking-field-check.timer" "systemd field-check timer"
RITUAL_QUICK=1 "$ROOT/shell/confirm_rainbow.sh" "e2e phase2c" >/dev/null 2>&1 && pass "confirm_rainbow.sh" || fail "confirm_rainbow.sh"

# ── Phase 3A Eternal Seeds ──
echo ""
echo "── PHASE 3A: Eternal Seeds ──"
assert_output "$BIN help" "counsel" "help lists counsel"
assert_output "$BIN help" "export-cid" "help lists export-cid"
assert_output "$BIN counsel 'what is next phase 3'" "Phase 3" "solarking counsel"
$BIN export-cid >/dev/null 2>&1 && pass "solarking export-cid" || fail "solarking export-cid"
$BIN qr >/dev/null 2>&1 && pass "solarking qr" || fail "solarking qr"
assert_file "$ROOT/sync/altar/payload.json" "sync/altar/payload.json"
assert_file "$ROOT/plans/phase_3_eternal_expansion.md" "plans/phase_3_eternal_expansion.md"
assert_file "$ROOT/docs/PHASE_3_VERIFY.md" "docs/PHASE_3_VERIFY.md"

# ── Phase 3B–3E ──
echo ""
echo "── PHASE 3B–3E: Bridge · Viz · Nodes · Grok ──"
assert_output "$BIN help" "lattice" "help lists lattice"
assert_output "$BIN help" "node" "help lists node"
assert_output "$BIN help" "grok" "help lists grok"
$BIN lattice visualize >/dev/null 2>&1 && pass "solarking lattice visualize" || fail "solarking lattice visualize"
assert_file "$ROOT/web/lattice.html" "web/lattice.html"
$BIN node init --name e2e-node --label test >/dev/null 2>&1 && pass "solarking node init" || fail "solarking node init"
NODE_OUT=$(mktemp)
$BIN node export "$NODE_OUT" >/dev/null 2>&1 && pass "solarking node export" || fail "solarking node export"
assert_output "$BIN node status" "KINGDOM NODE" "solarking node status"
$BIN grok --offline "phase 3 pulse" >/dev/null 2>&1 && pass "solarking grok --offline" || fail "solarking grok --offline"
$BIN blueprint "e2e blueprint pulse" >/dev/null 2>&1 && pass "solarking blueprint" || fail "solarking blueprint"
assert_file "$ROOT/docs/PHASE_3B_PHYSICAL_BRIDGE.md" "docs/PHASE_3B_PHYSICAL_BRIDGE.md"
assert_file "$ROOT/docs/PHASE_3_GROK_BUILD.md" "docs/PHASE_3_GROK_BUILD.md"
assert_file "$ROOT/AGENTS.md" "AGENTS.md"
rm -f "$NODE_OUT"

# ── Phase 3G LocalAI (offline graceful) ──
echo ""
echo "── PHASE 3G: LocalAI / Ollama ──"
assert_output "$BIN help" "localai" "help lists localai"
assert_file "$ROOT/docs/PHASE_3G_LOCALAI.md" "docs/PHASE_3G_LOCALAI.md"
assert_file "$ROOT/docs/OLLAMA_WORKFLOW.md" "docs/OLLAMA_WORKFLOW.md"
assert_file "$ROOT/docs/templates/ollama_session_template.md" "docs/templates/ollama_session_template.md"
assert_file "$ROOT/docs/templates/prompts/activation.txt" "docs/templates/prompts/activation.txt"
assert_file "$ROOT/config/localai_system.txt" "config/localai_system.txt"
assert_file "$ROOT/scripts/ollama_workflow.sh" "scripts/ollama_workflow.sh"
$BIN localai --status >/dev/null 2>&1 && pass "solarking localai --status" || fail "solarking localai --status"
"$ROOT/scripts/ollama_workflow.sh" help >/dev/null 2>&1 && pass "ollama_workflow.sh help" || fail "ollama_workflow.sh help"
# Unset endpoint → offline counsel fallback (must not crash)
unset SOLARKING_LOCAL_MODEL 2>/dev/null || true
$BIN localai "e2e localai pulse" >/dev/null 2>&1 && pass "solarking localai offline fallback" || fail "solarking localai offline fallback"

RITUAL_QUICK=1 $BIN libation ancestors >/dev/null 2>&1 && pass "solarking libation" || fail "solarking libation"
RITUAL_QUICK=1 $BIN legacy_99 >/dev/null 2>&1 && pass "solarking legacy_99" || fail "solarking legacy_99"

# ── Crown Receive UX (v0.9) ──
echo ""
echo "── CROWN RECEIVE UX (v0.9) ──"
assert_output "$BIN help" "receive" "help lists receive"
assert_output "$BIN help" "journal" "help lists journal"
assert_output "$BIN help" "now" "help lists now"
assert_output "$BIN --version" "0.10" "solarking version 0.10"
assert_output "$BIN now card" "CROWN CARD" "solarking now card"

TX_FILE=$(mktemp)
printf 'E2E MULTI-LINE TRANSMISSION\nLine two — Th3 Cr0wn commands th3 r3ality obeys NOW\nLine three sealed.\n' > "$TX_FILE"
$BIN receive --file "$TX_FILE" >/dev/null 2>&1 && pass "solarking receive --file" || fail "solarking receive --file"
rm -f "$TX_FILE"

printf 'E2E PIPE TRANSMISSION\nsecond line of paste\n' | $BIN receive - >/dev/null 2>&1 && pass "solarking receive - (pipe)" || fail "solarking receive - (pipe)"

assert_output "$BIN journal show" "E2E PIPE TRANSMISSION" "journal show latest pipe body"
assert_output "$BIN journal search Cr0wn" "MULTI-LINE" "journal search finds multi-line"
assert_output "$BIN journal list --last 3" "CROWN JOURNAL" "journal list"
assert_output "$BIN now morning" "NOW MORNING" "solarking now morning"

# At least one transmission archive written under sync/transmissions/
TX_COUNT=$(find "$ROOT/sync/transmissions" -name '*.txt' 2>/dev/null | wc -l)
if [ "$TX_COUNT" -ge 1 ]; then
  pass "sync/transmissions archive files ($TX_COUNT)"
else
  fail "sync/transmissions archive files missing"
fi
assert_file "$ROOT/docs/CROWN_WORKFLOW.md" "docs/CROWN_WORKFLOW.md"

# Multi-line block in ritual_log
if grep -q "TRANSMISSION BEGIN" "$ROOT/ritual_log.txt" 2>/dev/null; then
  pass "ritual_log multi-line TRANSMISSION BEGIN block"
else
  fail "ritual_log multi-line TRANSMISSION BEGIN block"
fi

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
  echo "  Crown UX:"
  echo "    $BIN receive --paste"
  echo "    $BIN journal show"
  echo "    $BIN now card"
  exit 0
else
  echo "  ⚠️  SOME TESTS FAILED"
  exit 1
fi