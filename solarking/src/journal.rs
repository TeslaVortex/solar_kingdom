//! Crown journal — read visions, ritual log, and transmission archives.

use std::fs;
use std::path::Path;

use crate::error::Result;
use crate::ledger::KingdomLedger;

/// List last N visions (newest last in print order: oldest of the window first).
pub fn list_visions(ledger: &KingdomLedger, last: usize, json: bool) {
    let n = last.max(1);
    let total = ledger.visions.len();
    let start = total.saturating_sub(n);

    if json {
        let items: Vec<serde_json::Value> = ledger.visions[start..]
            .iter()
            .enumerate()
            .map(|(i, v)| {
                let abs = start + i;
                let from_end = (total as i64) - (abs as i64);
                serde_json::json!({
                    "index": abs,
                    "from_end": from_end,
                    "ts": v.ts,
                    "tags": v.tags,
                    "bytes": v.text.len(),
                    "preview": first_line_preview(&v.text, 100),
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "total": total,
                "showing": items.len(),
                "visions": items,
            }))
            .unwrap_or_else(|_| "{}".into())
        );
        return;
    }

    println!("📜 CROWN JOURNAL — {} visions (showing last {})", total, n.min(total));
    println!("THE CROWN COMMANDS. REALITY OBEYS.\n");
    if total == 0 {
        println!("  (empty — solarking receive --paste to anchor the first transmission)");
        return;
    }
    for (i, v) in ledger.visions[start..].iter().enumerate() {
        let abs = start + i;
        let from_end = (total as i64) - (abs as i64);
        let tags = if v.tags.is_empty() {
            String::new()
        } else {
            format!(" [{}]", v.tags.join(", "))
        };
        let ts = if v.ts.is_empty() {
            "(legacy no-ts)"
        } else {
            v.ts.as_str()
        };
        println!(
            "  #{abs} (from_end={from_end}) {ts} · {}b{tags}",
            v.text.len()
        );
        println!("     {}", first_line_preview(&v.text, 100));
    }
    println!("\n  Full text: solarking journal show");
    println!("  Search:    solarking journal search <kw>");
}

/// Show full vision text. `from_end`: 1 = latest, 2 = second latest, …
pub fn show_vision(ledger: &KingdomLedger, from_end: i64, json: bool) {
    let Some((abs, v)) = resolve_from_end(ledger, from_end) else {
        println!("⚠️  No vision at from_end={from_end} (total={})", ledger.visions.len());
        return;
    };

    if json {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "index": abs,
                "from_end": from_end.max(1),
                "ts": v.ts,
                "tags": v.tags,
                "text": v.text,
            }))
            .unwrap_or_else(|_| "{}".into())
        );
        return;
    }

    println!("📜 TRANSMISSION #{abs} (from_end={})", from_end.max(1));
    if !v.ts.is_empty() {
        println!("   ts: {}", v.ts);
    }
    if !v.tags.is_empty() {
        println!("   tags: {}", v.tags.join(", "));
    }
    println!("   bytes: {}", v.text.len());
    println!("────────────────────────────────");
    println!("{}", v.text);
    println!("────────────────────────────────");
}

pub fn search_visions(ledger: &KingdomLedger, query: &str, json: bool) {
    let q = query.to_lowercase();
    let hits: Vec<(usize, &crate::ledger::VisionEntry)> = ledger
        .visions
        .iter()
        .enumerate()
        .filter(|(_, v)| {
            v.text.to_lowercase().contains(&q)
                || v.tags.iter().any(|t| t.to_lowercase().contains(&q))
        })
        .collect();

    if json {
        let items: Vec<serde_json::Value> = hits
            .iter()
            .map(|(i, v)| {
                serde_json::json!({
                    "index": i,
                    "ts": v.ts,
                    "tags": v.tags,
                    "preview": first_line_preview(&v.text, 120),
                })
            })
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "query": query,
                "hits": items.len(),
                "results": items,
            }))
            .unwrap_or_else(|_| "{}".into())
        );
        return;
    }

    println!("🔍 JOURNAL SEARCH — \"{}\" ({} hits)", query, hits.len());
    if hits.is_empty() {
        println!("  No matches.");
        return;
    }
    let total = ledger.visions.len();
    for (i, v) in hits {
        let from_end = total - i;
        println!(
            "  #{i} (from_end={from_end}) {} · {}b",
            if v.ts.is_empty() { "…" } else { &v.ts },
            v.text.len()
        );
        println!("     {}", first_line_preview(&v.text, 100));
    }
}

pub fn tail_ritual_log(root: &Path, tail: usize) -> Result<()> {
    let path = root.join("ritual_log.txt");
    if !path.exists() {
        println!("⚠️  No ritual_log.txt yet.");
        return Ok(());
    }
    let body = fs::read_to_string(&path)?;
    let lines: Vec<&str> = body.lines().collect();
    let n = tail.max(1);
    let start = lines.len().saturating_sub(n);
    println!(
        "📜 ritual_log.txt — last {} of {} lines",
        lines.len().saturating_sub(start),
        lines.len()
    );
    println!("   {}", path.display());
    println!("────────────────────────────────");
    for line in &lines[start..] {
        println!("{line}");
    }
    Ok(())
}

pub fn list_transmission_files(root: &Path, json: bool) -> Result<()> {
    let dir = root.join("sync").join("transmissions");
    if !dir.exists() {
        if json {
            println!(r#"{{"files":[]}}"#);
        } else {
            println!("📂 No transmissions yet (sync/transmissions/).");
            println!("   Receive with: solarking receive --paste");
        }
        return Ok(());
    }
    let mut files: Vec<_> = fs::read_dir(&dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| {
            p.extension()
                .map(|e| e == "txt")
                .unwrap_or(false)
        })
        .collect();
    files.sort();

    if json {
        let names: Vec<String> = files
            .iter()
            .map(|p| p.display().to_string())
            .collect();
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({ "files": names }))
                .unwrap_or_else(|_| "{}".into())
        );
        return Ok(());
    }

    println!("📂 Transmission archives ({})", files.len());
    for p in files.iter().rev().take(20) {
        let meta = fs::metadata(p).ok();
        let size = meta.map(|m| m.len()).unwrap_or(0);
        println!("  {} ({}b)", p.display(), size);
    }
    if files.len() > 20 {
        println!("  … {} more", files.len() - 20);
    }
    Ok(())
}

pub fn open_transmission_file(root: &Path, from_end: i64) -> Result<()> {
    let dir = root.join("sync").join("transmissions");
    if !dir.exists() {
        println!("⚠️  No sync/transmissions/ yet.");
        return Ok(());
    }
    let mut files: Vec<_> = fs::read_dir(&dir)?
        .filter_map(|e| e.ok())
        .map(|e| e.path())
        .filter(|p| p.extension().map(|e| e == "txt").unwrap_or(false))
        .collect();
    files.sort();
    if files.is_empty() {
        println!("⚠️  No .txt archives.");
        return Ok(());
    }
    let idx_from_end = from_end.max(1) as usize;
    if idx_from_end > files.len() {
        println!(
            "⚠️  from_end={} out of range ({} files)",
            idx_from_end,
            files.len()
        );
        return Ok(());
    }
    let path = &files[files.len() - idx_from_end];
    println!("{}", path.display());
    let body = fs::read_to_string(path)?;
    println!("────────────────────────────────");
    println!("{body}");
    Ok(())
}

fn resolve_from_end(
    ledger: &KingdomLedger,
    from_end: i64,
) -> Option<(usize, &crate::ledger::VisionEntry)> {
    let total = ledger.visions.len();
    if total == 0 {
        return None;
    }
    let n = if from_end <= 0 { 1 } else { from_end as usize };
    if n > total {
        return None;
    }
    let abs = total - n;
    Some((abs, &ledger.visions[abs]))
}

fn first_line_preview(text: &str, max: usize) -> String {
    let line = text.lines().next().unwrap_or(text).trim();
    if line.chars().count() <= max {
        return line.to_string();
    }
    let truncated: String = line.chars().take(max.saturating_sub(1)).collect();
    format!("{truncated}…")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ledger::VisionEntry;

    #[test]
    fn resolve_latest() {
        let mut ledger = KingdomLedger::default();
        ledger.visions.push(VisionEntry {
            ts: "a".into(),
            text: "first".into(),
            tags: vec![],
        });
        ledger.visions.push(VisionEntry {
            ts: "b".into(),
            text: "second".into(),
            tags: vec![],
        });
        let (i, v) = resolve_from_end(&ledger, 1).unwrap();
        assert_eq!(i, 1);
        assert_eq!(v.text, "second");
        let (i, v) = resolve_from_end(&ledger, 2).unwrap();
        assert_eq!(i, 0);
        assert_eq!(v.text, "first");
    }
}
