use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

use crate::crypto::{decrypt_ledger, encrypt_ledger, encryption_enabled};
use crate::genesis::{load_genesis, GenesisRecord};

#[derive(Serialize, Deserialize, Default, Clone)]
pub struct KingdomLedger {
    pub harmonic_369: u64,
    pub harmonic_999: u64,
    pub visions: Vec<String>,
    pub last_ritual: String,
    pub genesis_tx: Option<String>,
    pub sync_hash: Option<String>,
}

pub fn plain_ledger_path(root: &PathBuf) -> PathBuf {
    root.join("kingdom_ledger.json")
}

pub fn load_ledger(root: &PathBuf) -> KingdomLedger {
    let enc_path = root.join("kingdom_ledger.json.enc");
    let plain_path = root.join("kingdom_ledger.json");

    let mut ledger = if encryption_enabled() && enc_path.exists() {
        fs::read(&enc_path)
            .ok()
            .and_then(|bytes| decrypt_ledger(&bytes).ok())
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_default()
    } else if plain_path.exists() {
        fs::read_to_string(&plain_path)
            .ok()
            .and_then(|data| serde_json::from_str(&data).ok())
            .unwrap_or_default()
    } else {
        KingdomLedger::default()
    };

    if ledger.genesis_tx.is_none() {
        if let Some(g) = load_genesis(root) {
            ledger.genesis_tx = Some(g.tx_hash);
        }
    }

    ledger
}

pub fn save_ledger(root: &PathBuf, ledger: &KingdomLedger) {
    let data = match serde_json::to_string_pretty(ledger) {
        Ok(d) => d,
        Err(_) => return,
    };

    if encryption_enabled() {
        if let Ok(bytes) = encrypt_ledger(data.as_bytes()) {
            let _ = fs::write(root.join("kingdom_ledger.json.enc"), bytes);
        }
    } else {
        let _ = fs::write(plain_ledger_path(root), data);
    }
}

pub fn show_status(ledger: &KingdomLedger, root: &PathBuf) {
    println!("📊 KINGDOM STATUS — PHASE 1 CORE");
    println!("369 Cycles      : {}", ledger.harmonic_369);
    println!("999 Completions : {}", ledger.harmonic_999);
    println!("Visions logged  : {}", ledger.visions.len());
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
        if encryption_enabled() { "ACTIVE" } else { "off (set SOLARKING_PASSPHRASE)" }
    );
    if !ledger.visions.is_empty() {
        println!("\nLatest vision:");
        println!("  {}", ledger.visions.last().unwrap());
    }
    if load_genesis(root).is_some() {
        println!("\nPhase 1         : COMPLETE — query • sync • genesis integrated");
    }
}

pub fn show_genesis(root: &PathBuf) {
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

pub fn append_ritual_log(root: &PathBuf, entry: &str) {
    let log_path = root.join("ritual_log.txt");
    let _ = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut f| std::io::Write::write_all(&mut f, entry.as_bytes()));
}

pub fn log_vision(root: &PathBuf, ledger: &mut KingdomLedger, vision: Option<&str>) {
    if let Some(v) = vision {
        if !ledger.visions.last().map(|s| s == v).unwrap_or(false) {
            ledger.visions.push(v.to_string());
        }
        let entry = format!(
            "{} | VISION: {}\n",
            Local::now().format("%a %b %d %H:%M:%S %Z %Y"),
            v
        );
        append_ritual_log(root, &entry);
        println!("📜 Vision anchored: {}", v);
    } else {
        println!("Enter vision after 'log' command.");
    }
}

pub fn show_help() {
    println!("👑 SOLARKING COMMANDS — PHASE 1");
    println!("  ritual              Full visual ritual sequence");
    println!("  torus               ASCII torus visualization");
    println!("  log <vision>        Anchor a vision to the ledger");
    println!("  query <question>    First-principles truth engine");
    println!("  sync                Export local sync bundle + manifest");
    println!("  status              Kingdom harmonics + genesis");
    println!("  genesis             Display genesis sacrifice record");
    println!("  libation [target]   Ancestor libation (default: ancestors)");
    println!("  legacy_99           Activate 99 legacy from genesis IDM");
    println!("\nSet SOLARKING_PASSPHRASE to enable encrypted ledger storage.");
}