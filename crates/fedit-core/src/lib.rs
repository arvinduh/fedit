//! Save archive management, automated backups, and atomic persistence.
//!
//! Owns directory-level multi-slot save discovery, pre-mutation backup
//! snapshots, and atomic file serialization.

pub mod atomic;
pub mod backup;
pub mod error;

pub use atomic::write_atomic;
pub use error::Error;
