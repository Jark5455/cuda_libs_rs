//! `cuda_rustfmt` — rustfmt wrapper that handles native CUDA chevron syntax.
//!
//! `kernel<<<g, b>>>(args)` is not valid Rust. Vanilla `rustfmt` (which uses
//! the `syn`/rustc parser) rejects any source containing it. This wrapper
//! substitutes each chevron launch with a placeholder macro invocation
//! (`__cuda_chevron_<idx>!()`), which *is* valid Rust syntax that rustfmt
//! formats normally; after rustfmt is done, the placeholders are restored to
//! the original chevron text.
//!
//! Usage mirrors plain `rustfmt`:
//!
//!     cuda_rustfmt path/to/file.rs                # in-place
//!     cuda_rustfmt --check path/to/file.rs        # check-only
//!     cuda_rustfmt --emit stdout path/to/file.rs  # to stdout
//!     cuda_rustfmt < input.rs > output.rs         # stdin/stdout
//!
//! Any flag that the wrapper does not understand is forwarded verbatim to the
//! downstream `rustfmt` binary.

use std::env;
use std::ffi::OsString;
use std::io::{Read, Write};
use std::path::PathBuf;
use std::process::{Command, ExitCode, Stdio};

use cuda_rustc::preproc::rewrite::{LaunchSpan, find_launch_spans};

const PLACEHOLDER_PREFIX: &str = "__cuda_chevron_";

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(err) => {
            eprintln!("cuda_rustfmt: {err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<u8, String> {
    let mut args: Vec<OsString> = env::args_os().skip(1).collect();
    let stdin_mode = args.iter().all(|a| !a.to_string_lossy().ends_with(".rs"));

    if stdin_mode {
        let mut buf = String::new();
        std::io::stdin().read_to_string(&mut buf).map_err(|e| format!("read stdin: {e}"))?;
        let formatted = run_rustfmt_on_text(&buf, &args)?;
        std::io::stdout().write_all(formatted.as_bytes()).map_err(|e| format!("write stdout: {e}"))?;
        return Ok(0);
    }

    // File mode. Pull all `.rs` paths out of the arg list and replace each in
    // place with the formatted output. Non-`.rs` args (flags) are forwarded.
    let mut file_args: Vec<(usize, PathBuf)> = Vec::new();
    for (i, a) in args.iter().enumerate() {
        let s = a.to_string_lossy();
        if !s.starts_with('-') && s.ends_with(".rs") {
            file_args.push((i, PathBuf::from(a.as_os_str())));
        }
    }

    let check_only = args.iter().any(|a| a == "--check");
    let emit_stdout = args.windows(2).any(|w| w[0] == "--emit" && w[1] == "stdout");

    // Formatting only relies on the file content, not the file argument itself,
    // so strip file paths from the forwarded args (we feed source via stdin).
    for (i, _) in file_args.iter().rev() {
        args.remove(*i);
    }

    let mut any_changed = false;
    for (_, path) in &file_args {
        let original = std::fs::read_to_string(path).map_err(|e| format!("read {}: {e}", path.display()))?;
        let formatted = run_rustfmt_on_text(&original, &args)?;
        if formatted != original {
            any_changed = true;
            if emit_stdout {
                std::io::stdout().write_all(formatted.as_bytes()).map_err(|e| format!("write stdout: {e}"))?;
            } else if !check_only {
                std::fs::write(path, &formatted).map_err(|e| format!("write {}: {e}", path.display()))?;
            }
        }
    }

    if check_only && any_changed {
        return Ok(1);
    }
    Ok(0)
}

fn run_rustfmt_on_text(src: &str, fwd_args: &[OsString]) -> Result<String, String> {
    let (sub, mapping) = substitute_chevrons(src);

    let rustfmt = env::var_os("CUDA_RUSTFMT_BACKEND").unwrap_or_else(|| OsString::from("rustfmt"));
    let mut cmd = Command::new(&rustfmt);

    // Forward any wrapper-passthrough flags except the ones we strip (file
    // paths handled by the caller and `--check` which we apply ourselves).
    for a in fwd_args {
        let s = a.to_string_lossy();
        if s == "--check" || s == "--emit" || s == "stdout" {
            continue;
        }
        cmd.arg(a);
    }

    cmd.stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit());

    let mut child = cmd.spawn().map_err(|e| format!("spawn rustfmt: {e}"))?;
    child.stdin.as_mut().ok_or_else(|| "rustfmt stdin missing".to_string())?.write_all(sub.as_bytes()).map_err(|e| format!("write rustfmt stdin: {e}"))?;
    let output = child.wait_with_output().map_err(|e| format!("wait rustfmt: {e}"))?;
    if !output.status.success() {
        return Err(format!("rustfmt exited with {}", output.status));
    }

    let formatted = String::from_utf8(output.stdout).map_err(|e| format!("rustfmt stdout not utf-8: {e}"))?;
    let restored = restore_chevrons(&formatted, &mapping);
    Ok(restored)
}

/// Replace each chevron launch with a placeholder macro invocation that
/// rustfmt's parser accepts. The placeholder is `__cuda_chevron_<idx>!()` —
/// a zero-arg macro call with a unique index. Returns the rewritten source
/// and the index→original-text mapping for restoration.
fn substitute_chevrons(src: &str) -> (String, Vec<LaunchSpan>) {
    let spans = find_launch_spans(src);
    if spans.is_empty() {
        return (src.to_owned(), Vec::new());
    }

    let mut out = String::with_capacity(src.len());
    let mut cursor = 0usize;
    for (idx, span) in spans.iter().enumerate() {
        out.push_str(&src[cursor..span.start]);
        out.push_str(&format!("{PLACEHOLDER_PREFIX}{idx}!()"));
        cursor = span.end;
    }
    out.push_str(&src[cursor..]);
    (out, spans)
}

/// Replace each `__cuda_chevron_<idx>!()` macro invocation in the formatted
/// rustfmt output with the original chevron text. rustfmt may insert spaces
/// around the bang and parens (`__cuda_chevron_0 ! ()` is the typical form
/// after macro-call formatting), so the matcher tolerates whitespace
/// between the ident, `!`, `(`, and `)`.
fn restore_chevrons(formatted: &str, mapping: &[LaunchSpan]) -> String {
    if mapping.is_empty() {
        return formatted.to_owned();
    }
    let bytes = formatted.as_bytes();
    let mut out = String::with_capacity(formatted.len());
    let mut i = 0usize;

    let push_one_char = |out: &mut String, src: &str, idx: usize| -> usize {
        let ch = src[idx..].chars().next().expect("non-empty in-bounds slice");
        let len = ch.len_utf8();
        out.push(ch);
        len
    };

    while i < bytes.len() {
        if !starts_with_ident_at(formatted, i, PLACEHOLDER_PREFIX) {
            i += push_one_char(&mut out, formatted, i);
            continue;
        }
        let ident_start = i;
        let mut j = i + PLACEHOLDER_PREFIX.len();
        let digits_start = j;
        while j < bytes.len() && bytes[j].is_ascii_digit() {
            j += 1;
        }
        if j == digits_start {
            i += push_one_char(&mut out, formatted, i);
            continue;
        }
        let idx: usize = match formatted[digits_start..j].parse() {
            Ok(n) => n,
            Err(_) => {
                i += push_one_char(&mut out, formatted, i);
                continue;
            }
        };
        // Skip whitespace, then `!`, whitespace, `(`, whitespace, `)`.
        let mut k = j;
        while k < bytes.len() && bytes[k].is_ascii_whitespace() {
            k += 1;
        }
        if k >= bytes.len() || bytes[k] != b'!' {
            i += push_one_char(&mut out, formatted, i);
            continue;
        }
        k += 1;
        while k < bytes.len() && bytes[k].is_ascii_whitespace() {
            k += 1;
        }
        if k >= bytes.len() || bytes[k] != b'(' {
            i += push_one_char(&mut out, formatted, i);
            continue;
        }
        k += 1;
        while k < bytes.len() && bytes[k].is_ascii_whitespace() {
            k += 1;
        }
        if k >= bytes.len() || bytes[k] != b')' {
            i += push_one_char(&mut out, formatted, i);
            continue;
        }
        k += 1;

        if let Some(span) = mapping.get(idx) {
            out.push_str(&span.text);
            i = k;
        } else {
            // Unknown placeholder — preserve verbatim.
            i = ident_start + push_one_char(&mut out, formatted, ident_start);
        }
    }
    out
}

fn starts_with_ident_at(s: &str, idx: usize, ident: &str) -> bool {
    if !s[idx..].starts_with(ident) {
        return false;
    }
    if idx > 0 {
        let prev = s.as_bytes()[idx - 1];
        if prev.is_ascii_alphanumeric() || prev == b'_' {
            return false;
        }
    }
    true
}
