//! Unit class definitions and lookups for Fire Emblem: Awakening.
//!
//! Owns the canonical `Class` enum representing character job types,
//! converting between raw class identifier bytes and domain variants.

use std::fmt;

use crate::slot::convoy::item;
use crate::slot::roster::unit::character;

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

/// Gender restriction rule for character classes.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ClassGenderRestriction {
  /// Class can be assumed by either male or female units.
  Universal,
  /// Class can only be assumed by male units.
  MaleOnly,
  /// Class can only be assumed by female units.
  FemaleOnly,
}

impl Class {
  /// Converts a raw byte to a [`Class`] variant.
  #[must_use]
  pub const fn from_raw(raw: u8) -> Self {
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

  /// Automatically resolves gender-paired equivalents (e.g. Cleric <-> Priest).
  #[must_use]
  pub const fn resolve_for_gender(self, gender: character::Gender) -> Self {
    match (self, gender) {
      (Self::Cleric, character::Gender::Male) => Self::Priest,
      (Self::Priest, character::Gender::Female) => Self::Cleric,
      (Self::WarMonk, character::Gender::Female) => Self::WarCleric,
      (Self::WarCleric, character::Gender::Male) => Self::WarMonk,
      (other, _) => other,
    }
  }

  /// Converts a [`Class`] variant back to its default raw byte representation.
  #[must_use]
  pub const fn raw_id(self) -> u8 {
    self.raw_id_for_gender(character::Gender::Male)
  }

  /// Converts a [`Class`] variant into its raw byte ID given a unit's [`character::Gender`].
  #[must_use]
  #[expect(
    clippy::too_many_lines,
    reason = "exhaustive mapping of 82 classes across male/female variants"
  )]
  pub const fn raw_id_for_gender(self, gender: character::Gender) -> u8 {
    match self {
      Self::Lord => match gender {
        character::Gender::Male => 0,
        character::Gender::Female => 1,
      },
      Self::GreatLord => match gender {
        character::Gender::Male => 2,
        character::Gender::Female => 3,
      },
      Self::Tactician => match gender {
        character::Gender::Male => 4,
        character::Gender::Female => 5,
      },
      Self::Grandmaster => match gender {
        character::Gender::Male => 6,
        character::Gender::Female => 7,
      },
      Self::Cavalier => match gender {
        character::Gender::Male => 8,
        character::Gender::Female => 9,
      },
      Self::Knight => match gender {
        character::Gender::Male => 10,
        character::Gender::Female => 11,
      },
      Self::Paladin => match gender {
        character::Gender::Male => 12,
        character::Gender::Female => 13,
      },
      Self::GreatKnight => match gender {
        character::Gender::Male => 14,
        character::Gender::Female => 15,
      },
      Self::General => match gender {
        character::Gender::Male => 16,
        character::Gender::Female => 17,
      },
      Self::Barbarian => 18,
      Self::Fighter => 19,
      Self::Mercenary => match gender {
        character::Gender::Male => 20,
        character::Gender::Female => 21,
      },
      Self::Archer => match gender {
        character::Gender::Male => 22,
        character::Gender::Female => 23,
      },
      Self::Berserker => 24,
      Self::Warrior => 25,
      Self::Hero => match gender {
        character::Gender::Male => 26,
        character::Gender::Female => 27,
      },
      Self::BowKnight => match gender {
        character::Gender::Male => 28,
        character::Gender::Female => 29,
      },
      Self::Sniper => match gender {
        character::Gender::Male => 30,
        character::Gender::Female => 31,
      },
      Self::Myrmidon => match gender {
        character::Gender::Male => 32,
        character::Gender::Female => 33,
      },
      Self::Thief => match gender {
        character::Gender::Male => 34,
        character::Gender::Female => 35,
      },
      Self::Swordmaster => match gender {
        character::Gender::Male => 36,
        character::Gender::Female => 37,
      },
      Self::Assassin => match gender {
        character::Gender::Male => 38,
        character::Gender::Female => 39,
      },
      Self::Trickster => match gender {
        character::Gender::Male => 40,
        character::Gender::Female => 41,
      },
      Self::PegasusKnight => 42,
      Self::FalconKnight => 43,
      Self::DarkFlier => 44,
      Self::WyvernRider => match gender {
        character::Gender::Male => 45,
        character::Gender::Female => 46,
      },
      Self::WyvernLord => match gender {
        character::Gender::Male => 47,
        character::Gender::Female => 48,
      },
      Self::GriffonRider => match gender {
        character::Gender::Male => 49,
        character::Gender::Female => 50,
      },
      Self::Troubadour => 51,
      Self::Priest => 52,
      Self::Cleric => 53,
      Self::Mage => match gender {
        character::Gender::Male => 54,
        character::Gender::Female => 55,
      },
      Self::DarkMage => match gender {
        character::Gender::Male => 56,
        character::Gender::Female => 57,
      },
      Self::Valkyrie => 58,
      Self::WarMonk => 59,
      Self::WarCleric => 60,
      Self::Sage => match gender {
        character::Gender::Male => 61,
        character::Gender::Female => 62,
      },
      Self::DarkKnight => match gender {
        character::Gender::Male => 63,
        character::Gender::Female => 64,
      },
      Self::Sorcerer => match gender {
        character::Gender::Male => 65,
        character::Gender::Female => 66,
      },
      Self::Dancer => 67,
      Self::Manakete => 68,
      Self::Taguel => match gender {
        character::Gender::Male => 69,
        character::Gender::Female => 70,
      },
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
    }
  }

  /// Returns the gender restriction rule for this class.
  #[must_use]
  pub const fn gender_restriction(self) -> ClassGenderRestriction {
    match self {
      Self::PegasusKnight
      | Self::FalconKnight
      | Self::DarkFlier
      | Self::Troubadour
      | Self::Cleric
      | Self::Valkyrie
      | Self::WarCleric
      | Self::Dancer
      | Self::Manakete
      | Self::Merchant
      | Self::Bride => ClassGenderRestriction::FemaleOnly,

      Self::Barbarian
      | Self::Fighter
      | Self::Berserker
      | Self::Warrior
      | Self::Priest
      | Self::WarMonk
      | Self::Soldier
      | Self::Villager
      | Self::Lodestar
      | Self::DreadFighter
      | Self::Conqueror => ClassGenderRestriction::MaleOnly,

      _ => ClassGenderRestriction::Universal,
    }
  }

  /// Returns whether this class is permitted for the given biological gender.
  #[must_use]
  pub const fn is_eligible_for_gender(self, gender: character::Gender) -> bool {
    match self.gender_restriction() {
      ClassGenderRestriction::Universal => true,
      ClassGenderRestriction::MaleOnly => {
        matches!(gender, character::Gender::Male)
      }
      ClassGenderRestriction::FemaleOnly => {
        matches!(gender, character::Gender::Female)
      }
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
    assert_eq!(Class::from_raw(4), Class::Tactician);
    assert_eq!(Class::from_raw(5), Class::Tactician);
    assert_eq!(Class::from_raw(14), Class::GreatKnight);
    assert_eq!(Class::from_raw(15), Class::GreatKnight);
    assert_eq!(
      Class::Tactician.raw_id_for_gender(character::Gender::Male),
      4
    );
    assert_eq!(
      Class::Tactician.raw_id_for_gender(character::Gender::Female),
      5
    );
    assert_eq!(
      Class::GreatKnight.raw_id_for_gender(character::Gender::Male),
      14
    );
    assert_eq!(
      Class::GreatKnight.raw_id_for_gender(character::Gender::Female),
      15
    );
    assert_eq!(
      Class::Cleric.raw_id_for_gender(character::Gender::Female),
      53
    );
    assert_eq!(Class::Priest.raw_id_for_gender(character::Gender::Male), 52);
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
