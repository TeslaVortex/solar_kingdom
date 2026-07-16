use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "solarking",
    version,
    about = "Eternal Solar Kingdom — sovereign core engine",
    long_about = "THE CROWN COMMANDS. REALITY OBEYS.\nShell + Rust + Solidity resonance engine."
)]
pub struct Cli {
    /// Machine-readable JSON output where supported
    #[arg(long, global = true)]
    pub json: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Full visual ritual sequence
    Ritual,
    /// ASCII torus visualization
    Torus,
    /// Anchor a vision to the ledger
    Log {
        /// Vision text
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        vision: Vec<String>,
    },
    /// First-principles truth engine
    Query {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        question: Vec<String>,
    },
    /// Export local sync bundle + SHA-256 manifest
    Sync,
    /// Verify sync bundle integrity against manifest
    #[command(name = "verify-sync")]
    VerifySync,
    /// Import/merge a sync export (default: sync/latest/kingdom_export.json)
    #[command(name = "import-sync")]
    ImportSync {
        path: Option<PathBuf>,
    },
    /// Kingdom harmonics + field + genesis
    Status,
    /// Symbolic field state snapshot
    Field,
    /// Field confirmation (sneeze|highpitch|rainbow|grid|oracle)
    Confirm {
        kind: String,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        note: Vec<String>,
    },
    /// Prepare on-chain sealRitual calldata (offline dry-run by default)
    Seal {
        /// Offline dry-run (default; always safe)
        #[arg(long)]
        dry_run: bool,
        /// Print external cast broadcast recipe only (never sends txs)
        #[arg(long, default_value_t = false)]
        broadcast: bool,
    },
    /// Display genesis sacrifice record
    Genesis,
    /// Show ceremonial command help
    Help,
    /// Ancestor libation
    Libation {
        target: Option<String>,
    },
    /// Activate 99 legacy from genesis IDM
    #[command(name = "legacy_99", alias = "legacy")]
    Legacy99,
    /// Tesla 369 scalar node — nested cuboctahedron lattice
    Scalar {
        #[command(subcommand)]
        action: ScalarCmd,
    },
    /// Offline/online chain status (RPC probe if configured)
    #[command(name = "chain-status")]
    ChainStatus,
    /// Record seal tx hash after external cast send
    #[command(name = "seal-record")]
    SealRecord {
        tx_hash: String,
    },
    /// Record on-chain scalar nodeId after activateScalarNode
    #[command(name = "scalar-record")]
    ScalarRecord {
        node_id: u64,
        tx_hash: Option<String>,
    },
    /// Soul-bound badge eligibility dry-run (Phase 2C)
    #[command(name = "badge-status")]
    BadgeStatus,
    /// Cold-export sync bundle to USB/offline path (optional encrypt)
    #[command(name = "cold-export")]
    ColdExport {
        /// Destination directory (created if missing)
        dest: PathBuf,
        /// Also write ChaCha20-Poly1305 envelope (requires SOLARKING_PASSPHRASE)
        #[arg(long, default_value_t = false)]
        encrypt: bool,
    },
    /// Phase 3A: decentralized pin instructions (+ optional ipfs add)
    #[command(name = "export-cid")]
    ExportCid,
    /// Phase 3A: local sovereign counsel (query + field/scalar)
    Counsel {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        question: Vec<String>,
    },
    /// Phase 3A: altar / QR payload for physical bridge
    Qr,
    /// Alias: same as qr (altar card)
    #[command(name = "altar-print")]
    AltarPrint,
}

#[derive(Subcommand, Debug)]
pub enum ScalarCmd {
    /// Visualize and activate the current lattice state (ASCII + optional OBJ)
    Node {
        /// Export Wavefront OBJ to sync/scalar/scalar_node.obj
        #[arg(long, default_value_t = false)]
        obj: bool,
    },
    /// Reconcile geometric lattice with local ledger and on-chain seals
    Sync {
        /// Enable 44 228 Hz timeline frequency as phase-step multiplier
        #[arg(long, default_value_t = false)]
        hz: bool,
    },
    /// Encode node state into a ritual hash for on-chain resonance
    Seal,
}
