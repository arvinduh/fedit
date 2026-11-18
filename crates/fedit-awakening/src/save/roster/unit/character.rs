//! Canonical character enumeration and slug mapping for Fire Emblem: Awakening.
//!
//! Owns the `Character` domain enum, converting between raw unit identifier
//! numbers, display names, and asset file slugs.

use std::fmt;

/// Canonical character unit identifier for Fire Emblem: Awakening.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum Character {
  /// Robin (Male) avatar tactician.
  RobinM,
  /// Robin (Female) avatar tactician.
  RobinF,
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
  /// Morgan (Male) tactician in training.
  MorganM,
  /// Morgan (Female) tactician in training.
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
  /// Unknown or custom DLC unit identifier.
  Unknown(u16),
}

impl Character {
  /// Canonical character cast identifier.
  #[must_use]
  pub const fn id(self) -> u16 {
    self.raw_id()
  }

  /// Resolves a character from its canonical identifier.
  #[must_use]
  pub const fn from_id(id: u16) -> Self {
    Self::from_raw(id)
  }

  /// Converts a raw 16-bit identifier into a [`Character`] variant.
  #[must_use]
  pub(crate) const fn from_raw(raw: u16) -> Self {
    match raw {
      0 => Self::RobinM,
      1 => Self::RobinF,
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
      39 => Self::MorganM,
      40 => Self::MorganF,
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

  /// Converts a [`Character`] variant back into its exact raw 16-bit identifier.
  #[must_use]
  pub(crate) const fn raw_id(self) -> u16 {
    match self {
      Self::RobinM => 0,
      Self::RobinF => 1,
      Self::Chrom => 3,
      Self::Lissa => 4,
      Self::Frederick => 5,
      Self::Virion => 6,
      Self::Sully => 7,
      Self::Vaike => 8,
      Self::Stahl => 9,
      Self::Miriel => 10,
      Self::Kellam => 11,
      Self::Sumia => 12,
      Self::LonQu => 13,
      Self::Ricken => 14,
      Self::Maribelle => 15,
      Self::Panne => 16,
      Self::Gaius => 17,
      Self::Cordelia => 18,
      Self::Gregor => 19,
      Self::Nowi => 20,
      Self::Libra => 21,
      Self::Tharja => 22,
      Self::Olivia => 23,
      Self::Cherche => 24,
      Self::Henry => 25,
      Self::Lucina => 26,
      Self::SayRi => 27,
      Self::Basilio => 28,
      Self::Flavia => 29,
      Self::Donnel => 30,
      Self::Anna => 31,
      Self::Owain => 32,
      Self::Inigo => 33,
      Self::Brady => 34,
      Self::Kjelle => 35,
      Self::Cynthia => 36,
      Self::Severa => 37,
      Self::Gerome => 38,
      Self::MorganM => 39,
      Self::MorganF => 40,
      Self::Yarne => 41,
      Self::Laurent => 42,
      Self::Noire => 43,
      Self::Nah => 44,
      Self::Tiki => 45,
      Self::Gangrel => 46,
      Self::Walhart => 47,
      Self::Emmeryn => 48,
      Self::YenFay => 49,
      Self::Aversa => 50,
      Self::Priam => 51,
      Self::Unknown(id) => id,
    }
  }

  /// Returns true if this character variant is Robin (Male or Female).
  #[must_use]
  pub const fn is_robin(self) -> bool {
    match self {
      Self::RobinM | Self::RobinF => true,
      _ => false,
    }
  }

  /// Returns true if this character variant is Morgan (Male or Female).
  #[must_use]
  pub const fn is_morgan(self) -> bool {
    match self {
      Self::MorganM | Self::MorganF => true,
      _ => false,
    }
  }

  /// Returns the inherent biological gender of the character.
  #[must_use]
  pub const fn gender(self) -> Gender {
    match self {
      Self::RobinF
      | Self::MorganF
      | Self::Lissa
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
      | Self::Noire
      | Self::Nah
      | Self::Emmeryn
      | Self::Aversa => Gender::Female,
      _ => Gender::Male,
    }
  }

  /// Returns the human-readable canonical display name for the character.
  #[must_use]
  pub const fn name(self) -> &'static str {
    match self {
      Self::RobinM | Self::RobinF => "Robin",
      Self::MorganM | Self::MorganF => "Morgan",
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
      Self::Unknown(_) => "Unknown",
    }
  }

  /// Returns the canonical asset filename slug for character portrait lookups.
  #[must_use]
  pub const fn slug(self) -> &'static str {
    match self {
      Self::RobinM => "robin_m",
      Self::RobinF => "robin_f",
      Self::MorganM => "morgan_m",
      Self::MorganF => "morgan_f",
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
      Self::Unknown(_) => "unknown",
    }
  }

  /// Returns true if this character is an essential story anchor.
  ///
  /// Anchor characters cannot be permanently killed or deleted without breaking
  /// core story cutscenes and game-over trigger checks.
  #[must_use]
  pub const fn is_story_anchor(self) -> bool {
    match self {
      Self::Chrom | Self::RobinM | Self::RobinF | Self::Lucina => true,
      _ => false,
    }
  }

  /// Returns true if this character is a mandatory force-deploy in main chapters.
  #[must_use]
  pub const fn is_force_deployed(self) -> bool {
    match self {
      Self::Chrom | Self::RobinM | Self::RobinF => true,
      _ => false,
    }
  }

  /// Returns true if this character represents a chapter story boss whose defeat trips event flags.
  #[must_use]
  pub const fn is_story_boss(self) -> bool {
    match self {
      Self::Gangrel | Self::Walhart | Self::Aversa | Self::YenFay => true,
      _ => false,
    }
  }

  /// Returns true if this character begins as an enemy but can be recruited mid-battle.
  #[must_use]
  pub const fn is_recruitable_enemy(self) -> bool {
    match self {
      Self::Tharja | Self::Gaius => true,
      _ => false,
    }
  }

  /// Returns true if this character represents a protected friendly NPC whose loss triggers game over.
  #[must_use]
  pub const fn is_protected_story_npc(self) -> bool {
    match self {
      Self::Emmeryn | Self::Tiki => true,
      _ => false,
    }
  }

  /// Canonical main story chapter (0..=26) where this character is recruited.
  #[must_use]
  pub const fn recruitment_chapter(self) -> Option<u8> {
    match self {
      Self::Chrom
      | Self::RobinM
      | Self::RobinF
      | Self::Lissa
      | Self::Frederick => Some(0),
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

  /// Character base stat additions (HP, Str, Mag, Skl, Spd, Lck, Def, Res).
  #[must_use]
  pub const fn additions(self) -> [i8; 8] {
    match self {
      Self::RobinM | Self::RobinF => [3, 2, 2, 0, 1, 4, 1, 1],
      Self::MorganM | Self::MorganF => [9, 6, 8, 7, 6, 7, 3, 7],
      Self::Chrom => [2, 1, 1, 3, 1, 5, 0, 1],
      Self::Lissa => [1, 1, 2, 2, 0, 8, 2, -2],
      Self::Frederick => [2, 2, 2, 6, 5, 6, 0, 2],
      Self::Virion => [3, 1, 0, 1, -1, 7, 1, 1],
      Self::Sully => [2, 1, 1, 3, 2, 6, 0, 2],
      Self::Vaike => [4, 1, 0, 3, 1, 4, 1, 0],
      Self::Stahl => [4, 2, 0, 2, 0, 5, 1, 1],
      Self::Miriel => [2, 0, 2, 2, 3, 6, 1, 1],
      Self::Kellam => [3, 2, 0, 3, 3, 3, 1, 2],
      Self::Sumia => [2, 2, 1, 4, 3, 8, 1, 1],
      Self::LonQu => [4, 2, 0, 3, 3, 7, 3, 1],
      Self::Ricken => [4, 3, 4, 3, 1, 10, 4, 0],
      Self::Maribelle => [2, 0, 2, 2, 1, 5, 2, 1],
      Self::Panne => [10, 6, 1, 5, 5, 8, 4, 1],
      Self::Gaius => [6, 4, 0, 7, 7, 6, 3, 2],
      Self::Cordelia => [9, 5, 1, 6, 4, 9, 4, 2],
      Self::Gregor => [12, 7, 0, 5, 4, 8, 5, 2],
      Self::Nowi => [0, 2, 0, 1, 2, 8, 0, 0],
      Self::Libra => [14, 9, 10, 8, 7, 10, 5, 10],
      Self::Tharja => [7, 3, 8, 3, 9, 3, 6, 3],
      Self::Olivia => [2, 2, 0, 3, 1, 5, 0, 1],
      Self::Cherche => [11, 7, 1, 6, 6, 8, 7, 2],
      Self::Henry => [10, 5, 10, 12, 5, 10, 8, 1],
      Self::Lucina => [12, 5, 1, 8, 4, 13, 3, 3],
      Self::SayRi => [19, 10, 5, 12, 13, 20, 6, 6],
      Self::Basilio => [28, 18, 3, 17, 14, 18, 13, 5],
      Self::Flavia => [26, 17, 4, 17, 16, 21, 15, 8],
      Self::Donnel => [0, 3, 0, 1, 2, 11, 2, 0],
      Self::Anna => [16, 8, 13, 12, 10, 25, 5, 5],
      Self::Owain => [10, 4, 4, 5, 6, 9, 6, 5],
      Self::Inigo => [11, 5, 2, 4, 9, 12, 4, 4],
      Self::Brady => [9, 6, 5, 3, 2, 10, 7, 4],
      Self::Kjelle => [10, 6, 2, 6, 5, 11, 3, 3],
      Self::Cynthia => [7, 5, 2, 4, 10, 17, 6, 6],
      Self::Severa => [8, 6, 1, 7, 6, 6, 6, 5],
      Self::Gerome => [13, 8, 0, 4, 8, 5, 5, 1],
      Self::Yarne => [16, 9, 1, 4, 4, 13, 6, 1],
      Self::Laurent => [10, 3, 7, 7, 4, 11, 4, 5],
      Self::Noire => [8, 5, 3, 4, 7, 10, 4, 6],
      Self::Nah => [5, 3, 3, 5, 6, 8, 3, 3],
      Self::Tiki => [21, 16, 10, 13, 15, 18, 13, 10],
      Self::Gangrel => [30, 17, 16, 19, 22, 15, 15, 12],
      Self::Walhart => [47, 29, 13, 24, 24, 30, 23, 14],
      Self::Emmeryn => [22, 4, 19, 18, 18, 13, 8, 15],
      Self::YenFay => [40, 23, 3, 28, 27, 28, 15, 14],
      Self::Aversa => [36, 19, 25, 21, 22, 26, 16, 19],
      Self::Priam => [52, 32, 2, 33, 28, 35, 31, 22],
      Self::Unknown(_) => [0; 8],
    }
  }

  /// Character stat modifiers (HP, Str, Mag, Skl, Spd, Lck, Def, Res).
  #[must_use]
  pub const fn modifiers(self) -> [i8; 8] {
    match self {
      Self::Chrom => [0, 1, 0, 1, 1, 1, -1, -1],
      Self::Lissa => [0, -2, 2, -1, 0, 2, -1, 1],
      Self::Frederick => [0, 2, -2, 2, -2, 0, 2, 0],
      Self::Virion => [0, 0, 0, 2, 2, -1, -2, 0],
      Self::Sully => [0, -1, -1, 2, 2, 0, -1, 0],
      Self::Vaike => [0, 3, -2, 1, 1, -1, 0, -2],
      Self::Stahl => [0, 2, -1, 1, 0, -2, 2, -1],
      Self::Miriel => [0, -2, 3, 1, 1, 0, -2, 0],
      Self::Kellam => [0, 1, 0, 1, -2, -2, 3, 0],
      Self::Sumia => [0, -2, 0, 2, 3, 0, -2, 1],
      Self::LonQu => [0, 0, 0, 3, 3, 0, -2, -2],
      Self::Ricken => [0, -1, 2, 0, 0, 1, -1, 0],
      Self::Maribelle => [0, -3, 2, 1, 0, 3, -3, 2],
      Self::Panne => [0, 2, -1, 2, 3, -1, 1, -1],
      Self::Gaius => [0, 1, -1, 2, 2, -2, -1, 0],
      Self::Cordelia => [0, 1, -1, 2, 2, -1, 0, -1],
      Self::Gregor => [0, 2, -1, 2, 0, -1, 1, -2],
      Self::Nowi => [0, 1, 1, -1, -2, 1, 3, 2],
      Self::Libra => [0, 0, 1, 1, 0, -1, 0, 1],
      Self::Tharja => [0, 0, 3, -1, 1, -3, 1, 0],
      Self::Olivia => [0, 0, 0, 1, 1, 0, -1, -1],
      Self::Cherche => [0, 3, 0, -1, -1, 0, 2, -2],
      Self::Henry => [0, 1, 1, 2, 0, -2, 1, -1],
      Self::SayRi => [0, 1, -1, 1, 1, -1, 0, 1],
      Self::Basilio => [0, 3, -2, 1, 1, -1, 1, -1],
      Self::Flavia => [0, 1, -1, 2, 1, 0, -1, 0],
      Self::Donnel => [0, 1, -1, -1, -1, 3, 1, -1],
      Self::Anna => [0, -1, 0, 1, 0, 3, -1, 0],
      Self::Tiki => [0, 0, -1, 0, 1, 2, 1, 2],
      Self::Gangrel => [0, -2, 0, 3, 3, -1, -1, 0],
      Self::Walhart => [0, 4, -2, 0, -1, -1, 4, -2],
      Self::Emmeryn => [0, -2, 4, 0, 1, 0, -2, 1],
      Self::YenFay => [0, 1, -2, 2, 4, 0, -1, -2],
      Self::Aversa => [0, -1, 3, 1, 1, -2, 0, 0],
      Self::Priam => [0, 3, -2, 1, 0, 0, 2, -2],
      Self::RobinM
      | Self::RobinF
      | Self::Lucina
      | Self::Owain
      | Self::Inigo
      | Self::Brady
      | Self::Kjelle
      | Self::Cynthia
      | Self::Severa
      | Self::Gerome
      | Self::MorganM
      | Self::MorganF
      | Self::Yarne
      | Self::Laurent
      | Self::Noire
      | Self::Nah
      | Self::Unknown(_) => [0; 8],
    }
  }

  /// Returns the canonical immutable mother or primary parent of this child unit.
  ///
  /// Returns `None` for non-child characters or characters with variable parentage.
  #[must_use]
  pub const fn primary_parent(self) -> Option<Self> {
    match self {
      Self::Lucina => Some(Self::Chrom),
      Self::Owain => Some(Self::Lissa),
      Self::Inigo => Some(Self::Olivia),
      Self::Brady => Some(Self::Maribelle),
      Self::Kjelle => Some(Self::Sully),
      Self::Cynthia => Some(Self::Sumia),
      Self::Severa => Some(Self::Cordelia),
      Self::Gerome => Some(Self::Cherche),
      Self::MorganM => Some(Self::RobinF),
      Self::MorganF => Some(Self::RobinM),
      Self::Yarne => Some(Self::Panne),
      Self::Laurent => Some(Self::Miriel),
      Self::Noire => Some(Self::Tharja),
      Self::Nah => Some(Self::Nowi),
      _ => None,
    }
  }

  /// Returns the world map child paralogue chapter index associated with this character.
  ///
  /// Returns `None` if this character is not recruited via a child paralogue.
  #[must_use]
  pub const fn paralogue(self) -> Option<u8> {
    match self {
      Self::Owain => Some(5),
      Self::Inigo => Some(6),
      Self::Brady => Some(7),
      Self::Kjelle => Some(8),
      Self::Cynthia => Some(9),
      Self::Severa => Some(10),
      Self::Gerome => Some(11),
      Self::MorganM => Some(12),
      Self::Yarne => Some(13),
      Self::Laurent => Some(14),
      Self::Noire => Some(15),
      Self::Nah => Some(16),
      _ => None,
    }
  }

  /// Resolves the canonical child character recruited in a world map paralogue chapter.
  #[must_use]
  pub const fn from_paralogue(paralogue: u8) -> Option<Self> {
    match paralogue {
      5 => Some(Self::Owain),
      6 => Some(Self::Inigo),
      7 => Some(Self::Brady),
      8 => Some(Self::Kjelle),
      9 => Some(Self::Cynthia),
      10 => Some(Self::Severa),
      11 => Some(Self::Gerome),
      12 => Some(Self::MorganM),
      13 => Some(Self::Yarne),
      14 => Some(Self::Laurent),
      15 => Some(Self::Noire),
      16 => Some(Self::Nah),
      _ => None,
    }
  }
}

/// Inherent biological gender of a character unit.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
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
    assert_eq!(Character::Chrom.raw_id(), 3);
    assert_eq!(Character::RobinM.raw_id(), 0);
    assert_eq!(Character::RobinF.raw_id(), 1);
    assert_eq!(Character::MorganM.raw_id(), 39);
    assert_eq!(Character::MorganF.raw_id(), 40);

    assert_eq!(Character::Chrom.name(), "Chrom");
    assert_eq!(Character::Chrom.slug(), "chrom");
    assert!(Character::Chrom.is_story_anchor());
    assert!(Character::RobinM.is_story_anchor());
    assert!(Character::RobinF.is_story_anchor());
    assert!(Character::Lucina.is_story_anchor());
    assert!(!Character::Donnel.is_story_anchor());

    assert_eq!(Character::from_raw(0), Character::RobinM);
    assert_eq!(Character::from_raw(1), Character::RobinF);
    assert_eq!(Character::from_raw(39), Character::MorganM);
    assert_eq!(Character::from_raw(40), Character::MorganF);

    assert!(Character::RobinM.is_robin());
    assert!(Character::RobinF.is_robin());
    assert!(!Character::Chrom.is_robin());

    assert!(Character::MorganM.is_morgan());
    assert!(Character::MorganF.is_morgan());
    assert!(!Character::Chrom.is_morgan());

    assert_eq!(Character::RobinM.gender(), Gender::Male);
    assert_eq!(Character::RobinF.gender(), Gender::Female);
    assert_eq!(Character::MorganM.gender(), Gender::Male);
    assert_eq!(Character::MorganF.gender(), Gender::Female);
    assert_eq!(Character::Chrom.gender(), Gender::Male);
    assert_eq!(Character::Lucina.gender(), Gender::Female);
  }
}
