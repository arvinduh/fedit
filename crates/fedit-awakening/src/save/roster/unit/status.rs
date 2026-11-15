//! Operational combat and deployment status of a unit.
//!
//! Tracks unit combat states (active, retreated, or permadeath) and army
//! availability for battle deployment.

/// Operational combat and deployment status of a unit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UnitStatus {
  /// Unit is actively deployed and available for battle.
  Active,
  /// Unit was downed in battle and retreated from combat.
  Retreated,
  /// Unit fell in battle on Classic mode (permadeath).
  Dead,
}

/// High-level combat deployment availability status.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Availability {
  /// Unit is active and available for battle deployment.
  Active,
  /// Unit is incapacitated, fallen, or retreated from combat.
  Inactive,
}
