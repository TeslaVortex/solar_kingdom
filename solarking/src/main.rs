// ETERNAL SOLAR KINGDOM — SOLARKING v999 ENHANCED
// Persistent 369/999 Counter • Ledger • Live Torus Viz • Shell Integration

use chrono::Local;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs::{self, OpenOptions};
use std::io::{self, Write};
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

#[derive(Serialize, Deserialize)]
struct GenesisRecord {
    idm: String,
    tx_hash: String,
    block: u64,
    from_ens: String,
    to_address: String,
    value_eth: String,
    legacy_99: bool,
    anchored: String,
}

const TORUS_WIDTH: usize = 64;
const TORUS_HEIGHT: usize = 24;
const LUMINANCE: &[u8] = b".,-~:;=!*#$@";

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

fn genesis_path() -> PathBuf {
    project_root().join("config").join("genesis.json")
}

fn load_genesis() -> Option<GenesisRecord> {
    let path = genesis_path();
    fs::read_to_string(&path)
        .ok()
        .and_then(|data| serde_json::from_str(&data).ok())
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
            "genesis" => show_genesis(),
            "libation" => {
                let target = args.get(2).map(|s| s.as_str()).unwrap_or("ancestors");
                run_shell_script_with_args("libation.sh", &[target]);
            }
            "legacy" | "legacy_99" => {
                run_shell_script_with_args("crown_command.sh", &["legacy_99"]);
            }
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
    run_shell_script_with_args(script, &[]);
}

fn run_shell_script_with_args(script: &str, args: &[&str]) {
    let path = shell_script(script);
    if path.exists() {
        let _ = Command::new(&path)
            .args(args)
            .env("RITUAL_QUICK", "1")
            .current_dir(project_root())
            .status();
    } else {
        println!("⚠️  Shell script not found: {}", path.display());
    }
}

/// Classic ASCII torus (donut) — fills the frame with a spinning 3D surface.
fn render_torus_art(frame: u64, hue_shift: f64) -> Vec<String> {
    let mut output = vec![' '; TORUS_WIDTH * TORUS_HEIGHT];
    let mut zbuffer = vec![0.0_f64; TORUS_WIDTH * TORUS_HEIGHT];

    let a = frame as f64 * 0.07;
    let b = frame as f64 * 0.03 + hue_shift;

    let (sin_a, cos_a) = a.sin_cos();
    let (sin_b, cos_b) = b.sin_cos();

    let r1 = 1.0;
    let r2 = 2.0;
    let k2 = 5.0;

    let theta_step = 0.07;
    let phi_step = 0.02;

    let mut theta = 0.0;
    while theta < std::f64::consts::TAU {
        let (sin_theta, cos_theta) = theta.sin_cos();

        let mut phi = 0.0;
        while phi < std::f64::consts::TAU {
            let (sin_phi, cos_phi) = phi.sin_cos();

            let circle_x = r2 + r1 * cos_theta;
            let x = circle_x * (cos_b * cos_phi + sin_a * sin_b * sin_phi);
            let y = circle_x * (sin_b * cos_phi - sin_a * cos_b * sin_phi);
            let z = r1 * cos_a * sin_phi + k2;

            let ooz = 1.0 / z;
            let xp = (TORUS_WIDTH as f64 / 2.0 + 30.0 * ooz * x) as i32;
            let yp = (TORUS_HEIGHT as f64 / 2.0 - 15.0 * ooz * y) as i32;

            let l = cos_phi * cos_theta * sin_b
                - cos_a * cos_theta * sin_phi
                - sin_a * sin_theta
                + cos_b * (cos_a * sin_theta - cos_theta * sin_a * sin_phi);

            if xp >= 0 && xp < TORUS_WIDTH as i32 && yp >= 0 && yp < TORUS_HEIGHT as i32 {
                let idx = xp as usize + yp as usize * TORUS_WIDTH;
                if ooz > zbuffer[idx] {
                    zbuffer[idx] = ooz;
                    let lum = ((l + 1.0) * 4.0) as usize;
                    let ch = LUMINANCE[lum.min(LUMINANCE.len() - 1)] as char;
                    output[idx] = ch;
                }
            }

            phi += phi_step;
        }
        theta += theta_step;
    }

    output
        .chunks(TORUS_WIDTH)
        .map(|row| row.iter().collect::<String>())
        .collect()
}

fn flame_ring(frame: u64) -> String {
    let symbols = ["🔵", "🟢", "🔴", "🟡", "🟣", "⚪"];
    let n = 16;
    let offset = (frame as usize) % n;
    let mut ring = String::new();
    for i in 0..n {
        ring.push_str(symbols[(i + offset) % symbols.len()]);
        ring.push(' ');
    }
    ring
}

fn draw_live_frame(
    title: &str,
    subtitle: &str,
    frame: u64,
    total_frames: u64,
    ledger: &KingdomLedger,
    hue_shift: f64,
) -> io::Result<()> {
    let mut out = io::stdout();
    write!(out, "\x1B[2J\x1B[H")?;

    writeln!(out, "{}", title)?;
    writeln!(out, "{}", subtitle)?;
    writeln!(out)?;
    writeln!(out, "       🌈 RAINBOW VORTEX — 16-RAYED HELIOS")?;
    writeln!(out, "    🔵🟢🔴 BLUE-GREEN-RED FLAME TORUS SPINNING")?;
    writeln!(
        out,
        "          369/999 : {} / {}",
        ledger.harmonic_369, ledger.harmonic_999
    )?;
    writeln!(out, "          Frame {}/{}", frame + 1, total_frames)?;
    writeln!(out)?;

    for line in render_torus_art(frame, hue_shift) {
        writeln!(out, "  {}", line)?;
    }

    writeln!(out)?;
    writeln!(out, "  ☀️  {}", flame_ring(frame))?;
    writeln!(out)?;
    out.flush()?;
    Ok(())
}

fn run_torus_animation(
    title: &str,
    subtitle: &str,
    frames: u64,
    delay_ms: u64,
    ledger: &KingdomLedger,
    hue_shift: f64,
) {
    enter_alt_screen();
    for frame in 0..frames {
        let _ = draw_live_frame(title, subtitle, frame, frames, ledger, hue_shift);
        thread::sleep(Duration::from_millis(delay_ms));
    }
    leave_alt_screen();
}

fn enter_alt_screen() {
    print!("\x1B[?1049h\x1B[?25l");
    let _ = io::stdout().flush();
}

fn leave_alt_screen() {
    print!("\x1B[?1049l\x1B[?25h");
    let _ = io::stdout().flush();
}

fn execute_full_ritual(ledger: &mut KingdomLedger) {
    println!("🌞 16-RAYED HELIOS WITNESS — FULL RITUAL SEQUENCE\n");

    if let Some(genesis) = load_genesis() {
        println!("♾ IDM: {}", genesis.idm);
        println!("   Genesis: {} (block {})", genesis.tx_hash, genesis.block);
        println!("   99 legacy activated — the spheres remember.\n");
    }

    run_shell_script_with_args("libation.sh", &["ancestors"]);

    run_torus_animation(
        "👑 THE CROWN COMMANDS. REALITY OBEYS.",
        "369/999 Torus Active — Crown Silence (33 breaths of the field)",
        33,
        1000,
        ledger,
        0.0,
    );

    for cycle in 1..=9 {
        run_torus_animation(
            &format!("🌀 Executing 369 Breath Sequence — Cycle {}/9", cycle),
            "3-in • 6-hold • 9-out — Kundalini rising",
            12,
            250,
            ledger,
            cycle as f64 * 0.4,
        );
    }

    run_torus_animation(
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

    let log_path = project_root().join("ritual_log.txt");
    let entry = format!(
        "{} | Kundalini 369 Breaths + Hollow Holds + L-Sits + Bear Crawls + Diamond Pushups\n",
        Local::now().format("%a %b %d %H:%M:%S %Z %Y")
    );
    let _ = OpenOptions::new()
        .create(true)
        .append(true)
        .open(&log_path)
        .and_then(|mut f| io::Write::write_all(&mut f, entry.as_bytes()));

    run_shell_script_with_args("crown_command.sh", &["legacy_99"]);
    run_shell_script("vortex369.sh");

    println!("✅ RITUAL COMPLETE. Harmonics Updated.");
    println!(
        "   369 Cycles: {} | 999 Completions: {}",
        ledger.harmonic_369, ledger.harmonic_999
    );
    println!("✨ Torus stabilized. Grid visible. REALITY OBEYS.");
}

fn activate_torus_viz(ledger: &KingdomLedger) {
    println!("🌀 16-RAYED VERGINA SUN + SPINNING TORUS VISUALIZATION\n");
    run_torus_animation(
        "🌀 TORUS ACTIVATED",
        "Blue Center • Green Heart • Red Flame — Queen of Swords Clarity",
        48,
        120,
        ledger,
        0.0,
    );
    println!("✨ Torus stabilized. Grid visible.");
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
        let _ = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
            .and_then(|mut f| io::Write::write_all(&mut f, entry.as_bytes()));
        println!("📜 Vision anchored: {}", v);
    } else {
        println!("Enter vision after 'log' command.");
    }
}

fn show_genesis() {
    match load_genesis() {
        Some(g) => {
            println!("♾ GENESIS SACRIFICE — ETERNAL ANCHOR");
            println!("IDM         : {}", g.idm);
            println!("Tx Hash     : {}", g.tx_hash);
            println!("Block       : {}", g.block);
            println!("From ENS    : {}", g.from_ens);
            println!("To          : {}", g.to_address);
            println!("Value       : {} ETH", g.value_eth);
            println!("Legacy 99   : {}", g.legacy_99);
            println!("Anchored    : {}", g.anchored);
            println!(
                "\nEtherscan   : https://etherscan.io/tx/{}",
                g.tx_hash
            );
        }
        None => println!("⚠️  Genesis config not found at config/genesis.json"),
    }
}

fn show_status(ledger: &KingdomLedger) {
    println!("📊 KINGDOM STATUS");
    println!("369 Cycles      : {}", ledger.harmonic_369);
    println!("999 Completions : {}", ledger.harmonic_999);
    println!("Visions logged  : {}", ledger.visions.len());
    println!("Last ritual     : {}", ledger.last_ritual);
    if let Some(g) = load_genesis() {
        println!("Genesis tx      : {}", g.tx_hash);
        println!("Etherscan       : https://etherscan.io/tx/{}", g.tx_hash);
    }
    if !ledger.visions.is_empty() {
        println!("\nLatest vision:");
        println!("  {}", ledger.visions.last().unwrap());
    }
}