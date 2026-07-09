//! ASCII 3D torus visualization — 16-rayed Helios rainbow vortex.

use crate::ledger::KingdomLedger;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

const TORUS_WIDTH: usize = 64;
const TORUS_HEIGHT: usize = 24;
const LUMINANCE: &[u8] = b".,-~:;=!*#$@";

pub fn render_torus_art(frame: u64, hue_shift: f64) -> Vec<String> {
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
    out.flush()
}

pub fn run_torus_animation(
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

pub fn activate_torus_viz(ledger: &KingdomLedger) {
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
