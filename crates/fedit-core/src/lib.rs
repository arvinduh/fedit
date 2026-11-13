//! Save archive management, automated backups, and atomic persistence.
//!
//! Owns directory-level multi-slot save discovery, pre-mutation backup
//! snapshots, and atomic file serialization.

pub mod error;

pub use error::Error;
