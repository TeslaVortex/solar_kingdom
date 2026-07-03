use crate::genesis::GenesisRecord;
use crate::ledger::KingdomLedger;

const PRINCIPLES: &[(&str, &str)] = &[
    ("truth", "Seek what is verified. On-chain genesis is immutable proof."),
    ("resonance", "369/999 harmonics align field and code. Breath compounds legacy."),
    ("harmony", "Ancestors, nature, and code move as one torus."),
    ("legacy", "Every ritual, libation, and vision stacks eternally."),
    ("abundance", "Zero marginal cost: open source, local-first, sovereign."),
];

pub fn run_query(question: &str, ledger: &KingdomLedger, genesis: Option<&GenesisRecord>) -> String {
    let q = question.to_lowercase();
    let mut out = Vec::new();

    out.push("🔍 FIRST-PRINCIPLES QUERY ENGINE".to_string());
    out.push("THE CROWN COMMANDS. REALITY OBEYS.".to_string());
    out.push(String::new());
    out.push(format!("Question: {}", question));
    out.push(String::new());

    out.push("── Active Principles ──".to_string());
    for (name, guidance) in PRINCIPLES {
        if q.contains(name)
            || matches_principle(&q, name)
            || universal_trigger(&q, name)
        {
            out.push(format!("  [{}] {}", name.to_uppercase(), guidance));
        }
    }

    if out.len() == 5 {
        out.push("  [ALL] Applying full crown counsel:".to_string());
        for (name, guidance) in PRINCIPLES {
            out.push(format!("    {} — {}", name, guidance));
        }
    }

    out.push(String::new());
    out.push("── Field State ──".to_string());
    out.push(format!(
        "  369 cycles: {} | 999 completions: {} | visions: {}",
        ledger.harmonic_369,
        ledger.harmonic_999,
        ledger.visions.len()
    ));

    if let Some(g) = genesis {
        out.push(format!("  Genesis sealed: block {} — {}", g.block, g.tx_hash));
        if q.contains("legacy") || q.contains("99") || q.contains("genesis") {
            out.push(format!("  IDM: {}", g.idm));
        }
    }

    if q.contains("random") || q.contains("zero randomness") {
        out.push(String::new());
        out.push("  Verdict: ZERO RANDOMNESS — pattern is sovereign, field is intentional.".to_string());
    }
    if q.contains("flame") || q.contains("torus") || q.contains("vortex") {
        out.push("  Verdict: Red flame dominant. Blue/light vortex = portal forming.".to_string());
    }
    if q.contains("heart") || q.contains("equation") {
        out.push("  Verdict: Heart equation = red flame + blue vortex. Mathematical core of Kingdom.".to_string());
    }

    out.push(String::new());
    out.push("── Crown Counsel ──".to_string());
    out.push(decree_for(&q, ledger));
    out.push(String::new());
    out.push("SO IT IS. SO IT SHALL BE ETERNAL.".to_string());

    out.join("\n")
}

fn matches_principle(q: &str, principle: &str) -> bool {
    match principle {
        "truth" => q.contains("law") || q.contains("expose") || q.contains("verify"),
        "resonance" => q.contains("369") || q.contains("999") || q.contains("hz"),
        "harmony" => q.contains("ancestor") || q.contains("turtle") || q.contains("cat"),
        "legacy" => q.contains("crown") || q.contains("kingdom") || q.contains("eternal"),
        "abundance" => q.contains("cost") || q.contains("free") || q.contains("open"),
        _ => false,
    }
}

fn universal_trigger(q: &str, principle: &str) -> bool {
    matches!(principle, "legacy" | "resonance") && (q.contains("ritual") || q.contains("next"))
}

fn decree_for(q: &str, ledger: &KingdomLedger) -> String {
    if ledger.harmonic_369 == 0 {
        "Proceed: run `solarking ritual` to anchor the first harmonic.".to_string()
    } else if q.contains("next") || q.contains("phase") {
        "Phase 1 complete. Phase 2 awaits: on-chain seal via Vortex369.sol.".to_string()
    } else if q.contains("should") || q.contains("what") {
        "Act from first principles. Log the vision. Feed the field. The torus spins.".to_string()
    } else {
        "Reality obeys the aligned will. Continue the sequence.".to_string()
    }
}