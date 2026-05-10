//! Minimal Rust-aware byte scanner.
//!
//! We don't need a full tokenizer. We need to walk a `&str` and tell, for any
//! given byte index, whether we are *inside* a context where `<<<` does not mean
//! a kernel launch — i.e. inside a comment, string, char, byte string, or raw
//! string. Code regions are everything else.
//!
//! `code_regions` returns half-open `[start, end)` byte ranges that contain
//! "real" Rust code, sorted and non-overlapping.

#[derive(Debug, Clone, Copy)]
pub struct Range {
    pub start: usize,
    pub end: usize,
}

/// Walk `src` once, return code regions (regions where `<<<` should be considered
/// a launch). Skipped regions are comments and string/char/byte literals.
pub fn code_regions(src: &str) -> Vec<Range> {
    let bytes = src.as_bytes();
    let n = bytes.len();
    let mut out = Vec::new();
    let mut i = 0;
    let mut code_start = 0;

    while i < n {
        let b = bytes[i];

        // line comment
        if b == b'/' && i + 1 < n && bytes[i + 1] == b'/' {
            push_range(&mut out, code_start, i);
            i += 2;
            while i < n && bytes[i] != b'\n' {
                i += 1;
            }
            // include the newline back into code so line counts are easy
            code_start = i;
            continue;
        }

        // block comment (nestable per Rust spec)
        if b == b'/' && i + 1 < n && bytes[i + 1] == b'*' {
            push_range(&mut out, code_start, i);
            i += 2;
            let mut depth: u32 = 1;
            while i < n && depth > 0 {
                if i + 1 < n && bytes[i] == b'/' && bytes[i + 1] == b'*' {
                    depth += 1;
                    i += 2;
                } else if i + 1 < n && bytes[i] == b'*' && bytes[i + 1] == b'/' {
                    depth -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            code_start = i;
            continue;
        }

        // raw string: optional `b`, then `r`, then `#`*, then `"`, then content,
        // then `"`, then matching number of `#`s.
        if let Some(after) = try_skip_raw_string(bytes, i) {
            push_range(&mut out, code_start, i);
            i = after;
            code_start = i;
            continue;
        }

        // byte char literal: b'…'
        if b == b'b' && i + 1 < n && bytes[i + 1] == b'\'' {
            push_range(&mut out, code_start, i);
            i = skip_char_literal(bytes, i + 1);
            code_start = i;
            continue;
        }

        // byte string literal: b"…"
        if b == b'b' && i + 1 < n && bytes[i + 1] == b'"' {
            push_range(&mut out, code_start, i);
            i = skip_string_literal(bytes, i + 1);
            code_start = i;
            continue;
        }

        // char literal — but only when actually a char and not a lifetime/label.
        if b == b'\'' && looks_like_char_literal(bytes, i) {
            push_range(&mut out, code_start, i);
            i = skip_char_literal(bytes, i);
            code_start = i;
            continue;
        }

        // string literal: "…"
        if b == b'"' {
            push_range(&mut out, code_start, i);
            i = skip_string_literal(bytes, i);
            code_start = i;
            continue;
        }

        i += 1;
    }

    push_range(&mut out, code_start, n);
    out
}

fn push_range(out: &mut Vec<Range>, start: usize, end: usize) {
    if end > start {
        out.push(Range { start, end });
    }
}

/// Try to recognize a raw string literal starting at `i`. Returns the index *after*
/// the literal if recognized, else `None`. Handles `r"…"`, `r#"…"#`, `r##"…"##`,
/// and the `b`-prefixed `br"…"` / `br#"…"#` forms.
fn try_skip_raw_string(bytes: &[u8], i: usize) -> Option<usize> {
    let n = bytes.len();
    let mut p = i;

    // Optional `b` prefix for byte-raw strings.
    if p < n && bytes[p] == b'b' {
        if p + 1 >= n || bytes[p + 1] != b'r' {
            return None;
        }
        p += 1;
    }

    if p >= n || bytes[p] != b'r' {
        return None;
    }

    // Must be at the *start* of an identifier — otherwise `xr"…"` would be misread.
    // The `b` case above already fixed its anchor; for plain `r`, check the prior
    // byte.
    if i > 0 {
        let prev = bytes[i - 1];
        let is_ident_continue = prev.is_ascii_alphanumeric() || prev == b'_';
        if is_ident_continue {
            return None;
        }
    }

    p += 1; // past 'r'
    let mut hashes = 0;
    while p < n && bytes[p] == b'#' {
        hashes += 1;
        p += 1;
    }
    if p >= n || bytes[p] != b'"' {
        return None;
    }
    p += 1; // past opening quote

    while p < n {
        if bytes[p] == b'"' {
            // need `hashes` consecutive `#`s after this quote
            let mut k = 0;
            while k < hashes && p + 1 + k < n && bytes[p + 1 + k] == b'#' {
                k += 1;
            }
            if k == hashes {
                return Some(p + 1 + hashes);
            }
        }
        p += 1;
    }
    Some(n) // unterminated; treat rest of file as string region
}

fn skip_string_literal(bytes: &[u8], start: usize) -> usize {
    debug_assert_eq!(bytes[start], b'"');
    let n = bytes.len();
    let mut p = start + 1;
    while p < n {
        match bytes[p] {
            b'\\' if p + 1 < n => p += 2,
            b'"' => return p + 1,
            _ => p += 1,
        }
    }
    n
}

fn skip_char_literal(bytes: &[u8], start: usize) -> usize {
    debug_assert_eq!(bytes[start], b'\'');
    let n = bytes.len();
    let mut p = start + 1;
    while p < n {
        match bytes[p] {
            b'\\' if p + 1 < n => p += 2,
            b'\'' => return p + 1,
            _ => p += 1,
        }
    }
    n
}

/// `'` introduces either a char literal (`'a'`, `'\n'`, `'\u{1f}'`, `'\xFF'`) or a
/// lifetime/label (`'a`, `'static`). Distinguish heuristically: a char literal has
/// a closing `'` within a small window and contains no whitespace at the start.
fn looks_like_char_literal(bytes: &[u8], i: usize) -> bool {
    let n = bytes.len();
    debug_assert_eq!(bytes[i], b'\'');
    if i + 1 >= n {
        return false;
    }
    let next = bytes[i + 1];
    // `''` is illegal; treat as not a char.
    if next == b'\'' {
        return false;
    }
    // `'_` followed by non-ident-cont could be a lifetime; but `'_'` is a valid char.
    if next == b'\\' {
        return true; // escape sequence — definitely a char literal
    }
    // Look for a closing quote within 8 bytes. Lifetimes never have a closing quote
    // before whitespace/punctuation that ends the identifier.
    let limit = (i + 9).min(n);
    let mut p = i + 1;
    // Skip the first content character (1 byte UTF-8 unit; multibyte chars handled
    // conservatively as chars-literal candidates).
    p += 1;
    while p < limit {
        if bytes[p] == b'\'' {
            return true;
        }
        if !(bytes[p].is_ascii_alphanumeric() || bytes[p] == b'_') {
            // Identifier-end without closing quote → lifetime/label.
            return false;
        }
        p += 1;
    }
    false
}
