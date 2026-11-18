//! Character support relationships and marriage invariants.
//!
//! Owns support rank levels and the army-wide support table, enforcing the
//! single S-rank (marriage) invariant and partnership compatibility.

use std::collections;
use std::fmt;

use thiserror;

use crate::save::roster::unit::character;

/// Errors that can occur when modifying support table ranks.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  /// Characters cannot support each other.
  #[error("characters '{a}' and '{b}' cannot support each other")]
  IncompatibleSupportPair {
    /// First character.
    a: character::Character,
    /// Second character.
    b: character::Character,
  },
  /// Character is already married to another partner.
  #[error("character '{character}' is already married to '{spouse}'")]
  AlreadyMarried {
    /// Target character.
    character: character::Character,
    /// Existing spouse.
    spouse: character::Character,
  },
}

/// Canonical support affinity ranks between character pairs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SupportRank {
  /// No active support rank.
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
struct PairKey(character::Character, character::Character);

impl PairKey {
  /// Creates a canonical unordered pair key.
  fn new(a: character::Character, b: character::Character) -> Self {
    if a <= b { Self(a, b) } else { Self(b, a) }
  }
}

/// Army-wide support tracking table enforcing marriage and affinity rules.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SupportTable {
  /// Stored pairing ranks.
  ranks: collections::HashMap<PairKey, SupportRank>,
  /// Raw backing slice for the 62UD payload.
  raw: Vec<u8>,
}

impl SupportTable {
  /// Deserializes a support table from a 62UD payload slice.
  #[must_use]
  pub(crate) fn from_bytes(slice: &[u8]) -> Self {
    let mut table = Self {
      ranks: collections::HashMap::new(),
      raw: slice.to_vec(),
    };
    if slice.len() >= 14 && &slice[8..12] == b"SUPP" {
      let count = usize::from(u16::from_le_bytes([slice[12], slice[13]]));
      let mut offset = 14;
      for _ in 0..count {
        if offset + 5 <= slice.len() {
          let id_a = u16::from_le_bytes([slice[offset], slice[offset + 1]]);
          let id_b = u16::from_le_bytes([slice[offset + 2], slice[offset + 3]]);
          let rank_val = slice[offset + 4];
          let rank = match rank_val {
            1 => SupportRank::C,
            2 => SupportRank::B,
            3 => SupportRank::A,
            4 => SupportRank::S,
            _ => SupportRank::None,
          };
          let char_a = character::Character::from_raw(id_a);
          let char_b = character::Character::from_raw(id_b);
          let _ = table.set_rank(char_a, char_b, rank);
          offset += 5;
        }
      }
    }
    table
  }

  /// Serializes the support table back into a 62UD payload slice.
  pub(crate) fn write_back(&self, slice: &mut [u8]) {
    let copy_len = self.raw.len().min(slice.len());
    slice[..copy_len].copy_from_slice(&self.raw[..copy_len]);

    if !self.ranks.is_empty() && slice.len() >= 14 {
      slice[8..12].copy_from_slice(b"SUPP");
      let count = u16::try_from(self.ranks.len()).unwrap_or(u16::MAX);
      slice[12..14].copy_from_slice(&count.to_le_bytes());
      let mut offset = 14;
      for (key, &rank) in &self.ranks {
        if offset + 5 <= slice.len() {
          slice[offset..offset + 2]
            .copy_from_slice(&key.0.raw_id().to_le_bytes());
          slice[offset + 2..offset + 4]
            .copy_from_slice(&key.1.raw_id().to_le_bytes());
          slice[offset + 4] = rank as u8;
          offset += 5;
        }
      }
    }
  }

  /// Returns the current support rank between two characters.
  #[must_use]
  pub fn rank_between(
    &self,
    a: character::Character,
    b: character::Character,
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
    character: character::Character,
  ) -> Option<character::Character> {
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
  ///   [`Error::AlreadyMarried`].
  ///
  /// # Errors
  ///
  /// - Returns [`Error::IncompatibleSupportPair`] if `a == b`.
  /// - Returns [`Error::AlreadyMarried`] if setting rank S and either unit
  ///   is already married to another character.
  pub fn set_rank(
    &mut self,
    a: character::Character,
    b: character::Character,
    rank: SupportRank,
  ) -> Result<(), Error> {
    if a == b {
      return Err(Error::IncompatibleSupportPair { a, b });
    }

    if rank == SupportRank::S {
      if let Some(spouse) = self.spouse_of(a)
        && spouse != b
      {
        return Err(Error::AlreadyMarried {
          character: a,
          spouse,
        });
      }
      if let Some(spouse) = self.spouse_of(b)
        && spouse != a
      {
        return Err(Error::AlreadyMarried {
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
    let mut table = SupportTable::from_bytes(&[]);
    let chrom = character::Character::Chrom;
    let sumia = character::Character::Sumia;
    let robin = character::Character::RobinF;

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
      Error::AlreadyMarried { character, spouse } => {
        assert_eq!(character, chrom);
        assert_eq!(spouse, sumia);
      }
      other @ Error::IncompatibleSupportPair { .. } => {
        panic!("expected AlreadyMarried error, got: {other:?}");
      }
    }

    // Attempting self-support fails
    assert!(table.set_rank(chrom, chrom, SupportRank::A).is_err());
  }

  #[test]
  fn test_support_table_payload_roundtrip() {
    let mut table = SupportTable::from_bytes(&[]);
    let chrom = character::Character::Chrom;
    let sumia = character::Character::Sumia;
    assert!(table.set_rank(chrom, sumia, SupportRank::S).is_ok());

    let mut buf = vec![0u8; 100];
    table.write_back(&mut buf);

    let reloaded = SupportTable::from_bytes(&buf);
    assert_eq!(reloaded.rank_between(chrom, sumia), SupportRank::S);
    assert_eq!(reloaded.spouse_of(chrom), Some(sumia));
  }
}
