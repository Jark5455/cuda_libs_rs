//! `cuda_rust_analyzer` — LSP middleware that lets `rust-analyzer` work on
//! sources containing native CUDA chevron syntax (`kernel<<<g, b>>>(args)`).
//!
//! Architecture:
//!
//!   IDE  ⇄  cuda_rust_analyzer  ⇄  rust-analyzer
//!         (parses+rewrites          (sees only valid
//!          textDocument content)     Rust source)
//!
//! For each `textDocument/didOpen` and `textDocument/didChange` notification
//! whose payload carries full source text, the proxy substitutes every
//! chevron launch with a `__cuda_chevron_<idx>!()` placeholder macro call (the
//! same trick `cuda_rustfmt` uses) so `rust-analyzer`'s parser accepts the
//! file. All other LSP traffic is forwarded verbatim.
//!
//! ## Usage
//!
//! Set the IDE's `rust-analyzer.server.path` (or equivalent) to the
//! `cuda_rust_analyzer` binary. The proxy spawns the real `rust-analyzer`
//! (resolved from `RUST_ANALYZER_BACKEND` env var, then PATH) and pipes stdio
//! through.
//!
//! ## Known limitations
//!
//! 1. **Span mapping is identity.** Diagnostics emitted by `rust-analyzer`
//!    report positions in the *rewritten* document, not the user's source.
//!    Lines are preserved (chevron rewrite pads newlines) but columns near a
//!    chevron call site shift by the length-delta between
//!    `kernel<<<…>>>(args)` and `__cuda_chevron_N!()`. A future iteration
//!    needs an offset table keyed per (URI, version) and a positional
//!    projection step on `publishDiagnostics`, code-action ranges, etc.
//!
//! 2. **Incremental text sync (`TextDocumentSyncKind::Incremental`) is not
//!    rewritten.** Only the *full* `text` payload of `didOpen` and the
//!    full-replacement variant of `didChange` are processed. To force full
//!    sync, set `rust-analyzer`'s `serverCapabilities.textDocumentSync` to
//!    `Full` via initialization options, or use an editor that defaults to
//!    full sync.
//!
//! 3. **Encoding:** the proxy assumes UTF-8 source files (LSP default).

use std::env;
use std::io::{Read, Write};
use std::process::{Command, ExitCode, Stdio};
use std::sync::{Arc, Mutex};
use std::thread;

use cuda_rustc::preproc::lexer::code_regions;
use cuda_rustc::preproc::rewrite::find_launch_spans;
use serde_json::Value;

const PLACEHOLDER_PREFIX: &str = "__cuda_chevron_";

fn main() -> ExitCode {
    match run() {
        Ok(code) => ExitCode::from(code),
        Err(err) => {
            eprintln!("cuda_rust_analyzer: {err}");
            ExitCode::from(1)
        }
    }
}

fn run() -> Result<u8, String> {
    let backend = env::var("RUST_ANALYZER_BACKEND").unwrap_or_else(|_| "rust-analyzer".into());
    let mut child = Command::new(&backend).args(env::args().skip(1)).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::inherit()).spawn().map_err(|e| format!("spawn {backend}: {e}"))?;

    let child_stdin = child.stdin.take().ok_or("child stdin missing")?;
    let child_stdout = child.stdout.take().ok_or("child stdout missing")?;

    // Shared state: tracks per-URI substitution maps. Empty for now — span
    // mapping is identity. The mutex is here so a future iteration can add
    // diagnostic projection without restructuring the threading.
    let state = Arc::new(Mutex::new(ProxyState::default()));

    let state_c2s = state.clone();
    let mut child_stdin_locked = child_stdin;
    let client_to_server = thread::spawn(move || -> Result<(), String> {
        let mut stdin = std::io::stdin().lock();
        loop {
            let frame = match read_frame(&mut stdin)? {
                Some(f) => f,
                None => return Ok(()), // client closed
            };
            let rewritten = rewrite_client_to_server(&frame, &state_c2s);
            write_frame(&mut child_stdin_locked, &rewritten)?;
        }
    });

    let mut child_stdout_buf = child_stdout;
    let server_to_client = thread::spawn(move || -> Result<(), String> {
        let mut stdout = std::io::stdout().lock();
        loop {
            let frame = match read_frame(&mut child_stdout_buf)? {
                Some(f) => f,
                None => return Ok(()), // server closed
            };
            // Server → client traffic is forwarded verbatim. Span mapping for
            // diagnostics happens here in a future iteration; the offset
            // tables live in `state`.
            write_frame(&mut stdout, &frame)?;
        }
    });

    // Run both directions concurrently. Whichever side closes first ends the
    // session; we wait for both threads to drain so we don't lose buffered
    // diagnostics or shutdown notifications.
    let _ = client_to_server.join();
    let _ = server_to_client.join();

    let status = child.wait().map_err(|e| format!("wait child: {e}"))?;
    Ok(status.code().unwrap_or(0).clamp(0, 255) as u8)
}

#[derive(Default)]
struct ProxyState {
    // URI -> Vec<LaunchSpan> describing chevron substitutions made the last
    // time this document was synced. Reserved for future span-mapping.
    #[allow(dead_code)]
    docs: std::collections::HashMap<String, Vec<cuda_rustc::preproc::rewrite::LaunchSpan>>,
}

/// Parse a single LSP frame (`Content-Length: N\r\n\r\n<body>`). Returns
/// `Ok(None)` on EOF.
fn read_frame<R: Read>(r: &mut R) -> Result<Option<Vec<u8>>, String> {
    let mut headers = Vec::new();
    let mut byte = [0u8; 1];
    let mut content_length: Option<usize> = None;

    loop {
        // Read until \r\n\r\n.
        let n = r.read(&mut byte).map_err(|e| format!("read header: {e}"))?;
        if n == 0 {
            // Clean EOF only if we have nothing buffered.
            if headers.is_empty() {
                return Ok(None);
            }
            return Err("unexpected EOF inside LSP header".into());
        }
        headers.push(byte[0]);
        if headers.len() >= 4 && &headers[headers.len() - 4..] == b"\r\n\r\n" {
            break;
        }
        if headers.len() > 64 * 1024 {
            return Err("LSP header exceeded 64 KiB".into());
        }
    }

    // Parse headers we care about.
    let header_str = std::str::from_utf8(&headers).map_err(|e| format!("header utf8: {e}"))?;
    for line in header_str.split("\r\n") {
        if let Some(rest) = line.strip_prefix("Content-Length:").or_else(|| line.strip_prefix("content-length:")) {
            content_length = rest.trim().parse::<usize>().ok();
        }
    }
    let len = content_length.ok_or_else(|| "missing Content-Length header".to_string())?;

    let mut body = vec![0u8; len];
    r.read_exact(&mut body).map_err(|e| format!("read body: {e}"))?;

    let mut frame = Vec::with_capacity(headers.len() + body.len());
    frame.extend_from_slice(&headers);
    frame.extend_from_slice(&body);
    Ok(Some(frame))
}

fn write_frame<W: Write>(w: &mut W, frame: &[u8]) -> Result<(), String> {
    w.write_all(frame).map_err(|e| format!("write frame: {e}"))?;
    w.flush().map_err(|e| format!("flush: {e}"))?;
    Ok(())
}

/// Inspect a client→server frame; if it carries `textDocument/didOpen` or
/// `didChange` with a full text payload, run the chevron preprocessor on the
/// text and rebuild the frame. Otherwise return the frame unchanged.
fn rewrite_client_to_server(frame: &[u8], state: &Arc<Mutex<ProxyState>>) -> Vec<u8> {
    let Some(body_start) = frame.windows(4).position(|w| w == b"\r\n\r\n") else {
        return frame.to_vec();
    };
    let body_start = body_start + 4;
    let body = &frame[body_start..];

    let mut json: Value = match serde_json::from_slice(body) {
        Ok(v) => v,
        Err(_) => return frame.to_vec(),
    };

    let Some(method) = json.get("method").and_then(|v| v.as_str()).map(|s| s.to_owned()) else {
        return frame.to_vec();
    };

    let modified = match method.as_str() {
        "textDocument/didOpen" => maybe_rewrite_did_open(&mut json, state),
        "textDocument/didChange" => maybe_rewrite_did_change(&mut json, state),
        _ => false,
    };

    if !modified {
        return frame.to_vec();
    }

    let new_body = match serde_json::to_vec(&json) {
        Ok(b) => b,
        Err(_) => return frame.to_vec(),
    };
    let mut new_frame = format!("Content-Length: {}\r\n\r\n", new_body.len()).into_bytes();
    new_frame.extend_from_slice(&new_body);
    new_frame
}

fn maybe_rewrite_did_open(msg: &mut Value, state: &Arc<Mutex<ProxyState>>) -> bool {
    let params = msg.get_mut("params").and_then(|v| v.as_object_mut());
    let Some(params) = params else { return false };
    let Some(td) = params.get_mut("textDocument").and_then(|v| v.as_object_mut()) else { return false };
    let uri = td.get("uri").and_then(|v| v.as_str()).map(|s| s.to_owned());
    let text_val = td.get_mut("text");
    let Some(text_val) = text_val else { return false };
    let Some(text) = text_val.as_str() else { return false };

    let (rewritten, spans) = substitute_chevrons(text);
    if rewritten == text {
        return false;
    }
    *text_val = Value::String(rewritten);
    if let Some(uri) = uri {
        state.lock().unwrap().docs.insert(uri, spans);
    }
    true
}

fn maybe_rewrite_did_change(msg: &mut Value, state: &Arc<Mutex<ProxyState>>) -> bool {
    let params = msg.get_mut("params").and_then(|v| v.as_object_mut());
    let Some(params) = params else { return false };
    let uri = params.get("textDocument").and_then(|v| v.get("uri")).and_then(|v| v.as_str()).map(|s| s.to_owned());
    let Some(changes) = params.get_mut("contentChanges").and_then(|v| v.as_array_mut()) else {
        return false;
    };

    let mut modified = false;
    let mut last_spans: Option<Vec<cuda_rustc::preproc::rewrite::LaunchSpan>> = None;
    for change in changes.iter_mut() {
        let Some(obj) = change.as_object_mut() else { continue };
        // Full-text replacement: `{"text": "..."}` with no `range` field.
        if obj.contains_key("range") {
            continue;
        }
        let Some(text_val) = obj.get_mut("text") else { continue };
        let Some(text) = text_val.as_str() else { continue };
        let (rewritten, spans) = substitute_chevrons(text);
        if rewritten == text {
            continue;
        }
        last_spans = Some(spans);
        *text_val = Value::String(rewritten);
        modified = true;
    }

    if let (Some(uri), Some(spans)) = (uri, last_spans) {
        state.lock().unwrap().docs.insert(uri, spans);
    }

    modified
}

/// Two text-level substitutions, in order:
///
/// 1. `kernel<<<…>>>(args)` → `__cuda_chevron_<idx>!()` (same shape as
///    `cuda_rustfmt`). The placeholder is a valid Rust macro call so
///    rust-analyzer's parser accepts it.
/// 2. `#[global]` / `#[shared]` → whitespace padding of equal byte length.
///    The marker proc macros have been retired; rust-analyzer would otherwise
///    fail to resolve them. Padding preserves byte offsets and line counts so
///    diagnostic spans elsewhere in the file stay aligned.
fn substitute_chevrons(src: &str) -> (String, Vec<cuda_rustc::preproc::rewrite::LaunchSpan>) {
    let spans = find_launch_spans(src);

    let chevroned = if spans.is_empty() {
        src.to_owned()
    } else {
        let mut out = String::with_capacity(src.len());
        let mut cursor = 0usize;
        for (idx, span) in spans.iter().enumerate() {
            out.push_str(&src[cursor..span.start]);
            out.push_str(&format!("{PLACEHOLDER_PREFIX}{idx}!()"));
            cursor = span.end;
        }
        out.push_str(&src[cursor..]);
        out
    };

    let stripped = strip_marker_attrs(&chevroned);
    (stripped, spans)
}

/// Byte-level scan: replace `#[global]` and `#[shared]` with whitespace-only
/// padding of equal length, but only in code regions (skipping comments,
/// strings, char literals, etc. via the chevron lexer). Tolerates inner
/// whitespace — `#[ global ]` / `#[ shared ]` also match.
fn strip_marker_attrs(src: &str) -> String {
    let regions = code_regions(src);
    let bytes = src.as_bytes();
    let mut out: Vec<u8> = bytes.to_vec();
    for region in regions {
        let mut i = region.start;
        while i < region.end {
            if bytes[i] != b'#' {
                i += 1;
                continue;
            }
            // Match `#[<ws>* (global|shared) <ws>* ]`.
            let mut j = i + 1;
            if j >= region.end || bytes[j] != b'[' {
                i += 1;
                continue;
            }
            j += 1;
            while j < region.end && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            let name_start = j;
            while j < region.end && (bytes[j].is_ascii_alphanumeric() || bytes[j] == b'_') {
                j += 1;
            }
            let name = &bytes[name_start..j];
            if name != b"global" && name != b"shared" {
                i += 1;
                continue;
            }
            while j < region.end && bytes[j].is_ascii_whitespace() {
                j += 1;
            }
            if j >= region.end || bytes[j] != b']' {
                i += 1;
                continue;
            }
            // Replace [i..=j] with spaces, preserving any \n inside the span
            // (none possible — only ASCII whitespace allowed in our match —
            // but be defensive).
            for k in i..=j {
                if bytes[k] != b'\n' {
                    out[k] = b' ';
                }
            }
            i = j + 1;
        }
    }
    // SAFETY: every replaced byte was already ASCII; rest of buffer is the
    // original src bytes which are valid UTF-8.
    String::from_utf8(out).expect("strip_marker_attrs produced invalid utf-8")
}
