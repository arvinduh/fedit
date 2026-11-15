//! Canonical character enumeration and slug mapping for Fire Emblem: Awakening.
//!
//! Owns the `Character` domain enum, converting between raw unit identifier
//! numbers, display names, and asset file slugs.

use std::fmt;

/// Canonical character unit identifier for Fire Emblem: Awakening.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Character {
  /// Chrom, Prince of Ylisse.
  Chrom,
  /// Lissa, Princess of Ylisse.
  Lissa,
  /// Frederick, Ylissean knight lieutenant.
  Frederick,
  /// Sully, Shepherd cavalier.
  Sully,
  /// Virion, Archest archer of Rosanne.
  Virion,
  /// Stahl, Shepherd cavalier.
  Stahl,
  /// Vaike, self-proclaimed Chrom rival.
  Vaike,
  /// Miriel, Shepherd mage researcher.
  Miriel,
  /// Sumia, Pegasus knight recruit.
  Sumia,
  /// Kellam, overlooked Shepherd knight.
  Kellam,
  /// Donnel, village farmboy hero.
  Donnel,
  /// Lon'qu, Chon'sin myrmidon.
  LonQu,
  /// Ricken, youthful Shepherd mage.
  Ricken,
  /// Maribelle, noble troubadour.
  Maribelle,
  /// Panne, last taguel shapeshifter.
  Panne,
  /// Gaius, confectionery thief.
  Gaius,
  /// Cordelia, veteran pegasus knight.
  Cordelia,
  /// Gregor, veteran mercenary.
  Gregor,
  /// Nowi, playful manakete.
  Nowi,
  /// Libra, Ylissean war monk priest.
  Libra,
  /// Tharja, Plegian dark mage.
  Tharja,
  /// Anna, traveling merchant.
  Anna,
  /// Olivia, traveling dancer.
  Olivia,
  /// Cherche, wyvern rider of Rosanne.
  Cherche,
  /// Henry, cheerful Plegian dark mage.
  Henry,
  /// Say'ri, Chon'sin swordmaster princess.
  SayRi,
  /// Tiki, Voice of the Divine Dragon Naga.
  Tiki,
  /// Basilio, West-Khan of Regna Ferox.
  Basilio,
  /// Flavia, East-Khan of Regna Ferox.
  Flavia,
  /// Lucina, Foreseer of Future Past.
  Lucina,
  /// Owain, Chosen in Blood and Sword.
  Owain,
  /// Inigo, dancing swordsman.
  Inigo,
  /// Brady, scarred priest.
  Brady,
  /// Kjelle, armored spearwoman.
  Kjelle,
  /// Cynthia, heroic pegasus knight.
  Cynthia,
  /// Severa, sharp-tongued mercenary.
  Severa,
  /// Gerome, masked wyvern rider.
  Gerome,
  /// Morgan (Male), tactician in training.
  MorganM,
  /// Morgan (Female), tactician in training.
  MorganF,
  /// Yarne, timid taguel survivor.
  Yarne,
  /// Laurent, observant mage scholar.
  Laurent,
  /// Noire, reserved archer daughter.
  Noire,
  /// Nah, responsible young manakete.
  Nah,
  /// Gangrel, Mad King of Plegia.
  Gangrel,
  /// Walhart, Conqueror of Valm.
  Walhart,
  /// Emmeryn, Holy Ruler of Ylisse.
  Emmeryn,
  /// Yen'fay, King of Chon'sin.
  YenFay,
  /// Aversa, Grimleal dark flier.
  Aversa,
  /// Priam, Radiant Hero descendant.
  Priam,
  /// Robin (Male), amnesiac tactician avatar.
  RobinM,
  /// Robin (Female), amnesiac tactician avatar.
  RobinF,
  /// Unknown or custom DLC unit identifier.
  Unknown(u16),
}

impl Character {
  /// Converts a raw 16-bit identifier into a [`Character`] variant.
  #[must_use]
  pub const fn from_raw(raw: u16) -> Self {
    match raw {
      1 => Self::Chrom,
      2 => Self::Lissa,
      3 => Self::Frederick,
      4 => Self::Sully,
      5 => Self::Virion,
      6 => Self::Stahl,
      7 => Self::Vaike,
      8 => Self::Miriel,
      9 => Self::Sumia,
      10 => Self::Kellam,
      11 => Self::Donnel,
      12 => Self::LonQu,
      13 => Self::Ricken,
      14 => Self::Maribelle,
      15 => Self::Panne,
      16 => Self::Gaius,
      17 => Self::Cordelia,
      18 => Self::Gregor,
      19 => Self::Nowi,
      20 => Self::Libra,
      21 => Self::Tharja,
      22 => Self::Anna,
      23 => Self::Olivia,
      24 => Self::Cherche,
      25 => Self::Henry,
      26 => Self::SayRi,
      27 => Self::Tiki,
      28 => Self::Basilio,
      29 => Self::Flavia,
      30 => Self::Lucina,
      31 => Self::Owain,
      32 => Self::Inigo,
      33 => Self::Brady,
      34 => Self::Kjelle,
      35 => Self::Cynthia,
      36 => Self::Severa,
      37 => Self::Gerome,
      38 => Self::MorganM,
      39 => Self::MorganF,
      40 => Self::Yarne,
      41 => Self::Laurent,
      42 => Self::Noire,
      43 => Self::Nah,
      44 => Self::Gangrel,
      45 => Self::Walhart,
      46 => Self::Emmeryn,
      47 => Self::YenFay,
      48 => Self::Aversa,
      49 => Self::Priam,
      61 => Self::RobinM,
      62 => Self::RobinF,
      other => Self::Unknown(other),
    }
  }

  /// Converts a [`Character`] variant back into its raw 16-bit identifier.
  #[must_use]
  pub const fn raw_id(self) -> u16 {
    match self {
      Self::Chrom => 1,
      Self::Lissa => 2,
      Self::Frederick => 3,
      Self::Sully => 4,
      Self::Virion => 5,
      Self::Stahl => 6,
      Self::Vaike => 7,
      Self::Miriel => 8,
      Self::Sumia => 9,
      Self::Kellam => 10,
      Self::Donnel => 11,
      Self::LonQu => 12,
      Self::Ricken => 13,
      Self::Maribelle => 14,
      Self::Panne => 15,
      Self::Gaius => 16,
      Self::Cordelia => 17,
      Self::Gregor => 18,
      Self::Nowi => 19,
      Self::Libra => 20,
      Self::Tharja => 21,
      Self::Anna => 22,
      Self::Olivia => 23,
      Self::Cherche => 24,
      Self::Henry => 25,
      Self::SayRi => 26,
      Self::Tiki => 27,
      Self::Basilio => 28,
      Self::Flavia => 29,
      Self::Lucina => 30,
      Self::Owain => 31,
      Self::Inigo => 32,
      Self::Brady => 33,
      Self::Kjelle => 34,
      Self::Cynthia => 35,
      Self::Severa => 36,
      Self::Gerome => 37,
      Self::MorganM => 38,
      Self::MorganF => 39,
      Self::Yarne => 40,
      Self::Laurent => 41,
      Self::Noire => 42,
      Self::Nah => 43,
      Self::Gangrel => 44,
      Self::Walhart => 45,
      Self::Emmeryn => 46,
      Self::YenFay => 47,
      Self::Aversa => 48,
      Self::Priam => 49,
      Self::RobinM => 61,
      Self::RobinF => 62,
      Self::Unknown(id) => id,
    }
  }

  /// Returns the human-readable canonical display name for the character.
  #[must_use]
  pub const fn name(self) -> &'static str {
    match self {
      Self::Chrom => "Chrom",
      Self::Lissa => "Lissa",
      Self::Frederick => "Frederick",
      Self::Sully => "Sully",
      Self::Virion => "Virion",
      Self::Stahl => "Stahl",
      Self::Vaike => "Vaike",
      Self::Miriel => "Miriel",
      Self::Sumia => "Sumia",
      Self::Kellam => "Kellam",
      Self::Donnel => "Donnel",
      Self::LonQu => "Lon'qu",
      Self::Ricken => "Ricken",
      Self::Maribelle => "Maribelle",
      Self::Panne => "Panne",
      Self::Gaius => "Gaius",
      Self::Cordelia => "Cordelia",
      Self::Gregor => "Gregor",
      Self::Nowi => "Nowi",
      Self::Libra => "Libra",
      Self::Tharja => "Tharja",
      Self::Anna => "Anna",
      Self::Olivia => "Olivia",
      Self::Cherche => "Cherche",
      Self::Henry => "Henry",
      Self::SayRi => "Say'ri",
      Self::Tiki => "Tiki",
      Self::Basilio => "Basilio",
      Self::Flavia => "Flavia",
      Self::Lucina => "Lucina",
      Self::Owain => "Owain",
      Self::Inigo => "Inigo",
      Self::Brady => "Brady",
      Self::Kjelle => "Kjelle",
      Self::Cynthia => "Cynthia",
      Self::Severa => "Severa",
      Self::Gerome => "Gerome",
      Self::MorganM => "Morgan (M)",
      Self::MorganF => "Morgan (F)",
      Self::Yarne => "Yarne",
      Self::Laurent => "Laurent",
      Self::Noire => "Noire",
      Self::Nah => "Nah",
      Self::Gangrel => "Gangrel",
      Self::Walhart => "Walhart",
      Self::Emmeryn => "Emmeryn",
      Self::YenFay => "Yen'fay",
      Self::Aversa => "Aversa",
      Self::Priam => "Priam",
      Self::RobinM => "Robin (M)",
      Self::RobinF => "Robin (F)",
      Self::Unknown(_) => "Unknown",
    }
  }

  /// Returns the canonical asset filename slug for character portrait lookups.
  #[must_use]
  pub const fn slug(self) -> &'static str {
    match self {
      Self::Chrom => "chrom",
      Self::Lissa => "lissa",
      Self::Frederick => "frederick",
      Self::Sully => "sully",
      Self::Virion => "virion",
      Self::Stahl => "stahl",
      Self::Vaike => "vaike",
      Self::Miriel => "miriel",
      Self::Sumia => "sumia",
      Self::Kellam => "kellam",
      Self::Donnel => "donnel",
      Self::LonQu => "lon_qu",
      Self::Ricken => "ricken",
      Self::Maribelle => "maribelle",
      Self::Panne => "panne",
      Self::Gaius => "gaius",
      Self::Cordelia => "cordelia",
      Self::Gregor => "gregor",
      Self::Nowi => "nowi",
      Self::Libra => "libra",
      Self::Tharja => "tharja",
      Self::Anna => "anna",
      Self::Olivia => "olivia",
      Self::Cherche => "cherche",
      Self::Henry => "henry",
      Self::SayRi => "say_ri",
      Self::Tiki => "tiki",
      Self::Basilio => "basilio",
      Self::Flavia => "flavia",
      Self::Lucina => "lucina",
      Self::Owain => "owain",
      Self::Inigo => "inigo",
      Self::Brady => "brady",
      Self::Kjelle => "kjelle",
      Self::Cynthia => "cynthia",
      Self::Severa => "severa",
      Self::Gerome => "gerome",
      Self::MorganM => "morgan_m",
      Self::MorganF => "morgan_f",
      Self::Yarne => "yarne",
      Self::Laurent => "laurent",
      Self::Noire => "noire",
      Self::Nah => "nah",
      Self::Gangrel => "gangrel",
      Self::Walhart => "walhart",
      Self::Emmeryn => "emmeryn",
      Self::YenFay => "yen_fay",
      Self::Aversa => "aversa",
      Self::Priam => "priam",
      Self::RobinM => "robin_m",
      Self::RobinF => "robin_f",
      Self::Unknown(_) => "unknown",
    }
  }

  /// Returns true if this character is an essential story anchor.
  ///
  /// Anchor characters cannot be permanently killed or deleted without breaking
  /// core story cutscenes and game-over trigger checks.
  #[must_use]
  pub const fn is_story_anchor(self) -> bool {
    matches!(
      self,
      Self::Chrom | Self::RobinM | Self::RobinF | Self::Lucina
    )
  }
}

impl fmt::Display for Character {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Unknown(id) => write!(f, "Character({id})"),
      _ => write!(f, "{}", self.name()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_character_roundtrip() {
    assert_eq!(Character::from_raw(1), Character::Chrom);
    assert_eq!(Character::Chrom.raw_id(), 1);
    assert_eq!(Character::Chrom.name(), "Chrom");
    assert_eq!(Character::Chrom.slug(), "chrom");
    assert!(Character::Chrom.is_story_anchor());
    assert!(Character::Lucina.is_story_anchor());
    assert!(!Character::Donnel.is_story_anchor());
  }
}
