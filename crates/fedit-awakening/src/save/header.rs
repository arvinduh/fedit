//! Save slot header representations: EDNI index, preview header, and user state.
//!
//! Owns the uncompressed preview header, master block offset index table,
//! and campaign user records.

pub(crate) mod config;
pub(crate) mod edni;
pub(crate) mod preview;
pub(crate) mod user;

pub use config::Difficulty;
pub use config::GameMode;
pub use preview::PREVIEW_LEN;
pub use preview::PreviewMut;
pub use preview::PreviewRef;
pub use user::UserRef;
