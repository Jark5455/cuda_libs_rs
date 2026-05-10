//! Text-level preprocessor pass: synthesize an initializer for `#[shared]`
//! statics that omit one.
//!
//! Rust's grammar requires every `static` item to have an `= EXPR` initializer.
//! Users writing dynamic-shared kernel state would otherwise have to type
//! `static mut DYN: &mut [f32] = &mut [];` — visually noisy. This pass scans
//! source for the no-RHS form `#[shared] static mut IDENT: &mut? [T] ;` and
//! injects a synthetic `= &mut []` (or `= &[]`) just before the `;`. The AST
//! stage that follows then sees a syntactically valid static and proceeds to
//! detect the slice-typed marker pattern.
//!
//! Only `&[T]` and `&mut [T]` types trigger injection. Any other no-RHS form
//! is left alone — rustc will surface its own diagnostic.

use super::lexer::{Range, code_regions};

pub fn inject_missing_rhs(src: &str) -> String {
    let regions = code_regions(src);
    let mut insertions: Vec<(usize, &'static str)> = Vec::new();

    for region in &regions {
        scan_region(src, *region, &mut insertions);
    }

    if insertions.is_empty() {
        return src.to_owned();
    }

    insertions.sort_by_key(|(pos, _)| *pos);

    let mut out = String::with_capacity(src.len() + insertions.len() * 16);
    let mut cursor = 0usize;
    for (pos, text) in &insertions {
        out.push_str(&src[cursor..*pos]);
        out.push_str(text);
        cursor = *pos;
    }
    out.push_str(&src[cursor..]);
    out
}

fn scan_region(src: &str, region: Range, out: &mut Vec<(usize, &'static str)>) {
    let bytes = src.as_bytes();
    let mut i = region.start;

    while i + 9 < region.end {
        // Match the literal `#[shared]` prefix (allow some whitespace inside).
        if bytes[i] == b'#' && peek_attr_is_shared(bytes, i, region.end) {
            let after_attr = skip_attr(bytes, i, region.end);
            let after_ws = skip_ws_and_inline_attrs(bytes, after_attr, region.end);

            if let Some((semi_pos, kind)) = match_static_decl_no_rhs(bytes, after_ws, region.end) {
                out.push((semi_pos, kind));
                i = semi_pos + 1;
                continue;
            }
        }
        i += 1;
    }
}

/// Confirm `bytes[i..]` starts with `#[shared]`, possibly with whitespace
/// inside the brackets (`#[ shared ]`). Tokens-only — we don't try to handle
/// `#[shared(arg)]` (passes that through unchanged because we don't recognize it).
fn peek_attr_is_shared(bytes: &[u8], i: usize, end: usize) -> bool {
    if i + 9 > end {
        return false;
    }
    if bytes[i] != b'#' || bytes[i + 1] != b'[' {
        return false;
    }
    let mut p = i + 2;
    while p < end && bytes[p].is_ascii_whitespace() {
        p += 1;
    }
    let kw = b"shared";
    if p + kw.len() > end {
        return false;
    }
    if &bytes[p..p + kw.len()] != kw {
        return false;
    }
    p += kw.len();
    // Must be followed by `]` (with possible whitespace), not `(` (which would
    // mean `#[shared(...)]`).
    while p < end && bytes[p].is_ascii_whitespace() {
        p += 1;
    }
    p < end && bytes[p] == b']'
}

fn skip_attr(bytes: &[u8], i: usize, end: usize) -> usize {
    // `#[...]` — skip until matching `]`. Brackets cannot nest in a Rust attr
    // path/list at the outer level except inside parens, but we keep it simple.
    let mut p = i + 1;
    let mut depth = 0i32;
    while p < end {
        match bytes[p] {
            b'[' => depth += 1,
            b']' => {
                depth -= 1;
                if depth == 0 {
                    return p + 1;
                }
            }
            _ => {}
        }
        p += 1;
    }
    end
}

fn skip_ws_and_inline_attrs(bytes: &[u8], mut i: usize, end: usize) -> usize {
    loop {
        while i < end && bytes[i].is_ascii_whitespace() {
            i += 1;
        }
        if i < end && bytes[i] == b'#' {
            i = skip_attr(bytes, i, end);
            continue;
        }
        // line comments
        if i + 1 < end && bytes[i] == b'/' && bytes[i + 1] == b'/' {
            while i < end && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        break;
    }
    i
}

/// Try to match `static (mut)? IDENT : &mut? [...] ;` with no `=` between TYPE
/// and `;`. Returns `Some((semi_index, " = &mut []"))` if the injection should
/// happen.
fn match_static_decl_no_rhs(bytes: &[u8], i: usize, end: usize) -> Option<(usize, &'static str)> {
    let kw = b"static";
    if i + kw.len() > end || &bytes[i..i + kw.len()] != kw {
        return None;
    }
    let mut p = i + kw.len();
    if p >= end || !is_word_boundary(bytes, p) {
        return None;
    }
    p = skip_ws(bytes, p, end);

    // Optional `mut`.
    if p + 3 <= end && &bytes[p..p + 3] == b"mut" && p + 3 < end && is_word_boundary(bytes, p + 3) {
        p += 3;
        p = skip_ws(bytes, p, end);
    }

    // IDENT.
    let ident_start = p;
    while p < end && (bytes[p].is_ascii_alphanumeric() || bytes[p] == b'_') {
        p += 1;
    }
    if p == ident_start {
        return None;
    }
    p = skip_ws(bytes, p, end);

    // `:`
    if p >= end || bytes[p] != b':' {
        return None;
    }
    p += 1;
    p = skip_ws(bytes, p, end);

    // TYPE — read up to a top-level `=` or `;`.
    let ty_start = p;
    let (term, semi_or_eq) = read_type_until_terminator(bytes, p, end)?;
    if term == b'=' {
        // RHS already present; nothing to inject.
        return None;
    }

    let ty = std::str::from_utf8(&bytes[ty_start..semi_or_eq]).ok()?.trim();
    let injection = classify_slice_type(ty)?;
    Some((semi_or_eq, injection))
}

fn read_type_until_terminator(bytes: &[u8], i: usize, end: usize) -> Option<(u8, usize)> {
    let mut p = i;
    let mut depth_paren: i32 = 0;
    let mut depth_bracket: i32 = 0;
    let mut depth_angle: i32 = 0;
    while p < end {
        let b = bytes[p];
        match b {
            b'(' => depth_paren += 1,
            b')' => depth_paren -= 1,
            b'[' => depth_bracket += 1,
            b']' => depth_bracket -= 1,
            b'<' => depth_angle += 1,
            b'>' => depth_angle -= 1,
            b'=' | b';' if depth_paren == 0 && depth_bracket == 0 && depth_angle == 0 => {
                return Some((b, p));
            }
            _ => {}
        }
        p += 1;
    }
    None
}

fn classify_slice_type(ty: &str) -> Option<&'static str> {
    let ty = ty.trim();
    if !ty.starts_with('&') {
        return None;
    }
    let after_amp = ty[1..].trim_start();
    if let Some(rest) = after_amp.strip_prefix("mut") {
        let rest = rest.trim_start();
        if !is_word_break_str(after_amp.as_bytes(), 3) {
            return None;
        }
        if rest.starts_with('[') && rest.ends_with(']') {
            return Some(" = &mut []");
        }
        return None;
    }
    if after_amp.starts_with('[') && after_amp.ends_with(']') {
        return Some(" = &[]");
    }
    None
}

fn skip_ws(bytes: &[u8], mut i: usize, end: usize) -> usize {
    while i < end && bytes[i].is_ascii_whitespace() {
        i += 1;
    }
    i
}

fn is_word_boundary(bytes: &[u8], i: usize) -> bool {
    if i >= bytes.len() {
        return true;
    }
    let b = bytes[i];
    !(b.is_ascii_alphanumeric() || b == b'_')
}

fn is_word_break_str(bytes: &[u8], i: usize) -> bool {
    if i >= bytes.len() {
        return true;
    }
    let b = bytes[i];
    !(b.is_ascii_alphanumeric() || b == b'_')
}
