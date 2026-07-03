use chrono::Local;
use serde::Serialize;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;

use crate::genesis::{load_genesis, GenesisRecord};
use crate::ledger::KingdomLedger;

#[derive(Serialize)]
struct SyncBundle {
    exported_at: String,
    ledger: KingdomLedger,
    genesis: Option<GenesisRecord>,
    ritual_log_sha256: String,
    ledger_sha256: String,
}

#[derive(Serialize)]
struct SyncManifest {
    last_sync: String,
    bundle_path: String,
    bundle_sha256: String,
    ritual_log_sha256: String,
    ledger_sha256: String,
}

pub fn run_sync(root: &PathBuf, ledger: &mut KingdomLedger) -> Result<(), String> {
    let sync_dir = root.join("sync");
    let latest_dir = sync_dir.join("latest");
    fs::create_dir_all(&latest_dir).map_err(|e| e.to_string())?;

    let ritual_log_path = root.join("ritual_log.txt");
    let ritual_log_sha256 = hash_file(&ritual_log_path).unwrap_or_default();

    let ledger_json = serde_json::to_string(ledger).map_err(|e| e.to_string())?;
    let ledger_sha256 = hex_sha256(ledger_json.as_bytes());

    let bundle = SyncBundle {
        exported_at: Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string(),
        ledger: ledger.clone(),
        genesis: load_genesis(root),
        ritual_log_sha256: ritual_log_sha256.clone(),
        ledger_sha256: ledger_sha256.clone(),
    };

    let bundle_json = serde_json::to_string_pretty(&bundle).map_err(|e| e.to_string())?;
    let bundle_sha256 = hex_sha256(bundle_json.as_bytes());

    let bundle_path = latest_dir.join("kingdom_export.json");
    fs::write(&bundle_path, &bundle_json).map_err(|e| e.to_string())?;

    let manifest = SyncManifest {
        last_sync: bundle.exported_at.clone(),
        bundle_path: bundle_path.display().to_string(),
        bundle_sha256: bundle_sha256.clone(),
        ritual_log_sha256,
        ledger_sha256: ledger_sha256.clone(),
    };

    let manifest_json = serde_json::to_string_pretty(&manifest).map_err(|e| e.to_string())?;
    fs::write(sync_dir.join("manifest.json"), manifest_json).map_err(|e| e.to_string())?;

    ledger.sync_hash = Some(bundle_sha256.clone());

    println!("🔄 LOCAL SYNC COMPLETE");
    println!("Bundle    : {}", bundle_path.display());
    println!("Manifest  : {}", sync_dir.join("manifest.json").display());
    println!("SHA-256   : {}", bundle_sha256);
    println!("THE CROWN COMMANDS. REALITY OBEYS.");

    Ok(())
}

fn hash_file(path: &PathBuf) -> Option<String> {
    let data = fs::read(path).ok()?;
    Some(hex_sha256(&data))
}

fn hex_sha256(data: &[u8]) -> String {
    let hash = Sha256::digest(data);
    hash.iter().map(|b| format!("{:02x}", b)).collect()
}