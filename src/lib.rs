//! # azathoth_utils
//!
//! A small collection of utilities shared across the AzathothC2 ecosystem.
//!
//! ## Crate goals
//! - `no_std` by default (uses `alloc` where needed).
//! - Opt-in modules via Cargo features to keep binary size and surface area small.
//!
//! ## Cargo features
//! - **`hasher`** – Identifier/symbol hashing helpers.
//! - **`formatter`** – Lightweight formatting helpers for constrained environments.
//! - **`psearch`** – Extendable pattern search utilities over byte slices.
//! - **`codec`** – Minimal data encoding/decoding helpers.
//!
//! Each feature gates its corresponding module. Modules are excluded from the
//! build unless their feature is enabled.
//!
//! ## Examples
//! Computing a CRC32 checksum:
//! ```no_run
//! use azathoth_utils::crc32;
//!
//! let c = crc32(b"deadbeef");
//! assert_eq!(c, 0x52_8f_6f_ca); // value will remain stable given the same table
//! ```
#![no_std]

extern crate alloc;

/// Error types used by azathoth utilities.
///
/// This module is always available and provides error enums and aliases shared
/// by other feature-gated modules in this crate.
pub mod errors;

#[cfg(feature="hasher")]
/// Identifier and symbol hashing helpers.
///
/// Typical use-cases include obfuscated or stable hash-based lookups where
/// string literals are undesirable at runtime.
pub mod hasher;

#[cfg(feature="formatter")]
/// Lightweight formatting helpers.
///
/// Useful in `no_std` contexts where calling the `alloc` crate formatter and related functions can cause crashes
pub mod formatter;

#[cfg(feature="psearch")]
/// Extendable pattern search utilities over byte slices.
///
/// Provides building blocks for scanning memory regions with optional wildcard support
pub mod psearch;

#[cfg(feature = "codec")]
/// Minimal data encoding/decoding helpers.
pub mod codec;

/// Compute a CRC32 checksum over `data`.
///
/// This implementation uses the precomputed `azathoth_core::CRC32_TABLE`.
///
/// # Examples
/// ```no_run
/// use azathoth_utils::crc32;
///
/// assert_eq!(crc32(b""), 0xFFFF_FFFFu32 ^ 0xFFFF_FFFFu32); // CRC32 of empty input
/// assert_eq!(crc32(b"123456789"), 0xCBF43926);
/// ```
#[inline(always)]
pub fn crc32(data: impl AsRef<[u8]>) -> u32 {
    let mut crc: u32 = 0xFFFF_FFFF;
    let table = azathoth_core::CRC32_TABLE;

    for &byte in data.as_ref() {
        let index = ((crc ^ byte as u32) & 0xFF) as usize;
        crc = (crc >> 8) ^ table[index];
    }
    !crc
}
