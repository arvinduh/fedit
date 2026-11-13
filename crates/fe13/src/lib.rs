//! Fire Emblem: Awakening save domain models and binary structures.
//!
//! Owns save slot preview headers, EDNI block indexing, user state, unit
//! roster, and convoy storage. Lower layers handle raw container decompression;
//! higher layers handle process and file I/O.

#![deny(clippy::print_stdout, clippy::print_stderr)]

pub mod convoy;
pub mod error;
pub mod index;
pub mod preview;
pub mod slot;
pub mod text;
pub mod types;
pub mod unit;
pub mod user;

pub use convoy::ConvoyMut;
pub use convoy::ConvoyRef;
pub use error::Error;
pub use index::EdniIndex;
pub use preview::PreviewMut;
pub use preview::PreviewRef;
pub use slot::SaveSlot;
pub use types::ClassId;
pub use types::Difficulty;
pub use types::GameMode;
pub use types::ItemId;
pub use types::UnitId;
pub use unit::UnitIter;
pub use unit::UnitRef;
pub use user::MAX_GOLD;
pub use user::UserMut;
pub use user::UserRef;
