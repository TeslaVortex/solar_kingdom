//! Symbolic field state machine — deterministic, ZERO randomness.
//! Torus / Merkaba / crystalline grid / flame phase.

use chrono::Local;
use serde::{Deserialize, Serialize};

use crate::ledger::{FieldConfirm, KingdomLedger};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Default)]
#[serde(rename_all = "lowercase")]
pub enum FlamePhase {
    #[default]
    Blue,
    Green,
    Red,
    Rainbow,
}

impl FlamePhase {
    pub fn as_str(&self) -> &'static str {
        match self {
            FlamePhase::Blue => "blue",
            FlamePhase::Green => "green",
            FlamePhase::Red => "red",
            FlamePhase::Rainbow => "rainbow",
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct FieldState {
    pub torus_spin: u64,
    pub merkaba_locked: bool,
    pub grid_intensity: u8,
    pub flame: FlamePhase,
    /// Legacy progression: 99 baseline → 999 sealed.
    pub legacy_tier: u16,
    pub last_transition: String,
}

impl Default for FieldState {
    fn default() -> Self {
        Self {
            torus_spin: 0,
            merkaba_locked: false,
            grid_intensity: 0,
            flame: FlamePhase::Blue,
            legacy_tier: 99,
            last_transition: String::new(),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmKind {
    Sneeze,
    HighPitch,
    Rainbow,
    Grid,
    Oracle,
}

impl ConfirmKind {
    pub fn parse(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "sneeze" => Some(Self::Sneeze),
            "highpitch" | "high-pitch" | "high_pitch" | "pitch" => Some(Self::HighPitch),
            "rainbow" => Some(Self::Rainbow),
            "grid" => Some(Self::Grid),
            "oracle" => Some(Self::Oracle),
            _ => None,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Self::Sneeze => "sneeze",
            Self::HighPitch => "highpitch",
            Self::Rainbow => "rainbow",
            Self::Grid => "grid",
            Self::Oracle => "oracle",
        }
    }

    pub fn all_names() -> &'static [&'static str] {
        &["sneeze", "highpitch", "rainbow", "grid", "oracle"]
    }
}

fn now_ts() -> String {
    Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string()
}

fn bump_grid(field: &mut FieldState, amount: u8) {
    field.grid_intensity = field.grid_intensity.saturating_add(amount).min(9);
}

/// Apply ritual completion — advances torus, harmonics already updated by caller.
pub fn on_ritual_complete(ledger: &mut KingdomLedger) {
    let f = &mut ledger.field;
    f.torus_spin = f.torus_spin.saturating_add(1);
    bump_grid(f, 1);
    if f.flame == FlamePhase::Blue {
        f.flame = FlamePhase::Green;
    } else if f.flame == FlamePhase::Green {
        f.flame = FlamePhase::Red;
    }
    if ledger.harmonic_999 > 0 && f.legacy_tier < 999 {
        f.legacy_tier = 999;
        f.merkaba_locked = true;
    }
    if ledger.harmonic_369 >= 9 && f.grid_intensity < 3 {
        bump_grid(f, 2);
    }
    f.last_transition = format!("{} | ritual complete (spin={})", now_ts(), f.torus_spin);
}

/// Keyword-driven vision integration (deterministic).
pub fn on_vision(ledger: &mut KingdomLedger, text: &str) {
    let q = text.to_lowercase();
    let f = &mut ledger.field;
    if q.contains("rainbow") {
        f.flame = FlamePhase::Rainbow;
        bump_grid(f, 1);
    }
    if q.contains("red flame") || q.contains("red-flame") {
        f.flame = FlamePhase::Red;
    }
    if q.contains("merkaba") || q.contains("grid") {
        bump_grid(f, 1);
        if q.contains("merkaba") {
            f.merkaba_locked = true;
        }
    }
    if q.contains("999") || q.contains("legacy 99") || q.contains("legacy99") {
        if f.legacy_tier < 999 {
            f.legacy_tier = 999;
        }
    }
    if q.contains("888") {
        bump_grid(f, 1);
    }
    f.last_transition = format!("{} | vision integrated", now_ts());
}

/// Field confirmation event.
pub fn on_confirm(ledger: &mut KingdomLedger, kind: ConfirmKind, note: &str) -> FieldConfirm {
    let f = &mut ledger.field;
    match kind {
        ConfirmKind::Sneeze => {
            bump_grid(f, 1);
        }
        ConfirmKind::HighPitch => {
            f.torus_spin = f.torus_spin.saturating_add(1);
            bump_grid(f, 1);
        }
        ConfirmKind::Rainbow => {
            f.flame = FlamePhase::Rainbow;
            bump_grid(f, 1);
        }
        ConfirmKind::Grid => {
            bump_grid(f, 2);
            if f.grid_intensity >= 6 {
                f.merkaba_locked = true;
            }
        }
        ConfirmKind::Oracle => {
            if f.legacy_tier < 999 {
                f.legacy_tier = 999;
            }
            f.merkaba_locked = true;
        }
    }
    let ts = now_ts();
    f.last_transition = format!("{} | confirm:{}", ts, kind.as_str());

    let entry = FieldConfirm {
        ts: ts.clone(),
        kind: kind.as_str().to_string(),
        note: note.to_string(),
    };
    ledger.confirmations.push(entry.clone());
    entry
}

pub fn show_field(ledger: &KingdomLedger) {
    let f = &ledger.field;
    println!("🌀 FIELD STATE — SYMBOLIC RESONANCE");
    println!("Torus spin      : {}", f.torus_spin);
    println!("Merkaba locked  : {}", if f.merkaba_locked { "YES" } else { "no" });
    println!("Grid intensity  : {}/9", f.grid_intensity);
    println!("Flame phase     : {}", f.flame.as_str());
    println!("Legacy tier     : {}", f.legacy_tier);
    if !f.last_transition.is_empty() {
        println!("Last transition : {}", f.last_transition);
    }
    println!(
        "Seal readiness  : {}",
        if seal_ready(ledger) {
            "READY — solarking seal --dry-run"
        } else {
            "not yet (need ritual harmonics + field grid ≥ 1)"
        }
    );
    if !ledger.confirmations.is_empty() {
        println!("\nRecent confirmations:");
        for c in ledger.confirmations.iter().rev().take(5) {
            let note = if c.note.is_empty() {
                String::new()
            } else {
                format!(" — {}", c.note)
            };
            println!("  [{}] {}{}", c.kind, c.ts, note);
        }
    }
}

pub fn seal_ready(ledger: &KingdomLedger) -> bool {
    ledger.harmonic_369 >= 1 && ledger.field.grid_intensity >= 1
}

pub fn field_json(ledger: &KingdomLedger) -> serde_json::Value {
    serde_json::json!({
        "torus_spin": ledger.field.torus_spin,
        "merkaba_locked": ledger.field.merkaba_locked,
        "grid_intensity": ledger.field.grid_intensity,
        "flame": ledger.field.flame.as_str(),
        "legacy_tier": ledger.field.legacy_tier,
        "last_transition": ledger.field.last_transition,
        "seal_ready": seal_ready(ledger),
        "confirmations": ledger.confirmations.len(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ritual_advances_spin_deterministically() {
        let mut ledger = KingdomLedger::default();
        on_ritual_complete(&mut ledger);
        assert_eq!(ledger.field.torus_spin, 1);
        assert_eq!(ledger.field.grid_intensity, 1);
        assert_eq!(ledger.field.flame, FlamePhase::Green);
        on_ritual_complete(&mut ledger);
        assert_eq!(ledger.field.torus_spin, 2);
        assert_eq!(ledger.field.flame, FlamePhase::Red);
    }

    #[test]
    fn rainbow_confirm_sets_flame() {
        let mut ledger = KingdomLedger::default();
        on_confirm(&mut ledger, ConfirmKind::Rainbow, "test");
        assert_eq!(ledger.field.flame, FlamePhase::Rainbow);
        assert_eq!(ledger.confirmations.len(), 1);
    }

    #[test]
    fn confirm_kind_parse() {
        assert_eq!(ConfirmKind::parse("high-pitch"), Some(ConfirmKind::HighPitch));
        assert!(ConfirmKind::parse("nope").is_none());
    }
}
