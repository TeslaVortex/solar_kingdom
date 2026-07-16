//! Phase 3E — Grok Build CLI bridge (https://github.com/xai-org/grok-build)
//! Offline counsel first; optional `grok -p` single-turn when binary present.
//! Never embeds API keys — grok uses its own auth (~/.grok).

use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::error::{Result, SolarkingError};
use crate::ledger::KingdomLedger;
use crate::phase3;

/// Build a sovereign context brief for Grok / local counsel.
pub fn context_brief(_root: &Path, ledger: &KingdomLedger) -> String {
    format!(
        "You are counsel to the Eternal Solar Kingdom (solarking). \
         First principles: truth, resonance, harmony, legacy, abundance, sovereignty, offline-first. \
         Mainnet is Phase 3F LAST — never push mainnet without explicit Crown command. \
         Field: flame={} grid={}/9 legacy={} spin={}. \
         Scalar: phase={}/9 idx={} shells={}/6 hz44228={} nodeId={:?}. \
         Harmonics: 369={} 999={}. Sync={:?}. Genesis={:?}. \
         Repo: solar_kingdom on Phase-3-Eternal-Expansion. \
         Motto: THE CROWN COMMANDS. REALITY OBEYS. I do not chase — I receive.\n",
        ledger.field.flame.as_str(),
        ledger.field.grid_intensity,
        ledger.field.legacy_tier,
        ledger.field.torus_spin,
        ledger.scalar.phase,
        ledger.scalar.harmonic_index,
        ledger.scalar.shell_coherence,
        ledger.scalar.timeline_hz_active,
        ledger.scalar.onchain_node_id,
        ledger.harmonic_369,
        ledger.harmonic_999,
        ledger.sync_hash,
        ledger.genesis_tx,
    )
}

pub fn grok_available() -> bool {
    Command::new("which")
        .arg("grok")
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Invoke Grok Build headless single-turn (`grok -p`). Falls back to offline counsel.
pub fn run_grok(
    root: &Path,
    ledger: &KingdomLedger,
    prompt: &str,
    offline_only: bool,
) -> Result<()> {
    let brief = context_brief(root, ledger);
    let full = format!(
        "{brief}\nUser question / task:\n{prompt}\n\n\
         Answer as sovereign counsel. Prefer local tools and Base Sepolia testnet. \
         Do not invent mainnet addresses. Keep secrets out of answers."
    );

    // Always write prompt for audit / resume
    let dir = root.join("sync").join("grok");
    fs::create_dir_all(&dir)?;
    let prompt_path = dir.join("last_prompt.txt");
    fs::write(&prompt_path, &full)?;

    if offline_only || !grok_available() {
        if !grok_available() {
            println!("⚠️  `grok` CLI not on PATH — offline counsel only.");
            println!("   Install: curl -fsSL https://x.ai/cli/install.sh | bash");
            println!("   Repo: https://github.com/xai-org/grok-build");
        } else {
            println!("📴 Offline-only flag set — not calling grok.");
        }
        println!();
        println!("{}", phase3::counsel(root, ledger, prompt, false));
        return Ok(());
    }

    println!("🪄 GROK BUILD CLI — single-turn (-p)");
    println!("───────────────────────────────");
    println!("Prompt file: {}", prompt_path.display());
    println!("Auth: grok's own session (~/.grok) — solarking never holds xAI keys.");
    println!();

    let output = Command::new("grok")
        .arg("-p")
        .arg(&full)
        .arg("--cwd")
        .arg(root)
        .arg("--output-format")
        .arg("plain")
        .current_dir(root)
        .output()
        .map_err(|e| SolarkingError::Msg(format!("failed to spawn grok: {e}")))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !stdout.trim().is_empty() {
        println!("{stdout}");
        fs::write(dir.join("last_response.txt"), stdout.as_bytes())?;
    }
    if !output.status.success() {
        if !stderr.trim().is_empty() {
            eprintln!("{stderr}");
        }
        println!();
        println!("⚠️  grok exited non-zero — falling back to offline counsel:");
        println!("{}", phase3::counsel(root, ledger, prompt, false));
    }
    Ok(())
}

/// Append a short blueprint evolution note via offline counsel (3E seed).
pub fn evolve_blueprint(root: &Path, ledger: &KingdomLedger, note: &str) -> Result<()> {
    let path = root.join("docs").join("PHASE_3_BLUEPRINT.md");
    let stamp = chrono::Local::now().format("%Y-%m-%d %H:%M:%S %Z");
    let guidance = phase3::counsel(root, ledger, note, false);
    let appendix = format!(
        "\n\n---\n\n## Blueprint pulse — {stamp}\n\n**Seed:** {note}\n\n```\n{guidance}\n```\n"
    );
    if path.exists() {
        let mut body = fs::read_to_string(&path)?;
        body.push_str(&appendix);
        fs::write(&path, body)?;
    } else {
        fs::create_dir_all(root.join("docs"))?;
        fs::write(&path, format!("# Phase 3 Blueprint\n{appendix}"))?;
    }
    println!("📜 Blueprint evolved: {}", path.display());
    println!("Offline counsel appended. Optional: solarking grok \"refine blueprint\"");
    Ok(())
}

/// Optional local OpenAI-compatible endpoint (SOLARKING_LOCAL_MODEL=base_url).
/// Placeholder: prints how to wire; does not require network for default path.
pub fn local_model_status() {
    match env::var("SOLARKING_LOCAL_MODEL") {
        Ok(u) if !u.is_empty() => {
            println!("Local model endpoint: {u}");
            println!("(POST /v1/chat/completions — not auto-called in v0.8; use grok -p or counsel)");
        }
        _ => println!("SOLARKING_LOCAL_MODEL unset — pure offline counsel + optional grok CLI"),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::KingdomLedger;

    #[test]
    fn brief_contains_motto_fields() {
        let ledger = KingdomLedger::default();
        let b = context_brief(std::path::Path::new("."), &ledger);
        assert!(b.contains("CROWN COMMANDS") || b.contains("sovereign"));
        assert!(b.contains("369"));
    }
}
