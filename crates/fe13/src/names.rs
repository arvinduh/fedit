//! Canonical entity name lookups for Fire Emblem: Awakening.
//!
//! Provides static strings for character, class, and item identifiers.

/// Returns the canonical character name for a unit ID.
#[must_use]
pub const fn character_name(id: u16) -> Option<&'static str> {
  match id {
    1 => Some("Chrom"),
    2 => Some("Lissa"),
    3 => Some("Frederick"),
    4 => Some("Sully"),
    5 => Some("Virion"),
    6 => Some("Stahl"),
    7 => Some("Vaike"),
    8 => Some("Miriel"),
    9 => Some("Sumia"),
    10 => Some("Kellam"),
    11 => Some("Donnel"),
    12 => Some("Lon'qu"),
    13 => Some("Ricken"),
    14 => Some("Maribelle"),
    15 => Some("Panne"),
    16 => Some("Gaius"),
    17 => Some("Cordelia"),
    18 => Some("Gregor"),
    19 => Some("Nowi"),
    20 => Some("Libra"),
    21 => Some("Tharja"),
    22 => Some("Anna"),
    23 => Some("Olivia"),
    24 => Some("Cherche"),
    25 => Some("Henry"),
    _ => None,
  }
}
