//! Unit class definitions and lookups for Fire Emblem: Awakening.
//!
//! Owns the canonical `Class` enum representing character job types,
//! converting between raw class identifier bytes and domain variants.

use std::fmt;

use crate::slot::convoy::item;

/// Canonical class identifier for Fire Emblem: Awakening units.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
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
  /// Unknown or custom class identifier.
  Unknown(u8),
}

impl Class {
  /// Converts a raw byte to a [`Class`] variant.
  #[must_use]
  pub const fn from_raw(raw: u8) -> Self {
    match raw {
      1 => Self::Tactician,
      2 => Self::GreatKnight,
      3 => Self::Lord,
      7 => Self::Swordmaster,
      8 => Self::PegasusKnight,
      10 => Self::Villager,
      14 => Self::Archer,
      17 => Self::Mage,
      20 => Self::Manakete,
      21 => Self::GreatLord,
      22 => Self::Cavalier,
      25 => Self::Sage,
      27 => Self::FalconKnight,
      42 => Self::Myrmidon,
      48 => Self::WyvernLord,
      51 => Self::Taguel,
      53 => Self::Paladin,
      54 => Self::Valkyrie,
      55 => Self::Knight,
      56 => Self::Sorcerer,
      67 => Self::Dancer,
      68 => Self::WarMonk,
      70 => Self::Assassin,
      71 => Self::Grandmaster,
      72 => Self::General,
      73 => Self::Mercenary,
      74 => Self::Hero,
      75 => Self::Fighter,
      76 => Self::Warrior,
      77 => Self::Barbarian,
      78 => Self::Berserker,
      79 => Self::Sniper,
      80 => Self::BowKnight,
      81 => Self::Thief,
      82 => Self::Trickster,
      83 => Self::DarkFlier,
      84 => Self::WyvernRider,
      85 => Self::GriffonRider,
      86 => Self::DarkKnight,
      87 => Self::DarkMage,
      88 => Self::Cleric,
      89 => Self::Priest,
      90 => Self::WarCleric,
      91 => Self::Troubadour,
      92 => Self::Lodestar,
      93 => Self::DreadFighter,
      94 => Self::Bride,
      95 => Self::Conqueror,
      other => Self::Unknown(other),
    }
  }

  /// Converts a [`Class`] variant back to its raw byte representation.
  #[must_use]
  pub const fn raw_id(self) -> u8 {
    match self {
      Self::Tactician => 1,
      Self::GreatKnight => 2,
      Self::Lord => 3,
      Self::Swordmaster => 7,
      Self::PegasusKnight => 8,
      Self::Villager => 10,
      Self::Archer => 14,
      Self::Mage => 17,
      Self::Manakete => 20,
      Self::GreatLord => 21,
      Self::Cavalier => 22,
      Self::Sage => 25,
      Self::FalconKnight => 27,
      Self::Myrmidon => 42,
      Self::WyvernLord => 48,
      Self::Taguel => 51,
      Self::Paladin => 53,
      Self::Valkyrie => 54,
      Self::Knight => 55,
      Self::Sorcerer => 56,
      Self::Dancer => 67,
      Self::WarMonk => 68,
      Self::Assassin => 70,
      Self::Grandmaster => 71,
      Self::General => 72,
      Self::Mercenary => 73,
      Self::Hero => 74,
      Self::Fighter => 75,
      Self::Warrior => 76,
      Self::Barbarian => 77,
      Self::Berserker => 78,
      Self::Sniper => 79,
      Self::BowKnight => 80,
      Self::Thief => 81,
      Self::Trickster => 82,
      Self::DarkFlier => 83,
      Self::WyvernRider => 84,
      Self::GriffonRider => 85,
      Self::DarkKnight => 86,
      Self::DarkMage => 87,
      Self::Cleric => 88,
      Self::Priest => 89,
      Self::WarCleric => 90,
      Self::Troubadour => 91,
      Self::Lodestar => 92,
      Self::DreadFighter => 93,
      Self::Bride => 94,
      Self::Conqueror => 95,
      Self::Unknown(id) => id,
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
      Self::Unknown(_) => "Unknown",
    }
  }

  /// Returns true if this class is proficient in using the specified item kind.
  #[must_use]
  #[expect(
    clippy::too_many_lines,
    reason = "comprehensive class weapon proficiency matrix"
  )]
  pub const fn can_use(self, kind: item::ItemKind) -> bool {
    match kind {
      item::ItemKind::Sword => matches!(
        self,
        Self::Lord
          | Self::GreatLord
          | Self::Tactician
          | Self::Grandmaster
          | Self::Cavalier
          | Self::Paladin
          | Self::GreatKnight
          | Self::Myrmidon
          | Self::Swordmaster
          | Self::Mercenary
          | Self::Hero
          | Self::Thief
          | Self::Assassin
          | Self::Trickster
          | Self::BowKnight
          | Self::DarkKnight
          | Self::Dancer
          | Self::Lodestar
          | Self::DreadFighter
          | Self::Conqueror
          | Self::Unknown(_)
      ),
      item::ItemKind::Lance => matches!(
        self,
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
          | Self::Villager
          | Self::Bride
          | Self::Conqueror
          | Self::Unknown(_)
      ),
      item::ItemKind::Axe => matches!(
        self,
        Self::Fighter
          | Self::Warrior
          | Self::Barbarian
          | Self::Berserker
          | Self::Knight
          | Self::General
          | Self::GreatKnight
          | Self::Hero
          | Self::WyvernRider
          | Self::WyvernLord
          | Self::GriffonRider
          | Self::WarMonk
          | Self::WarCleric
          | Self::DreadFighter
          | Self::Conqueror
          | Self::Unknown(_)
      ),
      item::ItemKind::Bow => matches!(
        self,
        Self::Archer
          | Self::Sniper
          | Self::BowKnight
          | Self::Assassin
          | Self::Warrior
          | Self::Bride
          | Self::Unknown(_)
      ),
      item::ItemKind::Tome => matches!(
        self,
        Self::Mage
          | Self::Sage
          | Self::DarkMage
          | Self::Sorcerer
          | Self::DarkKnight
          | Self::DarkFlier
          | Self::Tactician
          | Self::Grandmaster
          | Self::Valkyrie
          | Self::DreadFighter
          | Self::Unknown(_)
      ),
      item::ItemKind::Staff => matches!(
        self,
        Self::Cleric
          | Self::Priest
          | Self::WarMonk
          | Self::WarCleric
          | Self::Troubadour
          | Self::Valkyrie
          | Self::Sage
          | Self::FalconKnight
          | Self::Trickster
          | Self::Bride
          | Self::Unknown(_)
      ),
      item::ItemKind::Stone => {
        matches!(self, Self::Manakete | Self::Taguel | Self::Unknown(_))
      }
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
  fn test_class_roundtrip() {
    assert_eq!(Class::from_raw(1), Class::Tactician);
    assert_eq!(Class::Tactician.raw_id(), 1);
    assert_eq!(Class::Tactician.name(), "Tactician");
    assert_eq!(format!("{}", Class::Tactician), "Tactician");
  }

  #[test]
  fn test_class_unknown() {
    let unk = Class::from_raw(255);
    assert_eq!(unk, Class::Unknown(255));
    assert_eq!(unk.raw_id(), 255);
    assert_eq!(unk.name(), "Unknown");
    assert_eq!(format!("{unk}"), "Class(255)");
  }
}
