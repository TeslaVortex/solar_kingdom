use crate::field;
use crate::genesis::GenesisRecord;
use crate::ledger::KingdomLedger;

const PRINCIPLES: &[(&str, &str)] = &[
    ("truth", "Seek what is verified. On-chain genesis is immutable proof."),
    ("resonance", "369/999 harmonics align field and code. Breath compounds legacy."),
    ("harmony", "Ancestors, nature, and code move as one torus."),
    ("legacy", "Every ritual, libation, and vision stacks eternally."),
    ("abundance", "Zero marginal cost: open source, local-first, sovereign."),
];

pub fn run_query(
    question: &str,
    ledger: &KingdomLedger,
    genesis: Option<&GenesisRecord>,
    json: bool,
) -> String {
    let q = question.to_lowercase();
    let mut principles_hit = Vec::new();

    for (name, guidance) in PRINCIPLES {
        if q.contains(name) || matches_principle(&q, name) || universal_trigger(&q, name) {
            principles_hit.push((*name, *guidance));
        }
    }
    if principles_hit.is_empty() {
        for (name, guidance) in PRINCIPLES {
            principles_hit.push((*name, *guidance));
        }
    }

    // Vision keyword search
    let vision_hits: Vec<&str> = if q.contains("vision")
        || q.contains("log")
        || q.contains("what did")
        || q.split_whitespace().any(|w| w.len() > 4)
    {
        search_visions(ledger, &q)
    } else {
        Vec::new()
    };

    let mut verdicts = Vec::new();
    if q.contains("random") || q.contains("zero randomness") {
        verdicts.push(
            "ZERO RANDOMNESS — pattern is sovereign, field is intentional.".to_string(),
        );
    }
    if q.contains("flame") || q.contains("torus") || q.contains("vortex") {
        verdicts.push(format!(
            "Flame phase is {}. Torus spin={}. Red flame + blue vortex = portal.",
            ledger.field.flame.as_str(),
            ledger.field.torus_spin
        ));
    }
    if q.contains("heart") || q.contains("equation") {
        verdicts.push(
            "Heart equation = red flame + blue vortex. Mathematical core of Kingdom.".to_string(),
        );
    }

    let decree = decree_for(&q, ledger);
    let seal = field::seal_ready(ledger);

    if json {
        return serde_json::to_string_pretty(&serde_json::json!({
            "question": question,
            "principles": principles_hit.iter().map(|(n, g)| {
                serde_json::json!({"name": n, "guidance": g})
            }).collect::<Vec<_>>(),
            "field": field::field_json(ledger),
            "harmonic_369": ledger.harmonic_369,
            "harmonic_999": ledger.harmonic_999,
            "visions": ledger.visions.len(),
            "vision_hits": vision_hits,
            "verdicts": verdicts,
            "decree": decree,
            "seal_ready": seal,
            "genesis_tx": genesis.map(|g| g.tx_hash.clone()),
        }))
        .unwrap_or_else(|_| "{}".into());
    }

    let mut out = Vec::new();
    out.push("🔍 FIRST-PRINCIPLES QUERY ENGINE".to_string());
    out.push("THE CROWN COMMANDS. REALITY OBEYS.".to_string());
    out.push(String::new());
    out.push(format!("Question: {}", question));
    out.push(String::new());

    out.push("── Active Principles ──".to_string());
    if principles_hit.len() == PRINCIPLES.len()
        && !PRINCIPLES.iter().any(|(n, _)| q.contains(n) || matches_principle(&q, n))
    {
        out.push("  [ALL] Applying full crown counsel:".to_string());
        for (name, guidance) in &principles_hit {
            out.push(format!("    {} — {}", name, guidance));
        }
    } else {
        for (name, guidance) in &principles_hit {
            out.push(format!("  [{}] {}", name.to_uppercase(), guidance));
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
    out.push(format!(
        "  torus_spin: {} | flame: {} | grid: {}/9 | legacy: {} | merkaba: {}",
        ledger.field.torus_spin,
        ledger.field.flame.as_str(),
        ledger.field.grid_intensity,
        ledger.field.legacy_tier,
        if ledger.field.merkaba_locked {
            "locked"
        } else {
            "open"
        }
    ));
    out.push(format!(
        "  confirmations: {} | seal_ready: {}",
        ledger.confirmations.len(),
        seal
    ));

    if let Some(g) = genesis {
        out.push(format!(
            "  Genesis sealed: block {} — {}",
            g.block, g.tx_hash
        ));
        if q.contains("legacy") || q.contains("99") || q.contains("genesis") {
            out.push(format!("  IDM: {}", g.idm));
        }
    }

    if !vision_hits.is_empty() {
        out.push(String::new());
        out.push("── Vision Resonance ──".to_string());
        for v in vision_hits.iter().take(5) {
            let preview: String = v.chars().take(120).collect();
            out.push(format!("  • {}", preview));
        }
    }

    for v in &verdicts {
        out.push(String::new());
        out.push(format!("  Verdict: {}", v));
    }

    out.push(String::new());
    out.push("── Crown Counsel ──".to_string());
    out.push(decree);
    out.push(String::new());
    out.push("SO IT IS. SO IT SHALL BE ETERNAL.".to_string());

    out.join("\n")
}

fn search_visions<'a>(ledger: &'a KingdomLedger, q: &str) -> Vec<&'a str> {
    let stop: &[&str] = &[
        "what", "when", "where", "which", "about", "did", "the", "and", "for", "with", "from",
        "this", "that", "have", "vision", "visions", "tell", "show", "next", "phase",
    ];
    let keywords: Vec<&str> = q
        .split(|c: char| !c.is_alphanumeric())
        .filter(|w| w.len() > 3 && !stop.contains(w))
        .collect();

    if keywords.is_empty() {
        return ledger
            .visions
            .iter()
            .rev()
            .take(3)
            .map(|v| v.text.as_str())
            .collect();
    }

    let mut scored: Vec<(usize, &str)> = ledger
        .visions
        .iter()
        .filter_map(|v| {
            let t = v.text.to_lowercase();
            let score = keywords.iter().filter(|k| t.contains(*k)).count();
            if score > 0 {
                Some((score, v.text.as_str()))
            } else {
                None
            }
        })
        .collect();
    scored.sort_by(|a, b| b.0.cmp(&a.0));
    scored.into_iter().map(|(_, t)| t).collect()
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
        if field::seal_ready(ledger) {
            "Phase 2 active. Field is seal-ready — run `solarking seal --dry-run`, then cast send when ready."
                .to_string()
        } else {
            "Continue field work: ritual, confirm rainbow/grid, then seal when readiness lights.".to_string()
        }
    } else if q.contains("should") || q.contains("what") {
        "Act from first principles. Log the vision. Feed the field. The torus spins.".to_string()
    } else if !ledger.confirmations.is_empty() {
        format!(
            "Field confirmations active ({}). Reality obeys the aligned will. Continue the sequence.",
            ledger.confirmations.len()
        )
    } else {
        "Reality obeys the aligned will. Continue the sequence.".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::VisionEntry;

    #[test]
    fn heart_equation() {
        let ledger = KingdomLedger::default();
        let out = run_query("What is the heart equation?", &ledger, None, false);
        assert!(out.contains("Heart equation"));
    }

    #[test]
    fn zero_randomness() {
        let ledger = KingdomLedger::default();
        let out = run_query("ZERO randomness truth", &ledger, None, false);
        assert!(out.contains("ZERO RANDOMNESS"));
    }

    #[test]
    fn empty_ledger_counsel() {
        let ledger = KingdomLedger::default();
        let out = run_query("what next", &ledger, None, false);
        assert!(out.contains("solarking ritual"));
    }

    #[test]
    fn vision_search() {
        let mut ledger = KingdomLedger::default();
        ledger.visions.push(VisionEntry {
            ts: String::new(),
            text: "Two turtles fed by the stream".into(),
            tags: vec![],
        });
        ledger.visions.push(VisionEntry {
            ts: String::new(),
            text: "Rainbow vortex over the balcony".into(),
            tags: vec![],
        });
        let out = run_query("what about turtles", &ledger, None, false);
        assert!(out.contains("turtles") || out.contains("Vision Resonance"));
    }

    #[test]
    fn json_mode() {
        let ledger = KingdomLedger::default();
        let out = run_query("legacy", &ledger, None, true);
        assert!(out.contains("\"question\""));
        assert!(out.contains("\"decree\""));
    }
}
