//! Nintendo 3DS Fire Emblem compression and container codecs.
//!
//! Owns the Nintendo 0x28 Huffman codec, PMOC container header, and CRC-32
//! checksums. Higher layers own game-specific save structures and file I/O.

#![deny(clippy::print_stdout, clippy::print_stderr)]

pub mod error;
pub mod pmoc;
pub use error::Error;
