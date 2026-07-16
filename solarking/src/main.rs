// ETERNAL SOLAR KINGDOM — SOLARKING v0.9
// Crown Receive UX: receive · journal · now

mod badge;
mod chain;
mod cli;
mod crypto;
mod error;
mod field;
mod genesis;
mod grok_bridge;
mod intake;
mod journal;
mod ledger;
mod node;
mod now;
mod phase3;
mod query;
mod ritual;
mod scalar;
mod sync;
mod torus;
mod viz;

use clap::Parser;
use cli::{Cli, Commands, JournalCmd, LatticeCmd, NodeCmd, NowCmd, ScalarCmd};
use error::Result;
use field::ConfirmKind;
use ledger::{load_ledger, save_ledger, show_genesis, show_help, show_status};
use std::env;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

fn project_root() -> PathBuf {
    if let Ok(root) = env::var("SOLARKING_ROOT") {
        let p = PathBuf::from(root);
        if p.is_dir() {
            return p;
        }
    }
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
        println!("👑 SOLARKING ENGINE v0.9 — CROWN RECEIVE UX");
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
        Some(Commands::Log {
            vision,
            file,
            paste,
        }) => {
            let text = intake::resolve_crown_text(
                &vision,
                file.as_ref(),
                paste,
                false,
                "Usage: solarking log \"vision\" | log --paste | log --file PATH | log -",
            )?;
            match text {
                Some(t) => ledger::log_vision(&root, &mut ledger, Some(&t))?,
                None => println!(
                    "Enter vision after 'log', or: log --paste | log --file PATH | pipe via log -"
                ),
            }
        }
        Some(Commands::Receive {
            text,
            file,
            paste,
            counsel,
            confirm,
            title,
        }) => {
            let body = intake::resolve_crown_text(
                &text,
                file.as_ref(),
                paste,
                true, // interactive paste when empty on TTY
                "Usage: solarking receive --paste | receive --file PATH | receive - | pipe text",
            )?
            .ok_or_else(|| {
                error::SolarkingError::Msg(
                    "empty receive — paste a transmission or use --file".into(),
                )
            })?;
            ledger::receive_transmission(&root, &mut ledger, &body, title.as_deref())?;
            if let Some(kind) = confirm.as_deref() {
                handle_confirm(&root, &mut ledger, kind, "receive")?;
            }
            if counsel {
                println!();
                println!("{}", phase3::counsel(&root, &ledger, &body, json));
            }
        }
        Some(Commands::Journal { action }) => {
            handle_journal(&root, &ledger, action, json)?;
        }
        Some(Commands::Now { action }) => {
            handle_now(&root, &mut ledger, action, json)?;
        }
        Some(Commands::Query {
            question,
            file,
            paste,
        }) => {
            let q = intake::resolve_crown_text(
                &question,
                file.as_ref(),
                paste,
                false,
                "Enter question after 'query'",
            )?;
            match q {
                Some(q) => {
                    let answer = query::run_query(
                        &q,
                        &ledger,
                        genesis::load_genesis(&root).as_ref(),
                        json,
                    );
                    println!("{}", answer);
                }
                None => println!("Enter question after 'query' command."),
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
        Some(Commands::Counsel {
            question,
            file,
            paste,
        }) => {
            let q = intake::resolve_crown_text(
                &question,
                file.as_ref(),
                paste,
                false,
                "Usage: solarking counsel \"question\"",
            )?
            .unwrap_or_else(|| "What is the next sovereign step?".to_string());
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
        Some(Commands::Grok {
            prompt,
            offline,
            file,
            paste,
        }) => {
            let p = intake::resolve_crown_text(
                &prompt,
                file.as_ref(),
                paste,
                false,
                "Usage: solarking grok \"prompt\"",
            )?
            .unwrap_or_else(|| {
                "What is the next sovereign step for the Eternal Solar Kingdom?".to_string()
            });
            grok_bridge::local_model_status();
            grok_bridge::run_grok(&root, &ledger, &p, offline)?;
        }
        Some(Commands::Blueprint {
            note,
            file,
            paste,
        }) => {
            let n = intake::resolve_crown_text(
                &note,
                file.as_ref(),
                paste,
                false,
                "Usage: solarking blueprint \"note\"",
            )?
            .unwrap_or_else(|| {
                "Pulse the Phase 3 blueprint from current field state.".to_string()
            });
            grok_bridge::evolve_blueprint(&root, &ledger, &n)?;
        }
    }

    save_ledger(&root, &ledger)?;
    Ok(())
}

fn handle_journal(
    root: &Path,
    ledger: &ledger::KingdomLedger,
    action: Option<JournalCmd>,
    json: bool,
) -> Result<()> {
    match action {
        None => {
            journal::list_visions(ledger, 10, json);
        }
        Some(JournalCmd::List { last }) => {
            journal::list_visions(ledger, last, json);
        }
        Some(JournalCmd::Show { n }) => {
            journal::show_vision(ledger, n, json);
        }
        Some(JournalCmd::Search { query }) => {
            if query.is_empty() {
                println!("Usage: solarking journal search <keyword…>");
            } else {
                journal::search_visions(ledger, &query.join(" "), json);
            }
        }
        Some(JournalCmd::Log { tail }) => {
            journal::tail_ritual_log(root, tail)?;
        }
        Some(JournalCmd::Files) => {
            journal::list_transmission_files(root, json)?;
        }
        Some(JournalCmd::Open { n }) => {
            journal::open_transmission_file(root, n)?;
        }
    }
    Ok(())
}

fn handle_now(
    root: &Path,
    ledger: &mut ledger::KingdomLedger,
    action: Option<NowCmd>,
    json: bool,
) -> Result<()> {
    match action {
        None | Some(NowCmd::Card) => {
            now::print_crown_card();
        }
        Some(NowCmd::Morning) => {
            now::recipe_morning(root, ledger, json)?;
        }
        Some(NowCmd::Receive) => {
            let body = intake::resolve_crown_text(
                &[],
                None,
                true,
                true,
                "Usage: solarking now receive  (paste then Ctrl-D)",
            )?
            .ok_or_else(|| {
                error::SolarkingError::Msg("empty paste — nothing received".into())
            })?;
            ledger::receive_transmission(root, ledger, &body, None)?;
        }
        Some(NowCmd::Seal) => {
            now::recipe_seal(root, ledger, json)?;
        }
        Some(NowCmd::Sync) => {
            now::recipe_sync(root, ledger, json)?;
        }
        Some(NowCmd::Pulse) => {
            now::recipe_pulse(root, ledger, json)?;
        }
    }
    Ok(())
}

fn handle_confirm(
    root: &Path,
    ledger: &mut ledger::KingdomLedger,
    kind: &str,
    note: &str,
) -> Result<()> {
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
