//! Pattern rewrite: `IDENT <<< cfg >>> ( args )` → `launch_kernel(...)`.
//!
//! The rewrite is byte-level over a `&str`. For every code region (per
//! `lexer::code_regions`), we scan for the launch pattern and emit replacements.
//! We deliberately avoid using `syn` here because the input is *not* valid Rust
//! yet — that's what we are fixing.
//!
//! Constraints (matching nvcc's chevron behavior):
//! - The chevron payload (between `<<<` and `>>>`) must not contain `<` or `>`
//!   tokens. So `kernel<<<a < b, c>>>(x)` is a syntax error — same as nvcc.
//! - The `<<<` must be three `<` bytes with no whitespace between them, and must
//!   come immediately after a Rust path expression with no whitespace. This rules
//!   out `1 << 2 < 3` from being misread.
//! - After `>>>`, the next non-whitespace byte must be `(`. Inside the call's
//!   parens we count parens to find the matching close — `<`/`>` are fine there.

use super::lexer::{Range, code_regions};

/// Byte span of a chevron-style launch expression in a source file.
/// `start..end` is exclusive; `text` is the verbatim launch expression
/// (e.g. `kernel<<<1, 1>>>(args)`).
#[derive(Debug, Clone)]
pub struct LaunchSpan {
    pub start: usize,
    pub end: usize,
    pub text: String,
}

/// Find every chevron launch expression in `src`. Used by `cuda_rustfmt` to
/// substitute placeholder macro invocations before delegating to rustfmt.
pub fn find_launch_spans(src: &str) -> Vec<LaunchSpan> {
    let regions = code_regions(src);
    let mut launches: Vec<Launch> = Vec::new();
    for region in &regions {
        find_launches_in_region(src, *region, &mut launches);
    }
    launches.sort_by_key(|l| (l.path_start, l.paren_close));
    launches
        .into_iter()
        .map(|l| LaunchSpan {
            start: l.path_start,
            end: l.paren_close + 1,
            text: src[l.path_start..l.paren_close + 1].to_owned(),
        })
        .collect()
}

/// Rewrite `src`. If no launch is found, returns the original string unchanged
/// (caller can avoid touching the cache file in that case).
pub fn rewrite(src: &str) -> String {
    let regions = code_regions(src);
    let mut launches: Vec<Launch> = Vec::new();

    for region in &regions {
        find_launches_in_region(src, *region, &mut launches);
    }

    // The chevron walker can produce overlapping matches when the call-arg span of
    // an outer launch contains a nested launch; deduplicate by start offset and let
    // the outer rewrite swallow the inner.
    launches.sort_by_key(|l| (l.path_start, l.paren_close));

    if launches.is_empty() {
        return src.to_owned();
    }

    apply_rewrites(src, &launches)
}

#[derive(Debug)]
struct Launch {
    /// Byte offset of the start of the path expression (e.g. start of `my_kernel`).
    path_start: usize,
    /// Byte offset of the closing `)` (inclusive end of replacement, exclusive index = +1).
    paren_close: usize,
    /// `cfg_args` is the contents between `<<<` and `>>>` split on top-level commas.
    cfg_args: Vec<String>,
    /// `call_args` is the contents between `(` and `)` (forwarded verbatim, including
    /// internal whitespace, but the surrounding parens are dropped).
    call_args: String,
    /// Bare path text used for the `__cuda_kernel_<name>` handle reference. We strip
    /// any turbofish (`::<…>`) for the handle key — handles are by symbol name only.
    name: String,
}

fn find_launches_in_region(src: &str, region: Range, out: &mut Vec<Launch>) {
    let bytes = src.as_bytes();
    let mut i = region.start;
    while i < region.end {
        // Look for `<<<`
        if i + 2 < region.end && bytes[i] == b'<' && bytes[i + 1] == b'<' && bytes[i + 2] == b'<' {
            // The byte just before must be the end of a path expression (no whitespace).
            // Walk backwards over a path: identifier characters, `::`, and turbofishes
            // `::<…>`. A turbofish is rare immediately before a launch but we handle it.
            if let Some(path_start) = match_path_back(bytes, i, region.start) {
                if let Some(launch) = match_launch_forward(src, bytes, path_start, i, region.end) {
                    let next = launch.paren_close + 1;
                    out.push(launch);
                    i = next;
                    continue;
                }
            }
        }
        i += 1;
    }
}

/// Walk backwards from `i` (the position of the first `<` in `<<<`) and identify
/// the path-expression token directly preceding it. Returns `(start, end)` byte
/// offsets such that `&src[start..end]` is the path. Returns `None` if no path
/// is directly adjacent (e.g. there is whitespace, or it's a numeric literal,
/// or there is no identifier at all).
fn match_path_back(bytes: &[u8], i: usize, lower_bound: usize) -> Option<usize> {
    if i == 0 {
        return None;
    }
    // No whitespace allowed between path and `<<<`.
    let prev = bytes[i - 1];
    if !is_path_tail_byte(prev) {
        return None;
    }

    let mut p = i;
    while p > lower_bound {
        let c = bytes[p - 1];
        if is_path_tail_byte(c) {
            p -= 1;
        } else {
            break;
        }
    }

    if p >= i {
        return None;
    }
    let first = bytes[p];
    if !(first.is_ascii_alphabetic() || first == b'_') {
        return None;
    }

    Some(p)
}

fn is_path_tail_byte(c: u8) -> bool {
    c.is_ascii_alphanumeric() || c == b'_' || c == b':'
}

/// Given that `src[chevron_open..chevron_open+3] == "<<<"`, try to match the
/// rest of the launch pattern: `<<<` cfg `>>>` `(` args `)`. Returns the parsed
/// launch on success.
fn match_launch_forward(src: &str, bytes: &[u8], path_start: usize, chevron_open: usize, region_end: usize) -> Option<Launch> {
    let path_end = chevron_open;
    let cfg_start = chevron_open + 3;

    // Find `>>>`. Inside the chevron payload we forbid `<` and `>` entirely so
    // the close is unambiguous. nvcc enforces the same constraint.
    let mut p = cfg_start;
    let mut cfg_end_excl = None;
    while p + 2 < region_end {
        let b = bytes[p];
        if b == b'<' {
            // Stray `<` in chevron payload → not a launch (or malformed).
            return None;
        }
        if b == b'>' {
            if bytes[p + 1] == b'>' && bytes[p + 2] == b'>' {
                cfg_end_excl = Some(p);
                break;
            }
            // Single `>` not followed by `>>` is not allowed inside chevrons.
            return None;
        }
        p += 1;
    }
    let cfg_end = cfg_end_excl?;
    let chevron_close_end = cfg_end + 3;

    // After `>>>`, skip whitespace, then require `(`.
    let mut q = chevron_close_end;
    while q < region_end && bytes[q].is_ascii_whitespace() {
        q += 1;
    }
    if q >= region_end || bytes[q] != b'(' {
        return None;
    }
    let paren_open = q;
    let paren_close = match_paren(bytes, paren_open, region_end)?;

    let cfg_text = &src[cfg_start..cfg_end];
    let cfg_args = split_top_level_commas(cfg_text);
    if !(2..=4).contains(&cfg_args.len()) {
        return None;
    }

    let call_args = src[paren_open + 1..paren_close].to_owned();

    let path_text = &src[path_start..path_end];
    let name = strip_turbofish_for_handle(path_text);

    let _ = path_end;
    let _ = chevron_open;
    Some(Launch { path_start, paren_close, cfg_args, call_args, name })
}

fn match_paren(bytes: &[u8], open: usize, end: usize) -> Option<usize> {
    debug_assert_eq!(bytes[open], b'(');
    // We need to balance only parens; string/comment regions inside the call are
    // handled correctly because `find_launches_in_region` is called per code region
    // — but the call-args span *can* include strings/comments because we are not
    // re-skipping them here. Walk with a tiny inline lexer to be safe.
    let mut depth: i32 = 0;
    let mut i = open;
    while i < end {
        let b = bytes[i];
        // Skip strings/comments inline. This is a simplified fallback for the
        // call-arg span only; we accept rare false matches (e.g. `>` in a string).
        if b == b'"' {
            i = skip_string(bytes, i, end);
            continue;
        }
        if b == b'\'' && i + 2 < end && bytes[i + 2] == b'\'' {
            i += 3;
            continue;
        }
        if b == b'/' && i + 1 < end && bytes[i + 1] == b'/' {
            while i < end && bytes[i] != b'\n' {
                i += 1;
            }
            continue;
        }
        if b == b'/' && i + 1 < end && bytes[i + 1] == b'*' {
            i += 2;
            let mut depth_c: u32 = 1;
            while i + 1 < end && depth_c > 0 {
                if bytes[i] == b'/' && bytes[i + 1] == b'*' {
                    depth_c += 1;
                    i += 2;
                } else if bytes[i] == b'*' && bytes[i + 1] == b'/' {
                    depth_c -= 1;
                    i += 2;
                } else {
                    i += 1;
                }
            }
            continue;
        }
        if b == b'(' {
            depth += 1;
        } else if b == b')' {
            depth -= 1;
            if depth == 0 {
                return Some(i);
            }
        }
        i += 1;
    }
    None
}

fn skip_string(bytes: &[u8], i: usize, end: usize) -> usize {
    debug_assert_eq!(bytes[i], b'"');
    let mut p = i + 1;
    while p < end {
        match bytes[p] {
            b'\\' if p + 1 < end => p += 2,
            b'"' => return p + 1,
            _ => p += 1,
        }
    }
    end
}

fn split_top_level_commas(s: &str) -> Vec<String> {
    let bytes = s.as_bytes();
    let mut out = Vec::new();
    let mut start = 0usize;
    let mut depth: i32 = 0; // for () [] {}
    let mut i = 0usize;
    while i < bytes.len() {
        let b = bytes[i];
        match b {
            b'(' | b'[' | b'{' => depth += 1,
            b')' | b']' | b'}' => depth -= 1,
            b',' if depth == 0 => {
                out.push(s[start..i].trim().to_owned());
                start = i + 1;
            }
            _ => {}
        }
        i += 1;
    }
    out.push(s[start..].trim().to_owned());
    out.into_iter().filter(|p| !p.is_empty()).collect()
}

fn strip_turbofish_for_handle(path: &str) -> String {
    // `foo::bar::<T>` → `foo::bar`. We keep the path components for the
    // handle-static reference but drop generic args.
    if let Some(idx) = path.find("::<") {
        return path[..idx].to_owned();
    }
    path.to_owned()
}

fn apply_rewrites(src: &str, launches: &[Launch]) -> String {
    // launches are already in source order because we walked once forward.
    let mut out = String::with_capacity(src.len() + launches.len() * 64);
    let mut cursor = 0usize;

    for launch in launches {
        out.push_str(&src[cursor..launch.path_start]);
        let replacement = render_launch(src, launch);
        out.push_str(&replacement);
        cursor = launch.paren_close + 1;
    }

    out.push_str(&src[cursor..]);
    out
}

fn render_launch(src: &str, launch: &Launch) -> String {
    // The replacement spans from `path_start` to `paren_close + 1`. We need to
    // keep the source's line count identical between the original span and the
    // replacement so rustc diagnostics still line up. We do this by counting
    // newlines in the original span and appending that many trailing newlines
    // to the rendered replacement (those newlines go inside a synthetic block,
    // safely ignored by the parser).
    let original_span = &src[launch.path_start..launch.paren_close + 1];
    let original_newlines = original_span.bytes().filter(|b| *b == b'\n').count();

    // The same chevron call site appears in both the host crate (where the
    // kernel body has been extracted away and only `__cuda_kernel_<name>`
    // exists) and the device crate (where the kernel function is defined as a
    // regular item but `__cuda_kernel_<name>` does not exist). We emit two
    // cfg-gated arms; rustc strips the wrong arm before name resolution, so
    // each side only sees the symbols it can actually resolve.
    let handle_path = handle_static_path(&launch.name);
    let path_text = &src[launch.path_start..launch.path_start + launch.name.len()];
    // Use the original path text (including any `::` qualifiers) so device-side
    // resolution preserves the user's module structure.
    let _ = path_text;

    let grid_expr = format!("::cuda_rustc_runtime::launch::Dim3::from({})", launch.cfg_args[0]);
    let block_expr = format!("::cuda_rustc_runtime::launch::Dim3::from({})", launch.cfg_args[1]);
    let shmem_expr = launch.cfg_args.get(2).cloned().unwrap_or_else(|| "0u32".to_owned());
    let stream_expr = launch.cfg_args.get(3).cloned().unwrap_or_else(|| "::core::ptr::null_mut()".to_owned());

    let args_array = render_args_array(&launch.call_args);
    let device_call = render_device_launch(launch, &grid_expr, &block_expr, &shmem_expr, &stream_expr);

    let mut s = format!(
        "{{ \
            #[cfg(not(target_arch = \"nvptx64\"))] \
            {{ ::cuda_rustc_runtime::launch::launch_kernel(&{handle_path}, {grid_expr}, {block_expr}, ({shmem_expr}) as u32, ({stream_expr}) as ::cuda_rustc_runtime::launch::CUstream, {args_array}); }} \
            #[cfg(target_arch = \"nvptx64\")] \
            {{ {device_call} }} \
        }}"
    );

    for _ in 0..original_newlines {
        s.push('\n');
    }
    s
}

/// Render the device-side arm of the chevron rewrite. Builds a packed
/// parameter buffer from the call-site arg expressions and dispatches via
/// `cudaLaunchDevice` (CUDA dynamic parallelism).
///
/// Args are placed into a Rust tuple and the tuple's bytes are forwarded to
/// `launch_kernel_device`. This works because both host and device crates are
/// compiled by the same rustc, so kernel signatures see the same Rust ABI on
/// both ends. Note that `cudaLaunchDevice` expects PTX-ABI laid-out args, which
/// differ from Rust tuple layout; a `#[repr(C)]` parameter-buffer shim per
/// kernel signature is needed for non-trivial dyn-parallelism arg lists.
fn render_device_launch(launch: &Launch, grid: &str, block: &str, shmem: &str, stream: &str) -> String {
    let path = &launch.name;
    let parts = split_top_level_commas(launch.call_args.trim());

    if parts.is_empty() {
        return format!("let _ = unsafe {{ ::cuda_rustc_runtime::launch::launch_kernel_device({path} as *mut ::core::ffi::c_void, {grid}, {block}, ({shmem}) as u32, ({stream}) as ::cuda_rustc_runtime::launch::CUstream, &[]) }};");
    }

    let mut tuple = String::from("(");
    for (i, p) in parts.iter().enumerate() {
        if i > 0 {
            tuple.push_str(", ");
        }
        tuple.push_str(p);
    }
    if parts.len() == 1 {
        tuple.push(',');
    }
    tuple.push(')');

    format!(
        "let __cuda_args = {tuple}; \
         let __cuda_arg_bytes = unsafe {{ ::core::slice::from_raw_parts(&__cuda_args as *const _ as *const u8, ::core::mem::size_of_val(&__cuda_args)) }}; \
         let _ = unsafe {{ ::cuda_rustc_runtime::launch::launch_kernel_device({path} as *mut ::core::ffi::c_void, {grid}, {block}, ({shmem}) as u32, ({stream}) as ::cuda_rustc_runtime::launch::CUstream, __cuda_arg_bytes) }};"
    )
}

fn handle_static_path(path: &str) -> String {
    // The kernel name *is* the handle name. The AST stage replaces the
    // original `#[global] fn name(...)` item with `pub static name:
    // KernelHandle`, so the chevron site references the same path the user
    // wrote — no mangling, and `use crate::kernels::print_arr` brings the
    // handle into scope.
    path.to_string()
}

fn render_args_array(call_args: &str) -> String {
    let trimmed = call_args.trim();
    if trimmed.is_empty() {
        return "&[]".to_owned();
    }
    let parts = split_top_level_commas(trimmed);
    let mut s = String::from("&[");
    for (i, p) in parts.iter().enumerate() {
        if i > 0 {
            s.push_str(", ");
        }
        // Take a pointer to the value. The user's argument expression is forwarded
        // through a temporary to capture its address.
        s.push_str("&{ ");
        s.push_str(p);
        s.push_str(" } as *const _ as *const ::core::ffi::c_void");
    }
    s.push(']');
    s
}
