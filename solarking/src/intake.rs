//! Crown text intake — multi-line paste, file, pipe, or one-liner args.
//! Priority: --file → --paste / `-` (stdin) → trailing args → interactive paste (optional).

use std::fs;
use std::io::{self, IsTerminal, Read};
use std::path::PathBuf;

use crate::error::{Result, SolarkingError};

/// Resolve Crown text from CLI flags and trailing arguments.
///
/// * `allow_interactive` — when true and args empty on a TTY, prompt and read stdin until EOF.
pub fn resolve_crown_text(
    args: &[String],
    file: Option<&PathBuf>,
    paste: bool,
    allow_interactive: bool,
    usage_hint: &str,
) -> Result<Option<String>> {
    if let Some(path) = file {
        let raw = fs::read_to_string(path).map_err(|e| {
            SolarkingError::Msg(format!("failed to read {}: {e}", path.display()))
        })?;
        let text = raw.trim_end_matches(['\r', '\n']).to_string();
        if text.is_empty() {
            return Err(SolarkingError::Msg(format!(
                "file is empty: {}",
                path.display()
            )));
        }
        return Ok(Some(text));
    }

    // Explicit stdin only: --paste or sole arg "-"
    let wants_stdin = paste || (args.len() == 1 && args[0] == "-");

    if wants_stdin {
        if paste && io::stdin().is_terminal() {
            eprintln!("📥 Paste transmission. End with Ctrl-D (Linux/macOS) or Ctrl-Z Enter (Windows).");
            eprintln!("   THE CROWN COMMANDS. REALITY OBEYS. I do not chase — I receive.");
        }
        let text = read_stdin_all()?;
        if text.is_empty() {
            return Err(SolarkingError::Msg(
                "empty stdin — paste a transmission or pipe text".into(),
            ));
        }
        return Ok(Some(text));
    }

    if !args.is_empty() {
        let text = args.join(" ");
        if text.trim().is_empty() {
            return Ok(None);
        }
        return Ok(Some(text));
    }

    if allow_interactive && io::stdin().is_terminal() {
        eprintln!("📥 Paste transmission. End with Ctrl-D when done.");
        eprintln!("   THE CROWN COMMANDS. REALITY OBEYS. I do not chase — I receive.");
        let text = read_stdin_all()?;
        if text.is_empty() {
            return Err(SolarkingError::Msg(
                "empty paste — nothing received".into(),
            ));
        }
        return Ok(Some(text));
    }

    if allow_interactive {
        // Non-TTY empty args without pipe data already handled above
        return Err(SolarkingError::Msg(usage_hint.into()));
    }

    Ok(None)
}

fn read_stdin_all() -> Result<String> {
    let mut buf = String::new();
    io::stdin().read_to_string(&mut buf)?;
    let text = buf.trim_end_matches(['\r', '\n']).to_string();
    Ok(text)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Write;
    use tempfile::NamedTempFile;

    #[test]
    fn file_intake_reads_multiline() {
        let mut f = NamedTempFile::new().unwrap();
        write!(f, "Line A\nLine B\nCrown\n").unwrap();
        let path = f.path().to_path_buf();
        let got = resolve_crown_text(&[], Some(&path), false, false, "hint")
            .unwrap()
            .unwrap();
        assert!(got.contains("Line A"));
        assert!(got.contains("Line B"));
        assert!(got.contains("Crown"));
    }

    #[test]
    fn args_join_one_liner() {
        let args = vec!["hello".into(), "crown".into()];
        let got = resolve_crown_text(&args, None, false, false, "hint")
            .unwrap()
            .unwrap();
        assert_eq!(got, "hello crown");
    }

    #[test]
    fn empty_args_no_interactive_returns_none() {
        let got = resolve_crown_text(&[], None, false, false, "hint").unwrap();
        assert!(got.is_none());
    }
}
