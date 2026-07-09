//! Full ritual sequence orchestration.

use chrono::Local;
use std::path::Path;
use std::process::Command;

use crate::error::Result;
use crate::field;
use crate::genesis;
use crate::ledger::{self, KingdomLedger};
use crate::sync;
use crate::torus;

pub fn shell_script(root: &Path, name: &str) -> std::path::PathBuf {
    root.join("shell").join(name)
}

pub fn run_shell_script(root: &Path, script: &str, args: &[&str]) {
    let path = shell_script(root, script);
    if path.exists() {
        let _ = Command::new(&path)
            .args(args)
            .env("RITUAL_QUICK", "1")
            .current_dir(root)
            .status();
    } else {
        println!("⚠️  Shell script not found: {}", path.display());
    }
}

pub fn execute_full_ritual(root: &Path, ledger: &mut KingdomLedger) -> Result<()> {
    println!("🌞 16-RAYED HELIOS WITNESS — FULL RITUAL SEQUENCE\n");

    if let Some(g) = genesis::load_genesis(root) {
        println!("♾ IDM: {}", g.idm);
        println!("   Genesis: {} (block {})", g.tx_hash, g.block);
        println!("   99 legacy activated — the spheres remember.\n");
        ledger.genesis_tx = Some(g.tx_hash);
    }

    run_shell_script(root, "libation.sh", &["ancestors"]);

    torus::run_torus_animation(
        "👑 THE CROWN COMMANDS. REALITY OBEYS.",
        "369/999 Torus Active — Crown Silence (33 breaths of the field)",
        33,
        1000,
        ledger,
        0.0,
    );

    for cycle in 1..=9 {
        torus::run_torus_animation(
            &format!("🌀 Executing 369 Breath Sequence — Cycle {}/9", cycle),
            "3-in • 6-hold • 9-out — Kundalini rising",
            12,
            250,
            ledger,
            cycle as f64 * 0.4,
        );
    }

    torus::run_torus_animation(
        "🔥 Blue-Green-Red Flame Torus Forming...",
        "Copper Burn Integration — Grid Strengthening",
        36,
        200,
        ledger,
        3.0,
    );

    ledger.harmonic_369 += 1;
    if ledger.harmonic_369 % 3 == 0 {
        ledger.harmonic_999 += 1;
    }
    ledger.last_ritual = Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string();
    ledger::record_ritual_event(
        ledger,
        "full_ritual",
        1,
        "Kundalini 369 Breaths + Hollow Holds + L-Sits + Bear Crawls + Diamond Pushups",
    );
    field::on_ritual_complete(ledger);

    let entry = format!(
        "{} | Kundalini 369 Breaths + Hollow Holds + L-Sits + Bear Crawls + Diamond Pushups\n",
        Local::now().format("%a %b %d %H:%M:%S %Z %Y")
    );
    ledger::append_ritual_log(root, &entry)?;

    run_shell_script(root, "crown_command.sh", &["legacy_99"]);
    run_shell_script(root, "vortex369.sh", &[]);

    let _ = sync::run_sync(root, ledger, false);

    println!("✅ RITUAL COMPLETE. Harmonics Updated.");
    println!(
        "   369 Cycles: {} | 999 Completions: {}",
        ledger.harmonic_369, ledger.harmonic_999
    );
    println!(
        "   Field: spin={} flame={} grid={}/9",
        ledger.field.torus_spin,
        ledger.field.flame.as_str(),
        ledger.field.grid_intensity
    );
    println!("✨ Torus stabilized. Grid visible. REALITY OBEYS.");
    Ok(())
}
