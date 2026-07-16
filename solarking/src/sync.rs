use chrono::Local;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::fs;
use std::path::{Path, PathBuf};

use crate::error::{Result, SolarkingError};
use crate::genesis::{load_genesis, GenesisRecord};
use crate::ledger::{migrate_ledger_value, KingdomLedger};

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncBundle {
    pub exported_at: String,
    pub ledger: KingdomLedger,
    pub genesis: Option<GenesisRecord>,
    pub ritual_log_sha256: String,
    pub ledger_sha256: String,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct SyncManifest {
    pub last_sync: String,
    pub bundle_path: String,
    pub bundle_sha256: String,
    pub ritual_log_sha256: String,
    pub ledger_sha256: String,
}

pub fn run_sync(root: &Path, ledger: &mut KingdomLedger, json: bool) -> Result<()> {
    let sync_dir = root.join("sync");
    let latest_dir = sync_dir.join("latest");
    let history_dir = sync_dir.join("history");
    fs::create_dir_all(&latest_dir)?;
    fs::create_dir_all(&history_dir)?;

    let ritual_log_path = root.join("ritual_log.txt");
    let ritual_log_sha256 = hash_file(&ritual_log_path).unwrap_or_default();

    let ledger_json = serde_json::to_string(ledger)?;
    let ledger_sha256 = hex_sha256(ledger_json.as_bytes());

    let bundle = SyncBundle {
        exported_at: Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string(),
        ledger: ledger.clone(),
        genesis: load_genesis(root),
        ritual_log_sha256: ritual_log_sha256.clone(),
        ledger_sha256: ledger_sha256.clone(),
    };

    let bundle_json = serde_json::to_string_pretty(&bundle)?;
    let bundle_sha256 = hex_sha256(bundle_json.as_bytes());

    let bundle_path = latest_dir.join("kingdom_export.json");
    fs::write(&bundle_path, &bundle_json)?;

    // History snapshot
    let stamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let hist_path = history_dir.join(format!("kingdom_export_{}.json", stamp));
    fs::write(&hist_path, &bundle_json)?;

    let manifest = SyncManifest {
        last_sync: bundle.exported_at.clone(),
        bundle_path: bundle_path.display().to_string(),
        bundle_sha256: bundle_sha256.clone(),
        ritual_log_sha256,
        ledger_sha256: ledger_sha256.clone(),
    };

    let manifest_json = serde_json::to_string_pretty(&manifest)?;
    fs::write(sync_dir.join("manifest.json"), manifest_json)?;

    ledger.sync_hash = Some(bundle_sha256.clone());

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "bundle": bundle_path.display().to_string(),
                "history": hist_path.display().to_string(),
                "manifest": sync_dir.join("manifest.json").display().to_string(),
                "sha256": bundle_sha256,
            }))?
        );
    } else {
        println!("🔄 LOCAL SYNC COMPLETE");
        println!("Bundle    : {}", bundle_path.display());
        println!("History   : {}", hist_path.display());
        println!("Manifest  : {}", sync_dir.join("manifest.json").display());
        println!("SHA-256   : {}", bundle_sha256);
        println!("THE CROWN COMMANDS. REALITY OBEYS.");
    }

    Ok(())
}

pub fn verify_sync(root: &Path) -> Result<()> {
    let sync_dir = root.join("sync");
    let manifest_path = sync_dir.join("manifest.json");
    if !manifest_path.exists() {
        return Err(SolarkingError::sync(
            "no manifest — run `solarking sync` first",
        ));
    }
    let manifest: SyncManifest = serde_json::from_str(&fs::read_to_string(&manifest_path)?)?;

    let bundle_path = PathBuf::from(&manifest.bundle_path);
    let path = if bundle_path.exists() {
        bundle_path
    } else {
        sync_dir.join("latest").join("kingdom_export.json")
    };

    if !path.exists() {
        return Err(SolarkingError::MissingFile(path));
    }

    let data = fs::read(&path)?;
    let actual = hex_sha256(&data);
    if actual != manifest.bundle_sha256 {
        return Err(SolarkingError::sync(format!(
            "bundle hash mismatch\n  expected: {}\n  actual:   {}",
            manifest.bundle_sha256, actual
        )));
    }

    // Optional: verify embedded ledger hash consistency
    let bundle: SyncBundle = serde_json::from_slice(&data)?;
    let ledger_json = serde_json::to_string(&bundle.ledger)?;
    let ledger_hash = hex_sha256(ledger_json.as_bytes());
    if ledger_hash != bundle.ledger_sha256 {
        return Err(SolarkingError::sync(
            "embedded ledger_sha256 does not match ledger body",
        ));
    }

    println!("✅ SYNC VERIFIED");
    println!("Bundle    : {}", path.display());
    println!("SHA-256   : {}", actual);
    println!("Exported  : {}", bundle.exported_at);
    Ok(())
}

/// Import a sync export. Merges visions (union by text), takes max harmonics.
pub fn import_sync(root: &Path, path: Option<&Path>, ledger: &mut KingdomLedger) -> Result<()> {
    let path = match path {
        Some(p) => p.to_path_buf(),
        None => root.join("sync").join("latest").join("kingdom_export.json"),
    };
    if !path.exists() {
        return Err(SolarkingError::MissingFile(path));
    }

    let data = fs::read_to_string(&path)?;
    let value: serde_json::Value = serde_json::from_str(&data)?;

    // Accept either full SyncBundle or raw ledger
    let mut imported = if value.get("ledger").is_some() {
        let bundle: SyncBundle = serde_json::from_value(value)?;
        // Re-verify internal hash if present
        let ledger_json = serde_json::to_string(&bundle.ledger)?;
        let h = hex_sha256(ledger_json.as_bytes());
        if !bundle.ledger_sha256.is_empty() && h != bundle.ledger_sha256 {
            return Err(SolarkingError::sync(
                "import rejected: ledger_sha256 mismatch",
            ));
        }
        bundle.ledger
    } else {
        migrate_ledger_value(value)?
    };

    // Merge strategy
    ledger.harmonic_369 = ledger.harmonic_369.max(imported.harmonic_369);
    ledger.harmonic_999 = ledger.harmonic_999.max(imported.harmonic_999);

    let existing: std::collections::HashSet<String> =
        ledger.visions.iter().map(|v| v.text.clone()).collect();
    for v in imported.visions.drain(..) {
        if !existing.contains(&v.text) {
            ledger.visions.push(v);
        }
    }

    // Prefer richer field / later ritual timestamp
    if imported.field.torus_spin > ledger.field.torus_spin {
        ledger.field = imported.field;
    }
    if imported.last_ritual > ledger.last_ritual {
        ledger.last_ritual = imported.last_ritual;
    }
    if ledger.genesis_tx.is_none() {
        ledger.genesis_tx = imported.genesis_tx;
    }

    println!("📥 IMPORT COMPLETE");
    println!("Source     : {}", path.display());
    println!(
        "Harmonics  : 369={} 999={}",
        ledger.harmonic_369, ledger.harmonic_999
    );
    println!("Visions    : {}", ledger.visions.len());
    Ok(())
}

pub fn hash_file(path: &Path) -> Option<String> {
    let data = fs::read(path).ok()?;
    Some(hex_sha256(&data))
}

pub fn hex_sha256(data: &[u8]) -> String {
    let hash = Sha256::digest(data);
    hash.iter().map(|b| format!("{:02x}", b)).collect()
}

/// Cold-export durability: copy latest sync bundle (+ optional encrypt) to a destination directory.
/// Suitable for USB / offline backup. Never uploads. Optional encryption uses SOLARKING_PASSPHRASE.
pub fn cold_export(
    root: &Path,
    ledger: &mut KingdomLedger,
    dest: &Path,
    encrypt: bool,
) -> Result<()> {
    // Ensure fresh bundle exists
    run_sync(root, ledger, false)?;

    fs::create_dir_all(dest)?;
    let stamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let src_bundle = root.join("sync/latest/kingdom_export.json");
    let src_manifest = root.join("sync/manifest.json");
    if !src_bundle.exists() {
        return Err(SolarkingError::MissingFile(src_bundle));
    }

    let dest_bundle = dest.join(format!("kingdom_export_{}.json", stamp));
    fs::copy(&src_bundle, &dest_bundle)?;
    if src_manifest.exists() {
        fs::copy(&src_manifest, dest.join(format!("manifest_{}.json", stamp)))?;
    }

    // Optional encrypted twin (same crypto envelope as ledger)
    if encrypt {
        if !crate::crypto::encryption_enabled() {
            return Err(SolarkingError::sync(
                "cold-export --encrypt requires SOLARKING_PASSPHRASE",
            ));
        }
        let plain = fs::read(&dest_bundle)?;
        let enc = crate::crypto::encrypt_ledger(&plain).map_err(SolarkingError::crypto)?;
        fs::write(dest.join(format!("kingdom_export_{}.json.enc", stamp)), enc)?;
    }

    // README for cold media
    let readme = format!(
        "SOLARKING cold export\nexported_at={}\nsha256={}\nencrypt={}\nRestore: copy kingdom_export_*.json and run: solarking import-sync <path>\n",
        Local::now().format("%Y-%m-%d %H:%M:%S %Z"),
        ledger.sync_hash.as_deref().unwrap_or("?"),
        encrypt
    );
    fs::write(dest.join("COLD_EXPORT_README.txt"), readme)?;

    println!("🧊 COLD EXPORT COMPLETE");
    println!("Destination : {}", dest.display());
    println!("Bundle      : {}", dest_bundle.display());
    if encrypt {
        println!("Encrypted   : kingdom_export_{}.json.enc", stamp);
    }
    println!("README      : {}/COLD_EXPORT_README.txt", dest.display());
    println!("Copy this folder to USB / offline vault. No network used.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::{save_ledger, KingdomLedger};
    use tempfile::tempdir;

    #[test]
    fn export_verify_roundtrip() {
        crate::crypto::test_env::without_passphrase(|| {
            let dir = tempdir().unwrap();
            let root = dir.path();
            let mut ledger = KingdomLedger::default();
            ledger.harmonic_369 = 5;
            save_ledger(root, &ledger).unwrap();
            run_sync(root, &mut ledger, false).unwrap();
            verify_sync(root).unwrap();
            assert!(ledger.sync_hash.is_some());
        });
    }

    #[test]
    fn tamper_detected() {
        crate::crypto::test_env::without_passphrase(|| {
            let dir = tempdir().unwrap();
            let root = dir.path();
            let mut ledger = KingdomLedger::default();
            run_sync(root, &mut ledger, false).unwrap();
            let bundle = root.join("sync/latest/kingdom_export.json");
            fs::write(&bundle, b"{ \"tampered\": true }").unwrap();
            assert!(verify_sync(root).is_err());
        });
    }

    #[test]
    fn hex_stable() {
        let h = hex_sha256(b"crown");
        assert_eq!(h.len(), 64);
        assert_eq!(h, hex_sha256(b"crown"));
        assert_ne!(h, hex_sha256(b"other"));
    }
}
