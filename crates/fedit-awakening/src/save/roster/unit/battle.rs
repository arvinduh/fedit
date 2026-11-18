//! Mid-map battle deployment state and combat grid coordinates.
//!
//! Encapsulates live battle attributes including current hit points,
//! map placement grid coordinates, and pair-up partner coordinates.

/// Active battle state for deployed player, enemy, and allied combatants.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct BattleState {
  /// Current hit points during battle.
  pub current_hp: u8,
  /// Permanent boots movement bonus attribute stored at offset 0x15.
  pub boots_movement: u8,
  /// Battle map grid coordinates `[x, y]`.
  pub coordinates: [u8; 2],
  /// Pair-up partner position coordinates `[x, y]`.
  pub pair_up_coordinates: [u8; 2],
}

impl BattleState {
  /// Constructs a new [`BattleState`] record with specified coordinates.
  #[must_use]
  pub const fn new(
    current_hp: u8,
    boots_movement: u8,
    coordinates: [u8; 2],
    pair_up_coordinates: [u8; 2],
  ) -> Self {
    Self {
      current_hp,
      boots_movement,
      coordinates,
      pair_up_coordinates,
    }
  }
}
