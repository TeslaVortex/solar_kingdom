use std::io;
use std::path::PathBuf;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, SolarkingError>;

#[derive(Error, Debug)]
pub enum SolarkingError {
    #[error("IO error: {0}")]
    Io(#[from] io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("crypto error: {0}")]
    Crypto(String),

    #[error("ledger error: {0}")]
    Ledger(String),

    #[error("sync error: {0}")]
    Sync(String),

    #[error("chain error: {0}")]
    Chain(String),

    #[error("missing file: {0}")]
    MissingFile(PathBuf),

    #[error("{0}")]
    Msg(String),
}

impl SolarkingError {
    pub fn crypto(msg: impl Into<String>) -> Self {
        Self::Crypto(msg.into())
    }

    pub fn sync(msg: impl Into<String>) -> Self {
        Self::Sync(msg.into())
    }

    pub fn chain(msg: impl Into<String>) -> Self {
        Self::Chain(msg.into())
    }

    pub fn ledger(msg: impl Into<String>) -> Self {
        Self::Ledger(msg.into())
    }
}
