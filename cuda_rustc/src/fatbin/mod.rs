//! Fatbin packager.
//!
//! Produces a minimal fatbin payload: NUL-terminated PTX text. The CUDA driver's
//! `cuModuleLoadData` accepts three input shapes — a real fatbin blob (magic
//! `0xBA55ED50`), an ELF cubin, or PTX text — so a NUL-terminated PTX string is
//! a valid payload that the runtime can load directly. The blob is placed in the
//! host binary's `.nv_fatbin` section.

use std::path::Path;

/// Wrap a PTX text payload as a fatbin blob. NUL-terminates so
/// `cuModuleLoadData` recognizes it as PTX.
pub fn package_ptx(ptx: &[u8]) -> Vec<u8> {
    let mut out = Vec::with_capacity(ptx.len() + 1);
    out.extend_from_slice(ptx);
    if out.last().copied() != Some(0u8) {
        out.push(0u8);
    }
    out
}

/// Write the packaged fatbin to disk if the contents differ from what's already
/// there. Returns the final path the bytes live at.
pub fn write_blob(path: &Path, bytes: &[u8]) -> std::io::Result<()> {
    if let Ok(existing) = std::fs::read(path)
        && existing == bytes
    {
        return Ok(());
    }
    std::fs::write(path, bytes)
}
