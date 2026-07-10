//! Scalar Node — Tesla 369 in 3D.
//! Nested cuboctahedron lattice · central scalar point · phase shifts over 1296 harmonics.
//! Optional resonance key: 44 228 Hz dream timeline frequency.
//! ZERO randomness — all phase advances are deterministic from ledger harmonics.

use chrono::Local;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::f64::consts::PI;
use std::fs;
use std::io::{self, Write};
use std::path::Path;

use crate::error::Result;
use crate::field;
use crate::ledger::{self, KingdomLedger};

/// Nested cuboctahedron shells around the central scalar point (Tesla 3-6-9 geometry).
pub const NESTED_SHELLS: usize = 6;
/// Full harmonic lattice period (36² = 6⁴ = 1296). Digital root = 9.
pub const HARMONIC_PERIOD: u32 = 1296;
/// Dream-timeline frequency (Hz) — optional resonance multiplier.
pub const TIMELINE_HZ: u32 = 44_228;
/// Cuboctahedron vertex count per shell.
pub const CUBOCTA_VERTICES: usize = 12;

/// Persistent scalar node state on the kingdom ledger.
#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ScalarNodeState {
    /// Phase in 1..=9
    pub phase: u8,
    /// Position in the 1296 harmonic lattice [0, 1295]
    pub harmonic_index: u32,
    /// Whether timeline Hz multiplier is active
    pub timeline_hz_active: bool,
    /// Last computed seal hash (hex SHA-256 of node snapshot)
    pub last_seal_hash: Option<String>,
    /// Last sync / activation timestamp
    pub last_sync: String,
    /// Activation count (deterministic counters only)
    pub activations: u64,
    /// Shell coherence 0–6 (how many nested shells are “lit”)
    pub shell_coherence: u8,
}

impl Default for ScalarNodeState {
    fn default() -> Self {
        Self {
            phase: 1,
            harmonic_index: 0,
            timeline_hz_active: false,
            last_seal_hash: None,
            last_sync: String::new(),
            activations: 0,
            shell_coherence: 0,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// Unit cuboctahedron vertices (all permutations of (±1, ±1, 0)).
pub fn cuboctahedron_unit_vertices() -> [Vec3; CUBOCTA_VERTICES] {
    [
        Vec3 { x: 1.0, y: 1.0, z: 0.0 },
        Vec3 { x: 1.0, y: -1.0, z: 0.0 },
        Vec3 { x: -1.0, y: 1.0, z: 0.0 },
        Vec3 { x: -1.0, y: -1.0, z: 0.0 },
        Vec3 { x: 1.0, y: 0.0, z: 1.0 },
        Vec3 { x: 1.0, y: 0.0, z: -1.0 },
        Vec3 { x: -1.0, y: 0.0, z: 1.0 },
        Vec3 { x: -1.0, y: 0.0, z: -1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 1.0 },
        Vec3 { x: 0.0, y: 1.0, z: -1.0 },
        Vec3 { x: 0.0, y: -1.0, z: 1.0 },
        Vec3 { x: 0.0, y: -1.0, z: -1.0 },
    ]
}

/// Nested shell vertices: shell `s` (1..=6) scaled by s, rotated by phase angle.
pub fn nested_shell_vertices(shell: usize, phase: u8) -> Vec<Vec3> {
    let scale = shell as f64;
    let angle = phase_angle(phase) + (shell as f64) * (PI / 9.0);
    let (sa, ca) = angle.sin_cos();
    cuboctahedron_unit_vertices()
        .into_iter()
        .map(|v| {
            // Rotate around Z (Kagome-friendly plan view)
            let x = v.x * ca - v.y * sa;
            let y = v.x * sa + v.y * ca;
            Vec3 {
                x: x * scale,
                y: y * scale,
                z: v.z * scale,
            }
        })
        .collect()
}

pub fn phase_angle(phase: u8) -> f64 {
    let p = phase.clamp(1, 9) as f64;
    (p - 1.0) * (2.0 * PI / 9.0)
}

/// Deterministic phase 1..=9 from ledger harmonics + scalar index.
pub fn compute_phase(harmonic_369: u64, harmonic_999: u64, harmonic_index: u32) -> u8 {
    let raw = harmonic_369
        .wrapping_mul(3)
        .wrapping_add(harmonic_999.wrapping_mul(6))
        .wrapping_add(harmonic_index as u64)
        .wrapping_add(9);
    ((raw % 9) as u8) + 1
}

/// Advance harmonic index in the 1296 lattice (optional 44228 Hz multiplier).
pub fn advance_harmonic_index(current: u32, steps: u32, timeline_hz_active: bool) -> u32 {
    let step = if timeline_hz_active {
        steps.wrapping_mul(TIMELINE_HZ % HARMONIC_PERIOD).max(1)
    } else {
        steps.max(1)
    };
    (current + step) % HARMONIC_PERIOD
}

/// Shell coherence from field grid + nested shell geometry.
pub fn compute_shell_coherence(ledger: &KingdomLedger) -> u8 {
    let from_grid = (ledger.field.grid_intensity.min(9) as u8).min(NESTED_SHELLS as u8);
    let from_rituals = if ledger.harmonic_369 == 0 {
        0
    } else {
        ((ledger.harmonic_369.min(18) as u8) / 3).min(NESTED_SHELLS as u8)
    };
    from_grid.max(from_rituals).max(ledger.scalar.shell_coherence.min(NESTED_SHELLS as u8))
}

/// Recompute node from ledger field/harmonics (does not advance activations).
pub fn reconcile_from_ledger(ledger: &mut KingdomLedger) {
    let idx = ledger.scalar.harmonic_index;
    ledger.scalar.phase = compute_phase(ledger.harmonic_369, ledger.harmonic_999, idx);
    ledger.scalar.shell_coherence = compute_shell_coherence(ledger);
    ledger.scalar.last_sync = Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string();
}

/// Canonical snapshot bytes for hashing / on-chain resonance payload.
pub fn node_snapshot_bytes(ledger: &KingdomLedger) -> Vec<u8> {
    let s = &ledger.scalar;
    format!(
        "SOLARKING_SCALAR_NODE|v1|phase={}|idx={}|hz={}|shells={}|h369={}|h999={}|legacy={}|spin={}|seal_tx={}|genesis={}",
        s.phase,
        s.harmonic_index,
        if s.timeline_hz_active { TIMELINE_HZ } else { 0 },
        s.shell_coherence,
        ledger.harmonic_369,
        ledger.harmonic_999,
        ledger.field.legacy_tier,
        ledger.field.torus_spin,
        ledger.chain.last_seal_tx.as_deref().unwrap_or("-"),
        ledger.genesis_tx.as_deref().unwrap_or("-"),
    )
    .into_bytes()
}

pub fn seal_hash(ledger: &KingdomLedger) -> String {
    let digest = Sha256::digest(node_snapshot_bytes(ledger));
    digest.iter().map(|b| format!("{:02x}", b)).collect()
}

// ── CLI handlers ────────────────────────────────────────────────────────────

/// `solarking scalar node` — activate + visualize lattice.
pub fn cmd_node(root: &Path, ledger: &mut KingdomLedger, json: bool, export_obj: bool) -> Result<()> {
    ledger.scalar.activations = ledger.scalar.activations.saturating_add(1);
    ledger.scalar.harmonic_index = advance_harmonic_index(
        ledger.scalar.harmonic_index,
        3, // Tesla 3 step
        ledger.scalar.timeline_hz_active,
    );
    reconcile_from_ledger(ledger);
    // Light field coupling: spin +1, grid nudge
    ledger.field.torus_spin = ledger.field.torus_spin.saturating_add(1);
    if ledger.field.grid_intensity < 9 {
        ledger.field.grid_intensity = ledger.field.grid_intensity.saturating_add(1);
    }
    ledger.field.last_transition = format!(
        "{} | scalar node activation #{}",
        ledger.scalar.last_sync, ledger.scalar.activations
    );

    if json {
        println!("{}", serde_json::to_string_pretty(&node_json(ledger))?);
    } else {
        print_node_banner(ledger);
        print_ascii_lattice(ledger);
        print_node_stats(ledger);
    }

    if export_obj {
        let path = export_obj_file(root, ledger)?;
        if !json {
            println!("\n📐 3D export (OBJ): {}", path.display());
        }
    }

    let entry = format!(
        "{} | SCALAR NODE activate phase={} idx={} shells={}/6 hz={}\n",
        Local::now().format("%a %b %d %H:%M:%S %Z %Y"),
        ledger.scalar.phase,
        ledger.scalar.harmonic_index,
        ledger.scalar.shell_coherence,
        if ledger.scalar.timeline_hz_active {
            TIMELINE_HZ
        } else {
            0
        }
    );
    ledger::append_ritual_log(root, &entry)?;
    ledger::record_ritual_event(
        ledger,
        "scalar_node",
        0,
        &format!(
            "phase={} idx={} shells={}",
            ledger.scalar.phase, ledger.scalar.harmonic_index, ledger.scalar.shell_coherence
        ),
    );

    if !json {
        println!("\n✨ Scalar node activated. Lattice coherent. REALITY OBEYS.");
    }
    Ok(())
}

/// `solarking scalar sync` — reconcile lattice with ledger + chain seals.
pub fn cmd_sync(root: &Path, ledger: &mut KingdomLedger, json: bool, enable_hz: bool) -> Result<()> {
    if enable_hz {
        ledger.scalar.timeline_hz_active = true;
    }
    reconcile_from_ledger(ledger);

    // Align harmonic index subtly with local 369 (deterministic fold into 1296)
    let fold = ((ledger.harmonic_369.wrapping_mul(9)
        .wrapping_add(ledger.harmonic_999.wrapping_mul(36)))
        % HARMONIC_PERIOD as u64) as u32;
    // Blend: keep half prior index, half ledger fold (integer, deterministic)
    ledger.scalar.harmonic_index =
        (ledger.scalar.harmonic_index.wrapping_add(fold) / 2) % HARMONIC_PERIOD;
    ledger.scalar.phase = compute_phase(
        ledger.harmonic_369,
        ledger.harmonic_999,
        ledger.scalar.harmonic_index,
    );
    ledger.scalar.shell_coherence = compute_shell_coherence(ledger);
    ledger.scalar.last_sync = Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string();

    let seal_ready = field::seal_ready(ledger);
    let chain_tx = ledger.chain.last_seal_tx.clone();

    if json {
        let mut v = node_json(ledger);
        if let Some(obj) = v.as_object_mut() {
            obj.insert("seal_ready".into(), serde_json::json!(seal_ready));
            obj.insert(
                "last_seal_tx".into(),
                serde_json::json!(chain_tx),
            );
            obj.insert(
                "sync_hash".into(),
                serde_json::json!(ledger.sync_hash),
            );
        }
        println!("{}", serde_json::to_string_pretty(&v)?);
    } else {
        println!("🔷 SCALAR SYNC — LATTICE ⇄ LEDGER ⇄ CHAIN");
        println!("────────────────────────────────────────");
        print_node_stats(ledger);
        println!();
        println!(
            "Local harmonics : 369={}  999={}",
            ledger.harmonic_369, ledger.harmonic_999
        );
        println!(
            "Field           : spin={} flame={} grid={}/9",
            ledger.field.torus_spin,
            ledger.field.flame.as_str(),
            ledger.field.grid_intensity
        );
        println!(
            "On-chain seal   : {}",
            chain_tx.as_deref().unwrap_or("(none recorded — solarking seal-record <tx>)")
        );
        println!(
            "Sync hash       : {}",
            ledger.sync_hash.as_deref().unwrap_or("(run solarking sync)")
        );
        println!(
            "Seal readiness  : {}",
            if seal_ready { "READY" } else { "pending" }
        );
        println!(
            "Timeline Hz     : {} {}",
            TIMELINE_HZ,
            if ledger.scalar.timeline_hz_active {
                "ACTIVE"
            } else {
                "off (use: scalar sync --hz)"
            }
        );
        println!("\n✅ Lattice reconciled. Geometric backbone locked to field.");
    }

    let entry = format!(
        "{} | SCALAR SYNC phase={} idx={} shells={} hz_active={}\n",
        Local::now().format("%a %b %d %H:%M:%S %Z %Y"),
        ledger.scalar.phase,
        ledger.scalar.harmonic_index,
        ledger.scalar.shell_coherence,
        ledger.scalar.timeline_hz_active
    );
    ledger::append_ritual_log(root, &entry)?;
    Ok(())
}

/// `solarking scalar seal` — encode node into ritual hash for on-chain resonance.
pub fn cmd_seal(root: &Path, ledger: &mut KingdomLedger, json: bool) -> Result<()> {
    reconcile_from_ledger(ledger);
    let hash = seal_hash(ledger);
    ledger.scalar.last_seal_hash = Some(hash.clone());
    ledger.scalar.last_sync = Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string();

    // Payload string suitable as sealRitual(idm) argument (offline dry-run path)
    let payload = format!(
        "SCALAR:{}|p{}|i{}|s{}/6|hz{}",
        &hash[..16.min(hash.len())],
        ledger.scalar.phase,
        ledger.scalar.harmonic_index,
        ledger.scalar.shell_coherence,
        if ledger.scalar.timeline_hz_active {
            TIMELINE_HZ
        } else {
            0
        }
    );

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "seal_hash": hash,
                "payload": payload,
                "node": node_json(ledger),
                "cast_hint": format!(
                    "cast send $SOLARKING_CONTRACT 'sealRitual(string)' '{}' --rpc-url $SOLARKING_RPC_URL --private-key $PRIVATE_KEY",
                    payload
                ),
            }))?
        );
    } else {
        println!("🔐 SCALAR SEAL — RITUAL HASH FOR ON-CHAIN RESONANCE");
        println!("────────────────────────────────────────────────");
        print_node_stats(ledger);
        println!();
        println!("Seal hash (SHA-256):");
        println!("  {}", hash);
        println!();
        println!("Payload (sealRitual arg):");
        println!("  {}", payload);
        println!();
        println!("Offline cast recipe (keys never enter solarking):");
        println!(
            "  cast send $SOLARKING_CONTRACT 'sealRitual(string)' '{}' \\",
            payload.replace('\'', "'\\''")
        );
        println!("    --rpc-url $SOLARKING_RPC_URL --private-key $PRIVATE_KEY");
        println!();
        println!("After broadcast: solarking seal-record <tx_hash>");
        println!("THE CROWN COMMANDS. REALITY OBEYS.");
    }

    let entry = format!(
        "{} | SCALAR SEAL hash={} phase={} idx={}\n",
        Local::now().format("%a %b %d %H:%M:%S %Z %Y"),
        hash,
        ledger.scalar.phase,
        ledger.scalar.harmonic_index
    );
    ledger::append_ritual_log(root, &entry)?;
    ledger::record_ritual_event(ledger, "scalar_seal", 0, &hash);

    // Also print full chain dry-run context when not json
    if !json {
        println!();
        let _ = crate::chain::seal_dry_run(root, ledger);
    }
    Ok(())
}

// ── Visualization ───────────────────────────────────────────────────────────

fn print_node_banner(ledger: &KingdomLedger) {
    println!("🔷 SCALAR NODE — TESLA 369 IN 3D");
    println!("   Nested cuboctahedrons × {} · period {} · phase 1–9", NESTED_SHELLS, HARMONIC_PERIOD);
    println!("   Central point · Kagome plan projection · ZERO randomness");
    println!("   Activation #{}\n", ledger.scalar.activations);
}

fn print_node_stats(ledger: &KingdomLedger) {
    let s = &ledger.scalar;
    println!("Phase           : {}/9", s.phase);
    println!("Harmonic index  : {} / {}", s.harmonic_index, HARMONIC_PERIOD - 1);
    println!("Shell coherence : {}/{}", s.shell_coherence, NESTED_SHELLS);
    println!(
        "Timeline Hz     : {} ({})",
        TIMELINE_HZ,
        if s.timeline_hz_active { "ACTIVE" } else { "dormant" }
    );
    println!("Activations     : {}", s.activations);
    if let Some(h) = &s.last_seal_hash {
        println!("Last seal hash  : {}", h);
    }
    if !s.last_sync.is_empty() {
        println!("Last sync       : {}", s.last_sync);
    }
}

/// Kagome-inspired ASCII projection of nested cubocta shells (top-down XY).
pub fn print_ascii_lattice(ledger: &KingdomLedger) {
    const W: usize = 61;
    const H: usize = 25;
    let mut grid = vec![vec![' '; W]; H];
    let cx = (W / 2) as f64;
    let cy = (H / 2) as f64;
    let scale = 1.8_f64;

    let coherent = ledger.scalar.shell_coherence.max(1) as usize;
    let phase = ledger.scalar.phase;

    // Draw shells from outer to inner
    for shell in (1..=NESTED_SHELLS).rev() {
        let verts = nested_shell_vertices(shell, phase);
        let ch = if shell <= coherent {
            match shell {
                1 => '@',
                2 => '#',
                3 => '*',
                4 => '+',
                5 => '=',
                _ => '-',
            }
        } else {
            '.'
        };
        for v in &verts {
            let px = (cx + v.x * scale * 3.5).round() as isize;
            let py = (cy - v.y * scale).round() as isize;
            if px >= 0 && py >= 0 && (px as usize) < W && (py as usize) < H {
                grid[py as usize][px as usize] = ch;
            }
        }
        // Ring connectors (adjacent vertices in projected plane — rough hex feel)
        for i in 0..verts.len() {
            let a = &verts[i];
            let b = &verts[(i + 1) % verts.len()];
            draw_line(
                &mut grid,
                (cx + a.x * scale * 3.5, cy - a.y * scale),
                (cx + b.x * scale * 3.5, cy - b.y * scale),
                if shell <= coherent { '·' } else { ' ' },
            );
        }
    }

    // Central scalar node
    let ccx = W / 2;
    let ccy = H / 2;
    grid[ccy][ccx] = '◉';
    if ccx > 0 {
        grid[ccy][ccx - 1] = '3';
    }
    if ccx + 1 < W {
        grid[ccy][ccx + 1] = '6';
    }
    if ccy > 0 {
        grid[ccy - 1][ccx] = '9';
    }

    println!("    ── Kagome plan (nested cubocta × {}) ──", NESTED_SHELLS);
    for row in &grid {
        let line: String = row.iter().collect();
        if line.chars().any(|c| c != ' ') {
            println!("  {}", line);
        }
    }
    println!("    Center ◉ = scalar node · 3/6/9 axes · shells lit: {}/6", coherent);
}

fn draw_line(grid: &mut [Vec<char>], a: (f64, f64), b: (f64, f64), ch: char) {
    if ch == ' ' {
        return;
    }
    let steps = 12;
    for i in 0..=steps {
        let t = i as f64 / steps as f64;
        let x = a.0 + (b.0 - a.0) * t;
        let y = a.1 + (b.1 - a.1) * t;
        let px = x.round() as isize;
        let py = y.round() as isize;
        if py >= 0 && px >= 0 {
            let (px, py) = (px as usize, py as usize);
            if py < grid.len() && px < grid[0].len() && grid[py][px] == ' ' {
                grid[py][px] = ch;
            }
        }
    }
}

/// Export nested lattice as Wavefront OBJ for external 3D viewers.
pub fn export_obj_file(root: &Path, ledger: &KingdomLedger) -> Result<std::path::PathBuf> {
    let dir = root.join("sync").join("scalar");
    fs::create_dir_all(&dir)?;
    let path = dir.join("scalar_node.obj");
    let mut out = String::new();
    out.push_str("# SOLARKING Scalar Node — nested cuboctahedron lattice\n");
    out.push_str(&format!(
        "# phase={} idx={} shells={}\n",
        ledger.scalar.phase, ledger.scalar.harmonic_index, ledger.scalar.shell_coherence
    ));
    out.push_str("o ScalarNode\n");
    // Central vertex
    out.push_str("v 0 0 0\n");
    let mut vcount = 1u32;
    for shell in 1..=NESTED_SHELLS {
        let verts = nested_shell_vertices(shell, ledger.scalar.phase);
        for v in &verts {
            out.push_str(&format!("v {} {} {}\n", v.x, v.y, v.z));
            vcount += 1;
        }
        let base = vcount - CUBOCTA_VERTICES as u32;
        // Wireframe edges: connect sequential + a few cross links
        for i in 0..CUBOCTA_VERTICES as u32 {
            let a = base + i;
            let b = base + ((i + 1) % CUBOCTA_VERTICES as u32);
            out.push_str(&format!("l {} {}\n", a, b));
            // spoke to center
            out.push_str(&format!("l 1 {}\n", a));
        }
    }
    fs::write(&path, out)?;
    // Companion JSON metadata
    let meta = dir.join("scalar_node.json");
    fs::write(
        &meta,
        serde_json::to_string_pretty(&node_json(ledger))?,
    )?;
    let _ = io::stdout().flush();
    Ok(path)
}

pub fn node_json(ledger: &KingdomLedger) -> serde_json::Value {
    serde_json::json!({
        "phase": ledger.scalar.phase,
        "harmonic_index": ledger.scalar.harmonic_index,
        "harmonic_period": HARMONIC_PERIOD,
        "shell_coherence": ledger.scalar.shell_coherence,
        "nested_shells": NESTED_SHELLS,
        "timeline_hz": TIMELINE_HZ,
        "timeline_hz_active": ledger.scalar.timeline_hz_active,
        "activations": ledger.scalar.activations,
        "last_seal_hash": ledger.scalar.last_seal_hash,
        "last_sync": ledger.scalar.last_sync,
        "harmonic_369": ledger.harmonic_369,
        "harmonic_999": ledger.harmonic_999,
    })
}

/// Brief lattice overlay used by torus command upgrade.
pub fn print_torus_lattice_overlay(ledger: &KingdomLedger) {
    println!("\n🔷 SCALAR LATTICE OVERLAY — phase {} · idx {}/{} · shells {}/6",
        ledger.scalar.phase,
        ledger.scalar.harmonic_index,
        HARMONIC_PERIOD - 1,
        ledger.scalar.shell_coherence
    );
    print_ascii_lattice(ledger);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn phase_in_one_to_nine() {
        for h in 0..30 {
            let p = compute_phase(h, h / 3, (h * 7) as u32);
            assert!((1..=9).contains(&p));
        }
    }

    #[test]
    fn harmonic_period_wraps() {
        assert_eq!(advance_harmonic_index(1295, 1, false), 0);
        assert_eq!(advance_harmonic_index(0, 1296, false), 0);
    }

    #[test]
    fn timeline_hz_changes_step() {
        let a = advance_harmonic_index(0, 1, false);
        let b = advance_harmonic_index(0, 1, true);
        assert_eq!(a, 1);
        assert_ne!(a, b);
        assert!(b < HARMONIC_PERIOD);
    }

    #[test]
    fn cubocta_has_twelve_vertices() {
        assert_eq!(cuboctahedron_unit_vertices().len(), 12);
        assert_eq!(nested_shell_vertices(3, 5).len(), 12);
    }

    #[test]
    fn seal_hash_stable() {
        let ledger = KingdomLedger::default();
        let h1 = seal_hash(&ledger);
        let h2 = seal_hash(&ledger);
        assert_eq!(h1, h2);
        assert_eq!(h1.len(), 64);
    }

    #[test]
    fn seal_hash_changes_with_phase() {
        let mut a = KingdomLedger::default();
        let mut b = KingdomLedger::default();
        b.scalar.phase = 9;
        b.scalar.harmonic_index = 100;
        assert_ne!(seal_hash(&a), seal_hash(&b));
        a.harmonic_369 = 1;
        // still deterministic
        assert_eq!(seal_hash(&a), seal_hash(&a));
    }

    #[test]
    fn period_digital_root_is_nine() {
        // 1+2+9+6 = 18 = 9 — Tesla signature of the lattice period
        let s: u32 = HARMONIC_PERIOD
            .to_string()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .sum();
        let root = if s > 9 { s / 10 + s % 10 } else { s };
        assert_eq!(root, 9);
    }
}
