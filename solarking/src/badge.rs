//! Soul-bound badge eligibility dry-run (Phase 2C).
//! Does not mint on-chain — prints readiness vs SolarKingdom BadgeKind enum.

use crate::chain::load_chain_config;
use crate::field::{self, FlamePhase};
use crate::ledger::KingdomLedger;
use std::path::Path;

#[derive(Clone, Copy)]
pub struct BadgeEligibility {
    pub kind: &'static str,
    pub ready: bool,
    pub reason: &'static str,
}

pub fn evaluate(ledger: &KingdomLedger) -> Vec<BadgeEligibility> {
    let has_node = ledger.scalar.onchain_node_id.is_some() || ledger.scalar.activations > 0;
    let rainbow = ledger.field.flame == FlamePhase::Rainbow
        || ledger
            .confirmations
            .iter()
            .any(|c| c.kind == "rainbow");
    let legacy999 = ledger.field.legacy_tier >= 999 || ledger.harmonic_999 > 0;
    let phase_complete = ledger.scalar.phase == 1 && ledger.scalar.activations >= 9
        || ledger.scalar.harmonic_index > 0 && ledger.scalar.shell_coherence >= 6;

    vec![
        BadgeEligibility {
            kind: "NodeGuardian (369 Node Guardian)",
            ready: has_node,
            reason: if has_node {
                "scalar node activated / on-chain nodeId present"
            } else {
                "run scalar node + activateScalarNode"
            },
        },
        BadgeEligibility {
            kind: "LatticePhase (Lattice Phase Complete)",
            ready: phase_complete || ledger.scalar.shell_coherence >= 6,
            reason: if phase_complete || ledger.scalar.shell_coherence >= 6 {
                "shell coherence / lattice milestone met"
            } else {
                "raise shell coherence via rituals"
            },
        },
        BadgeEligibility {
            kind: "RainbowLattice (Double-Edged Rainbow)",
            ready: rainbow && has_node,
            reason: if rainbow && has_node {
                "rainbow flame/confirm + node"
            } else {
                "confirm rainbow + scalar node"
            },
        },
        BadgeEligibility {
            kind: "Legacy999",
            ready: legacy999,
            reason: if legacy999 {
                "legacy tier 999 or harmonic_999 > 0"
            } else {
                "complete 999 harmonics / oracle legacy"
            },
        },
    ]
}

pub fn show_badge_status(root: &Path, ledger: &KingdomLedger, json: bool) {
    let cfg = load_chain_config(root);
    let badges = evaluate(ledger);
    if json {
        let rows: Vec<_> = badges
            .iter()
            .map(|b| {
                serde_json::json!({
                    "kind": b.kind,
                    "ready": b.ready,
                    "reason": b.reason,
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "sbt_contract": cfg.sbt,
                "onchain_node_id": ledger.scalar.onchain_node_id,
                "seal_ready": field::seal_ready(ledger),
                "badges": rows,
                "mint_note": "mint is minter-gated on SolarKingdom; cast only with deployer key offline",
            }))
            .unwrap_or_else(|_| "{}".into())
        );
        return;
    }

    println!("🏅 BADGE STATUS — SOUL-BOUND (dry-run)");
    println!("─────────────────────────────────────");
    if let Some(s) = &cfg.sbt {
        println!("SolarKingdom    : {}", s);
    } else {
        println!("SolarKingdom    : (set config/chain.json sbt)");
    }
    println!(
        "On-chain nodeId : {}",
        ledger
            .scalar
            .onchain_node_id
            .map(|n| n.to_string())
            .unwrap_or_else(|| "(none)".into())
    );
    println!();
    for b in &badges {
        let mark = if b.ready { "✅ READY" } else { "⏳ pending" };
        println!("{}  {}", mark, b.kind);
        println!("         {}", b.reason);
    }
    println!();
    println!("Mint (external — deployer minter only):");
    if let Some(s) = &cfg.sbt {
        println!(
            "  cast send {} 'mintBadge(address,uint8,uint256)' <soul> <kind 0-3> <nodeId> \\",
            s
        );
        println!("    --rpc-url $SOLARKING_RPC_URL --private-key $PRIVATE_KEY");
    } else {
        println!(
            "  cast send $SOLARKING_SBT 'mintBadge(address,uint8,uint256)' <soul> <kind> <nodeId> ..."
        );
    }
    println!("Kinds: 0=NodeGuardian 1=LatticePhase 2=RainbowLattice 3=Legacy999");
    println!("solarking never holds private keys.");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::KingdomLedger;

    #[test]
    fn empty_not_all_ready() {
        let ledger = KingdomLedger::default();
        let b = evaluate(&ledger);
        assert!(b.iter().any(|x| !x.ready));
    }

    #[test]
    fn node_guardian_when_activated() {
        let mut ledger = KingdomLedger::default();
        ledger.scalar.activations = 1;
        let b = evaluate(&ledger);
        assert!(b[0].ready);
    }
}
