use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

use crate::fixture::{PreviewEnv, write_temp_config};

/// Invoke the local `starship` binary with the given TOML config and
/// capture its ANSI output as a string.
///
/// Returns the raw ANSI-escaped prompt string, or an error if `starship`
/// is not found or fails.
pub fn preview_starship(config_toml: &str) -> Result<String> {
    let env = PreviewEnv::new(true)?;
    let config_path = write_temp_config(env.path(), config_toml)?;
    invoke_starship(&config_path, env.path())
}

/// Low-level: run `starship prompt` with `STARSHIP_CONFIG` pointing at `config_path`
/// and `PWD` set to `work_dir`.
pub fn invoke_starship(config_path: &Path, work_dir: &Path) -> Result<String> {
    let output = Command::new("starship")
        .arg("prompt")
        .env("STARSHIP_CONFIG", config_path)
        .env("STARSHIP_SHELL", "")
        .env("TERM", "xterm-256color")
        .current_dir(work_dir)
        .output()
        .context("Failed to run `starship` — is it installed?")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        anyhow::bail!("`starship prompt` failed: {stderr}");
    }

    Ok(String::from_utf8_lossy(&output.stdout).into_owned())
}

/// Minimal ANSI-to-ratatui-spans conversion.
///
/// For the MVP we strip ANSI escape codes and return plain text.
/// A future iteration will parse SGR sequences into `ratatui::text::Span`
/// with appropriate `Style` applied.
pub fn strip_ansi(input: &str) -> String {
    let mut out = String::with_capacity(input.len());
    let mut in_escape = false;
    let mut chars = input.chars().peekable();
    while let Some(ch) = chars.next() {
        if in_escape {
            if ch.is_ascii_alphabetic() {
                in_escape = false;
            }
        } else if ch == '\x1b' {
            in_escape = true;
        } else if ch == '\\' && matches!(chars.peek(), Some('[' | ']')) {
            // Strip bash prompt escapes: literal \[ and \]
            chars.next();
        } else if ch == '\x01' || ch == '\x02' {
            // Strip readline bracket characters (SOH / STX)
        } else {
            out.push(ch);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_ansi_removes_escapes() {
        let input = "\x1b[31mhello\x1b[0m world";
        assert_eq!(strip_ansi(input), "hello world");
    }

    #[test]
    fn strip_ansi_plain_passthrough() {
        assert_eq!(strip_ansi("plain text"), "plain text");
    }

    #[test]
    fn strip_ansi_removes_bash_prompt_escapes() {
        let input = "\\[\\]hello\\[\\] world";
        assert_eq!(strip_ansi(input), "hello world");
    }

    #[test]
    fn strip_ansi_removes_readline_brackets() {
        let input = "\x01\x1b[31m\x02hello\x01\x1b[0m\x02 world";
        assert_eq!(strip_ansi(input), "hello world");
    }
}
