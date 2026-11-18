//! Unit class definitions and lookups for Fire Emblem: Awakening.
//!
//! Owns the canonical `Class` enum representing character job types,
//! converting between raw class identifier bytes and domain variants.

use std::fmt;

use crate::save::convoy::item;
use crate::save::roster::unit::character;

/// Gender restriction rule for character classes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ClassGenderRestriction {
  /// Class can be assumed by either male or female units.
  Universal,
  /// Class can only be assumed by male units.
  MaleOnly,
  /// Class can only be assumed by female units.
  FemaleOnly,
}

/// Canonical class identifier for Fire Emblem: Awakening units.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Class {
  /// Tactician base class.
  Tactician,
  /// Great Knight advanced mounted class.
  GreatKnight,
  /// Lord base royalty class.
  Lord,
  /// Swordmaster advanced sword infantry class.
  Swordmaster,
  /// Pegasus Knight base flying class.
  PegasusKnight,
  /// Villager base class.
  Villager,
  /// Archer base bow class.
  Archer,
  /// Mage base anima magic class.
  Mage,
  /// Manakete dragonkin transformation class.
  Manakete,
  /// Great Lord promoted royalty class.
  GreatLord,
  /// Cavalier base mounted class.
  Cavalier,
  /// Sage promoted magic and staff class.
  Sage,
  /// Falcon Knight promoted flying lance and staff class.
  FalconKnight,
  /// Myrmidon base agile sword class.
  Myrmidon,
  /// Wyvern Lord promoted flying axe and lance class.
  WyvernLord,
  /// Taguel beast transformation class.
  Taguel,
  /// Paladin promoted mounted sword and lance class.
  Paladin,
  /// Valkyrie promoted mounted staff and magic class.
  Valkyrie,
  /// Knight base armored defense class.
  Knight,
  /// Sorcerer promoted dark magic class.
  Sorcerer,
  /// Dancer supportive refresh class.
  Dancer,
  /// War Monk promoted physical and staff class.
  WarMonk,
  /// Assassin promoted lethal sword and bow class.
  Assassin,
  /// Grandmaster promoted magic and sword tactician.
  Grandmaster,
  /// General promoted heavy armored class.
  General,
  /// Mercenary base sword infantry class.
  Mercenary,
  /// Hero promoted sword and axe class.
  Hero,
  /// Fighter base axe infantry class.
  Fighter,
  /// Warrior promoted axe and bow class.
  Warrior,
  /// Barbarian base axe infantry class.
  Barbarian,
  /// Berserker promoted high-critical axe class.
  Berserker,
  /// Sniper promoted bow mastery class.
  Sniper,
  /// Bow Knight promoted mounted sword and bow class.
  BowKnight,
  /// Thief agile locktouch class.
  Thief,
  /// Trickster acrobatic sword and staff class.
  Trickster,
  /// Dark Flier promoted flying lance and magic class.
  DarkFlier,
  /// Wyvern Rider base flying axe class.
  WyvernRider,
  /// Griffon Rider agile flying axe class.
  GriffonRider,
  /// Dark Knight mounted sword and magic class.
  DarkKnight,
  /// Dark Mage base dark magic class.
  DarkMage,
  /// Cleric female base staff healing class.
  Cleric,
  /// Priest male base staff healing class.
  Priest,
  /// War Cleric female promoted axe and staff class.
  WarCleric,
  /// Troubadour base mounted staff class.
  Troubadour,
  /// Lodestar special DLC sword class.
  Lodestar,
  /// Dread Fighter special DLC sword, axe, and magic class.
  DreadFighter,
  /// Bride special DLC lance, bow, and staff class.
  Bride,
  /// Conqueror Walhart unique heavy cavalry class.
  Conqueror,
  /// Soldier NPC lance class.
  Soldier,
  /// Merchant merchant class.
  Merchant,
  /// Reverant monster class.
  Reverant,
  /// Entombed monster class.
  Entombed,
  /// Grima Fell Dragon class.
  Grima,
  /// Mirage class.
  Mirage,
  /// Dummy test class.
  Dummy,
  /// Unknown or custom class identifier.
  Unknown(u8),
}

impl Class {
  /// Resolves a class from its canonical numeric identifier.
  #[must_use]
  pub const fn from_id(id: u8) -> Self {
    Self::from_raw(id)
  }

  /// Converts a raw byte to a [`Class`] variant.
  pub(crate) const fn from_raw(raw: u8) -> Self {
    match raw {
      0 | 1 => Self::Lord,
      2 | 3 => Self::GreatLord,
      4 | 5 => Self::Tactician,
      6 | 7 => Self::Grandmaster,
      8 | 9 => Self::Cavalier,
      10 | 11 => Self::Knight,
      12 | 13 => Self::Paladin,
      14 | 15 => Self::GreatKnight,
      16 | 17 => Self::General,
      18 => Self::Barbarian,
      19 => Self::Fighter,
      20 | 21 => Self::Mercenary,
      22 | 23 => Self::Archer,
      24 => Self::Berserker,
      25 => Self::Warrior,
      26 | 27 => Self::Hero,
      28 | 29 => Self::BowKnight,
      30 | 31 => Self::Sniper,
      32 | 33 => Self::Myrmidon,
      34 | 35 => Self::Thief,
      36 | 37 => Self::Swordmaster,
      38 | 39 => Self::Assassin,
      40 | 41 => Self::Trickster,
      42 => Self::PegasusKnight,
      43 => Self::FalconKnight,
      44 => Self::DarkFlier,
      45 | 46 => Self::WyvernRider,
      47 | 48 => Self::WyvernLord,
      49 | 50 => Self::GriffonRider,
      51 => Self::Troubadour,
      52 => Self::Priest,
      53 => Self::Cleric,
      54 | 55 => Self::Mage,
      56 | 57 => Self::DarkMage,
      58 => Self::Valkyrie,
      59 => Self::WarMonk,
      60 => Self::WarCleric,
      61 | 62 => Self::Sage,
      63 | 64 => Self::DarkKnight,
      65 | 66 => Self::Sorcerer,
      67 => Self::Dancer,
      68 => Self::Manakete,
      69 | 70 => Self::Taguel,
      71 => Self::Soldier,
      72 => Self::Villager,
      73 => Self::Merchant,
      74 => Self::Reverant,
      75 => Self::Entombed,
      76 => Self::Conqueror,
      77 => Self::Lodestar,
      78 => Self::Grima,
      79 => Self::Mirage,
      80 => Self::DreadFighter,
      81 => Self::Bride,
      82 => Self::Dummy,
      other => Self::Unknown(other),
    }
  }

  /// Returns the canonical numeric ID for this class given biological [`character::Gender`].
  #[must_use]
  pub const fn id(self, gender: character::Gender) -> u8 {
    self.raw_id(gender)
  }

  /// Converts a [`Class`] into its raw byte ID given biological [`character::Gender`].
  #[must_use]
  #[expect(
    clippy::too_many_lines,
    reason = "Comprehensive gendered class raw ID lookup table"
  )]
  pub(crate) const fn raw_id(self, gender: character::Gender) -> u8 {
    match gender {
      character::Gender::Male => match self {
        Self::Lord => 0,
        Self::GreatLord => 2,
        Self::Tactician => 4,
        Self::Grandmaster => 6,
        Self::Cavalier => 8,
        Self::Knight => 10,
        Self::Paladin => 12,
        Self::GreatKnight => 14,
        Self::General => 16,
        Self::Barbarian => 18,
        Self::Fighter => 19,
        Self::Mercenary => 20,
        Self::Archer => 22,
        Self::Berserker => 24,
        Self::Warrior => 25,
        Self::Hero => 26,
        Self::BowKnight => 28,
        Self::Sniper => 30,
        Self::Myrmidon => 32,
        Self::Thief => 34,
        Self::Swordmaster => 36,
        Self::Assassin => 38,
        Self::Trickster => 40,
        Self::PegasusKnight => 42,
        Self::FalconKnight => 43,
        Self::DarkFlier => 44,
        Self::WyvernRider => 45,
        Self::WyvernLord => 47,
        Self::GriffonRider => 49,
        Self::Troubadour => 51,
        Self::Priest => 52,
        Self::Cleric => 53,
        Self::Mage => 54,
        Self::DarkMage => 56,
        Self::Valkyrie => 58,
        Self::WarMonk => 59,
        Self::WarCleric => 60,
        Self::Sage => 61,
        Self::DarkKnight => 63,
        Self::Sorcerer => 65,
        Self::Dancer => 67,
        Self::Manakete => 68,
        Self::Taguel => 69,
        Self::Soldier => 71,
        Self::Villager => 72,
        Self::Merchant => 73,
        Self::Reverant => 74,
        Self::Entombed => 75,
        Self::Conqueror => 76,
        Self::Lodestar => 77,
        Self::Grima => 78,
        Self::Mirage => 79,
        Self::DreadFighter => 80,
        Self::Bride => 81,
        Self::Dummy => 82,
        Self::Unknown(id) => id,
      },
      character::Gender::Female => match self {
        Self::Lord => 1,
        Self::GreatLord => 3,
        Self::Tactician => 5,
        Self::Grandmaster => 7,
        Self::Cavalier => 9,
        Self::Knight => 11,
        Self::Paladin => 13,
        Self::GreatKnight => 15,
        Self::General => 17,
        Self::Barbarian => 18,
        Self::Fighter => 19,
        Self::Mercenary => 21,
        Self::Archer => 23,
        Self::Berserker => 24,
        Self::Warrior => 25,
        Self::Hero => 27,
        Self::BowKnight => 29,
        Self::Sniper => 31,
        Self::Myrmidon => 33,
        Self::Thief => 35,
        Self::Swordmaster => 37,
        Self::Assassin => 39,
        Self::Trickster => 41,
        Self::PegasusKnight => 42,
        Self::FalconKnight => 43,
        Self::DarkFlier => 44,
        Self::WyvernRider => 46,
        Self::WyvernLord => 48,
        Self::GriffonRider => 50,
        Self::Troubadour => 51,
        Self::Priest => 52,
        Self::Cleric => 53,
        Self::Mage => 55,
        Self::DarkMage => 57,
        Self::Valkyrie => 58,
        Self::WarMonk => 59,
        Self::WarCleric => 60,
        Self::Sage => 62,
        Self::DarkKnight => 64,
        Self::Sorcerer => 66,
        Self::Dancer => 67,
        Self::Manakete => 68,
        Self::Taguel => 70,
        Self::Soldier => 71,
        Self::Villager => 72,
        Self::Merchant => 73,
        Self::Reverant => 74,
        Self::Entombed => 75,
        Self::Conqueror => 76,
        Self::Lodestar => 77,
        Self::Grima => 78,
        Self::Mirage => 79,
        Self::DreadFighter => 80,
        Self::Bride => 81,
        Self::Dummy => 82,
        Self::Unknown(id) => id,
      },
    }
  }

  /// Returns the gender restriction rule for this class.
  #[must_use]
  pub const fn gender_restriction(self) -> ClassGenderRestriction {
    match self {
      Self::Villager
      | Self::Fighter
      | Self::Warrior
      | Self::Barbarian
      | Self::Berserker
      | Self::WarMonk
      | Self::Priest
      | Self::Lodestar
      | Self::DreadFighter
      | Self::Conqueror
      | Self::Soldier => ClassGenderRestriction::MaleOnly,

      Self::PegasusKnight
      | Self::Manakete
      | Self::FalconKnight
      | Self::Valkyrie
      | Self::Dancer
      | Self::DarkFlier
      | Self::Cleric
      | Self::WarCleric
      | Self::Troubadour
      | Self::Bride
      | Self::Merchant => ClassGenderRestriction::FemaleOnly,

      _ => ClassGenderRestriction::Universal,
    }
  }

  /// Returns whether this class is permitted for the given biological gender.
  #[must_use]
  pub const fn is_eligible(self, gender: character::Gender) -> bool {
    match self.gender_restriction() {
      ClassGenderRestriction::Universal => true,
      ClassGenderRestriction::MaleOnly => match gender {
        character::Gender::Male => true,
        character::Gender::Female => false,
      },
      ClassGenderRestriction::FemaleOnly => match gender {
        character::Gender::Female => true,
        character::Gender::Male => false,
      },
    }
  }

  /// Returns the human-readable canonical display name of the class.
  #[must_use]
  pub const fn name(self) -> &'static str {
    match self {
      Self::Tactician => "Tactician",
      Self::GreatKnight => "Great Knight",
      Self::Lord => "Lord",
      Self::Swordmaster => "Swordmaster",
      Self::PegasusKnight => "Pegasus Knight",
      Self::Villager => "Villager",
      Self::Archer => "Archer",
      Self::Mage => "Mage",
      Self::Manakete => "Manakete",
      Self::GreatLord => "Great Lord",
      Self::Cavalier => "Cavalier",
      Self::Sage => "Sage",
      Self::FalconKnight => "Falcon Knight",
      Self::Myrmidon => "Myrmidon",
      Self::WyvernLord => "Wyvern Lord",
      Self::Taguel => "Taguel",
      Self::Paladin => "Paladin",
      Self::Valkyrie => "Valkyrie",
      Self::Knight => "Knight",
      Self::Sorcerer => "Sorcerer",
      Self::Dancer => "Dancer",
      Self::WarMonk => "War Monk",
      Self::Assassin => "Assassin",
      Self::Grandmaster => "Grandmaster",
      Self::General => "General",
      Self::Mercenary => "Mercenary",
      Self::Hero => "Hero",
      Self::Fighter => "Fighter",
      Self::Warrior => "Warrior",
      Self::Barbarian => "Barbarian",
      Self::Berserker => "Berserker",
      Self::Sniper => "Sniper",
      Self::BowKnight => "Bow Knight",
      Self::Thief => "Thief",
      Self::Trickster => "Trickster",
      Self::DarkFlier => "Dark Flier",
      Self::WyvernRider => "Wyvern Rider",
      Self::GriffonRider => "Griffon Rider",
      Self::DarkKnight => "Dark Knight",
      Self::DarkMage => "Dark Mage",
      Self::Cleric => "Cleric",
      Self::Priest => "Priest",
      Self::WarCleric => "War Cleric",
      Self::Troubadour => "Troubadour",
      Self::Lodestar => "Lodestar",
      Self::DreadFighter => "Dread Fighter",
      Self::Bride => "Bride",
      Self::Conqueror => "Conqueror",
      Self::Soldier => "Soldier",
      Self::Merchant => "Merchant",
      Self::Reverant => "Reverant",
      Self::Entombed => "Entombed",
      Self::Grima => "Grima",
      Self::Mirage => "Mirage",
      Self::Dummy => "Dummy",
      Self::Unknown(_) => "Unknown",
    }
  }

  /// Class base combat attributes (HP, Str, Mag, Skl, Spd, Lck, Def, Res).
  #[must_use]
  pub const fn base_stats(self) -> [u8; 8] {
    match self {
      Self::Tactician => [16, 4, 3, 5, 5, 0, 5, 3],
      Self::GreatKnight => [26, 11, 0, 6, 5, 0, 14, 1],
      Self::Lord => [18, 6, 0, 5, 7, 0, 7, 0],
      Self::Swordmaster => [20, 7, 2, 11, 13, 0, 6, 4],
      Self::PegasusKnight => [16, 4, 2, 7, 8, 0, 4, 6],
      Self::Villager => [16, 1, 0, 1, 1, 0, 1, 0],
      Self::Archer => [16, 5, 0, 8, 6, 0, 5, 0],
      Self::Mage => [16, 0, 4, 3, 4, 0, 2, 3],
      Self::Manakete => [18, 2, 0, 1, 1, 0, 2, 2],
      Self::GreatLord => [23, 10, 0, 7, 9, 0, 10, 3],
      Self::Cavalier => [18, 6, 0, 5, 6, 0, 7, 0],
      Self::Sage => [20, 1, 7, 5, 7, 0, 4, 5],
      Self::FalconKnight => [20, 6, 3, 10, 11, 0, 6, 9],
      Self::Myrmidon => [16, 4, 1, 9, 10, 0, 4, 1],
      Self::WyvernLord => [24, 11, 0, 8, 7, 0, 11, 3],
      Self::Taguel => [18, 3, 0, 4, 4, 0, 4, 1],
      Self::Paladin => [25, 9, 1, 7, 8, 0, 10, 6],
      Self::Valkyrie => [19, 0, 5, 4, 8, 0, 3, 8],
      Self::Knight => [18, 8, 0, 4, 2, 0, 11, 0],
      Self::Sorcerer => [23, 2, 6, 4, 4, 0, 7, 7],
      Self::Dancer => [16, 1, 1, 5, 8, 0, 3, 1],
      Self::WarMonk | Self::WarCleric => [24, 5, 5, 4, 6, 0, 6, 6],
      Self::Assassin => [21, 8, 0, 13, 12, 0, 5, 1],
      Self::Grandmaster => [20, 7, 6, 7, 7, 0, 7, 5],
      Self::General => [28, 12, 0, 7, 4, 0, 15, 3],
      Self::Mercenary => [18, 5, 0, 8, 7, 0, 5, 0],
      Self::Hero => [22, 8, 1, 11, 10, 0, 8, 3],
      Self::Fighter => [20, 8, 0, 5, 5, 0, 4, 0],
      Self::Warrior => [28, 12, 0, 8, 7, 0, 7, 3],
      Self::Barbarian => [22, 8, 0, 3, 8, 0, 3, 0],
      Self::Berserker => [30, 13, 0, 5, 11, 0, 5, 1],
      Self::Sniper => [20, 7, 1, 12, 9, 0, 10, 3],
      Self::BowKnight => [24, 8, 0, 10, 10, 0, 6, 2],
      Self::Thief => [16, 3, 0, 6, 8, 0, 2, 0],
      Self::Trickster => [19, 4, 4, 10, 11, 0, 3, 5],
      Self::DarkFlier => [19, 5, 6, 8, 10, 0, 5, 9],
      Self::WyvernRider => [19, 7, 0, 6, 5, 0, 8, 0],
      Self::GriffonRider => [22, 9, 0, 10, 9, 0, 8, 3],
      Self::DarkKnight => [25, 4, 5, 6, 5, 0, 9, 5],
      Self::DarkMage => [18, 1, 3, 2, 3, 0, 4, 4],
      Self::Cleric | Self::Priest => [16, 0, 3, 2, 4, 0, 1, 6],
      Self::Troubadour => [16, 0, 3, 2, 5, 0, 1, 5],
      Self::Lodestar => [21, 9, 1, 10, 10, 0, 8, 4],
      Self::DreadFighter => [30, 15, 15, 15, 15, 0, 15, 10],
      Self::Bride => [1, 0, 0, 0, 0, 0, 0, 0],
      Self::Conqueror => [24, 10, 2, 9, 8, 0, 12, 5],
      Self::Soldier => [16, 3, 0, 3, 3, 0, 3, 0],
      Self::Merchant => [18, 3, 0, 2, 2, 0, 4, 1],
      Self::Reverant => [18, 3, 0, 0, 0, 0, 0, 0],
      Self::Entombed => [20, 3, 1, 2, 2, 0, 2, 1],
      _ => [0; 8],
    }
  }

  /// Class maximum combat stat caps (HP, Str, Mag, Skl, Spd, Lck, Def, Res).
  #[must_use]
  pub const fn max_stats(self) -> [u8; 8] {
    match self {
      Self::Tactician => [60, 25, 25, 25, 25, 30, 25, 25],
      Self::GreatKnight => [80, 48, 20, 34, 37, 45, 48, 30],
      Self::Lord => [60, 27, 20, 25, 26, 30, 26, 25],
      Self::Swordmaster => [80, 38, 34, 44, 46, 45, 33, 38],
      Self::PegasusKnight => [60, 24, 23, 28, 27, 30, 22, 25],
      Self::Villager | Self::Bride | Self::Merchant => {
        [60, 20, 20, 20, 20, 30, 20, 20]
      }
      Self::Archer => [60, 26, 20, 29, 25, 30, 25, 21],
      Self::Mage => [60, 20, 28, 27, 26, 30, 21, 25],
      Self::Manakete => [80, 40, 35, 35, 35, 45, 40, 40],
      Self::GreatLord => [80, 43, 30, 40, 41, 45, 42, 40],
      Self::Cavalier => [60, 26, 20, 25, 25, 30, 26, 26],
      Self::Sage => [80, 30, 46, 43, 42, 45, 31, 40],
      Self::FalconKnight => [80, 38, 35, 45, 44, 45, 33, 40],
      Self::Myrmidon => [60, 24, 22, 27, 28, 30, 22, 24],
      Self::WyvernLord => [80, 46, 30, 38, 38, 45, 46, 30],
      Self::Taguel => [80, 35, 30, 40, 40, 45, 35, 30],
      Self::Paladin => [80, 42, 30, 40, 40, 45, 42, 42],
      Self::Valkyrie => [80, 30, 42, 38, 43, 45, 30, 45],
      Self::Knight => [60, 30, 20, 26, 23, 30, 30, 22],
      Self::Sorcerer => [80, 30, 44, 38, 40, 45, 41, 44],
      Self::Dancer => [80, 30, 30, 40, 40, 45, 30, 30],
      Self::WarMonk | Self::WarCleric => [80, 40, 40, 38, 41, 45, 38, 43],
      Self::Assassin => [80, 40, 30, 48, 46, 45, 31, 30],
      Self::Grandmaster => [80, 40, 40, 40, 40, 45, 40, 40],
      Self::General => [80, 50, 30, 41, 35, 45, 50, 35],
      Self::Mercenary => [60, 26, 20, 28, 26, 30, 25, 23],
      Self::Hero => [80, 42, 30, 46, 42, 45, 40, 36],
      Self::Fighter => [60, 29, 20, 26, 25, 30, 25, 23],
      Self::Warrior => [80, 48, 30, 42, 40, 45, 40, 35],
      Self::Barbarian => [60, 30, 20, 23, 27, 30, 22, 20],
      Self::Berserker => [80, 50, 30, 35, 44, 45, 34, 30],
      Self::Sniper => [80, 41, 30, 48, 40, 45, 40, 31],
      Self::BowKnight => [80, 40, 30, 43, 41, 45, 35, 30],
      Self::Thief => [60, 22, 20, 30, 28, 30, 21, 20],
      Self::Trickster => [80, 35, 38, 45, 43, 45, 30, 40],
      Self::DarkFlier => [80, 36, 42, 41, 42, 45, 32, 41],
      Self::WyvernRider => [60, 28, 20, 24, 24, 30, 28, 20],
      Self::GriffonRider => [80, 40, 30, 43, 41, 45, 40, 30],
      Self::DarkKnight => [80, 38, 41, 40, 40, 45, 42, 38],
      Self::DarkMage => [60, 20, 27, 25, 25, 30, 25, 27],
      Self::Cleric | Self::Priest => [60, 22, 25, 24, 25, 30, 22, 27],
      Self::Troubadour => [60, 20, 26, 24, 26, 30, 20, 28],
      Self::Lodestar => [80, 41, 30, 43, 43, 45, 41, 40],
      Self::DreadFighter => [99, 50, 40, 50, 45, 45, 50, 50],
      Self::Conqueror => [80, 45, 25, 40, 40, 45, 45, 35],
      Self::Soldier => [80, 30, 30, 30, 30, 45, 30, 30],
      Self::Reverant => [80, 30, 20, 25, 25, 30, 30, 30],
      Self::Entombed => [80, 40, 30, 35, 35, 45, 35, 30],
      _ => [80; 8],
    }
  }

  /// Canonical base movement tiles for this class.
  #[must_use]
  pub const fn base_movement(self) -> u8 {
    match self {
      Self::Knight => 4,
      Self::Paladin
      | Self::BowKnight
      | Self::FalconKnight
      | Self::DarkFlier
      | Self::WyvernLord
      | Self::GriffonRider
      | Self::DarkKnight
      | Self::Valkyrie
      | Self::Conqueror => 8,
      Self::Cavalier
      | Self::GreatKnight
      | Self::PegasusKnight
      | Self::WyvernRider
      | Self::Troubadour => 7,
      Self::GreatLord
      | Self::Grandmaster
      | Self::Swordmaster
      | Self::Assassin
      | Self::Trickster
      | Self::Hero
      | Self::Warrior
      | Self::Berserker
      | Self::Sniper
      | Self::Sage
      | Self::Sorcerer
      | Self::WarMonk
      | Self::WarCleric
      | Self::Dancer
      | Self::Taguel
      | Self::Manakete
      | Self::DreadFighter
      | Self::Bride
      | Self::Lodestar => 6,
      _ => 5,
    }
  }

  /// Returns true if this class is proficient in using the specified item kind.
  #[must_use]
  pub const fn can_use(self, kind: item::ItemKind) -> bool {
    match kind {
      item::ItemKind::Sword => match self {
        Self::Lord
        | Self::GreatLord
        | Self::Tactician
        | Self::Grandmaster
        | Self::Cavalier
        | Self::Paladin
        | Self::GreatKnight
        | Self::Mercenary
        | Self::Hero
        | Self::Myrmidon
        | Self::Swordmaster
        | Self::Thief
        | Self::Assassin
        | Self::Trickster
        | Self::DarkKnight
        | Self::Dancer
        | Self::Lodestar
        | Self::DreadFighter => true,
        _ => false,
      },
      item::ItemKind::Lance => match self {
        Self::GreatLord
        | Self::Cavalier
        | Self::Paladin
        | Self::Knight
        | Self::General
        | Self::GreatKnight
        | Self::PegasusKnight
        | Self::FalconKnight
        | Self::DarkFlier
        | Self::WyvernLord
        | Self::Soldier
        | Self::Bride => true,
        _ => false,
      },
      item::ItemKind::Axe => match self {
        Self::GreatKnight
        | Self::Knight
        | Self::General
        | Self::Barbarian
        | Self::Berserker
        | Self::Fighter
        | Self::Warrior
        | Self::Hero
        | Self::WyvernRider
        | Self::WyvernLord
        | Self::GriffonRider
        | Self::WarMonk
        | Self::WarCleric
        | Self::DreadFighter => true,
        _ => false,
      },
      item::ItemKind::Bow => match self {
        Self::Archer
        | Self::Sniper
        | Self::BowKnight
        | Self::Warrior
        | Self::Assassin
        | Self::Bride => true,
        _ => false,
      },
      item::ItemKind::Tome => match self {
        Self::Tactician
        | Self::Grandmaster
        | Self::Mage
        | Self::Sage
        | Self::DarkMage
        | Self::Sorcerer
        | Self::DarkKnight
        | Self::DarkFlier
        | Self::Valkyrie
        | Self::DreadFighter => true,
        _ => false,
      },
      item::ItemKind::Staff => match self {
        Self::Priest
        | Self::Cleric
        | Self::WarMonk
        | Self::WarCleric
        | Self::Sage
        | Self::FalconKnight
        | Self::Troubadour
        | Self::Valkyrie
        | Self::Trickster
        | Self::Bride => true,
        _ => false,
      },
      item::ItemKind::Stone => matches!(self, Self::Manakete | Self::Taguel),
      item::ItemKind::Consumable
      | item::ItemKind::Seal
      | item::ItemKind::StatBooster
      | item::ItemKind::Valuable
      | item::ItemKind::Unknown => false,
    }
  }
}

impl fmt::Display for Class {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Unknown(id) => write!(f, "Class({id})"),
      _ => write!(f, "{}", self.name()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_class_roundtrip_and_properties() {
    assert_eq!(Class::from_raw(0), Class::Lord);
    assert_eq!(Class::from_raw(1), Class::Lord);
    assert_eq!(Class::Lord.raw_id(character::Gender::Male), 0);
    assert_eq!(Class::Lord.raw_id(character::Gender::Female), 1);
    assert_eq!(Class::Lord.id(character::Gender::Male), 0);
    assert_eq!(Class::Lord.name(), "Lord");

    assert_eq!(Class::from_raw(4), Class::Tactician);
    assert_eq!(Class::from_raw(5), Class::Tactician);
    assert_eq!(Class::Tactician.name(), "Tactician");

    assert_eq!(Class::from_raw(42), Class::PegasusKnight);
    assert_eq!(
      Class::PegasusKnight.gender_restriction(),
      ClassGenderRestriction::FemaleOnly
    );
    assert!(!Class::PegasusKnight.is_eligible(character::Gender::Male));
    assert!(Class::PegasusKnight.is_eligible(character::Gender::Female));

    assert_eq!(Class::from_raw(99), Class::Unknown(99));
    assert_eq!(Class::Unknown(99).raw_id(character::Gender::Male), 99);
    assert_eq!(Class::Unknown(99).id(character::Gender::Male), 99);
  }
}
