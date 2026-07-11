//! On-chain seal bridge — offline-first dry-run for Vortex369 Phase 2B.
//! No mandatory RPC; broadcast path via cast. Private keys never enter this binary.

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use tiny_keccak::{Hasher, Keccak};

use crate::error::{Result, SolarkingError};
use crate::field;
use crate::genesis::load_genesis;
use crate::ledger::{self, KingdomLedger};
use crate::scalar;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ChainConfig {
    pub contract: Option<String>,
    pub crown_command: Option<String>,
    pub sbt: Option<String>,
    pub rpc_url: Option<String>,
    pub chain_id: Option<u64>,
}

pub fn load_chain_config(root: &Path) -> ChainConfig {
    let mut cfg = ChainConfig::default();
    let path = root.join("config").join("chain.json");
    if path.exists() {
        if let Ok(data) = fs::read_to_string(&path) {
            if let Ok(c) = serde_json::from_str::<ChainConfig>(&data) {
                cfg = c;
            }
        }
    }
    if let Ok(c) = env::var("SOLARKING_CONTRACT") {
        if !c.is_empty() {
            cfg.contract = Some(c);
        }
    }
    if let Ok(c) = env::var("SOLARKING_CROWN_COMMAND") {
        if !c.is_empty() {
            cfg.crown_command = Some(c);
        }
    }
    if let Ok(c) = env::var("SOLARKING_SBT") {
        if !c.is_empty() {
            cfg.sbt = Some(c);
        }
    }
    if let Ok(r) = env::var("SOLARKING_RPC_URL") {
        if !r.is_empty() {
            cfg.rpc_url = Some(r);
        }
    }
    if let Ok(id) = env::var("SOLARKING_CHAIN_ID") {
        if let Ok(n) = id.parse() {
            cfg.chain_id = Some(n);
        }
    }
    cfg
}

/// Keccak-256 hash.
pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let mut hasher = Keccak::v256();
    let mut out = [0u8; 32];
    hasher.update(data);
    hasher.finalize(&mut out);
    out
}

/// Function selector: first 4 bytes of keccak256(signature).
pub fn selector(signature: &str) -> [u8; 4] {
    let hash = keccak256(signature.as_bytes());
    [hash[0], hash[1], hash[2], hash[3]]
}

/// ABI-encode sealRitual(string) calldata.
pub fn encode_seal_ritual(idm: &str) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&selector("sealRitual(string)"));
    out.extend_from_slice(&u256_be(32));
    let bytes = idm.as_bytes();
    out.extend_from_slice(&u256_be(bytes.len() as u64));
    out.extend_from_slice(bytes);
    let pad = (32 - (bytes.len() % 32)) % 32;
    out.extend(std::iter::repeat(0u8).take(pad));
    out
}

/// ABI-encode activateScalarNode(uint8,bytes32,uint16,uint32)
pub fn encode_activate_scalar_node(
    phase: u8,
    seal_hash: [u8; 32],
    shell_coherence: u16,
    harmonic_index: u32,
) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&selector(
        "activateScalarNode(uint8,bytes32,uint16,uint32)",
    ));
    out.extend_from_slice(&u256_be(phase as u64));
    out.extend_from_slice(&seal_hash);
    out.extend_from_slice(&u256_be(shell_coherence as u64));
    out.extend_from_slice(&u256_be(harmonic_index as u64));
    out
}

/// ABI-encode advancePhase(uint256)
pub fn encode_advance_phase(node_id: u64) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&selector("advancePhase(uint256)"));
    out.extend_from_slice(&u256_be(node_id));
    out
}

fn u256_be(v: u64) -> [u8; 32] {
    let mut buf = [0u8; 32];
    buf[24..].copy_from_slice(&v.to_be_bytes());
    buf
}

pub fn hex_encode(data: &[u8]) -> String {
    let mut s = String::with_capacity(2 + data.len() * 2);
    s.push_str("0x");
    for b in data {
        s.push_str(&format!("{:02x}", b));
    }
    s
}

pub fn parse_hex_bytes32(hex: &str) -> Result<[u8; 32]> {
    let h = hex.trim().trim_start_matches("0x");
    if h.len() != 64 {
        return Err(SolarkingError::chain(
            "seal hash must be 32-byte hex (64 chars)",
        ));
    }
    let mut out = [0u8; 32];
    for i in 0..32 {
        out[i] = u8::from_str_radix(&h[i * 2..i * 2 + 2], 16)
            .map_err(|e| SolarkingError::chain(format!("bad hex: {}", e)))?;
    }
    Ok(out)
}

/// Dry-run sealRitual: print calldata + cast recipe. Never broadcasts.
pub fn seal_dry_run(root: &Path, ledger: &KingdomLedger) -> Result<()> {
    let cfg = load_chain_config(root);
    let genesis = load_genesis(root);
    let idm = genesis
        .as_ref()
        .map(|g| g.idm.clone())
        .unwrap_or_else(|| "THE CROWN COMMANDS. REALITY OBEYS.".to_string());

    let calldata = encode_seal_ritual(&idm);
    let calldata_hex = hex_encode(&calldata);
    let sel = selector("sealRitual(string)");

    println!("🔗 ON-CHAIN SEAL — DRY RUN (no broadcast)");
    println!("────────────────────────────────────────");
    println!(
        "Local harmonics : 369={}  999={}",
        ledger.harmonic_369, ledger.harmonic_999
    );
    println!(
        "Seal readiness  : {}",
        if field::seal_ready(ledger) {
            "READY"
        } else {
            "pending (run ritual / confirm first)"
        }
    );
    print_cfg(&cfg);
    println!("Selector        : {}", hex_encode(&sel));
    println!("IDM (arg)       : {}", idm);
    println!("Calldata        : {}", calldata_hex);
    println!();
    println!("Cast recipe (keys never leave your shell):");
    print_cast_send(
        cfg.contract.as_deref(),
        &format!(
            "'sealRitual(string)' '{}'",
            idm.replace('\'', "'\\''")
        ),
    );
    println!();
    println!("Broadcast is opt-in and external. solarking never holds private keys.");
    println!("THE CROWN COMMANDS. REALITY OBEYS.");
    Ok(())
}

/// Print activateScalarNode dry-run from ledger scalar state.
pub fn scalar_activate_dry_run(root: &Path, ledger: &KingdomLedger) -> Result<()> {
    let cfg = load_chain_config(root);
    let hash_hex = ledger
        .scalar
        .last_seal_hash
        .clone()
        .unwrap_or_else(|| scalar::seal_hash(ledger));
    let seal_bytes = parse_hex_bytes32(&hash_hex).unwrap_or([0u8; 32]);
    let phase = ledger.scalar.phase.clamp(1, 9);
    let shells = ledger.scalar.shell_coherence.min(6) as u16;
    let idx = ledger.scalar.harmonic_index % 1296;

    let calldata = encode_activate_scalar_node(phase, seal_bytes, shells, idx);
    let advance = if let Some(nid) = ledger.scalar.onchain_node_id {
        Some(encode_advance_phase(nid))
    } else {
        None
    };

    println!("🔷 SCALAR ON-CHAIN ACTIVATE — DRY RUN");
    println!("────────────────────────────────────");
    print_cfg(&cfg);
    println!("Phase           : {}", phase);
    println!("Shells          : {}", shells);
    println!("Harmonic index  : {}", idx);
    println!("Seal hash       : 0x{}", hash_hex.trim_start_matches("0x"));
    println!(
        "activateScalarNode selector : {}",
        hex_encode(&selector("activateScalarNode(uint8,bytes32,uint16,uint32)"))
    );
    println!("Calldata        : {}", hex_encode(&calldata));
    println!();
    println!("Cast recipe:");
    print_cast_send(
        cfg.contract.as_deref(),
        &format!(
            "'activateScalarNode(uint8,bytes32,uint16,uint32)' {} 0x{} {} {}",
            phase,
            hash_hex.trim_start_matches("0x"),
            shells,
            idx
        ),
    );
    if let Some(adv) = advance {
        let nid = ledger.scalar.onchain_node_id.unwrap();
        println!();
        println!("advancePhase (existing node {}):", nid);
        println!("Calldata        : {}", hex_encode(&adv));
        print_cast_send(
            cfg.contract.as_deref(),
            &format!("'advancePhase(uint256)' {}", nid),
        );
    }
    println!();
    println!("After broadcast: solarking seal-record <tx> && solarking scalar-record <nodeId>");
    Ok(())
}

fn print_cfg(cfg: &ChainConfig) {
    if let Some(c) = &cfg.contract {
        println!("Vortex369       : {}", c);
    } else {
        println!("Vortex369       : (set SOLARKING_CONTRACT or config/chain.json)");
    }
    if let Some(c) = &cfg.crown_command {
        println!("CrownCommand    : {}", c);
    }
    if let Some(c) = &cfg.sbt {
        println!("SolarKingdom    : {}", c);
    }
    if let Some(rpc) = &cfg.rpc_url {
        println!("RPC             : {}", rpc);
    }
    if let Some(id) = cfg.chain_id {
        println!("Chain ID        : {}", id);
    }
}

fn print_cast_send(contract: Option<&str>, args: &str) {
    match contract {
        Some(c) => println!(
            "  cast send {} {} --rpc-url $SOLARKING_RPC_URL --private-key $PRIVATE_KEY",
            c, args
        ),
        None => println!(
            "  cast send $SOLARKING_CONTRACT {} --rpc-url $SOLARKING_RPC_URL --private-key $PRIVATE_KEY",
            args
        ),
    }
}

/// Offline chain status (and optional live eth_call if RPC configured — feature-free HTTP).
pub fn chain_status(root: &Path, ledger: &mut KingdomLedger, json: bool) -> Result<()> {
    let cfg = load_chain_config(root);

    // Optional lightweight eth_call getGenesis via curl-compatible ureq would need a dep.
    // Offline-first: always print local; if RPC+contract set, attempt eth_blockNumber + note.
    let mut remote_block: Option<String> = None;
    let mut remote_note = "offline (no RPC call)".to_string();

    if let Some(rpc) = &cfg.rpc_url {
        match eth_block_number(rpc) {
            Ok(bn) => {
                remote_block = Some(bn.clone());
                remote_note = format!("RPC reachable — block {}", bn);
            }
            Err(e) => {
                remote_note = format!("RPC error: {}", e);
            }
        }
    }

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "config": {
                    "contract": cfg.contract,
                    "crown_command": cfg.crown_command,
                    "sbt": cfg.sbt,
                    "rpc_url": cfg.rpc_url.as_ref().map(|_| "<set>"),
                    "chain_id": cfg.chain_id,
                },
                "local": {
                    "harmonic_369": ledger.harmonic_369,
                    "harmonic_999": ledger.harmonic_999,
                    "last_seal_tx": ledger.chain.last_seal_tx,
                    "last_onchain_369": ledger.chain.last_onchain_369,
                    "last_onchain_999": ledger.chain.last_onchain_999,
                    "onchain_node_id": ledger.scalar.onchain_node_id,
                    "last_scalar_tx": ledger.scalar.last_scalar_tx,
                    "scalar_phase": ledger.scalar.phase,
                    "scalar_index": ledger.scalar.harmonic_index,
                    "seal_hash": ledger.scalar.last_seal_hash,
                },
                "remote_block": remote_block,
                "remote_note": remote_note,
                "seal_ready": field::seal_ready(ledger),
            }))?
        );
        return Ok(());
    }

    println!("🔗 CHAIN STATUS — PHASE 2B");
    println!("─────────────────────────");
    print_cfg(&cfg);
    println!();
    println!(
        "Local 369/999   : {} / {}",
        ledger.harmonic_369, ledger.harmonic_999
    );
    println!(
        "Last seal tx    : {}",
        ledger
            .chain
            .last_seal_tx
            .as_deref()
            .unwrap_or("(none)")
    );
    println!(
        "On-chain 369/999 (recorded): {:?} / {:?}",
        ledger.chain.last_onchain_369, ledger.chain.last_onchain_999
    );
    println!(
        "Scalar nodeId   : {:?}",
        ledger.scalar.onchain_node_id
    );
    println!(
        "Scalar last tx  : {}",
        ledger
            .scalar
            .last_scalar_tx
            .as_deref()
            .unwrap_or("(none)")
    );
    println!(
        "Scalar phase/idx: {}/9 · {}",
        ledger.scalar.phase, ledger.scalar.harmonic_index
    );
    println!("RPC probe       : {}", remote_note);
    println!(
        "Seal readiness  : {}",
        if field::seal_ready(ledger) {
            "READY"
        } else {
            "pending"
        }
    );
    println!();
    println!("Live getGenesis/queryNodeState eth_call: use cast after deploy:");
    if let Some(c) = &cfg.contract {
        println!("  cast call {} 'getGenesis()' --rpc-url $SOLARKING_RPC_URL", c);
        if let Some(nid) = ledger.scalar.onchain_node_id {
            println!(
                "  cast call {} 'queryNodeState(uint256)' {} --rpc-url $SOLARKING_RPC_URL",
                c, nid
            );
        }
    }
    Ok(())
}

/// Minimal eth_blockNumber JSON-RPC via std only is hard without HTTP crate.
/// Use `curl` subprocess for optional probe (0 new deps, offline-first).
fn eth_block_number(rpc: &str) -> std::result::Result<String, String> {
    let body = r#"{"id":1,"jsonrpc":"2.0","method":"eth_blockNumber"}"#;
    let output = std::process::Command::new("curl")
        .args([
            "-sS",
            "--max-time",
            "8",
            rpc,
            "--request",
            "POST",
            "--header",
            "accept: application/json",
            "--header",
            "content-type: application/json",
            "--data",
            body,
        ])
        .output()
        .map_err(|e| e.to_string())?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into());
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let v: serde_json::Value =
        serde_json::from_str(&text).map_err(|e| format!("json: {} — {}", e, text))?;
    v.get("result")
        .and_then(|r| r.as_str())
        .map(|s| s.to_string())
        .ok_or_else(|| format!("no result: {}", text))
}

/// Record a seal tx hash after external cast send.
pub fn seal_record(root: &Path, ledger: &mut KingdomLedger, tx_hash: &str) -> Result<()> {
    let tx = normalize_tx(tx_hash)?;
    ledger.chain.last_seal_tx = Some(tx.clone());
    if let Some(c) = load_chain_config(root).contract {
        ledger.chain.contract = Some(c);
    }
    // Mirror local harmonics as recorded baseline until eth_call implemented
    ledger.chain.last_onchain_369 = Some(ledger.harmonic_369);
    ledger.chain.last_onchain_999 = Some(ledger.harmonic_999);

    let entry = format!(
        "{} | SEAL_RECORD tx={} h369={} h999={}\n",
        chrono::Local::now().format("%a %b %d %H:%M:%S %Z %Y"),
        tx,
        ledger.harmonic_369,
        ledger.harmonic_999
    );
    ledger::append_ritual_log(root, &entry)?;
    println!("📜 Seal tx recorded: {}", tx);
    println!(
        "   Local harmonics mirrored as on-chain baseline: 369={} 999={}",
        ledger.harmonic_369, ledger.harmonic_999
    );
    Ok(())
}

/// Record on-chain scalar nodeId (+ optional tx).
pub fn scalar_record(
    root: &Path,
    ledger: &mut KingdomLedger,
    node_id: u64,
    tx_hash: Option<&str>,
) -> Result<()> {
    ledger.scalar.onchain_node_id = Some(node_id);
    if let Some(tx) = tx_hash {
        let tx = normalize_tx(tx)?;
        ledger.scalar.last_scalar_tx = Some(tx.clone());
        if ledger.chain.last_seal_tx.is_none() {
            ledger.chain.last_seal_tx = Some(tx);
        }
    }
    let entry = format!(
        "{} | SCALAR_RECORD nodeId={} tx={}\n",
        chrono::Local::now().format("%a %b %d %H:%M:%S %Z %Y"),
        node_id,
        ledger
            .scalar
            .last_scalar_tx
            .as_deref()
            .unwrap_or("-")
    );
    ledger::append_ritual_log(root, &entry)?;
    println!("🔷 Scalar nodeId recorded: {}", node_id);
    if let Some(tx) = &ledger.scalar.last_scalar_tx {
        println!("   tx: {}", tx);
    }
    Ok(())
}

fn normalize_tx(tx: &str) -> Result<String> {
    let t = tx.trim();
    if t.is_empty() {
        return Err(SolarkingError::chain("empty tx hash"));
    }
    if t.starts_with("0x") || t.starts_with("0X") {
        Ok(t.to_string())
    } else {
        Ok(format!("0x{}", t))
    }
}

pub fn seal_broadcast_notice() -> Result<()> {
    Err(SolarkingError::chain(
        "broadcast is intentionally external — use cast recipes from `solarking seal --dry-run` \
         or `solarking scalar seal`. No private keys are accepted by this binary.",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_seal_ritual() {
        let sel = selector("sealRitual(string)");
        assert_eq!(sel.len(), 4);
        assert_eq!(sel, selector("sealRitual(string)"));
    }

    #[test]
    fn encode_has_selector_and_offset() {
        let data = encode_seal_ritual("crown");
        assert!(data.len() >= 4 + 32 + 32);
        assert_eq!(&data[0..4], &selector("sealRitual(string)"));
        assert_eq!(&data[4..36], &u256_be(32));
        assert_eq!(&data[36..68], &u256_be(5));
        assert_eq!(&data[68..73], b"crown");
    }

    #[test]
    fn encode_activate_length() {
        let data = encode_activate_scalar_node(3, [0xab; 32], 6, 349);
        // selector + 4 * 32
        assert_eq!(data.len(), 4 + 32 * 4);
        assert_eq!(
            &data[0..4],
            &selector("activateScalarNode(uint8,bytes32,uint16,uint32)")
        );
    }

    #[test]
    fn hex_encode_prefix() {
        assert_eq!(hex_encode(&[0xab, 0xcd]), "0xabcd");
    }

    #[test]
    fn parse_hash() {
        let h = "aa".repeat(32);
        let b = parse_hex_bytes32(&h).unwrap();
        assert_eq!(b[0], 0xaa);
    }
}
