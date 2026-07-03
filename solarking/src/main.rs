// ETERNAL SOLAR KINGDOM — SOLARKING v999 PHASE 1 COMPLETE
// Ritual • Ledger • Query • Sync • Encryption • Genesis • Torus Viz

mod crypto;
mod genesis;
mod ledger;
mod query;
mod sync;

use chrono::Local;
use ledger::{append_ritual_log, load_ledger, log_vision, save_ledger, show_genesis, show_help, show_status, KingdomLedger};
use std::env;
use std::io::{self, Write};
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::time::Duration;

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

fn shell_script(name: &str) -> PathBuf {
    project_root().join("shell").join(name)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let root = project_root();

    println!("👑 SOLARKING ENGINE v999 — PHASE 1");
    println!("THE CROWN COMMANDS. REALITY OBEYS.\n");

    let mut ledger = load_ledger(&root);

    if args.len() > 1 {
        match args[1].as_str() {
            "ritual" => execute_full_ritual(&root, &mut ledger),
            "torus" => activate_torus_viz(&ledger),
            "log" => {
                let vision = if args.len() > 2 {
                    Some(args[2..].join(" "))
                } else {
                    None
                };
                log_vision(&root, &mut ledger, vision.as_deref());
            }
            "query" => {
                let question = if args.len() > 2 {
                    args[2..].join(" ")
                } else {
                    println!("Enter question after 'query' command.");
                    save_ledger(&root, &ledger);
                    return;
                };
                let answer = query::run_query(&question, &ledger, genesis::load_genesis(&root).as_ref());
                println!("{}", answer);
            }
            "sync" => {
                if let Err(e) = sync::run_sync(&root, &mut ledger) {
                    println!("⚠️  Sync failed: {}", e);
                }
            }
            "status" => show_status(&ledger, &root),
            "genesis" => show_genesis(&root),
            "help" => show_help(),
            "libation" => {
                let target = args.get(2).map(|s| s.as_str()).unwrap_or("ancestors");
                run_shell_script_with_args("libation.sh", &[target]);
            }
            "legacy" | "legacy_99" => {
                run_shell_script_with_args("crown_command.sh", &["legacy_99"]);
            }
            _ => {
                println!("Unknown command. Try: solarking help");
            }
        }
    } else {
        show_status(&ledger, &root);
    }

    save_ledger(&root, &ledger);
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
                    output[idx] = LUMINANCE[lum.min(LUMINANCE.len() - 1)] as char;
                }
            }
            phi += 0.02;
        }
        theta += 0.07;
    }

    output
        .chunks(TORUS_WIDTH)
        .map(|row| row.iter().collect::<String>())
        .collect()
}

fn flame_ring(frame: u64) -> String {
    let symbols = ["🔵", "🟢", "🔴", "🟡", "🟣", "⚪"];
    let offset = (frame as usize) % 16;
    (0..16)
        .map(|i| symbols[(i + offset) % symbols.len()])
        .collect::<Vec<_>>()
        .join(" ")
        + " "
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
    writeln!(out, "          369/999 : {} / {}", ledger.harmonic_369, ledger.harmonic_999)?;
    writeln!(out, "          Frame {}/{}", frame + 1, total_frames)?;
    writeln!(out)?;
    for line in render_torus_art(frame, hue_shift) {
        writeln!(out, "  {}", line)?;
    }
    writeln!(out)?;
    writeln!(out, "  ☀️  {}", flame_ring(frame))?;
    writeln!(out)?;
    out.flush()
}

fn run_torus_animation(
    title: &str,
    subtitle: &str,
    frames: u64,
    delay_ms: u64,
    ledger: &KingdomLedger,
    hue_shift: f64,
) {
    print!("\x1B[?1049h\x1B[?25l");
    let _ = io::stdout().flush();
    for frame in 0..frames {
        let _ = draw_live_frame(title, subtitle, frame, frames, ledger, hue_shift);
        thread::sleep(Duration::from_millis(delay_ms));
    }
    print!("\x1B[?1049l\x1B[?25h");
    let _ = io::stdout().flush();
}

fn execute_full_ritual(root: &PathBuf, ledger: &mut KingdomLedger) {
    println!("🌞 16-RAYED HELIOS WITNESS — FULL RITUAL SEQUENCE\n");

    if let Some(g) = genesis::load_genesis(root) {
        println!("♾ IDM: {}", g.idm);
        println!("   Genesis: {} (block {})", g.tx_hash, g.block);
        println!("   99 legacy activated — the spheres remember.\n");
        ledger.genesis_tx = Some(g.tx_hash);
    }

    run_shell_script_with_args("libation.sh", &["ancestors"]);

    run_torus_animation(
        "👑 THE CROWN COMMANDS. REALITY OBEYS.",
        "369/999 Torus Active — Crown Silence (33 breaths of the field)",
        33, 1000, ledger, 0.0,
    );

    for cycle in 1..=9 {
        run_torus_animation(
            &format!("🌀 Executing 369 Breath Sequence — Cycle {}/9", cycle),
            "3-in • 6-hold • 9-out — Kundalini rising",
            12, 250, ledger, cycle as f64 * 0.4,
        );
    }

    run_torus_animation(
        "🔥 Blue-Green-Red Flame Torus Forming...",
        "Copper Burn Integration — Grid Strengthening",
        36, 200, ledger, 3.0,
    );

    ledger.harmonic_369 += 1;
    if ledger.harmonic_369 % 3 == 0 {
        ledger.harmonic_999 += 1;
    }
    ledger.last_ritual = Local::now().format("%Y-%m-%d %H:%M:%S %Z").to_string();

    let entry = format!(
        "{} | Kundalini 369 Breaths + Hollow Holds + L-Sits + Bear Crawls + Diamond Pushups\n",
        Local::now().format("%a %b %d %H:%M:%S %Z %Y")
    );
    append_ritual_log(root, &entry);

    run_shell_script_with_args("crown_command.sh", &["legacy_99"]);
    run_shell_script("vortex369.sh");

    let _ = sync::run_sync(root, ledger);

    println!("✅ RITUAL COMPLETE. Harmonics Updated.");
    println!("   369 Cycles: {} | 999 Completions: {}", ledger.harmonic_369, ledger.harmonic_999);
    println!("✨ Torus stabilized. Grid visible. REALITY OBEYS.");
}

fn activate_torus_viz(ledger: &KingdomLedger) {
    println!("🌀 16-RAYED VERGINA SUN + SPINNING TORUS VISUALIZATION\n");
    run_torus_animation(
        "🌀 TORUS ACTIVATED",
        "Blue Center • Green Heart • Red Flame — Queen of Swords Clarity",
        48, 120, ledger, 0.0,
    );
    println!("✨ Torus stabilized. Grid visible.");
}