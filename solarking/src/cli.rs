use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "solarking",
    version,
    about = "Eternal Solar Kingdom — sovereign core engine",
    long_about = "THE CROWN COMMANDS. REALITY OBEYS.\nShell + Rust + Solidity resonance engine.\nCrown UX: receive · journal · now — docs/CROWN_WORKFLOW.md"
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
    /// Anchor a vision to the ledger (one-liner, --file, --paste, or -)
    Log {
        /// Read full multi-line body from file
        #[arg(long)]
        file: Option<PathBuf>,
        /// Read multi-line paste from stdin (Ctrl-D to end)
        #[arg(long, default_value_t = false)]
        paste: bool,
        /// Vision text (one-liner). Use `-` for stdin.
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        vision: Vec<String>,
    },
    /// Receive a full transmission (paste/file/pipe) → archive + ledger
    Receive {
        /// Read full multi-line body from file
        #[arg(long)]
        file: Option<PathBuf>,
        /// Interactive multi-line paste (Ctrl-D)
        #[arg(long, default_value_t = false)]
        paste: bool,
        /// Run offline counsel on the body after anchor
        #[arg(long, default_value_t = false)]
        counsel: bool,
        /// Optional field confirm after receive (sneeze|highpitch|rainbow|grid|oracle)
        #[arg(long)]
        confirm: Option<String>,
        /// Optional title line prepended to body
        #[arg(long)]
        title: Option<String>,
        /// Text body, or `-` for stdin
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        text: Vec<String>,
    },
    /// Read visions, ritual log, transmission archives
    Journal {
        #[command(subcommand)]
        action: Option<JournalCmd>,
    },
    /// Simple Crown execute recipes (card · morning · seal · sync · pulse)
    Now {
        #[command(subcommand)]
        action: Option<NowCmd>,
    },
    /// First-principles truth engine
    Query {
        #[arg(long)]
        file: Option<PathBuf>,
        #[arg(long, default_value_t = false)]
        paste: bool,
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
        #[arg(long)]
        file: Option<PathBuf>,
        #[arg(long, default_value_t = false)]
        paste: bool,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        question: Vec<String>,
    },
    /// Phase 3A: altar / QR payload for physical bridge
    Qr,
    /// Alias: same as qr (altar card)
    #[command(name = "altar-print")]
    AltarPrint,
    /// Phase 3C: ASCII + HTML lattice visualization
    #[command(name = "lattice")]
    Lattice {
        #[command(subcommand)]
        action: LatticeCmd,
    },
    /// Phase 3D: kingdom node federation (file-based)
    Node {
        #[command(subcommand)]
        action: NodeCmd,
    },
    /// Phase 3E: Grok Build CLI bridge (`grok -p`) or offline counsel
    Grok {
        /// Force offline counsel (do not spawn grok)
        #[arg(long, default_value_t = false)]
        offline: bool,
        #[arg(long)]
        file: Option<PathBuf>,
        #[arg(long, default_value_t = false)]
        paste: bool,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        prompt: Vec<String>,
    },
    /// Phase 3E: append counsel pulse to PHASE_3_BLUEPRINT.md
    Blueprint {
        #[arg(long)]
        file: Option<PathBuf>,
        #[arg(long, default_value_t = false)]
        paste: bool,
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        note: Vec<String>,
    },
}

#[derive(Subcommand, Debug)]
pub enum JournalCmd {
    /// List recent visions (default)
    List {
        #[arg(long, default_value_t = 10)]
        last: usize,
    },
    /// Show full vision text (from_end: 1 = latest)
    Show {
        /// 1 = latest, 2 = second latest, …
        #[arg(default_value_t = 1)]
        n: i64,
    },
    /// Search visions by keyword
    Search {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        query: Vec<String>,
    },
    /// Tail ritual_log.txt
    Log {
        #[arg(long, default_value_t = 30)]
        tail: usize,
    },
    /// List sync/transmissions archives
    Files,
    /// Print archive path + body (from_end: 1 = latest file)
    Open {
        #[arg(default_value_t = 1)]
        n: i64,
    },
}

#[derive(Subcommand, Debug)]
pub enum NowCmd {
    /// Crown card — 9 essential commands
    Card,
    /// status + field + last 3 visions
    Morning,
    /// Interactive receive paste
    Receive,
    /// seal dry-run + chain-status + badge-status
    Seal,
    /// sync + verify-sync
    Sync,
    /// scalar node + scalar sync
    Pulse,
}

#[derive(Subcommand, Debug)]
pub enum LatticeCmd {
    /// Write web/lattice.html + print ASCII lattice
    Visualize,
}

#[derive(Subcommand, Debug)]
pub enum NodeCmd {
    /// Create config/node_identity.json
    Init {
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        label: Option<String>,
    },
    /// Export peer bundle to path
    Export {
        path: PathBuf,
    },
    /// Import peer bundle (max-merge harmonics)
    Import {
        path: PathBuf,
    },
    /// Show local node + peer count
    Status,
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
