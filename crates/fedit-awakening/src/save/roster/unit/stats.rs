//! Combat attributes and character statistics.
//!
//! Owns hit point maximums and combat attributes, validating numerical ranges
//! and stat-cap boundaries.

use crate::error;

/// Combat attributes and character statistics.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UnitStats {
  /// Maximum hit points.
  pub hp: u8,
  /// Physical strength stat.
  pub strength: u8,
  /// Magic stat.
  pub magic: u8,
  /// Skill stat.
  pub skill: u8,
  /// Speed stat.
  pub speed: u8,
  /// Luck stat.
  pub luck: u8,
  /// Physical defense stat.
  pub defense: u8,
  /// Magical resistance stat.
  pub resistance: u8,
}

impl UnitStats {
  /// Sets hit points with bounds checking `[10..=99]`.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::StatOutOfBounds`] if `val` is outside `[10, 99]`.
  pub fn set_hp(&mut self, val: u8) -> Result<(), error::Error> {
    if !(10..=99).contains(&val) {
      return Err(error::Error::StatOutOfBounds {
        stat: "hp",
        value: val,
        min: 10,
        max: 99,
      });
    }
    self.hp = val;
    Ok(())
  }

  /// Sets combat stat value with upper bound validation `(0..=80)`.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::StatOutOfBounds`] if `val > 80`.
  pub fn set_combat_stat(
    &mut self,
    stat: &'static str,
    val: u8,
  ) -> Result<(), error::Error> {
    if val > 80 {
      return Err(error::Error::StatOutOfBounds {
        stat,
        value: val,
        min: 0,
        max: 80,
      });
    }
    match stat {
      "strength" => self.strength = val,
      "magic" => self.magic = val,
      "skill" => self.skill = val,
      "speed" => self.speed = val,
      "luck" => self.luck = val,
      "defense" => self.defense = val,
      "resistance" => self.resistance = val,
      _ => {}
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_unit_stats_combat_stat_bounds() {
    let mut stats = UnitStats::default();
    assert!(stats.set_combat_stat("strength", 81).is_err());
    assert!(stats.set_combat_stat("strength", 40).is_ok());
    assert_eq!(stats.strength, 40);
  }

  #[test]
  fn test_unit_stats_hp_bounds() {
    let mut stats = UnitStats::default();
    assert!(stats.set_hp(9).is_err());
    assert!(stats.set_hp(100).is_err());
    assert!(stats.set_hp(45).is_ok());
    assert_eq!(stats.hp, 45);
  }
}
