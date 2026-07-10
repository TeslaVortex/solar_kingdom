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
