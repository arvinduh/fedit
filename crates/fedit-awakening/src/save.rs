//! Save slot campaign structures, unit roster, convoy storage, and support tracking.
//!
//! Provides root abstractions for deserializing, modifying, and serializing
//! save slot archives and nested subsystems.

pub mod avatar;
pub mod convoy;
pub mod index;
pub mod paralogue;
pub mod preview;
pub mod roster;
pub mod slot;
pub mod support;
pub mod text;
pub mod time;
pub mod types;
pub mod user;

pub use avatar::AvatarCustomization;
pub use paralogue::ParalogueState;
pub use slot::SaveSlot;
