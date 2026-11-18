//! Support affinity relationships and paralogue progression.
//!
//! Owns social mechanics including army support tables, marriage invariants,
//! and world map child paralogue unlocking lifecycle.

pub(crate) mod paralogue;
pub(crate) mod support;

pub use paralogue::Error as ParalogueError;
pub use paralogue::ParalogueState;
pub use support::Error as SupportError;
pub use support::SupportRank;
pub use support::SupportTable;
