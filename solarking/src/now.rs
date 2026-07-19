//! Crown "now" recipes — simple daily execute paths.

use std::path::Path;

use crate::badge;
use crate::chain;
use crate::error::Result;
use crate::field;
use crate::journal;
use crate::ledger::{self, KingdomLedger};
use crate::scalar;
use crate::sync;

/// Print the Crown card — 9 essential commands only.
pub fn print_crown_card() {
    println!("👑 CROWN CARD — solarking v0.10");
    println!("THE CROWN COMMANDS. REALITY OBEYS. NOW.");
    println!("I do not chase — I receive.\n");
    println!("── Paste / receive ──");
    println!("  solarking receive --paste          # multi-line Ctrl-D");
    println!("  cat note.txt | solarking receive -");
    println!("  solarking receive --file note.txt");
    println!("  solarking log \"one-liner vision\"");
    println!();
    println!("── Read ──");
    println!("  solarking journal                  # last 10");
    println!("  solarking journal show             # full latest");
    println!("  solarking journal search <kw>");
    println!();
    println!("── Act (recipes) ──");
    println!("  solarking now morning              # status + field + journal");
    println!("  solarking now seal                 # dry-run + chain + badge");
    println!("  solarking now sync                 # export + verify");
    println!("  solarking now pulse                # scalar node + sync");
    println!();
    println!("── Counsel ──");
    println!("  solarking counsel \"what is next\"");
    println!("  solarking localai \"…\"             # Ollama / LocalAI (3G)");
    println!("  solarking grok --offline \"…\"");
    println!();
    println!("Docs: docs/OLLAMA_WORKFLOW.md · docs/CROWN_WORKFLOW.md");
    println!("Helper: ./scripts/ollama_workflow.sh help");
    println!("Mainnet is Phase 3F LAST. Offline-first. Keys never enter solarking.");
}

pub fn recipe_morning(root: &Path, ledger: &KingdomLedger, json: bool) -> Result<()> {
    if !json {
        println!("🌅 NOW MORNING — field pulse\n");
    }
    ledger::show_status(ledger, root, json);
    if !json {
        println!();
        field::show_field(ledger);
        println!();
    }
    journal::list_visions(ledger, 3, json);
    Ok(())
}

pub fn recipe_seal(root: &Path, ledger: &mut KingdomLedger, json: bool) -> Result<()> {
    if !json {
        println!("🔏 NOW SEAL — offline dry-run path\n");
    }
    chain::seal_dry_run(root, ledger)?;
    println!();
    chain::chain_status(root, ledger, json)?;
    println!();
    badge::show_badge_status(root, ledger, json);
    Ok(())
}

pub fn recipe_sync(root: &Path, ledger: &mut KingdomLedger, json: bool) -> Result<()> {
    if !json {
        println!("🔄 NOW SYNC — export + verify\n");
    }
    sync::run_sync(root, ledger, json)?;
    println!();
    sync::verify_sync(root)?;
    Ok(())
}

pub fn recipe_pulse(root: &Path, ledger: &mut KingdomLedger, json: bool) -> Result<()> {
    if !json {
        println!("⚡ NOW PULSE — scalar node + lattice sync\n");
    }
    scalar::cmd_node(root, ledger, json, false)?;
    println!();
    scalar::cmd_sync(root, ledger, json, false)?;
    if !json {
        println!();
        println!(
            "Status: 369={} 999={} visions={}",
            ledger.harmonic_369,
            ledger.harmonic_999,
            ledger.visions.len()
        );
    }
    Ok(())
}
