//! Canonical character enumeration and slug mapping for Fire Emblem: Awakening.
//!
//! Owns the `Character` domain enum, converting between raw unit identifier
//! numbers, display names, and asset file slugs.

use std::fmt;

/// Canonical character unit identifier for Fire Emblem: Awakening.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
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
  /// Morgan, tactician in training.
  Morgan,
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
  /// Robin, amnesiac tactician avatar.
  Robin,
  /// Unknown or custom DLC unit identifier.
  Unknown(u16),
}

impl Character {
  /// Converts a raw 16-bit identifier into a [`Character`] variant.
  #[must_use]
  pub const fn from_raw(raw: u16) -> Self {
    match raw {
      0 | 1 => Self::Robin,
      3 => Self::Chrom,
      4 => Self::Lissa,
      5 => Self::Frederick,
      6 => Self::Virion,
      7 => Self::Sully,
      8 => Self::Vaike,
      9 => Self::Stahl,
      10 => Self::Miriel,
      11 => Self::Kellam,
      12 => Self::Sumia,
      13 => Self::LonQu,
      14 => Self::Ricken,
      15 => Self::Maribelle,
      16 => Self::Panne,
      17 => Self::Gaius,
      18 => Self::Cordelia,
      19 => Self::Gregor,
      20 => Self::Nowi,
      21 => Self::Libra,
      22 => Self::Tharja,
      23 => Self::Olivia,
      24 => Self::Cherche,
      25 => Self::Henry,
      26 => Self::Lucina,
      27 => Self::SayRi,
      28 => Self::Basilio,
      29 => Self::Flavia,
      30 => Self::Donnel,
      31 => Self::Anna,
      32 => Self::Owain,
      33 => Self::Inigo,
      34 => Self::Brady,
      35 => Self::Kjelle,
      36 => Self::Cynthia,
      37 => Self::Severa,
      38 => Self::Gerome,
      39 | 40 => Self::Morgan,
      41 => Self::Yarne,
      42 => Self::Laurent,
      43 => Self::Noire,
      44 => Self::Nah,
      45 => Self::Tiki,
      46 => Self::Gangrel,
      47 => Self::Walhart,
      48 => Self::Emmeryn,
      49 => Self::YenFay,
      50 => Self::Aversa,
      51 => Self::Priam,
      other => Self::Unknown(other),
    }
  }

  /// Converts a raw 16-bit identifier into a [`Character`] variant and its encoded [`Gender`].
  ///
  /// The returned [`Gender`] represents the unit's own concrete biological gender decoded
  /// directly from the raw game save identifier (e.g. 0 -> Robin Male, 1 -> Robin Female,
  /// 39 -> Morgan Male, 40 -> Morgan Female).
  #[must_use]
  pub const fn from_raw_with_gender(raw: u16) -> (Self, Gender) {
    match raw {
      0 => (Self::Robin, Gender::Male),
      1 => (Self::Robin, Gender::Female),
      39 => (Self::Morgan, Gender::Male),
      40 => (Self::Morgan, Gender::Female),
      other => {
        let ch = Self::from_raw(other);
        let g = ch.default_gender();
        (ch, g)
      }
    }
  }

  /// Converts a [`Character`] variant back into its raw 16-bit identifier if fixed.
  ///
  /// Returns `None` for variable-gender characters (Robin and Morgan), whose raw IDs
  /// differ by gender (0/1 and 39/40). Use [`Character::raw_id_for_avatar_gender`]
  /// or [`Unit::raw_id`].
  #[must_use]
  pub const fn raw_id(self) -> Option<u16> {
    match self {
      Self::Robin | Self::Morgan => None,
      Self::Chrom => Some(3),
      Self::Lissa => Some(4),
      Self::Frederick => Some(5),
      Self::Virion => Some(6),
      Self::Sully => Some(7),
      Self::Vaike => Some(8),
      Self::Stahl => Some(9),
      Self::Miriel => Some(10),
      Self::Kellam => Some(11),
      Self::Sumia => Some(12),
      Self::LonQu => Some(13),
      Self::Ricken => Some(14),
      Self::Maribelle => Some(15),
      Self::Panne => Some(16),
      Self::Gaius => Some(17),
      Self::Cordelia => Some(18),
      Self::Gregor => Some(19),
      Self::Nowi => Some(20),
      Self::Libra => Some(21),
      Self::Tharja => Some(22),
      Self::Olivia => Some(23),
      Self::Cherche => Some(24),
      Self::Henry => Some(25),
      Self::Lucina => Some(26),
      Self::SayRi => Some(27),
      Self::Basilio => Some(28),
      Self::Flavia => Some(29),
      Self::Donnel => Some(30),
      Self::Anna => Some(31),
      Self::Owain => Some(32),
      Self::Inigo => Some(33),
      Self::Brady => Some(34),
      Self::Kjelle => Some(35),
      Self::Cynthia => Some(36),
      Self::Severa => Some(37),
      Self::Gerome => Some(38),
      Self::Yarne => Some(41),
      Self::Laurent => Some(42),
      Self::Noire => Some(43),
      Self::Nah => Some(44),
      Self::Tiki => Some(45),
      Self::Gangrel => Some(46),
      Self::Walhart => Some(47),
      Self::Emmeryn => Some(48),
      Self::YenFay => Some(49),
      Self::Aversa => Some(50),
      Self::Priam => Some(51),
      Self::Unknown(id) => Some(id),
    }
  }

  /// Converts a [`Character`] variant and the avatar's (Robin's) biological gender
  /// into its exact raw identifier.
  ///
  /// For Robin, the avatar's gender is used directly (Male -> 0, Female -> 1).
  /// For Morgan, their biological gender is strictly the inverse of the avatar's
  /// gender (Avatar Male -> Morgan Female 40; Avatar Female -> Morgan Male 39).
  /// For all other characters, returns their fixed raw identifier.
  #[must_use]
  pub const fn raw_id_for_avatar_gender(self, avatar_gender: Gender) -> u16 {
    match (self, avatar_gender) {
      (Self::Robin, Gender::Male) => 0,
      (Self::Robin, Gender::Female) => 1,
      (Self::Morgan, Gender::Male) => 40,
      (Self::Morgan, Gender::Female) => 39,
      _ => match self.raw_id() {
        Some(id) => id,
        None => 0,
      },
    }
  }

  /// Returns the immutable biological gender if fixed, or None for avatar/Morgan.
  #[must_use]
  pub const fn fixed_gender(self) -> Option<Gender> {
    match self {
      Self::Robin | Self::Morgan => None,
      _ => Some(self.default_gender()),
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
      Self::Morgan => "Morgan",
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
      Self::Robin => "Robin",
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
      Self::Morgan => "morgan",
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
      Self::Robin => "robin",
      Self::Unknown(_) => "unknown",
    }
  }

  /// Returns true if this character is an essential story anchor.
  ///
  /// Anchor characters cannot be permanently killed or deleted without breaking
  /// core story cutscenes and game-over trigger checks.
  #[must_use]
  pub const fn is_story_anchor(self) -> bool {
    matches!(self, Self::Chrom | Self::Robin | Self::Lucina)
  }

  /// Returns true if this character is a mandatory force-deploy in main chapters.
  #[must_use]
  pub const fn is_force_deployed(self) -> bool {
    matches!(self, Self::Chrom | Self::Robin)
  }

  /// Returns the default or fixed biological gender of the character.
  #[must_use]
  pub const fn default_gender(self) -> Gender {
    match self {
      Self::Lissa
      | Self::Sully
      | Self::Miriel
      | Self::Sumia
      | Self::Maribelle
      | Self::Panne
      | Self::Cordelia
      | Self::Nowi
      | Self::Tharja
      | Self::Anna
      | Self::Olivia
      | Self::Cherche
      | Self::SayRi
      | Self::Tiki
      | Self::Flavia
      | Self::Lucina
      | Self::Kjelle
      | Self::Cynthia
      | Self::Severa
      | Self::Morgan
      | Self::Noire
      | Self::Nah
      | Self::Emmeryn
      | Self::Aversa => Gender::Female,
      _ => Gender::Male,
    }
  }

  /// Returns the inherent biological gender of the character.
  #[must_use]
  pub const fn gender(self) -> Gender {
    self.default_gender()
  }

  /// Returns the gender assignment rule for this character.
  #[must_use]
  pub const fn gender_rule(self) -> GenderRule {
    match self {
      Self::Robin => GenderRule::Avatar,
      Self::Morgan => GenderRule::OppositeAvatar,
      other => GenderRule::Fixed(other.default_gender()),
    }
  }

  /// Canonical main story chapter (0..=26) where this character is recruited.
  #[must_use]
  pub const fn recruitment_chapter(self) -> Option<u8> {
    match self {
      Self::Chrom | Self::Robin | Self::Lissa | Self::Frederick => Some(0),
      Self::Sully | Self::Virion => Some(1),
      Self::Stahl | Self::Vaike => Some(2),
      Self::Miriel | Self::Kellam => Some(3),
      Self::Maribelle | Self::Ricken => Some(5),
      Self::Panne => Some(6),
      Self::Cordelia => Some(7),
      Self::Gregor | Self::Nowi => Some(8),
      Self::Libra | Self::Tharja | Self::Gaius => Some(9),
      Self::Olivia => Some(11),
      Self::Cherche => Some(12),
      Self::Henry | Self::Lucina => Some(13),
      Self::SayRi => Some(15),
      Self::Basilio | Self::Flavia => Some(23),
      _ => None,
    }
  }
}

/// Inherent biological gender of a character unit.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Gender {
  /// Male biological gender.
  Male,
  /// Female biological gender.
  Female,
}

impl Gender {
  /// Returns the opposite biological gender.
  #[must_use]
  pub const fn opposite(self) -> Self {
    match self {
      Self::Male => Self::Female,
      Self::Female => Self::Male,
    }
  }
}

/// Gender assignment rule for a character.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum GenderRule {
  /// Biologically fixed gender.
  Fixed(Gender),
  /// Defined dynamically by Robin's avatar customization.
  Avatar,
  /// Invariant: strictly the opposite of Robin's gender.
  OppositeAvatar,
}

impl fmt::Display for Gender {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Male => write!(f, "Male"),
      Self::Female => write!(f, "Female"),
    }
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
    assert_eq!(Character::from_raw(3), Character::Chrom);
    assert_eq!(Character::Chrom.raw_id(), Some(3));
    assert_eq!(Character::Robin.raw_id(), None);
    assert_eq!(Character::Morgan.raw_id(), None);
    assert_eq!(Character::Chrom.name(), "Chrom");
    assert_eq!(Character::Chrom.slug(), "chrom");
    assert!(Character::Chrom.is_story_anchor());
    assert!(Character::Lucina.is_story_anchor());
    assert!(!Character::Donnel.is_story_anchor());

    assert_eq!(Character::from_raw(0), Character::Robin);
    assert_eq!(Character::from_raw(1), Character::Robin);
    assert_eq!(Character::from_raw(39), Character::Morgan);
    assert_eq!(Character::from_raw(40), Character::Morgan);
    assert_eq!(Character::Robin.raw_id_for_avatar_gender(Gender::Male), 0);
    assert_eq!(Character::Robin.raw_id_for_avatar_gender(Gender::Female), 1);
    assert_eq!(Character::Morgan.raw_id_for_avatar_gender(Gender::Male), 40);
    assert_eq!(
      Character::Morgan.raw_id_for_avatar_gender(Gender::Female),
      39
    );
    assert_eq!(Character::Chrom.raw_id_for_avatar_gender(Gender::Male), 3);
    assert_eq!(Character::Robin.gender_rule(), GenderRule::Avatar);
    assert_eq!(Character::Morgan.gender_rule(), GenderRule::OppositeAvatar);
  }
}
