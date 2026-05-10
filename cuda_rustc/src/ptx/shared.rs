//! Rewrite the `.global` state space to `.shared` for marker-tagged statics.
//!
//! Pipeline:
//! 1. Scan the PTX for `.global` (or `.visible .global`, `.extern .global`)
//!    declarations whose symbol name contains the `__cuda_shared_` substring
//!    (the AST extractor pinned the LLVM symbol via `export_name`). Build a
//!    set of marker symbol names. Rewrite each declaration's `.global` keyword
//!    to `.shared`. Drop `.visible` (shared variables are local to the SM and
//!    cannot be `.visible`).
//! 2. Walk the body of each function. Maintain a per-function "shared-derived
//!    register set" — registers that hold the address of, or an offset into,
//!    one of the marker symbols. Seed the set from `mov.{u64,b64} %rN, SYM;`
//!    instructions where `SYM` is in the marker set. Propagate through
//!    address-arithmetic instructions (`add.s64`, `add.u64`, `mul.wide`,
//!    `cvta.to.global` on the same register, etc.).
//! 3. Rewrite every `(ld|st|atom|red).global.<type>` instruction whose address
//!    operand uses a shared-derived register, or whose immediate addressing
//!    mode points directly at a marker symbol. Change `.global` → `.shared`.
//!
//! The function-body walk is brace-delimited and does not attempt to follow
//! control flow. `mov`/`add` instructions can occur in any order with respect
//! to their uses, but in practice the LLVM backend always defines a register
//! before using it within a basic block, so a single forward pass suffices.

use std::collections::BTreeSet;

const MARKER_SUBSTR: &str = "__cuda_shared_";
const DYN_MARKER_SUBSTR: &str = "__cuda_shared_dyn_";

pub fn rewrite_shared(ptx: &str) -> String {
    let lines: Vec<&str> = ptx.lines().collect();

    let marker_symbols = collect_marker_symbols(&lines);
    if marker_symbols.is_empty() {
        return ptx.to_owned();
    }

    let mut out_lines: Vec<String> = Vec::with_capacity(lines.len());
    let mut shared_regs: BTreeSet<String> = BTreeSet::new();
    let mut brace_depth: i32 = 0;

    for raw in &lines {
        let prev_depth = brace_depth;
        brace_depth += count_char(raw, '{') as i32 - count_char(raw, '}') as i32;

        if prev_depth == 0 && brace_depth > 0 {
            shared_regs.clear();
        }

        let new_line = if prev_depth == 0 {
            // Top-level declaration line: maybe rewrite `.global` to `.shared`.
            rewrite_global_decl(raw, &marker_symbols)
        } else {
            // Inside function body.
            rewrite_body_line(raw, &marker_symbols, &mut shared_regs)
        };
        out_lines.push(new_line);
    }

    let mut joined = out_lines.join("\n");
    if ptx.ends_with('\n') {
        joined.push('\n');
    }
    joined
}

fn count_char(s: &str, c: char) -> usize {
    s.chars().filter(|x| *x == c).count()
}

/// Collect the set of PTX symbols whose name contains the marker substring and
/// that appear in a `.global` declaration. Symbols that *only* appear as
/// references (no declaration) are ignored — they were declared elsewhere and
/// belong to a different translation unit.
fn collect_marker_symbols(lines: &[&str]) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in lines {
        let trimmed = line.trim();
        if !trimmed.contains(".global") {
            continue;
        }
        if !trimmed.contains(MARKER_SUBSTR) {
            continue;
        }
        if let Some(sym) = decl_symbol_name(trimmed)
            && sym.contains(MARKER_SUBSTR)
        {
            out.insert(sym);
        }
    }
    out
}

/// Pull the symbol name out of a PTX global-variable declaration line. Handles:
///   .global .align N .Ttype SYMBOL[size];
///   .global .align N .Ttype SYMBOL;
///   .visible .global .align N .Ttype SYMBOL[size];
///   .extern .global .align N .Ttype SYMBOL[size];
fn decl_symbol_name(line: &str) -> Option<String> {
    let line = line.trim_end_matches(';').trim();
    let last = line.split_whitespace().last()?;
    let bare = last.split('[').next().unwrap_or(last);
    let bare = bare.trim_end_matches(',');
    if bare.is_empty() {
        return None;
    }
    Some(bare.to_owned())
}

fn rewrite_global_decl(line: &str, markers: &BTreeSet<String>) -> String {
    let trimmed = line.trim_start();
    if !trimmed.contains(".global") {
        return line.to_owned();
    }
    let sym = match decl_symbol_name(trimmed) {
        Some(s) if markers.contains(&s) => s,
        _ => return line.to_owned(),
    };

    let leading_len = line.len() - trimmed.len();
    let leading = &line[..leading_len];

    // Dynamic shared marker: zero-sized array static whose symbol carries the
    // `__cuda_shared_dyn_` prefix. Emit `.extern .shared … SYM[]` rather than
    // `.shared … SYM[0]` so the CUDA runtime allocates the per-launch dynamic
    // region for it (sized by the `<<<…, shmem_bytes>>>` chevron arg). The
    // unsized `[]` form is what nvcc emits for `extern __shared__` and is the
    // only way ptxas/nvlink will treat it as JIT-resolved per-launch rather
    // than as an undefined external symbol.
    if sym.contains(DYN_MARKER_SUBSTR) {
        let mut tail = trimmed.to_owned();
        tail = tail.replacen(".visible ", "", 1);
        // Strip any pre-existing `.extern` to avoid `.extern .extern`.
        tail = tail.replacen(".extern ", "", 1);
        tail = tail.replacen(".global", ".extern .shared", 1);
        // Replace the trailing `[<digits>];` (or absent brackets entirely)
        // with `[];`.
        tail = ensure_unsized_brackets(&tail);
        return format!("{leading}{tail}");
    }

    // Static shared: `.global … SYM[N];` → `.shared … SYM[N];`.
    let mut tail = trimmed.to_owned();
    tail = tail.replacen(".visible ", "", 1);
    tail = tail.replacen(".global", ".shared", 1);
    format!("{leading}{tail}")
}

/// Ensure the declaration ends with `[];` instead of `[<digits>];` or just
/// `;`. rustc's nvptx backend emits zero-sized arrays as scalar declarations
/// (no `[]` suffix at all), so we need to inject the unsized brackets in that
/// case. ptxas/nvlink only treat `.extern .shared` as JIT-resolved per-launch
/// when the array is unsized.
fn ensure_unsized_brackets(line: &str) -> String {
    // Case A: `[<digits>];` already present — strip the digits.
    if let Some(open) = line.rfind('[')
        && let Some(close_rel) = line[open..].find(']')
    {
        let close = open + close_rel;
        let inner = &line[open + 1..close];
        if inner.bytes().all(|b| b.is_ascii_digit()) {
            return format!("{}[]{}", &line[..open], &line[close + 1..]);
        }
    }

    // Case B: no brackets at all — insert `[]` before the trailing `;`.
    let trimmed_end = line.trim_end();
    let suffix = &line[trimmed_end.len()..];
    if let Some(stripped) = trimmed_end.strip_suffix(';') {
        return format!("{stripped}[];{suffix}");
    }

    // Fallback: append `[]` at the end (no semicolon to anchor against).
    format!("{line}[]")
}

fn rewrite_body_line(line: &str, markers: &BTreeSet<String>, shared_regs: &mut BTreeSet<String>) -> String {
    let trimmed = line.trim_start();

    // 1. Track register definitions that produce shared-derived addresses.
    if let Some((dst, src)) = parse_mov(trimmed)
        && markers.contains(src)
    {
        shared_regs.insert(dst.to_owned());
        return line.to_owned();
    }
    if let Some((dst, srcs)) = parse_add_or_arith(trimmed)
        && srcs.iter().any(|s| shared_regs.contains(*s))
    {
        shared_regs.insert(dst.to_owned());
        return line.to_owned();
    }
    if let Some((dst, src)) = parse_unary(trimmed)
        && shared_regs.contains(src)
    {
        // cvta / mov / and-with-imm forms that just relabel the register.
        shared_regs.insert(dst.to_owned());
        return line.to_owned();
    }

    // 2. Memory ops with `.global` whose address operand references a marker
    //    (either by symbol or via a shared-derived register).
    if let Some(rewritten) = rewrite_mem_op(line, markers, shared_regs) {
        return rewritten;
    }

    line.to_owned()
}

/// Parse `mov.{type} %dst, src;` where `src` may be a register or a symbol.
fn parse_mov(line: &str) -> Option<(&str, &str)> {
    let line = line.trim_end_matches(';').trim();
    let mut parts = line.splitn(2, char::is_whitespace);
    let opc = parts.next()?;
    if !opc.starts_with("mov.") {
        return None;
    }
    let rest = parts.next()?.trim();
    let mut operands = rest.splitn(2, ',');
    let dst = operands.next()?.trim();
    let src = operands.next()?.trim();
    if !dst.starts_with('%') {
        return None;
    }
    Some((dst, src))
}

/// Parse `OP.{type} %dst, %s1, %s2;` for binary integer ops that propagate
/// addresses (add, sub, mul.wide, mul, mad.wide, mad, or.b64, and.b64, xor.b64).
fn parse_add_or_arith(line: &str) -> Option<(&str, Vec<&str>)> {
    let line = line.trim_end_matches(';').trim();
    let mut parts = line.splitn(2, char::is_whitespace);
    let opc = parts.next()?;
    let propagating = ["add.", "sub.", "mul.", "mad.", "or.", "and.", "xor.", "shl.", "shr."];
    if !propagating.iter().any(|p| opc.starts_with(p)) {
        return None;
    }
    let rest = parts.next()?.trim();
    let operands: Vec<&str> = rest.splitn(4, ',').map(|s| s.trim()).collect();
    if operands.len() < 2 {
        return None;
    }
    let dst = operands[0];
    if !dst.starts_with('%') {
        return None;
    }
    let srcs: Vec<&str> = operands[1..].iter().filter(|s| s.starts_with('%')).copied().collect();
    Some((dst, srcs))
}

/// Parse single-source instructions like `cvta.to.global.u64 %d, %s;`.
fn parse_unary(line: &str) -> Option<(&str, &str)> {
    let line = line.trim_end_matches(';').trim();
    let mut parts = line.splitn(2, char::is_whitespace);
    let opc = parts.next()?;
    let single_src_propagators = ["cvta.", "cvt."];
    if !single_src_propagators.iter().any(|p| opc.starts_with(p)) {
        return None;
    }
    let rest = parts.next()?.trim();
    let mut operands = rest.splitn(2, ',');
    let dst = operands.next()?.trim();
    let src = operands.next()?.trim();
    if !dst.starts_with('%') || !src.starts_with('%') {
        return None;
    }
    Some((dst, src))
}

/// If this line is a `.global`-tagged memory op whose address operand reaches a
/// shared-derived register or marker symbol, return a rewritten line with
/// `.global` replaced by `.shared`. Otherwise return `None`.
fn rewrite_mem_op(line: &str, markers: &BTreeSet<String>, shared_regs: &BTreeSet<String>) -> Option<String> {
    let trimmed = line.trim_start();
    let leading_len = line.len() - trimmed.len();
    let leading = &line[..leading_len];

    let kinds = ["ld.global", "st.global", "atom.global", "red.global"];
    let kind_match = kinds.iter().find(|k| trimmed.starts_with(*k))?;

    let addr_operand = extract_bracketed_addr(trimmed)?;
    let (sym_or_reg, _offset) = parse_addr_operand(addr_operand);

    let touches_marker = markers.contains(sym_or_reg) || shared_regs.contains(sym_or_reg);
    if !touches_marker {
        return None;
    }

    let new_kind = kind_match.replace(".global", ".shared");
    let rewritten_tail = trimmed.replacen(kind_match, &new_kind, 1);
    Some(format!("{leading}{rewritten_tail}"))
}

/// Pull the bracketed address operand out of e.g. `ld.global.b32 %r, [%rd9];`.
fn extract_bracketed_addr(line: &str) -> Option<&str> {
    let lb = line.find('[')?;
    let rb = line[lb..].find(']')? + lb;
    Some(&line[lb + 1..rb])
}

/// Parse `[SYM+8]` / `[%rd9+8]` / `[%rd9]` / `[SYM]` into (base, offset_str).
fn parse_addr_operand(operand: &str) -> (&str, Option<&str>) {
    let operand = operand.trim();
    if let Some(idx) = operand.find('+') {
        return (operand[..idx].trim(), Some(operand[idx + 1..].trim()));
    }
    (operand, None)
}
