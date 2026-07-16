// ETERNAL SOLAR KINGDOM — SOLARKING v0.8
// Phase 3B–3E: physical bridge · viz · nodes · grok-build

mod badge;
mod chain;
mod cli;
mod crypto;
mod error;
mod field;
mod genesis;
mod grok_bridge;
mod ledger;
mod node;
mod phase3;
mod query;
mod ritual;
mod scalar;
mod sync;
mod torus;
mod viz;

use clap::Parser;
use cli::{Cli, Commands, LatticeCmd, NodeCmd, ScalarCmd};
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
        println!("👑 SOLARKING ENGINE v0.8 — PHASE 3B–3E LATTICE");
        println!("THE CROWN COMMANDS. REALITY OBEYS. I DO NOT CHASE — I RECEIVE.\n");
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
        Some(Commands::Scalar { action }) => match action {
            ScalarCmd::Node { obj } => {
                scalar::cmd_node(&root, &mut ledger, json, obj)?;
            }
            ScalarCmd::Sync { hz } => {
                scalar::cmd_sync(&root, &mut ledger, json, hz)?;
            }
            ScalarCmd::Seal => {
                scalar::cmd_seal(&root, &mut ledger, json)?;
            }
        },
        Some(Commands::ChainStatus) => {
            chain::chain_status(&root, &mut ledger, json)?;
        }
        Some(Commands::SealRecord { tx_hash }) => {
            chain::seal_record(&root, &mut ledger, &tx_hash)?;
        }
        Some(Commands::ScalarRecord { node_id, tx_hash }) => {
            chain::scalar_record(&root, &mut ledger, node_id, tx_hash.as_deref())?;
        }
        Some(Commands::BadgeStatus) => {
            badge::show_badge_status(&root, &ledger, json);
        }
        Some(Commands::ColdExport { dest, encrypt }) => {
            sync::cold_export(&root, &mut ledger, &dest, encrypt)?;
        }
        Some(Commands::ExportCid) => {
            phase3::export_cid(&root, &mut ledger, json)?;
        }
        Some(Commands::Counsel { question }) => {
            let q = if question.is_empty() {
                "What is the next sovereign step?".to_string()
            } else {
                question.join(" ")
            };
            println!("{}", phase3::counsel(&root, &ledger, &q, json));
        }
        Some(Commands::Qr) | Some(Commands::AltarPrint) => {
            phase3::qr_payload(&root, &ledger, json)?;
        }
        Some(Commands::Lattice { action }) => match action {
            LatticeCmd::Visualize => {
                viz::lattice_visualize(&root, &ledger, true)?;
            }
        },
        Some(Commands::Node { action }) => match action {
            NodeCmd::Init { name, label } => {
                node::node_init(&root, name.as_deref(), label.as_deref())?;
            }
            NodeCmd::Export { path } => {
                node::node_export(&root, &ledger, &path)?;
            }
            NodeCmd::Import { path } => {
                node::node_import(&root, &mut ledger, &path)?;
            }
            NodeCmd::Status => {
                node::node_status(&root, &ledger, json)?;
            }
        },
        Some(Commands::Grok { prompt, offline }) => {
            let p = if prompt.is_empty() {
                "What is the next sovereign step for the Eternal Solar Kingdom?".to_string()
            } else {
                prompt.join(" ")
            };
            grok_bridge::local_model_status();
            grok_bridge::run_grok(&root, &ledger, &p, offline)?;
        }
        Some(Commands::Blueprint { note }) => {
            let n = if note.is_empty() {
                "Pulse the Phase 3 blueprint from current field state.".to_string()
            } else {
                note.join(" ")
            };
            grok_bridge::evolve_blueprint(&root, &ledger, &n)?;
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
