//! On-chain seal bridge — offline-first dry-run for Vortex369.sealRitual(string).
//! No mandatory RPC; broadcast path is documented via cast, not embedded keys.

use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use tiny_keccak::{Hasher, Keccak};

use crate::error::{Result, SolarkingError};
use crate::field;
use crate::genesis::load_genesis;
use crate::ledger::KingdomLedger;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ChainConfig {
    pub contract: Option<String>,
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
    // head: offset to string data = 32
    out.extend_from_slice(&u256_be(32));
    // tail: length + utf8 bytes padded to 32
    let bytes = idm.as_bytes();
    out.extend_from_slice(&u256_be(bytes.len() as u64));
    out.extend_from_slice(bytes);
    let pad = (32 - (bytes.len() % 32)) % 32;
    out.extend(std::iter::repeat(0u8).take(pad));
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

/// Dry-run seal: print calldata + cast recipe. Never broadcasts.
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
    if let Some(c) = &cfg.contract {
        println!("Contract        : {}", c);
    } else {
        println!("Contract        : (set SOLARKING_CONTRACT or config/chain.json)");
    }
    if let Some(rpc) = &cfg.rpc_url {
        println!("RPC             : {}", rpc);
    }
    println!("Selector        : {}", hex_encode(&sel));
    println!("IDM (arg)       : {}", idm);
    println!("Calldata        : {}", calldata_hex);
    println!();
    println!("Cast recipe (you run this — keys never leave your shell):");
    if let Some(c) = &cfg.contract {
        println!(
            "  cast send {} 'sealRitual(string)' '{}' --rpc-url $SOLARKING_RPC_URL --private-key $PRIVATE_KEY",
            c,
            idm.replace('\'', "'\\''")
        );
    } else {
        println!(
            "  cast send $SOLARKING_CONTRACT 'sealRitual(string)' '{}' --rpc-url $SOLARKING_RPC_URL --private-key $PRIVATE_KEY",
            idm.replace('\'', "'\\''")
        );
    }
    println!();
    println!("Calldata-only:");
    println!(
        "  cast calldata 'sealRitual(string)' '{}'",
        idm.replace('\'', "'\\''")
    );
    println!();
    println!("Broadcast is opt-in and external. solarking never holds private keys.");
    println!("THE CROWN COMMANDS. REALITY OBEYS.");
    Ok(())
}

pub fn seal_broadcast_notice() -> Result<()> {
    Err(SolarkingError::chain(
        "broadcast is intentionally external — use the cast recipe from `solarking seal --dry-run`. \
         Set SOLARKING_CONTRACT + SOLARKING_RPC_URL + PRIVATE_KEY in your shell, then cast send. \
         No private keys are accepted by this binary.",
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn selector_seal_ritual() {
        // keccak256("sealRitual(string)") first 4 bytes — stable known value
        let sel = selector("sealRitual(string)");
        assert_eq!(sel.len(), 4);
        // recompute must be stable
        assert_eq!(sel, selector("sealRitual(string)"));
    }

    #[test]
    fn encode_has_selector_and_offset() {
        let data = encode_seal_ritual("crown");
        assert!(data.len() >= 4 + 32 + 32);
        assert_eq!(&data[0..4], &selector("sealRitual(string)"));
        // offset = 32
        assert_eq!(&data[4..36], &u256_be(32));
        // length = 5
        assert_eq!(&data[36..68], &u256_be(5));
        assert_eq!(&data[68..73], b"crown");
    }

    #[test]
    fn hex_encode_prefix() {
        assert_eq!(hex_encode(&[0xab, 0xcd]), "0xabcd");
    }
}
