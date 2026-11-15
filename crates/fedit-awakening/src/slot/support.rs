//! Character support relationships and marriage invariants.
//!
//! Owns support rank levels and the army-wide support table, enforcing the
//! single S-rank (marriage) invariant and partnership compatibility.

use std::collections;
use std::fmt;

use crate::error;
use crate::slot::roster;

/// Canonical support affinity ranks between character pairs.
#[derive(Clone, Copy, Debug, Default, Eq, Ord, PartialEq, PartialOrd)]
pub enum SupportRank {
  /// No active support rank.
  #[default]
  None = 0,
  /// Rank C support.
  C = 1,
  /// Rank B support.
  B = 2,
  /// Rank A support.
  A = 3,
  /// Rank S support (marriage / romantic union).
  S = 4,
}

impl fmt::Display for SupportRank {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::None => write!(f, "None"),
      Self::C => write!(f, "C"),
      Self::B => write!(f, "B"),
      Self::A => write!(f, "A"),
      Self::S => write!(f, "S"),
    }
  }
}

/// Canonical unordered character partnership key.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct PairKey(roster::Character, roster::Character);

impl PairKey {
  /// Creates a canonical unordered pair key.
  fn new(a: roster::Character, b: roster::Character) -> Self {
    if a.raw_id() <= b.raw_id() {
      Self(a, b)
    } else {
      Self(b, a)
    }
  }
}

/// Army-wide support tracking table enforcing marriage and affinity rules.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SupportTable {
  /// Stored pairing ranks.
  ranks: collections::HashMap<PairKey, SupportRank>,
}

impl SupportTable {
  /// Creates an empty support tracking table.
  #[must_use]
  pub fn new() -> Self {
    Self {
      ranks: collections::HashMap::new(),
    }
  }

  /// Returns the current support rank between two characters.
  #[must_use]
  pub fn rank_between(
    &self,
    a: roster::Character,
    b: roster::Character,
  ) -> SupportRank {
    if a == b {
      return SupportRank::None;
    }
    self
      .ranks
      .get(&PairKey::new(a, b))
      .copied()
      .unwrap_or(SupportRank::None)
  }

  /// Returns the current spouse (S-support partner) of a character, if any.
  #[must_use]
  pub fn spouse_of(
    &self,
    character: roster::Character,
  ) -> Option<roster::Character> {
    for (key, &rank) in &self.ranks {
      if rank == SupportRank::S {
        if key.0 == character {
          return Some(key.1);
        } else if key.1 == character {
          return Some(key.0);
        }
      }
    }
    None
  }

  /// Sets the support rank between two characters.
  ///
  /// # Invariants Enforced
  ///
  /// - A character cannot support themselves.
  /// - **Single Marriage Invariant**: A character can have at most one active S-rank.
  ///   Attempting to set an S-rank when either partner is already married returns
  ///   [`error::Error::AlreadyMarried`].
  ///
  /// # Errors
  ///
  /// - Returns [`error::Error::IncompatibleSupportPair`] if `a == b`.
  /// - Returns [`error::Error::AlreadyMarried`] if setting rank S and either unit
  ///   is already married to another character.
  pub fn set_rank(
    &mut self,
    a: roster::Character,
    b: roster::Character,
    rank: SupportRank,
  ) -> Result<(), error::Error> {
    if a == b {
      return Err(error::Error::IncompatibleSupportPair { a, b });
    }

    if rank == SupportRank::S {
      if let Some(spouse) = self.spouse_of(a)
        && spouse != b
      {
        return Err(error::Error::AlreadyMarried {
          character: a,
          spouse,
        });
      }
      if let Some(spouse) = self.spouse_of(b)
        && spouse != a
      {
        return Err(error::Error::AlreadyMarried {
          character: b,
          spouse,
        });
      }
    }

    let key = PairKey::new(a, b);
    if rank == SupportRank::None {
      self.ranks.remove(&key);
    } else {
      self.ranks.insert(key, rank);
    }
    Ok(())
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_single_marriage_invariant() {
    let mut table = SupportTable::new();
    let chrom = roster::Character::Chrom;
    let sumia = roster::Character::Sumia;
    let robin = roster::Character::RobinF;

    assert_eq!(table.rank_between(chrom, sumia), SupportRank::None);
    assert_eq!(table.spouse_of(chrom), None);

    // Set rank S between Chrom and Sumia
    assert!(table.set_rank(chrom, sumia, SupportRank::S).is_ok());
    assert_eq!(table.rank_between(chrom, sumia), SupportRank::S);
    assert_eq!(table.spouse_of(chrom), Some(sumia));
    assert_eq!(table.spouse_of(sumia), Some(chrom));

    // Attempting to set rank S between Chrom and Robin fails with AlreadyMarried
    let err = table.set_rank(chrom, robin, SupportRank::S);
    assert!(err.is_err());
    match err.unwrap_err() {
      error::Error::AlreadyMarried { character, spouse } => {
        assert_eq!(character, chrom);
        assert_eq!(spouse, sumia);
      }
      other => panic!("expected AlreadyMarried error, got: {other:?}"),
    }

    // Attempting self-support fails
    assert!(table.set_rank(chrom, chrom, SupportRank::A).is_err());
  }
}
