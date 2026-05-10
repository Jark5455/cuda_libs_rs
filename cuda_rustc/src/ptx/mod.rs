//! PTX text post-processor.
//!
//! The Rust nvptx backend has no first-class way to put a `static mut` in CUDA
//! shared memory — every static lands in the `.global` state space. This module
//! walks the LLVM-generated PTX text after the device build and rewrites the
//! `.global` storage class to `.shared` for symbols carrying our
//! `__cuda_shared_` marker prefix, plus every load/store/atomic operation that
//! transitively addresses one of those symbols.

mod shared;

pub use shared::rewrite_shared;
