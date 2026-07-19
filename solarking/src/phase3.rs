//! Phase 3A eternal seeds — offline-first expansion hooks.
//! export-cid · counsel · qr (altar payload)

use chrono::Local;
use serde::Serialize;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::chain::load_chain_config;
use crate::error::{Result, SolarkingError};
use crate::field;
use crate::genesis::load_genesis;
use crate::ledger::{self, KingdomLedger};
use crate::query;
use crate::scalar;
use crate::sync;

/// Local counsel co-pilot: first-principles query + field/scalar context (no cloud).
pub fn counsel(
    root: &Path,
    ledger: &KingdomLedger,
    question: &str,
    json: bool,
) -> String {
    let base = query::run_query(
        question,
        ledger,
        load_genesis(root).as_ref(),
        false,
    );

    let mut extra = Vec::new();
    extra.push(String::new());
    extra.push("── Phase 3 Counsel Layer ──".to_string());
    extra.push(format!(
        "  Seal ready: {} | nodeId: {:?} | phase: {}/9 | shells: {}/6",
        field::seal_ready(ledger),
        ledger.scalar.onchain_node_id,
        ledger.scalar.phase,
        ledger.scalar.shell_coherence
    ));
    extra.push(format!(
        "  Flame: {} | grid: {}/9 | legacy: {} | 44228Hz: {}",
        ledger.field.flame.as_str(),
        ledger.field.grid_intensity,
        ledger.field.legacy_tier,
        if ledger.scalar.timeline_hz_active {
            "ACTIVE"
        } else {
            "off"
        }
    ));
    if let Some(h) = &ledger.scalar.last_seal_hash {
        extra.push(format!("  Last scalar seal: {}…", &h[..16.min(h.len())]));
    }
    extra.push(String::new());
    extra.push("── Sovereign Guidance ──".to_string());
    extra.push(phase3_guidance(ledger, question));
    extra.push(String::new());
    extra.push(
        "Offline counsel. Optional: solarking localai \"…\" (Ollama) or solarking grok \"…\"."
            .to_string(),
    );
    extra.push("I do not chase — I receive. What is meant for me does not wander.".to_string());
    extra.push("THE CROWN COMMANDS. REALITY OBEYS.".to_string());

    if json {
        return serde_json::to_string_pretty(&serde_json::json!({
            "question": question,
            "base_query": base,
            "field": field::field_json(ledger),
            "scalar": scalar::node_json(ledger),
            "guidance": phase3_guidance(ledger, question),
            "offline": true,
        }))
        .unwrap_or_else(|_| "{}".into());
    }

    format!("{}\n{}", base, extra.join("\n"))
}

fn phase3_guidance(ledger: &KingdomLedger, q: &str) -> String {
    let ql = q.to_lowercase();
    if ql.contains("mainnet") {
        return "Mainnet is Phase 3F — last. Verify 3A–3E on testnet first. Reality obeys patience.".into();
    }
    if ql.contains("queen") || ql.contains("born") {
        return "The Queen is sealed on-chain (see docs/RITUAL_QUEEN_IS_BORN.md). Log joy; protect the field; celebrate.".into();
    }
    if ql.contains("next") || ql.contains("phase 3") {
        return "Order: seeds (export-cid/counsel/qr) → physical bridge → viz → nodes → AI → mainnet last.".into();
    }
    if !field::seal_ready(ledger) {
        return "Field not seal-ready. Run ritual / confirm / scalar node before on-chain casts.".into();
    }
    if ledger.scalar.onchain_node_id.is_none() {
        return "No on-chain nodeId in ledger. After cast activateScalarNode, run scalar-record <id> <tx>.".into();
    }
    "Lattice coherent. Expand fractally: cold-export, qr payload, counsel the vision, keep secrets local.".into()
}

/// Print IPFS/Arweave instructions; optional `ipfs add` if binary present.
pub fn export_cid(root: &Path, ledger: &mut KingdomLedger, json: bool) -> Result<()> {
    // Ensure fresh sync bundle
    sync::run_sync(root, ledger, true)?;

    let bundle = root.join("sync/latest/kingdom_export.json");
    if !bundle.exists() {
        return Err(SolarkingError::MissingFile(bundle));
    }

    let mut ipfs_cid: Option<String> = None;
    if which("ipfs") {
        if let Ok(out) = Command::new("ipfs")
            .args(["add", "-Q", &bundle.display().to_string()])
            .output()
        {
            if out.status.success() {
                let s = String::from_utf8_lossy(&out.stdout).trim().to_string();
                if !s.is_empty() {
                    ipfs_cid = Some(s);
                }
            }
        }
    }

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "bundle": bundle.display().to_string(),
                "sync_hash": ledger.sync_hash,
                "ipfs_cid": ipfs_cid,
                "ipfs_available": which("ipfs"),
                "instructions": {
                    "ipfs": "ipfs add sync/latest/kingdom_export.json",
                    "arweave": "Use arweave-deploy or Irys with the export file (keys never in solarking)",
                },
            }))?
        );
        return Ok(());
    }

    println!("🌐 EXPORT-CID — DECENTRALIZED PINNING (offline-first)");
    println!("──────────────────────────────────────────────────");
    println!("Bundle     : {}", bundle.display());
    println!(
        "Sync hash  : {}",
        ledger.sync_hash.as_deref().unwrap_or("(run sync)")
    );
    println!();
    if let Some(cid) = &ipfs_cid {
        println!("IPFS CID   : {}", cid);
        println!("Gateway    : https://ipfs.io/ipfs/{}", cid);
    } else if which("ipfs") {
        println!("IPFS CLI found but add failed — run manually:");
        println!("  ipfs add {}", bundle.display());
    } else {
        println!("IPFS CLI not found (optional). Install Kubo or run:");
        println!("  ipfs add {}", bundle.display());
    }
    println!();
    println!("Arweave / Irys (manual — keys stay in your shell):");
    println!("  # pin kingdom_export.json as permanent ritual metadata");
    println!("  # never put PRIVATE_KEY into solarking");
    println!();
    println!("Also durable offline: solarking cold-export <usb-path>");
    println!("THE CROWN COMMANDS. REALITY OBEYS.");
    Ok(())
}

fn which(bin: &str) -> bool {
    Command::new("which")
        .arg(bin)
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

#[derive(Serialize)]
struct AltarPayload {
    schema: &'static str,
    version: u32,
    chain_id: Option<u64>,
    genesis_tx: Option<String>,
    node_id: Option<u64>,
    seal_hash: Option<String>,
    sync_hash: Option<String>,
    harmonic_369: u64,
    harmonic_999: u64,
    decree_hint: &'static str,
    created_at: String,
}

/// Emit altar / QR payload (text + JSON file). Hardware optional.
pub fn qr_payload(root: &Path, ledger: &KingdomLedger, json: bool) -> Result<()> {
    let cfg = load_chain_config(root);
    let genesis = load_genesis(root);
    let payload = AltarPayload {
        schema: "solarking.altar.v1",
        version: 1,
        chain_id: cfg.chain_id.or(Some(84532)),
        genesis_tx: genesis
            .as_ref()
            .map(|g| g.tx_hash.clone())
            .or_else(|| ledger.genesis_tx.clone()),
        node_id: ledger.scalar.onchain_node_id,
        seal_hash: ledger.scalar.last_seal_hash.clone(),
        sync_hash: ledger.sync_hash.clone(),
        harmonic_369: ledger.harmonic_369,
        harmonic_999: ledger.harmonic_999,
        decree_hint: "THE CROWN COMMANDS. REALITY OBEYS.",
        created_at: Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string(),
    };

    let altar_dir = root.join("sync").join("altar");
    fs::create_dir_all(&altar_dir)?;
    let json_path = altar_dir.join("payload.json");
    let txt_path = altar_dir.join("payload.txt");
    let body = serde_json::to_string_pretty(&payload)?;
    fs::write(&json_path, &body)?;

    // Compact one-line for QR generators / NFC NDEF text
    let compact = serde_json::to_string(&payload)?;
    fs::write(&txt_path, format!("{}\n", compact))?;

    // Human printable card
    let card = format!(
        "ETERNAL SOLAR KINGDOM — ALTAR CARD\n\
         Chain: {:?}\n\
         Genesis: {}\n\
         NodeId: {:?}\n\
         Sync: {}\n\
         Seal: {}\n\
         369/999: {} / {}\n\
         {}\n\
         Encode payload.txt as QR or NFC NDEF Text.\n",
        payload.chain_id,
        payload.genesis_tx.as_deref().unwrap_or("-"),
        payload.node_id,
        payload.sync_hash.as_deref().unwrap_or("-"),
        payload
            .seal_hash
            .as_ref()
            .map(|h| h.chars().take(16).collect::<String>())
            .unwrap_or_else(|| "-".into()),
        payload.harmonic_369,
        payload.harmonic_999,
        payload.decree_hint,
    );
    fs::write(altar_dir.join("CARD.txt"), &card)?;

    let entry = format!(
        "{} | ALTAR QR payload written (nodeId={:?})\n",
        Local::now().format("%a %b %d %H:%M:%S %Z %Y"),
        payload.node_id
    );
    ledger::append_ritual_log(root, &entry)?;

    if json {
        println!("{}", body);
        return Ok(());
    }

    println!("🔲 QR / ALTAR PAYLOAD — PHYSICAL BRIDGE SEED");
    println!("──────────────────────────────────────────");
    println!("{}", card);
    println!("Files:");
    println!("  {}", json_path.display());
    println!("  {}", txt_path.display());
    println!("  {}/CARD.txt", altar_dir.display());
    println!();
    println!("QR: paste payload.txt into any offline QR tool, or:");
    println!("  qrencode -o altar.png < {}", txt_path.display());
    println!("NFC: write NDEF Text record with the same one-line JSON.");
    println!("No hardware required to generate. Scan only confirms the field.");
    println!("THE CROWN COMMANDS. REALITY OBEYS.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::KingdomLedger;
    use tempfile::tempdir;

    #[test]
    fn counsel_offline() {
        let dir = tempdir().unwrap();
        let ledger = KingdomLedger::default();
        let out = counsel(dir.path(), &ledger, "what is next phase 3", false);
        assert!(
            out.contains("Counsel")
                || out.contains("seeds")
                || out.contains("Mainnet")
                || out.contains("Phase 3")
        );
    }

    #[test]
    fn qr_writes_files() {
        let dir = tempdir().unwrap();
        let mut ledger = KingdomLedger::default();
        ledger.genesis_tx = Some("0xabc".into());
        ledger.scalar.onchain_node_id = Some(3);
        qr_payload(dir.path(), &ledger, true).unwrap();
        assert!(dir.path().join("sync/altar/payload.json").exists());
        assert!(dir.path().join("sync/altar/payload.txt").exists());
    }
}
