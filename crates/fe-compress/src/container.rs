//! PMOC archive container data structures.
//!
//! Owns container kind definitions and container payload structures.

/// Structural variant of a PMOC save container.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ContainerKind {
  /// Slot save with 192-byte preview header.
  Slot,
  /// Global save without preview header.
  Global,
}

/// An unpacked PMOC archive container.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Container {
  /// The container kind.
  pub kind: ContainerKind,
  /// Preview header bytes (192 bytes for Slot, empty for Global).
  pub preview: Vec<u8>,
  /// Decompressed payload bytes.
  pub payload: Vec<u8>,
}
