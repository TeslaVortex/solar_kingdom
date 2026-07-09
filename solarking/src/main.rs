// ETERNAL SOLAR KINGDOM — SOLARKING v0.3
// Ritual • Ledger v2 • Field • Query • Sync verify • Seal dry-run • Encryption

mod chain;
mod cli;
mod crypto;
mod error;
mod field;
mod genesis;
mod ledger;
mod query;
mod ritual;
mod sync;
mod torus;

use clap::Parser;
use cli::{Cli, Commands};
use error::Result;
use field::ConfirmKind;
use ledger::{load_ledger, save_ledger, show_genesis, show_help, show_status};
use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn project_root() -> PathBuf {
    let cwd = env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    if cwd.ends_with("solarking") {
        cwd.parent().unwrap_or(&cwd).to_path_buf()
    } else {
        cwd
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(e) => {
            eprintln!("⚠️  {}", e);
            ExitCode::FAILURE
        }
    }
}

fn run() -> Result<()> {
    let cli = Cli::parse();
    let root = project_root();
    let json = cli.json;

    if !json {
        println!("👑 SOLARKING ENGINE v0.3 — PHASE 2 RUST CORE");
        println!("THE CROWN COMMANDS. REALITY OBEYS.\n");
    }

    let mut ledger = load_ledger(&root)?;

    match cli.command {
        None | Some(Commands::Status) => {
            show_status(&ledger, &root, json);
        }
        Some(Commands::Ritual) => {
            ritual::execute_full_ritual(&root, &mut ledger)?;
        }
        Some(Commands::Torus) => {
            torus::activate_torus_viz(&ledger);
        }
        Some(Commands::Log { vision }) => {
            let text = if vision.is_empty() {
                None
            } else {
                Some(vision.join(" "))
            };
            ledger::log_vision(&root, &mut ledger, text.as_deref())?;
        }
        Some(Commands::Query { question }) => {
            if question.is_empty() {
                println!("Enter question after 'query' command.");
            } else {
                let q = question.join(" ");
                let answer = query::run_query(
                    &q,
                    &ledger,
                    genesis::load_genesis(&root).as_ref(),
                    json,
                );
                println!("{}", answer);
            }
        }
        Some(Commands::Sync) => {
            sync::run_sync(&root, &mut ledger, json)?;
        }
        Some(Commands::VerifySync) => {
            sync::verify_sync(&root)?;
        }
        Some(Commands::ImportSync { path }) => {
            sync::import_sync(&root, path.as_deref(), &mut ledger)?;
        }
        Some(Commands::Field) => {
            if json {
                println!(
                    "{}",
                    serde_json::to_string_pretty(&field::field_json(&ledger))?
                );
            } else {
                field::show_field(&ledger);
            }
        }
        Some(Commands::Confirm { kind, note }) => {
            handle_confirm(&root, &mut ledger, &kind, &note.join(" "))?;
        }
        Some(Commands::Seal {
            dry_run: _,
            broadcast,
        }) => {
            // Always offline dry-run first; never broadcast from this binary
            chain::seal_dry_run(&root, &ledger)?;
            if broadcast {
                println!();
                chain::seal_broadcast_notice()?;
            }
        }
        Some(Commands::Genesis) => {
            show_genesis(&root);
        }
        Some(Commands::Help) => {
            show_help();
        }
        Some(Commands::Libation { target }) => {
            let t = target.as_deref().unwrap_or("ancestors");
            ritual::run_shell_script(&root, "libation.sh", &[t]);
        }
        Some(Commands::Legacy99) => {
            ritual::run_shell_script(&root, "crown_command.sh", &["legacy_99"]);
        }
    }

    save_ledger(&root, &ledger)?;
    Ok(())
}

fn handle_confirm(root: &Path, ledger: &mut ledger::KingdomLedger, kind: &str, note: &str) -> Result<()> {
    let Some(k) = ConfirmKind::parse(kind) else {
        println!(
            "Unknown confirmation kind '{}'. Use: {}",
            kind,
            ConfirmKind::all_names().join(" | ")
        );
        return Ok(());
    };
    let entry = field::on_confirm(ledger, k, note);
    let log_line = format!(
        "{} | CONFIRM:{} {}\n",
        entry.ts,
        entry.kind,
        if note.is_empty() { "" } else { note }
    );
    ledger::append_ritual_log(root, &log_line)?;
    println!("✨ Field confirmation sealed: {}", entry.kind);
    if !note.is_empty() {
        println!("   note: {}", note);
    }
    field::show_field(ledger);
    Ok(())
}
