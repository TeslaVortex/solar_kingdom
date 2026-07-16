//! Phase 3D — Kingdom nodes: file-based peer federation (no central server).
//! Protocol v0: USB/email exchange of signed-by-hash node bundles.

use chrono::Local;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Result, SolarkingError};
use crate::genesis::load_genesis;
use crate::ledger::{self, KingdomLedger};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NodeIdentity {
    pub node_name: String,
    pub created_at: String,
    pub genesis_tx: Option<String>,
    /// Local sovereign label (not a chain address unless operator sets it)
    pub operator_label: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct NodeBundle {
    pub protocol: String,
    pub identity: NodeIdentity,
    pub exported_at: String,
    pub harmonic_369: u64,
    pub harmonic_999: u64,
    pub scalar_phase: u8,
    pub scalar_index: u32,
    pub shell_coherence: u8,
    pub onchain_node_id: Option<u64>,
    pub last_seal_hash: Option<String>,
    pub sync_hash: Option<String>,
    pub visions_count: usize,
    pub confirmations_count: usize,
    /// SHA-256 of canonical identity+harmonics body (integrity, not cryptographic signature)
    pub bundle_sha256: String,
}

fn identity_path(root: &Path) -> PathBuf {
    root.join("config").join("node_identity.json")
}

pub fn node_init(root: &Path, name: Option<&str>, label: Option<&str>) -> Result<()> {
    let path = identity_path(root);
    if path.exists() {
        println!("⚠️  Node identity already exists: {}", path.display());
        println!("   Delete it manually to re-init (sovereign choice).");
        return Ok(());
    }
    fs::create_dir_all(root.join("config"))?;
    let genesis = load_genesis(root);
    let id = NodeIdentity {
        node_name: name
            .unwrap_or("kingdom-node")
            .to_string(),
        created_at: Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string(),
        genesis_tx: genesis.map(|g| g.tx_hash),
        operator_label: label.unwrap_or("sovereign").to_string(),
    };
    fs::write(&path, serde_json::to_string_pretty(&id)?)?;
    println!("🌐 KINGDOM NODE INIT");
    println!("Name     : {}", id.node_name);
    println!("Operator : {}", id.operator_label);
    println!("Path     : {}", path.display());
    println!("Share via: solarking node export <path>");
    println!("THE CROWN COMMANDS. REALITY OBEYS.");
    Ok(())
}

fn load_identity(root: &Path) -> Result<NodeIdentity> {
    let path = identity_path(root);
    if !path.exists() {
        return Err(SolarkingError::Msg(
            "no node identity — run `solarking node init` first".into(),
        ));
    }
    Ok(serde_json::from_str(&fs::read_to_string(path)?)?)
}

fn bundle_hash(b: &NodeBundle) -> String {
    let canon = format!(
        "{}|{}|{}|{}|{}|{}|{}|{:?}|{:?}|{:?}",
        b.identity.node_name,
        b.identity.operator_label,
        b.harmonic_369,
        b.harmonic_999,
        b.scalar_phase,
        b.scalar_index,
        b.shell_coherence,
        b.onchain_node_id,
        b.last_seal_hash,
        b.sync_hash
    );
    let d = Sha256::digest(canon.as_bytes());
    d.iter().map(|x| format!("{:02x}", x)).collect()
}

pub fn node_export(root: &Path, ledger: &KingdomLedger, dest: &Path) -> Result<()> {
    let identity = load_identity(root)?;
    let mut bundle = NodeBundle {
        protocol: "solarking.node.v0".into(),
        identity,
        exported_at: Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string(),
        harmonic_369: ledger.harmonic_369,
        harmonic_999: ledger.harmonic_999,
        scalar_phase: ledger.scalar.phase,
        scalar_index: ledger.scalar.harmonic_index,
        shell_coherence: ledger.scalar.shell_coherence,
        onchain_node_id: ledger.scalar.onchain_node_id,
        last_seal_hash: ledger.scalar.last_seal_hash.clone(),
        sync_hash: ledger.sync_hash.clone(),
        visions_count: ledger.visions.len(),
        confirmations_count: ledger.confirmations.len(),
        bundle_sha256: String::new(),
    };
    bundle.bundle_sha256 = bundle_hash(&bundle);

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(dest, serde_json::to_string_pretty(&bundle)?)?;

    // Also drop a peer copy under sync/nodes/
    let peer_dir = root.join("sync").join("nodes");
    fs::create_dir_all(&peer_dir)?;
    let peer = peer_dir.join(format!(
        "{}_{}.json",
        bundle.identity.node_name,
        Local::now().format("%Y%m%d_%H%M%S")
    ));
    fs::copy(dest, &peer)?;

    println!("🌐 NODE EXPORT COMPLETE");
    println!("Bundle   : {}", dest.display());
    println!("Peer copy: {}", peer.display());
    println!("SHA-256  : {}", bundle.bundle_sha256);
    println!("Hand this file to a peer sovereign (USB/email). No central server.");
    Ok(())
}

pub fn node_import(root: &Path, ledger: &mut KingdomLedger, path: &Path) -> Result<()> {
    if !path.exists() {
        return Err(SolarkingError::MissingFile(path.to_path_buf()));
    }
    let bundle: NodeBundle = serde_json::from_str(&fs::read_to_string(path)?)?;
    let expect = bundle_hash(&bundle);
    if expect != bundle.bundle_sha256 {
        return Err(SolarkingError::sync(
            "node bundle hash mismatch — tampered or corrupt",
        ));
    }

    // Soft merge: take max harmonics; never overwrite local identity
    let before_369 = ledger.harmonic_369;
    let before_999 = ledger.harmonic_999;
    ledger.harmonic_369 = ledger.harmonic_369.max(bundle.harmonic_369);
    ledger.harmonic_999 = ledger.harmonic_999.max(bundle.harmonic_999);
    if bundle.shell_coherence > ledger.scalar.shell_coherence {
        ledger.scalar.shell_coherence = bundle.shell_coherence;
    }

    let peers = root.join("sync").join("peers");
    fs::create_dir_all(&peers)?;
    let stamp = Local::now().format("%Y%m%d_%H%M%S");
    fs::write(
        peers.join(format!("{}_{}.json", bundle.identity.node_name, stamp)),
        serde_json::to_string_pretty(&bundle)?,
    )?;

    let entry = format!(
        "{} | NODE IMPORT peer={} h369 {}→{} h999 {}→{}\n",
        Local::now().format("%a %b %d %H:%M:%S %Z %Y"),
        bundle.identity.node_name,
        before_369,
        ledger.harmonic_369,
        before_999,
        ledger.harmonic_999
    );
    ledger::append_ritual_log(root, &entry)?;

    println!("🌐 NODE IMPORT COMPLETE");
    println!("Peer     : {} ({})", bundle.identity.node_name, bundle.identity.operator_label);
    println!(
        "Harmonics: 369={} 999={} (merged max)",
        ledger.harmonic_369, ledger.harmonic_999
    );
    println!("Genesis  : {:?}", bundle.identity.genesis_tx);
    println!("Integrity: OK ({})", &bundle.bundle_sha256[..16]);
    Ok(())
}

pub fn node_status(root: &Path, ledger: &KingdomLedger, json: bool) -> Result<()> {
    let id = load_identity(root).ok();
    let peers_dir = root.join("sync").join("peers");
    let peer_count = fs::read_dir(&peers_dir)
        .map(|rd| rd.filter_map(|e| e.ok()).count())
        .unwrap_or(0);

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "identity": id,
                "local_harmonic_369": ledger.harmonic_369,
                "local_harmonic_999": ledger.harmonic_999,
                "peer_records": peer_count,
                "protocol": "solarking.node.v0",
            }))?
        );
        return Ok(());
    }

    println!("🌐 KINGDOM NODE STATUS");
    println!("─────────────────────");
    match id {
        Some(i) => {
            println!("Name     : {}", i.node_name);
            println!("Operator : {}", i.operator_label);
            println!("Created  : {}", i.created_at);
            println!("Genesis  : {:?}", i.genesis_tx);
        }
        None => println!("Identity : (none — run node init)"),
    }
    println!(
        "Local    : 369={} 999={} phase={}/9",
        ledger.harmonic_369, ledger.harmonic_999, ledger.scalar.phase
    );
    println!("Peers    : {} records in sync/peers/", peer_count);
    println!("Protocol : solarking.node.v0 (file federation, Base Sepolia-ready)");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::KingdomLedger;
    use tempfile::tempdir;

    #[test]
    fn export_import_roundtrip() {
        let dir = tempdir().unwrap();
        let root = dir.path();
        node_init(root, Some("alpha"), Some("test-op")).unwrap();
        let mut ledger = KingdomLedger::default();
        ledger.harmonic_369 = 10;
        ledger.harmonic_999 = 3;
        let dest = root.join("out.json");
        node_export(root, &ledger, &dest).unwrap();

        let dir2 = tempdir().unwrap();
        let mut ledger2 = KingdomLedger::default();
        ledger2.harmonic_369 = 5;
        node_import(dir2.path(), &mut ledger2, &dest).unwrap();
        assert_eq!(ledger2.harmonic_369, 10);
        assert_eq!(ledger2.harmonic_999, 3);
    }
}
