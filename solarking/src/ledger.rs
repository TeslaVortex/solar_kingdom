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

    println!("📊 KINGDOM STATUS — v0.10 CROWN");
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

/// Result of anchoring a vision / transmission.
#[derive(Debug, Clone)]
pub struct LogVisionResult {
    pub index: usize,
    pub ts: String,
    pub tags: Vec<String>,
    pub bytes: usize,
    pub multi_line: bool,
    pub sha8: String,
    pub archive_path: Option<PathBuf>,
    pub skipped_duplicate: bool,
}

pub fn log_vision(root: &Path, ledger: &mut KingdomLedger, vision: Option<&str>) -> Result<()> {
    match log_vision_ex(root, ledger, vision, false) {
        Ok(r) if r.skipped_duplicate => Ok(()),
        Ok(r) => {
            let preview = first_line_preview(vision.unwrap_or(""), 120);
            if r.multi_line {
                println!(
                    "📜 Vision anchored ({} lines, {}b) — {}",
                    vision.unwrap_or("").lines().count(),
                    r.bytes,
                    preview
                );
            } else {
                println!("📜 Vision anchored: {}", preview);
            }
            Ok(())
        }
        Err(e) => Err(e),
    }
}

/// Anchor vision; optionally write raw archive under sync/transmissions/.
pub fn log_vision_ex(
    root: &Path,
    ledger: &mut KingdomLedger,
    vision: Option<&str>,
    archive: bool,
) -> Result<LogVisionResult> {
    let Some(v) = vision else {
        return Err(SolarkingError::Msg(
            "Enter vision after 'log' / 'receive' (or use --paste / --file / -).".into(),
        ));
    };
    let v = v.trim_end_matches(['\r', '\n']);
    if v.is_empty() {
        return Err(SolarkingError::Msg("empty transmission — nothing to anchor".into()));
    }

    if ledger.latest_vision_text() == Some(v) {
        println!(
            "📜 Vision already latest (skipped duplicate): {}",
            first_line_preview(v, 80)
        );
        return Ok(LogVisionResult {
            index: ledger.visions.len().saturating_sub(1),
            ts: ledger.visions.last().map(|e| e.ts.clone()).unwrap_or_default(),
            tags: ledger.visions.last().map(|e| e.tags.clone()).unwrap_or_default(),
            bytes: v.len(),
            multi_line: v.contains('\n'),
            sha8: short_sha8(v.as_bytes()),
            archive_path: None,
            skipped_duplicate: true,
        });
    }

    let ts = Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string();
    let tags = extract_tags(v);
    let multi_line = v.contains('\n');
    let sha8 = short_sha8(v.as_bytes());

    ledger.visions.push(VisionEntry {
        ts: ts.clone(),
        text: v.to_string(),
        tags: tags.clone(),
    });
    field::on_vision(ledger, v);

    let ritual_ts = Local::now().format("%a %b %d %H:%M:%S %Z %Y");
    if multi_line {
        let entry = format!(
            "=== TRANSMISSION BEGIN {ritual_ts} ===\n{v}\n=== TRANSMISSION END sha={sha8} ===\n"
        );
        append_ritual_log(root, &entry)?;
    } else {
        let entry = format!("{ritual_ts} | VISION: {v}\n");
        append_ritual_log(root, &entry)?;
    }

    let archive_path = if archive {
        Some(write_transmission_archive(root, v, &ts, &sha8, &tags)?)
    } else {
        None
    };

    Ok(LogVisionResult {
        index: ledger.visions.len() - 1,
        ts,
        tags,
        bytes: v.len(),
        multi_line,
        sha8,
        archive_path,
        skipped_duplicate: false,
    })
}

/// Full Crown receive: archive + anchor + seal print.
pub fn receive_transmission(
    root: &Path,
    ledger: &mut KingdomLedger,
    body: &str,
    title: Option<&str>,
) -> Result<LogVisionResult> {
    let body = body.trim_end_matches(['\r', '\n']);
    let full = if let Some(t) = title.map(str::trim).filter(|s| !s.is_empty()) {
        if body.starts_with(t) {
            body.to_string()
        } else {
            format!("{t}\n\n{body}")
        }
    } else {
        body.to_string()
    };

    let result = log_vision_ex(root, ledger, Some(&full), true)?;
    if result.skipped_duplicate {
        return Ok(result);
    }

    println!("📥 TRANSMISSION RECEIVED");
    println!("   index  : #{}", result.index);
    println!("   ts     : {}", result.ts);
    println!("   bytes  : {}", result.bytes);
    if result.multi_line {
        println!("   form   : multi-line block");
    }
    if !result.tags.is_empty() {
        println!("   tags   : {}", result.tags.join(", "));
    }
    println!("   sha8   : {}", result.sha8);
    if let Some(p) = &result.archive_path {
        println!("   archive: {}", p.display());
    }
    println!("   read   : solarking journal show");
    println!("THE CROWN COMMANDS. REALITY OBEYS. I do not chase — I receive.");
    Ok(result)
}

fn write_transmission_archive(
    root: &Path,
    body: &str,
    ts: &str,
    sha8: &str,
    tags: &[String],
) -> Result<PathBuf> {
    let dir = root.join("sync").join("transmissions");
    fs::create_dir_all(&dir)?;
    let stamp = Local::now().format("%Y%m%d_%H%M%S");
    let base = format!("{stamp}_{sha8}");
    let txt_path = dir.join(format!("{base}.txt"));
    let meta_path = dir.join(format!("{base}.meta.json"));
    fs::write(&txt_path, body.as_bytes())?;
    let meta = serde_json::json!({
        "ts": ts,
        "bytes": body.len(),
        "sha256": crate::sync::hex_sha256(body.as_bytes()),
        "sha8": sha8,
        "tags": tags,
        "path": txt_path.display().to_string(),
    });
    fs::write(&meta_path, serde_json::to_string_pretty(&meta)?)?;
    Ok(txt_path)
}

fn short_sha8(data: &[u8]) -> String {
    crate::sync::hex_sha256(data).chars().take(8).collect()
}

fn first_line_preview(text: &str, max: usize) -> String {
    let line = text.lines().next().unwrap_or(text).trim();
    if line.chars().count() <= max {
        return line.to_string();
    }
    let truncated: String = line.chars().take(max.saturating_sub(1)).collect();
    format!("{truncated}…")
}

pub fn extract_tags(text: &str) -> Vec<String> {
    let q = text.to_lowercase();
    let mut tags = Vec::new();
    for t in [
        "rainbow",
        "torus",
        "vortex",
        "legacy",
        "grid",
        "flame",
        "ancestor",
        "999",
        "888",
        "369",
        "queen",
        "crown",
        "transmission",
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
    println!("👑 SOLARKING COMMANDS — v0.10 (Crown + LocalAI 3G)");
    println!("  receive [--paste|--file|-]   Paste whole transmission → ledger + archive");
    println!("  journal list|show|search|log|files   Read transmissions");
    println!("  now card|morning|seal|sync|pulse     Simple execute recipes");
    println!("  log · query · status · field · confirm · ritual · torus");
    println!("  counsel [q]         Local offline counsel");
    println!("  localai [prompt]    Ollama/LocalAI (OpenAI-compatible; --status)");
    println!("  grok [prompt]       Ladder: local → grok -p → offline (--offline)");
    println!("  blueprint [note]    Append counsel pulse to PHASE_3_BLUEPRINT.md");
    println!("  export-cid · qr · altar-print");
    println!("  lattice visualize   ASCII + web/lattice.html (Three.js CDN)");
    println!("  node init|export|import|status   Kingdom node federation");
    println!("  sync · verify-sync · import-sync · cold-export");
    println!("  badge-status · seal · chain-status · seal-record · scalar-record");
    println!("  scalar node|sync|seal · genesis · libation · legacy_99");
    println!("\nDocs: docs/CROWN_WORKFLOW.md");
    println!("Grok Build: https://github.com/xai-org/grok-build  (install: x.ai/cli)");
    println!("Mainnet is Phase 3F LAST. I do not chase — I receive.");
    println!("Keys never enter solarking. Offline-first.");
    println!("Canonical ledger: kingdom root kingdom_ledger.json (not solarking/).");
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
