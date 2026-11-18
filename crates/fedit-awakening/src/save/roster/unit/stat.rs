//! Combat stat attribute values and bounds.
//!
//! Encapsulates a single in-memory combat attribute with its base, cap, and
//! save growth EV.

use thiserror;

/// Upper limit for combat stats, weapon might, and movement across the engine.
pub const UNIVERSAL_STAT_CEILING: u8 = 99;

/// Maximum safe boots bonus: Paladin base 8 + 91 = 99 max movement.
pub const MAX_SAFE_BOOTS_BONUS: u8 = 91;

/// Identifies the eight combat attribute kinds.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum StatKind {
  /// Maximum hit points.
  Hp,
  /// Physical strength.
  Strength,
  /// Magic.
  Magic,
  /// Skill and accuracy.
  Skill,
  /// Speed and evasion.
  Speed,
  /// Luck.
  Luck,
  /// Physical defense.
  Defense,
  /// Magical resistance.
  Resist,
}

impl StatKind {
  /// All eight combat stat kinds in canonical in-game order.
  pub const ALL: [Self; 8] = [
    Self::Hp,
    Self::Strength,
    Self::Magic,
    Self::Skill,
    Self::Speed,
    Self::Luck,
    Self::Defense,
    Self::Resist,
  ];
}

/// Errors that can occur when modifying stat values.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  /// Stat value was outside the valid range.
  #[error("stat {stat:?} value {value} is out of bounds [{min}..={max}]")]
  StatOutOfBounds {
    /// Target combat stat kind.
    stat: StatKind,
    /// Provided value.
    value: u8,
    /// Minimum allowed value.
    min: u8,
    /// Maximum allowed value.
    max: u8,
  },
}

/// A 4-byte combat attribute maintaining base, cap, and growth EV.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Stat {
  /// Combat stat kind.
  pub kind: StatKind,
  /// Fixed base attribute from character additions and class base.
  pub base: u8,
  /// Vanilla stat cap from character modifiers and class max.
  pub cap: u8,
  /// Accumulated level-up effort value stored in the save file.
  pub growth: u8,
}

impl Stat {
  /// Constructs a new stat with the given kind, base, cap, and growth EV.
  #[must_use]
  pub const fn new(kind: StatKind, base: u8, cap: u8, growth: u8) -> Self {
    Self {
      kind,
      base,
      cap,
      growth,
    }
  }

  /// Calculates the total active stat: `min(base + growth, 99)`.
  #[must_use]
  #[expect(
    clippy::cast_possible_truncation,
    reason = "bounded by ceiling check"
  )]
  pub const fn total(&self) -> u8 {
    let t = self.base as u16 + self.growth as u16;
    if t > UNIVERSAL_STAT_CEILING as u16 {
      UNIVERSAL_STAT_CEILING
    } else {
      t as u8
    }
  }

  /// Returns true if the total stat meets or exceeds the vanilla class cap.
  #[must_use]
  pub const fn is_capped(&self) -> bool {
    self.total() >= self.cap
  }

  /// Sets the total stat value, adjusting growth EV accordingly.
  ///
  /// # Errors
  ///
  /// Returns [`Error::StatOutOfBounds`] if `target < base` or `target > 99`.
  pub fn set_total(&mut self, target: u8) -> Result<(), Error> {
    if target < self.base || target > UNIVERSAL_STAT_CEILING {
      return Err(Error::StatOutOfBounds {
        stat: self.kind,
        value: target,
        min: self.base,
        max: UNIVERSAL_STAT_CEILING,
      });
    }
    self.growth = target - self.base;
    Ok(())
  }

  /// Sets the growth EV points directly with ceiling bounds checking.
  ///
  /// # Errors
  ///
  /// Returns [`Error::StatOutOfBounds`] if `base + growth > 99`.
  pub fn set_growth(&mut self, growth: u8) -> Result<(), Error> {
    let total = u16::from(self.base) + u16::from(growth);
    if total > u16::from(UNIVERSAL_STAT_CEILING) {
      let bounded_val = u8::try_from(total.min(255)).unwrap_or(255);
      return Err(Error::StatOutOfBounds {
        stat: self.kind,
        value: bounded_val,
        min: self.base,
        max: UNIVERSAL_STAT_CEILING,
      });
    }
    self.growth = growth;
    Ok(())
  }

  /// Preserves growth EV on reclassing by clamping against the new base.
  pub fn clamp_on_reclass(&mut self, new_base: u8, new_cap: u8) {
    self.base = new_base;
    self.cap = new_cap;
    let max_growth = UNIVERSAL_STAT_CEILING.saturating_sub(new_base);
    if self.growth > max_growth {
      self.growth = max_growth;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_stat_calculations_and_bounds() {
    let mut stat = Stat::new(StatKind::Strength, 20, 40, 10);
    assert_eq!(stat.total(), 30);
    assert!(!stat.is_capped());

    assert!(stat.set_total(40).is_ok());
    assert_eq!(stat.growth, 20);
    assert_eq!(stat.total(), 40);
    assert!(stat.is_capped());

    // Below base fails
    assert!(stat.set_total(15).is_err());
    // Above 99 fails
    assert!(stat.set_total(100).is_err());
  }

  #[test]
  fn test_stat_reclass_clamping() {
    let mut stat = Stat::new(StatKind::Strength, 20, 40, 75);
    assert_eq!(stat.total(), 95);

    // Reclass to class with base 30: max growth is 99 - 30 = 69
    stat.clamp_on_reclass(30, 45);
    assert_eq!(stat.growth, 69);
    assert_eq!(stat.total(), 99);
  }
}
