use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Clone)]
pub struct GenesisRecord {
    pub idm: String,
    pub tx_hash: String,
    pub block: u64,
    pub from_ens: String,
    pub to_address: String,
    pub value_eth: String,
    pub legacy_99: bool,
    pub anchored: String,
}

pub fn genesis_path(root: &PathBuf) -> PathBuf {
    root.join("config").join("genesis.json")
}

pub fn load_genesis(root: &PathBuf) -> Option<GenesisRecord> {
    let path = genesis_path(root);
    fs::read_to_string(&path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
}