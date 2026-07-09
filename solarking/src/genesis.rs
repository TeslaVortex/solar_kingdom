use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone, Debug)]
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

pub fn genesis_path(root: &Path) -> std::path::PathBuf {
    root.join("config").join("genesis.json")
}

pub fn load_genesis(root: &Path) -> Option<GenesisRecord> {
    let path = genesis_path(root);
    fs::read_to_string(&path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn load_fixture() {
        let dir = tempdir().unwrap();
        let cfg = dir.path().join("config");
        fs::create_dir_all(&cfg).unwrap();
        fs::write(
            cfg.join("genesis.json"),
            r#"{
                "idm": "test idm",
                "tx_hash": "0xabc",
                "block": 1,
                "from_ens": "test.eth",
                "to_address": "0x369",
                "value_eth": "0.000999",
                "legacy_99": true,
                "anchored": "2026-07-02"
            }"#,
        )
        .unwrap();
        let g = load_genesis(dir.path()).unwrap();
        assert_eq!(g.tx_hash, "0xabc");
        assert_eq!(g.block, 1);
        assert!(g.legacy_99);
    }
}
