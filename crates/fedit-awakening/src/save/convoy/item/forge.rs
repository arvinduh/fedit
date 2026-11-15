//! Forged weapon modification parameters and bounds validation.
//!
//! Owns might, hit, crit adjustments and custom weapon naming.

use crate::error;

/// Forged weapon modification parameters.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Forge {
  /// Might modification (-5 to +5 in normal forge, up to +10).
  might: i8,
  /// Hit modification (-15 to +15 in normal forge, up to +30).
  hit: i8,
  /// Critical modification (-15 to +15 in normal forge, up to +30).
  crit: i8,
  /// Optional custom name for the forged weapon.
  custom_name: Option<String>,
}

impl Forge {
  /// Constructs validated forge modifications.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::ForgeOutOfBounds`] if modifications exceed limits.
  pub fn new(
    might: i8,
    hit: i8,
    crit: i8,
    custom_name: Option<String>,
  ) -> Result<Self, error::Error> {
    if !(-5..=10).contains(&might) {
      return Err(error::Error::ForgeOutOfBounds {
        stat: "might",
        value: might,
        min: -5,
        max: 10,
      });
    }
    if !(-15..=30).contains(&hit) {
      return Err(error::Error::ForgeOutOfBounds {
        stat: "hit",
        value: hit,
        min: -15,
        max: 30,
      });
    }
    if !(-15..=30).contains(&crit) {
      return Err(error::Error::ForgeOutOfBounds {
        stat: "crit",
        value: crit,
        min: -15,
        max: 30,
      });
    }
    Ok(Self {
      might,
      hit,
      crit,
      custom_name,
    })
  }

  /// Might modification.
  #[must_use]
  pub const fn might(&self) -> i8 {
    self.might
  }

  /// Hit modification.
  #[must_use]
  pub const fn hit(&self) -> i8 {
    self.hit
  }

  /// Critical modification.
  #[must_use]
  pub const fn crit(&self) -> i8 {
    self.crit
  }

  /// Custom forged weapon name.
  #[must_use]
  pub fn custom_name(&self) -> Option<&str> {
    self.custom_name.as_deref()
  }

  /// Sets might modification with bounds validation `[-5..=10]`.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::ForgeOutOfBounds`] if `might` is outside `[-5, 10]`.
  pub fn set_might(&mut self, might: i8) -> Result<(), error::Error> {
    if !(-5..=10).contains(&might) {
      return Err(error::Error::ForgeOutOfBounds {
        stat: "might",
        value: might,
        min: -5,
        max: 10,
      });
    }
    self.might = might;
    Ok(())
  }

  /// Sets hit modification with bounds validation `[-15..=30]`.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::ForgeOutOfBounds`] if `hit` is outside `[-15, 30]`.
  pub fn set_hit(&mut self, hit: i8) -> Result<(), error::Error> {
    if !(-15..=30).contains(&hit) {
      return Err(error::Error::ForgeOutOfBounds {
        stat: "hit",
        value: hit,
        min: -15,
        max: 30,
      });
    }
    self.hit = hit;
    Ok(())
  }

  /// Sets critical modification with bounds validation `[-15..=30]`.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::ForgeOutOfBounds`] if `crit` is outside `[-15, 30]`.
  pub fn set_crit(&mut self, crit: i8) -> Result<(), error::Error> {
    if !(-15..=30).contains(&crit) {
      return Err(error::Error::ForgeOutOfBounds {
        stat: "crit",
        value: crit,
        min: -15,
        max: 30,
      });
    }
    self.crit = crit;
    Ok(())
  }

  /// Sets or clears the custom forged weapon name.
  pub fn set_custom_name(&mut self, custom_name: Option<String>) {
    self.custom_name = custom_name;
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_forge_mutators() {
    let mut forge = Forge::new(0, 0, 0, None).expect("initial forge");
    assert!(forge.set_might(5).is_ok());
    assert_eq!(forge.might(), 5);
    assert!(forge.set_might(11).is_err());

    assert!(forge.set_hit(20).is_ok());
    assert_eq!(forge.hit(), 20);
    assert!(forge.set_hit(31).is_err());

    assert!(forge.set_crit(15).is_ok());
    assert_eq!(forge.crit(), 15);
    assert!(forge.set_crit(-16).is_err());

    forge.set_custom_name(Some("Blessed Falchion".to_string()));
    assert_eq!(forge.custom_name(), Some("Blessed Falchion"));
    forge.set_custom_name(None);
    assert_eq!(forge.custom_name(), None);
  }

  #[test]
  fn test_forge_bounds() {
    assert!(Forge::new(5, 10, 10, Some("Chrom's Blade".to_string())).is_ok());
    assert!(Forge::new(11, 0, 0, None).is_err());
    assert!(Forge::new(0, 31, 0, None).is_err());
    assert!(Forge::new(0, 0, -16, None).is_err());
  }
}
