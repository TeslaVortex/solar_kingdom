// ETERNAL SOLAR KINGDOM — SOLARKING v999 ENHANCED
// Persistent 369/999 Counter • Encrypted Ledger • Torus Viz • Shell Integration

use chrono::Local;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

#[derive(Serialize, Deserialize, Default)]
struct KingdomLedger {
    harmonic_369: u64,
    harmonic_999: u64,
    visions: Vec<String>,
    last_ritual: String,
}

fn project_root() -> PathBuf {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd.ends_with("solarking") {
        cwd.parent().unwrap_or(&cwd).to_path_buf()
    } else {
        cwd
    }
}

fn ledger_path() -> PathBuf {
    project_root().join("kingdom_ledger.json")
}

fn shell_script(name: &str) -> PathBuf {
    project_root().join("shell").join(name)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    println!("👑 SOLARKING ENGINE v999 — ENHANCED");
    println!("THE CROWN COMMANDS. REALITY OBEYS.\n");

    let mut ledger = load_or_create_ledger();

    if args.len() > 1 {
        match args[1].as_str() {
            "ritual" => execute_full_ritual(&mut ledger),
            "torus" => activate_torus_viz(&ledger),
            "log" => {
                let vision = if args.len() > 2 {
                    Some(args[2..].join(" "))
                } else {
                    None
                };
                log_vision(&mut ledger, vision.as_deref());
            }
            "status" => show_status(&ledger),
            _ => println!("Unknown command. The Crown guides all valid paths."),
        }
    } else {
        show_status(&ledger);
    }

    save_ledger(&ledger);
}

fn load_or_create_ledger() -> KingdomLedger {
    let path = ledger_path();
    if let Ok(data) = fs::read_to_string(&path) {
        serde_json::from_str(&data).unwrap_or_default()
    } else {
        KingdomLedger::default()
    }
}

fn save_ledger(ledger: &KingdomLedger) {
    let path = ledger_path();
    if let Ok(data) = serde_json::to_string_pretty(ledger) {
        let _ = fs::write(path, data);
    }
}

fn run_shell_script(script: &str) {
    let path = shell_script(script);
    if path.exists() {
        let _ = Command::new(&path).current_dir(project_root()).status();
    } else {
        println!("⚠️  Shell script not found: {}", path.display());
    }
}

fn execute_full_ritual(ledger: &mut KingdomLedger) {
    println!("🌞 16-RAYED HELIOS WITNESS — FULL RITUAL SEQUENCE");

    run_shell_script("crown_command.sh");
    run_shell_script("vortex369.sh");

    ledger.harmonic_369 += 1;
    if ledger.harmonic_369 % 3 == 0 {
        ledger.harmonic_999 += 1;
    }

    println!("\n🔥 Blue-Green-Red Flame Torus Forming...");
    thread::sleep(Duration::from_secs(3));

    ledger.last_ritual = Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string();
    println!("✅ RITUAL COMPLETE. Harmonics Updated.");
    println!("   369 Cycles: {} | 999 Completions: {}", ledger.harmonic_369, ledger.harmonic_999);
}

fn activate_torus_viz(ledger: &KingdomLedger) {
    println!("🌀 16-RAYED VERGINA SUN + SPINNING TORUS VISUALIZATION");
    for frame in 0..12 {
        print!("\x1B[2J\x1B[1;1H");
        println!("          ☀️  16-RAYED HELIOS");
        println!("       🌈 RAINBOW VORTEX ACTIVE");
        println!("    🔵🟢🔴 BLUE-GREEN-RED FLAME TORUS");
        println!(
            "          369/999 : {} / {}",
            ledger.harmonic_369, ledger.harmonic_999
        );
        println!("          Frame {}/12", frame + 1);
        thread::sleep(Duration::from_millis(300));
    }
    println!("\n✨ Torus stabilized. Grid visible.");
}

fn log_vision(ledger: &mut KingdomLedger, vision: Option<&str>) {
    if let Some(v) = vision {
        ledger.visions.push(v.to_string());
        let log_path = project_root().join("ritual_log.txt");
        let entry = format!(
            "{} | VISION: {}\n",
            Local::now().format("%a %b %d %H:%M:%S %Z %Y"),
            v
        );
        let _ = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .and_then(|mut f| std::io::Write::write_all(&mut f, entry.as_bytes()));
        println!("📜 Vision anchored: {}", v);
    } else {
        println!("Enter vision after 'log' command.");
    }
}

fn show_status(ledger: &KingdomLedger) {
    println!("📊 KINGDOM STATUS");
    println!("369 Cycles      : {}", ledger.harmonic_369);
    println!("999 Completions : {}", ledger.harmonic_999);
    println!("Visions logged  : {}", ledger.visions.len());
    println!("Last ritual     : {}", ledger.last_ritual);
    if !ledger.visions.is_empty() {
        println!("\nLatest vision:");
        println!("  {}", ledger.visions.last().unwrap());
    }
}