use chrono::Local;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::fs;
use std::path::{Path, PathBuf};

use crate::crypto::{decrypt_ledger, encrypt_ledger, encryption_enabled};
use crate::error::{Result, SolarkingError};
use crate::field::{self, FieldState};
use crate::genesis::{load_genesis, GenesisRecord};
use crate::scalar::ScalarNodeState;

pub const SCHEMA_VERSION: u32 = 2;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct VisionEntry {
    pub ts: String,
    pub text: String,
    #[serde(default)]
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct RitualEvent {
    pub ts: String,
    pub kind: String,
    pub delta_369: u64,
    #[serde(default)]
    pub note: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct FieldConfirm {
    pub ts: String,
    pub kind: String,
    #[serde(default)]
    pub note: String,
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct ChainState {
    pub contract: Option<String>,
    pub last_seal_tx: Option<String>,
    pub last_onchain_369: Option<u64>,
    pub last_onchain_999: Option<u64>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct KingdomLedger {
    #[serde(default = "default_schema_version")]
    pub schema_version: u32,
    pub harmonic_369: u64,
    pub harmonic_999: u64,
    #[serde(default)]
    pub visions: Vec<VisionEntry>,
    #[serde(default)]
    pub rituals: Vec<RitualEvent>,
    #[serde(default)]
    pub confirmations: Vec<FieldConfirm>,
    #[serde(default)]
    pub field: FieldState,
    /// Tesla 369 scalar node — nested cuboctahedron lattice state
    #[serde(default)]
    pub scalar: ScalarNodeState,
    pub last_ritual: String,
    pub genesis_tx: Option<String>,
    pub sync_hash: Option<String>,
    #[serde(default)]
    pub chain: ChainState,
}

fn default_schema_version() -> u32 {
    SCHEMA_VERSION
}

impl Default for KingdomLedger {
    fn default() -> Self {
        Self {
            schema_version: SCHEMA_VERSION,
            harmonic_369: 0,
            harmonic_999: 0,
            visions: Vec::new(),
            rituals: Vec::new(),
            confirmations: Vec::new(),
            field: FieldState::default(),
            scalar: ScalarNodeState::default(),
            last_ritual: String::new(),
            genesis_tx: None,
            sync_hash: None,
            chain: ChainState::default(),
        }
    }
}

impl KingdomLedger {
    pub fn latest_vision_text(&self) -> Option<&str> {
        self.visions.last().map(|v| v.text.as_str())
    }
}

pub fn plain_ledger_path(root: &Path) -> PathBuf {
    root.join("kingdom_ledger.json")
}

pub fn load_ledger(root: &Path) -> Result<KingdomLedger> {
    let enc_path = root.join("kingdom_ledger.json.enc");
    let plain_path = root.join("kingdom_ledger.json");

    let raw = if encryption_enabled() && enc_path.exists() {
        let bytes = fs::read(&enc_path)?;
        decrypt_ledger(&bytes).map_err(SolarkingError::crypto)?
    } else if plain_path.exists() {
        fs::read_to_string(&plain_path)?
    } else {
        let mut ledger = KingdomLedger::default();
        seed_genesis(&mut ledger, root);
        return Ok(ledger);
    };

    let value: Value = serde_json::from_str(&raw)?;
    let mut ledger = migrate_ledger_value(value)?;
    seed_genesis(&mut ledger, root);
    Ok(ledger)
}

fn seed_genesis(ledger: &mut KingdomLedger, root: &Path) {
    if ledger.genesis_tx.is_none() {
        if let Some(g) = load_genesis(root) {
            ledger.genesis_tx = Some(g.tx_hash);
        }
    }
}

/// Migrate v1 (string visions, no schema) → v2 structured ledger.
pub fn migrate_ledger_value(value: Value) -> Result<KingdomLedger> {
    let obj = value
        .as_object()
        .ok_or_else(|| SolarkingError::ledger("ledger root must be an object"))?;

    // Fast path: already v2-shaped visions array of objects
    if let Some(visions) = obj.get("visions") {
        if visions.as_array().map(|a| {
            a.is_empty()
                || a.iter()
                    .all(|v| v.is_object() || v.is_null())
        }) == Some(true)
        {
            let mut ledger: KingdomLedger = serde_json::from_value(value)?;
            ledger.schema_version = SCHEMA_VERSION;
            return Ok(ledger);
        }
    }

    // v1 path: visions are strings
    let harmonic_369 = obj
        .get("harmonic_369")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let harmonic_999 = obj
        .get("harmonic_999")
        .and_then(|v| v.as_u64())
        .unwrap_or(0);
    let last_ritual = obj
        .get("last_ritual")
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();
    let genesis_tx = obj
        .get("genesis_tx")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());
    let sync_hash = obj
        .get("sync_hash")
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let mut visions = Vec::new();
    if let Some(arr) = obj.get("visions").and_then(|v| v.as_array()) {
        let mut prev: Option<String> = None;
        for item in arr {
            if let Some(s) = item.as_str() {
                if prev.as_deref() == Some(s) {
                    continue; // consecutive dedupe
                }
                visions.push(VisionEntry {
                    ts: String::new(),
                    text: s.to_string(),
                    tags: Vec::new(),
                });
                prev = Some(s.to_string());
            } else if item.is_object() {
                if let Ok(entry) = serde_json::from_value::<VisionEntry>(item.clone()) {
                    visions.push(entry);
                }
            }
        }
    }

    let field = obj
        .get("field")
        .cloned()
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    let scalar = obj
        .get("scalar")
        .cloned()
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    let rituals = obj
        .get("rituals")
        .cloned()
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    let confirmations = obj
        .get("confirmations")
        .cloned()
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();
    let chain = obj
        .get("chain")
        .cloned()
        .and_then(|v| serde_json::from_value(v).ok())
        .unwrap_or_default();

    Ok(KingdomLedger {
        schema_version: SCHEMA_VERSION,
        harmonic_369,
        harmonic_999,
        visions,
        rituals,
        confirmations,
        field,
        scalar,
        last_ritual,
        genesis_tx,
        sync_hash,
        chain,
    })
}

pub fn save_ledger(root: &Path, ledger: &KingdomLedger) -> Result<()> {
    let mut to_save = ledger.clone();
    to_save.schema_version = SCHEMA_VERSION;

    let data = serde_json::to_string_pretty(&to_save)?;

    // One-time backup when migrating from plain v1 file
    let plain = plain_ledger_path(root);
    let bak = root.join("kingdom_ledger.json.bak");
    if plain.exists() && !bak.exists() {
        let _ = fs::copy(&plain, &bak);
    }

    if encryption_enabled() {
        let bytes = encrypt_ledger(data.as_bytes()).map_err(SolarkingError::crypto)?;
        fs::write(root.join("kingdom_ledger.json.enc"), bytes)?;
    } else {
        fs::write(plain, data)?;
    }
    Ok(())
}

pub fn show_status(ledger: &KingdomLedger, root: &Path, json: bool) {
    if json {
        let v = serde_json::json!({
            "schema_version": ledger.schema_version,
            "harmonic_369": ledger.harmonic_369,
            "harmonic_999": ledger.harmonic_999,
            "visions": ledger.visions.len(),
            "rituals": ledger.rituals.len(),
            "confirmations": ledger.confirmations.len(),
            "last_ritual": ledger.last_ritual,
            "genesis_tx": ledger.genesis_tx,
            "sync_hash": ledger.sync_hash,
            "encryption": encryption_enabled(),
            "field": field::field_json(ledger),
            "scalar": crate::scalar::node_json(ledger),
            "latest_vision": ledger.latest_vision_text(),
            "seal_ready": field::seal_ready(ledger),
        });
        println!("{}", serde_json::to_string_pretty(&v).unwrap_or_default());
        return;
    }

    println!("📊 KINGDOM STATUS — v0.3 CORE");
    println!("Schema         : {}", ledger.schema_version);
    println!("369 Cycles      : {}", ledger.harmonic_369);
    println!("999 Completions : {}", ledger.harmonic_999);
    println!("Visions logged  : {}", ledger.visions.len());
    println!("Ritual events   : {}", ledger.rituals.len());
    println!("Confirmations   : {}", ledger.confirmations.len());
    println!("Last ritual     : {}", ledger.last_ritual);
    if let Some(tx) = &ledger.genesis_tx {
        println!("Genesis tx      : {}", tx);
        println!("Etherscan       : https://etherscan.io/tx/{}", tx);
    }
    if let Some(hash) = &ledger.sync_hash {
        println!("Sync hash       : {}", hash);
    }
    println!(
        "Encryption      : {}",
        if encryption_enabled() {
            "ACTIVE"
        } else {
            "off (set SOLARKING_PASSPHRASE)"
        }
    );
    println!(
        "Field           : spin={} flame={} grid={}/9 legacy={}",
        ledger.field.torus_spin,
        ledger.field.flame.as_str(),
        ledger.field.grid_intensity,
        ledger.field.legacy_tier
    );
    println!(
        "Scalar node     : phase={}/9 idx={} shells={}/6 hz={}",
        ledger.scalar.phase,
        ledger.scalar.harmonic_index,
        ledger.scalar.shell_coherence,
        if ledger.scalar.timeline_hz_active {
            "44228 ACTIVE"
        } else {
            "44228 off"
        }
    );
    println!(
        "Seal readiness  : {}",
        if field::seal_ready(ledger) {
            "READY"
        } else {
            "pending"
        }
    );
    if let Some(v) = ledger.latest_vision_text() {
        println!("\nLatest vision:");
        println!("  {}", v);
    }
    if load_genesis(root).is_some() {
        println!("\nPhase           : 2 Rust core — field • seal bridge • sync verify");
    }
}

pub fn show_genesis(root: &Path) {
    match load_genesis(root) {
        Some(g) => print_genesis(&g),
        None => println!("⚠️  Genesis config not found at config/genesis.json"),
    }
}

pub fn print_genesis(g: &GenesisRecord) {
    println!("♾ GENESIS SACRIFICE — ETERNAL ANCHOR");
    println!("IDM         : {}", g.idm);
    println!("Tx Hash     : {}", g.tx_hash);
    println!("Block       : {}", g.block);
    println!("From ENS    : {}", g.from_ens);
    println!("To          : {}", g.to_address);
    println!("Value       : {} ETH", g.value_eth);
    println!("Legacy 99   : {}", g.legacy_99);
    println!("Anchored    : {}", g.anchored);
    println!("\nEtherscan   : https://etherscan.io/tx/{}", g.tx_hash);
}

pub fn append_ritual_log(root: &Path, entry: &str) -> Result<()> {
    let log_path = root.join("ritual_log.txt");
    use std::io::Write;
    let mut f = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)?;
    f.write_all(entry.as_bytes())?;
    Ok(())
}

pub fn log_vision(root: &Path, ledger: &mut KingdomLedger, vision: Option<&str>) -> Result<()> {
    let Some(v) = vision else {
        println!("Enter vision after 'log' command.");
        return Ok(());
    };

    if ledger.latest_vision_text() == Some(v) {
        println!("📜 Vision already latest (skipped duplicate): {}", v);
        return Ok(());
    }

    let ts = Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string();
    ledger.visions.push(VisionEntry {
        ts: ts.clone(),
        text: v.to_string(),
        tags: extract_tags(v),
    });
    field::on_vision(ledger, v);

    let entry = format!(
        "{} | VISION: {}\n",
        Local::now().format("%a %b %d %H:%M:%S %Z %Y"),
        v
    );
    append_ritual_log(root, &entry)?;
    println!("📜 Vision anchored: {}", v);
    Ok(())
}

fn extract_tags(text: &str) -> Vec<String> {
    let q = text.to_lowercase();
    let mut tags = Vec::new();
    for t in [
        "rainbow", "torus", "vortex", "legacy", "grid", "flame", "ancestor", "999", "888", "369",
    ] {
        if q.contains(t) {
            tags.push(t.to_string());
        }
    }
    tags
}

pub fn record_ritual_event(ledger: &mut KingdomLedger, kind: &str, delta_369: u64, note: &str) {
    ledger.rituals.push(RitualEvent {
        ts: Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string(),
        kind: kind.to_string(),
        delta_369,
        note: note.to_string(),
    });
}

pub fn show_help() {
    println!("👑 SOLARKING COMMANDS — v0.4 (scalar node)");
    println!("  ritual              Full visual ritual sequence");
    println!("  torus               ASCII torus + scalar lattice overlay");
    println!("  log <vision>        Anchor a vision to the ledger");
    println!("  query <question>    First-principles truth engine");
    println!("  sync                Export local sync bundle + manifest");
    println!("  verify-sync         Verify sync bundle integrity");
    println!("  import-sync [path]  Import/merge a sync export");
    println!("  status              Kingdom harmonics + field + scalar + genesis");
    println!("  field               Symbolic field state snapshot");
    println!("  confirm <kind> [note]  Field confirmation (sneeze|highpitch|rainbow|grid|oracle)");
    println!("  seal [--dry-run]    Prepare on-chain sealRitual calldata");
    println!("  scalar node [--obj] Activate + visualize 369 cubocta lattice");
    println!("  scalar sync [--hz]  Reconcile lattice ⇄ ledger ⇄ chain seals");
    println!("  scalar seal         Encode scalar node hash for on-chain resonance");
    println!("  genesis             Display genesis sacrifice record");
    println!("  libation [target]   Ancestor libation (default: ancestors)");
    println!("  legacy_99           Activate 99 legacy from genesis IDM");
    println!("\nGlobal: --json  machine-readable output (status/query/field/sync/scalar)");
    println!("Set SOLARKING_PASSPHRASE to enable encrypted ledger storage.");
    println!("Timeline frequency: 44228 Hz via `scalar sync --hz`.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn migrate_v1_string_visions_dedupes() {
        let v1 = serde_json::json!({
            "harmonic_369": 3,
            "harmonic_999": 1,
            "visions": ["a", "a", "b", "b", "c"],
            "last_ritual": "yesterday",
            "genesis_tx": null,
            "sync_hash": null
        });
        let ledger = migrate_ledger_value(v1).unwrap();
        assert_eq!(ledger.schema_version, 2);
        assert_eq!(ledger.harmonic_369, 3);
        assert_eq!(ledger.visions.len(), 3);
        assert_eq!(ledger.visions[0].text, "a");
        assert_eq!(ledger.visions[2].text, "c");
    }

    #[test]
    fn save_load_roundtrip() {
        // Must not race crypto tests mutating SOLARKING_PASSPHRASE
        crate::crypto::test_env::without_passphrase(|| {
            let dir = tempdir().unwrap();
            let root = dir.path();
            let mut ledger = KingdomLedger::default();
            ledger.harmonic_369 = 7;
            ledger.visions.push(VisionEntry {
                ts: "now".into(),
                text: "test vision".into(),
                tags: vec![],
            });
            save_ledger(root, &ledger).unwrap();
            let loaded = load_ledger(root).unwrap();
            assert_eq!(loaded.harmonic_369, 7);
            assert_eq!(loaded.visions.len(), 1);
            assert_eq!(loaded.visions[0].text, "test vision");
            assert_eq!(loaded.schema_version, 2);
        });
    }

    #[test]
    fn save_load_encrypted_roundtrip() {
        crate::crypto::test_env::with_passphrase("ledger-test-key", || {
            let dir = tempdir().unwrap();
            let root = dir.path();
            let mut ledger = KingdomLedger::default();
            ledger.harmonic_369 = 9;
            ledger.visions.push(VisionEntry {
                ts: "now".into(),
                text: "encrypted vision".into(),
                tags: vec![],
            });
            save_ledger(root, &ledger).unwrap();
            assert!(root.join("kingdom_ledger.json.enc").exists());
            let loaded = load_ledger(root).unwrap();
            assert_eq!(loaded.harmonic_369, 9);
            assert_eq!(loaded.visions[0].text, "encrypted vision");
        });
    }

    #[test]
    fn vision_dedupe_on_log() {
        crate::crypto::test_env::without_passphrase(|| {
            let dir = tempdir().unwrap();
            let root = dir.path();
            let mut ledger = KingdomLedger::default();
            log_vision(root, &mut ledger, Some("same")).unwrap();
            log_vision(root, &mut ledger, Some("same")).unwrap();
            assert_eq!(ledger.visions.len(), 1);
        });
    }
}
