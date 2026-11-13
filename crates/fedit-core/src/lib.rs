//! Save archive management, automated backups, and atomic persistence.
//!
//! Owns directory-level multi-slot save discovery, pre-mutation backup
//! snapshots, and atomic file serialization.

pub mod archive;
pub mod atomic;
pub mod backup;
pub mod error;

pub use archive::SaveArchive;
pub use atomic::write_atomic;
pub use backup::BackupManager;
pub use error::Error;
