//! Fire Emblem: Awakening save domain models and binary structures.
//!
//! Owns save slot preview headers, EDNI block indexing, user state, unit
//! roster, and convoy storage. Lower layers handle raw container decompression;
//! higher layers handle process and file I/O.

#![deny(clippy::print_stdout, clippy::print_stderr)]
