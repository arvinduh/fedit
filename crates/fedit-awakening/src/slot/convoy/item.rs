//! Item domain enumerations, weapon classifications, and inventory slots.
//!
//! Owns canonical weapon, staff, consumable, and seal definitions for FE13,
//! as well as item slot instances with durability and forging data.

use std::fmt;

use crate::error;

pub mod forge;

/// Category of an item determining combat proficiencies and equipment rules.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum ItemKind {
  /// Swords and katanas.
  Sword,
  /// Lances and naginatas.
  Lance,
  /// Axes and clubs.
  Axe,
  /// Bows and yumis.
  Bow,
  /// Elemental, light, and dark magic tomes.
  Tome,
  /// Healing and utility staves.
  Staff,
  /// Dragonstones and beaststones.
  Stone,
  /// Healing and restorative consumables.
  Consumable,
  /// Class progression and reclassing seals.
  Seal,
  /// Permanent stat enhancement items.
  StatBooster,
  /// Valuables and bullion.
  Valuable,
  /// Unrecognized or generic item category.
  Unknown,
}

impl ItemKind {
  /// Returns true if this item kind can be equipped into a unit's active hand.
  #[must_use]
  pub const fn is_equippable(&self) -> bool {
    match self {
      Self::Sword
      | Self::Lance
      | Self::Axe
      | Self::Bow
      | Self::Tome
      | Self::Staff
      | Self::Stone => true,
      Self::Consumable
      | Self::Seal
      | Self::StatBooster
      | Self::Valuable
      | Self::Unknown => false,
    }
  }
}

/// Strongly typed canonical item enumeration for Fire Emblem: Awakening.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Item {
  // Swords
  /// Bronze Sword (rank E, 50 uses).
  BronzeSword,
  /// Iron Sword (rank D, 40 uses).
  IronSword,
  /// Steel Sword (rank C, 35 uses).
  SteelSword,
  /// Silver Sword (rank B, 30 uses).
  SilverSword,
  /// Brave Sword (rank A, 30 uses).
  BraveSword,
  /// Killing Edge (rank C, 30 uses).
  KillingEdge,
  /// Wyrmslayer (rank C, 30 uses).
  Wyrmslayer,
  /// Armorslayer (rank D, 30 uses).
  Armorslayer,
  /// Rapier (Chrom/Lucina personal, 35 uses).
  Rapier,
  /// Noble Rapier (Chrom/Lucina personal, 25 uses).
  NobleRapier,
  /// Falchion (Chrom personal, infinite).
  Falchion,
  /// Parallel Falchion (Lucina personal, infinite).
  ParallelFalchion,
  /// Exalted Falchion (Chrom Awakened personal, infinite).
  ExaltedFalchion,
  /// Levin Sword (rank C magic sword, 25 uses).
  LevinSword,
  /// Wo Dao (rank B, 30 uses).
  WoDao,
  /// Mystletainn (rank A, 25 uses).
  Mystletainn,
  /// Tyrfing (rank A, 25 uses).
  Tyrfing,
  /// Ragnell (rank A, 25 uses).
  Ragnell,

  // Lances
  /// Bronze Lance (rank E, 50 uses).
  BronzeLance,
  /// Iron Lance (rank D, 40 uses).
  IronLance,
  /// Steel Lance (rank C, 35 uses).
  SteelLance,
  /// Silver Lance (rank B, 30 uses).
  SilverLance,
  /// Brave Lance (rank A, 30 uses).
  BraveLance,
  /// Killer Lance (rank C, 30 uses).
  KillerLance,
  /// Javelin (rank D 1-2 range, 25 uses).
  Javelin,
  /// Short Spear (rank C 1-2 range, 20 uses).
  ShortSpear,
  /// Spear (rank B 1-2 range, 15 uses).
  Spear,
  /// Horseslayer (rank D, 30 uses).
  Horseslayer,
  /// Beast Killer (rank C, 30 uses).
  BeastKiller,
  /// Blessed Lance (rank C, 30 uses).
  BlessedLance,
  /// Gradivus (rank A, 25 uses).
  Gradivus,

  // Axes
  /// Bronze Axe (rank E, 50 uses).
  BronzeAxe,
  /// Iron Axe (rank D, 40 uses).
  IronAxe,
  /// Steel Axe (rank C, 35 uses).
  SteelAxe,
  /// Silver Axe (rank B, 30 uses).
  SilverAxe,
  /// Brave Axe (rank A, 30 uses).
  BraveAxe,
  /// Killer Axe (rank C, 30 uses).
  KillerAxe,
  /// Hand Axe (rank D 1-2 range, 25 uses).
  HandAxe,
  /// Short Axe (rank C 1-2 range, 20 uses).
  ShortAxe,
  /// Tomahawk (rank B 1-2 range, 15 uses).
  Tomahawk,
  /// Hammer (rank D effective, 30 uses).
  Hammer,
  /// Bolt Axe (rank C magic axe, 25 uses).
  BoltAxe,
  /// Armads (rank A, 25 uses).
  Armads,
  /// Hauteclere (rank A, 25 uses).
  Hauteclere,
  /// Helswath (rank A, 25 uses).
  Helswath,

  // Bows
  /// Bronze Bow (rank E, 50 uses).
  BronzeBow,
  /// Iron Bow (rank D, 40 uses).
  IronBow,
  /// Steel Bow (rank C, 35 uses).
  SteelBow,
  /// Silver Bow (rank B, 30 uses).
  SilverBow,
  /// Brave Bow (rank A, 30 uses).
  BraveBow,
  /// Killer Bow (rank C, 30 uses).
  KillerBow,
  /// Short Bow (rank C, 25 uses).
  ShortBow,
  /// Longbow (rank D 2-3 range, 20 uses).
  Longbow,
  /// Blessed Bow (rank C, 30 uses).
  BlessedBow,
  /// Parthia (rank A, 25 uses).
  Parthia,
  /// Yewfelle (rank A, 25 uses).
  Yewfelle,

  // Tomes
  /// Fire (rank E, 45 uses).
  Fire,
  /// Elfire (rank D, 35 uses).
  Elfire,
  /// Arcfire (rank C, 30 uses).
  Arcfire,
  /// Bolganone (rank B, 25 uses).
  Bolganone,
  /// Valflame (rank A, 20 uses).
  Valflame,
  /// Wind (rank E, 45 uses).
  Wind,
  /// Elwind (rank D, 35 uses).
  Elwind,
  /// Arcwind (rank C, 30 uses).
  Arcwind,
  /// Rexcalibur (rank B, 25 uses).
  Rexcalibur,
  /// Forseti (rank A, 20 uses).
  Forseti,
  /// Thunder (rank E, 45 uses).
  Thunder,
  /// Elthunder (rank D, 35 uses).
  Elthunder,
  /// Arcthunder (rank C, 30 uses).
  Arcthunder,
  /// Thoron (rank B, 25 uses).
  Thoron,
  /// Mjolnir (rank A, 20 uses).
  Mjolnir,
  /// Flux (rank E dark, 45 uses).
  Flux,
  /// Nosferatu (rank D dark drain, 20 uses).
  Nosferatu,
  /// Ruin (rank C dark, 20 uses).
  Ruin,
  /// Waste (rank B dark brave, 20 uses).
  Waste,
  /// Goetia (rank A dark, 20 uses).
  Goetia,
  /// Mire (rank D dark siege, 10 uses).
  Mire,
  /// Aversa's Night (rank A dark drain, 15 uses).
  AversasNight,
  /// Grima's Truth (enemy dark tome, 20 uses).
  GrimasTruth,
  /// Slime (dark siege tome, 5 uses).
  Slime,

  // Staves
  /// Heal (rank E, 30 uses).
  Heal,
  /// Mend (rank D, 20 uses).
  Mend,
  /// Recover (rank C, 15 uses).
  Recover,
  /// Physic (rank B range heal, 15 uses).
  Physic,
  /// Fortify (rank A all heal, 8 uses).
  Fortify,
  /// Rescue (rank E warp ally, 5 uses).
  Rescue,
  /// Warp (rank A warp ally, 5 uses).
  Warp,
  /// Hammerne (rank C repair, 3 uses).
  Hammerne,

  // Stones
  /// Dragonstone (manakete, 50 uses).
  Dragonstone,
  /// Dragonstone+ (manakete, 30 uses).
  DragonstonePlus,
  /// Beaststone (taguel, 50 uses).
  Beaststone,
  /// Beaststone+ (taguel, 30 uses).
  BeaststonePlus,

  // Consumables & Boosters
  /// Vulnerary (3 uses).
  Vulnerary,
  /// Concoction (3 uses).
  Concoction,
  /// Elixir (3 uses).
  Elixir,
  /// Pure Water (3 uses).
  PureWater,
  /// Master Seal (promotes base class, 1 use).
  MasterSeal,
  /// Second Seal (reclasses unit, 1 use).
  SecondSeal,
  /// Energy Drop (+2 Str permanently, 1 use).
  EnergyDrop,
  /// Spirit Dust (+2 Mag permanently, 1 use).
  SpiritDust,
  /// Secret Book (+2 Skl permanently, 1 use).
  SecretBook,
  /// Speedwing (+2 Spd permanently, 1 use).
  Speedwing,
  /// Goddess Icon (+2 Lck permanently, 1 use).
  GoddessIcon,
  /// Dracoshield (+2 Def permanently, 1 use).
  Dracoshield,
  /// Talisman (+2 Res permanently, 1 use).
  Talisman,
  /// Seraph Robe (+5 HP permanently, 1 use).
  SeraphRobe,
  /// Boots (+2 Move permanently, 1 use).
  Boots,
  /// Seed of Trust (increases support gain, 1 use).
  SeedOfTrust,
  /// Bullion (S) (sells for 1,000 gold).
  BullionSmall,
  /// Bullion (M) (sells for 5,000 gold).
  BullionMedium,
  /// Bullion (L) (sells for 10,000 gold).
  BullionLarge,

  /// Unknown or custom DLC item identifier.
  Unknown(u16),
}

impl Item {
  /// Converts a raw 16-bit identifier into an [`Item`] variant.
  #[must_use]
  #[expect(
    clippy::too_many_lines,
    reason = "exhaustive match over all FE13 item variants"
  )]
  pub const fn from_raw(raw: u16) -> Self {
    match raw {
      1 => Self::BronzeSword,
      2 => Self::IronSword,
      3 => Self::SteelSword,
      4 => Self::SilverSword,
      5 => Self::BraveSword,
      6 => Self::KillingEdge,
      7 => Self::Wyrmslayer,
      8 => Self::Armorslayer,
      9 => Self::Rapier,
      10 => Self::NobleRapier,
      15 => Self::Falchion,
      16 => Self::ParallelFalchion,
      17 => Self::ExaltedFalchion,
      36 => Self::LevinSword,
      37 => Self::WoDao,
      38 => Self::Mystletainn,
      39 => Self::Tyrfing,
      40 => Self::Ragnell,
      41 => Self::BronzeLance,
      42 => Self::IronLance,
      43 => Self::SteelLance,
      44 => Self::SilverLance,
      45 => Self::BraveLance,
      46 => Self::KillerLance,
      48 => Self::Javelin,
      49 => Self::ShortSpear,
      50 => Self::Spear,
      51 => Self::Horseslayer,
      52 => Self::BeastKiller,
      53 => Self::BlessedLance,
      58 => Self::Gradivus,
      70 => Self::BronzeAxe,
      71 => Self::IronAxe,
      72 => Self::SteelAxe,
      73 => Self::SilverAxe,
      74 => Self::BraveAxe,
      75 => Self::KillerAxe,
      78 => Self::HandAxe,
      79 => Self::ShortAxe,
      80 => Self::Tomahawk,
      81 => Self::Hammer,
      83 => Self::BoltAxe,
      88 => Self::Armads,
      89 => Self::Hauteclere,
      90 => Self::Helswath,
      100 => Self::BronzeBow,
      101 => Self::IronBow,
      102 => Self::SteelBow,
      103 => Self::SilverBow,
      104 => Self::BraveBow,
      105 => Self::KillerBow,
      106 => Self::ShortBow,
      107 => Self::Longbow,
      108 => Self::BlessedBow,
      115 => Self::Parthia,
      116 => Self::Yewfelle,
      120 => Self::Fire,
      121 => Self::Elfire,
      122 => Self::Arcfire,
      123 => Self::Bolganone,
      124 => Self::Valflame,
      125 => Self::Wind,
      126 => Self::Elwind,
      127 => Self::Arcwind,
      128 => Self::Rexcalibur,
      129 => Self::Forseti,
      130 => Self::Thunder,
      131 => Self::Elthunder,
      132 => Self::Arcthunder,
      133 => Self::Thoron,
      134 => Self::Mjolnir,
      135 => Self::Flux,
      136 => Self::Nosferatu,
      137 => Self::Ruin,
      138 => Self::Waste,
      139 => Self::Goetia,
      140 => Self::Mire,
      141 => Self::AversasNight,
      142 => Self::GrimasTruth,
      143 => Self::Slime,
      145 => Self::Heal,
      146 => Self::Mend,
      147 => Self::Recover,
      148 => Self::Physic,
      149 => Self::Fortify,
      150 => Self::Rescue,
      151 => Self::Warp,
      152 => Self::Hammerne,
      160 => Self::Dragonstone,
      161 => Self::DragonstonePlus,
      162 => Self::Beaststone,
      163 => Self::BeaststonePlus,
      170 => Self::Vulnerary,
      171 => Self::Concoction,
      172 => Self::Elixir,
      173 => Self::PureWater,
      175 => Self::MasterSeal,
      176 => Self::SecondSeal,
      180 => Self::EnergyDrop,
      181 => Self::SpiritDust,
      182 => Self::SecretBook,
      183 => Self::Speedwing,
      184 => Self::GoddessIcon,
      185 => Self::Dracoshield,
      186 => Self::Talisman,
      187 => Self::SeraphRobe,
      188 => Self::Boots,
      190 => Self::SeedOfTrust,
      191 => Self::BullionSmall,
      192 => Self::BullionMedium,
      193 => Self::BullionLarge,
      other => Self::Unknown(other),
    }
  }

  /// Converts an [`Item`] variant back into its raw 16-bit game ID.
  #[must_use]
  #[expect(
    clippy::too_many_lines,
    reason = "exhaustive match over all FE13 item variants"
  )]
  pub const fn raw_id(&self) -> u16 {
    match self {
      Self::BronzeSword => 1,
      Self::IronSword => 2,
      Self::SteelSword => 3,
      Self::SilverSword => 4,
      Self::BraveSword => 5,
      Self::KillingEdge => 6,
      Self::Wyrmslayer => 7,
      Self::Armorslayer => 8,
      Self::Rapier => 9,
      Self::NobleRapier => 10,
      Self::Falchion => 15,
      Self::ParallelFalchion => 16,
      Self::ExaltedFalchion => 17,
      Self::LevinSword => 36,
      Self::WoDao => 37,
      Self::Mystletainn => 38,
      Self::Tyrfing => 39,
      Self::Ragnell => 40,
      Self::BronzeLance => 41,
      Self::IronLance => 42,
      Self::SteelLance => 43,
      Self::SilverLance => 44,
      Self::BraveLance => 45,
      Self::KillerLance => 46,
      Self::Javelin => 48,
      Self::ShortSpear => 49,
      Self::Spear => 50,
      Self::Horseslayer => 51,
      Self::BeastKiller => 52,
      Self::BlessedLance => 53,
      Self::Gradivus => 58,
      Self::BronzeAxe => 70,
      Self::IronAxe => 71,
      Self::SteelAxe => 72,
      Self::SilverAxe => 73,
      Self::BraveAxe => 74,
      Self::KillerAxe => 75,
      Self::HandAxe => 78,
      Self::ShortAxe => 79,
      Self::Tomahawk => 80,
      Self::Hammer => 81,
      Self::BoltAxe => 83,
      Self::Armads => 88,
      Self::Hauteclere => 89,
      Self::Helswath => 90,
      Self::BronzeBow => 100,
      Self::IronBow => 101,
      Self::SteelBow => 102,
      Self::SilverBow => 103,
      Self::BraveBow => 104,
      Self::KillerBow => 105,
      Self::ShortBow => 106,
      Self::Longbow => 107,
      Self::BlessedBow => 108,
      Self::Parthia => 115,
      Self::Yewfelle => 116,
      Self::Fire => 120,
      Self::Elfire => 121,
      Self::Arcfire => 122,
      Self::Bolganone => 123,
      Self::Valflame => 124,
      Self::Wind => 125,
      Self::Elwind => 126,
      Self::Arcwind => 127,
      Self::Rexcalibur => 128,
      Self::Forseti => 129,
      Self::Thunder => 130,
      Self::Elthunder => 131,
      Self::Arcthunder => 132,
      Self::Thoron => 133,
      Self::Mjolnir => 134,
      Self::Flux => 135,
      Self::Nosferatu => 136,
      Self::Ruin => 137,
      Self::Waste => 138,
      Self::Goetia => 139,
      Self::Mire => 140,
      Self::AversasNight => 141,
      Self::GrimasTruth => 142,
      Self::Slime => 143,
      Self::Heal => 145,
      Self::Mend => 146,
      Self::Recover => 147,
      Self::Physic => 148,
      Self::Fortify => 149,
      Self::Rescue => 150,
      Self::Warp => 151,
      Self::Hammerne => 152,
      Self::Dragonstone => 160,
      Self::DragonstonePlus => 161,
      Self::Beaststone => 162,
      Self::BeaststonePlus => 163,
      Self::Vulnerary => 170,
      Self::Concoction => 171,
      Self::Elixir => 172,
      Self::PureWater => 173,
      Self::MasterSeal => 175,
      Self::SecondSeal => 176,
      Self::EnergyDrop => 180,
      Self::SpiritDust => 181,
      Self::SecretBook => 182,
      Self::Speedwing => 183,
      Self::GoddessIcon => 184,
      Self::Dracoshield => 185,
      Self::Talisman => 186,
      Self::SeraphRobe => 187,
      Self::Boots => 188,
      Self::SeedOfTrust => 190,
      Self::BullionSmall => 191,
      Self::BullionMedium => 192,
      Self::BullionLarge => 193,
      Self::Unknown(id) => *id,
    }
  }

  /// Returns the human-readable canonical name of the item.
  #[must_use]
  #[expect(
    clippy::too_many_lines,
    reason = "exhaustive match over all FE13 item variants"
  )]
  pub const fn name(&self) -> &'static str {
    match self {
      Self::BronzeSword => "Bronze Sword",
      Self::IronSword => "Iron Sword",
      Self::SteelSword => "Steel Sword",
      Self::SilverSword => "Silver Sword",
      Self::BraveSword => "Brave Sword",
      Self::KillingEdge => "Killing Edge",
      Self::Wyrmslayer => "Wyrmslayer",
      Self::Armorslayer => "Armorslayer",
      Self::Rapier => "Rapier",
      Self::NobleRapier => "Noble Rapier",
      Self::Falchion => "Falchion",
      Self::ParallelFalchion => "Parallel Falchion",
      Self::ExaltedFalchion => "Exalted Falchion",
      Self::LevinSword => "Levin Sword",
      Self::WoDao => "Wo Dao",
      Self::Mystletainn => "Mystletainn",
      Self::Tyrfing => "Tyrfing",
      Self::Ragnell => "Ragnell",
      Self::BronzeLance => "Bronze Lance",
      Self::IronLance => "Iron Lance",
      Self::SteelLance => "Steel Lance",
      Self::SilverLance => "Silver Lance",
      Self::BraveLance => "Brave Lance",
      Self::KillerLance => "Killer Lance",
      Self::Javelin => "Javelin",
      Self::ShortSpear => "Short Spear",
      Self::Spear => "Spear",
      Self::Horseslayer => "Horseslayer",
      Self::BeastKiller => "Beast Killer",
      Self::BlessedLance => "Blessed Lance",
      Self::Gradivus => "Gradivus",
      Self::BronzeAxe => "Bronze Axe",
      Self::IronAxe => "Iron Axe",
      Self::SteelAxe => "Steel Axe",
      Self::SilverAxe => "Silver Axe",
      Self::BraveAxe => "Brave Axe",
      Self::KillerAxe => "Killer Axe",
      Self::HandAxe => "Hand Axe",
      Self::ShortAxe => "Short Axe",
      Self::Tomahawk => "Tomahawk",
      Self::Hammer => "Hammer",
      Self::BoltAxe => "Bolt Axe",
      Self::Armads => "Armads",
      Self::Hauteclere => "Hauteclere",
      Self::Helswath => "Helswath",
      Self::BronzeBow => "Bronze Bow",
      Self::IronBow => "Iron Bow",
      Self::SteelBow => "Steel Bow",
      Self::SilverBow => "Silver Bow",
      Self::BraveBow => "Brave Bow",
      Self::KillerBow => "Killer Bow",
      Self::ShortBow => "Short Bow",
      Self::Longbow => "Longbow",
      Self::BlessedBow => "Blessed Bow",
      Self::Parthia => "Parthia",
      Self::Yewfelle => "Yewfelle",
      Self::Fire => "Fire",
      Self::Elfire => "Elfire",
      Self::Arcfire => "Arcfire",
      Self::Bolganone => "Bolganone",
      Self::Valflame => "Valflame",
      Self::Wind => "Wind",
      Self::Elwind => "Elwind",
      Self::Arcwind => "Arcwind",
      Self::Rexcalibur => "Rexcalibur",
      Self::Forseti => "Forseti",
      Self::Thunder => "Thunder",
      Self::Elthunder => "Elthunder",
      Self::Arcthunder => "Arcthunder",
      Self::Thoron => "Thoron",
      Self::Mjolnir => "Mjolnir",
      Self::Flux => "Flux",
      Self::Nosferatu => "Nosferatu",
      Self::Ruin => "Ruin",
      Self::Waste => "Waste",
      Self::Goetia => "Goetia",
      Self::Mire => "Mire",
      Self::AversasNight => "Aversa's Night",
      Self::GrimasTruth => "Grima's Truth",
      Self::Slime => "Slime",
      Self::Heal => "Heal",
      Self::Mend => "Mend",
      Self::Recover => "Recover",
      Self::Physic => "Physic",
      Self::Fortify => "Fortify",
      Self::Rescue => "Rescue",
      Self::Warp => "Warp",
      Self::Hammerne => "Hammerne",
      Self::Dragonstone => "Dragonstone",
      Self::DragonstonePlus => "Dragonstone+",
      Self::Beaststone => "Beaststone",
      Self::BeaststonePlus => "Beaststone+",
      Self::Vulnerary => "Vulnerary",
      Self::Concoction => "Concoction",
      Self::Elixir => "Elixir",
      Self::PureWater => "Pure Water",
      Self::MasterSeal => "Master Seal",
      Self::SecondSeal => "Second Seal",
      Self::EnergyDrop => "Energy Drop",
      Self::SpiritDust => "Spirit Dust",
      Self::SecretBook => "Secret Book",
      Self::Speedwing => "Speedwing",
      Self::GoddessIcon => "Goddess Icon",
      Self::Dracoshield => "Dracoshield",
      Self::Talisman => "Talisman",
      Self::SeraphRobe => "Seraph Robe",
      Self::Boots => "Boots",
      Self::SeedOfTrust => "Seed of Trust",
      Self::BullionSmall => "Bullion (S)",
      Self::BullionMedium => "Bullion (M)",
      Self::BullionLarge => "Bullion (L)",
      Self::Unknown(_) => "Unknown Item",
    }
  }

  /// Returns the category of the item.
  #[must_use]
  #[expect(
    clippy::too_many_lines,
    reason = "exhaustive match over all FE13 item variants"
  )]
  pub const fn kind(&self) -> ItemKind {
    match self {
      Self::BronzeSword
      | Self::IronSword
      | Self::SteelSword
      | Self::SilverSword
      | Self::BraveSword
      | Self::KillingEdge
      | Self::Wyrmslayer
      | Self::Armorslayer
      | Self::Rapier
      | Self::NobleRapier
      | Self::Falchion
      | Self::ParallelFalchion
      | Self::ExaltedFalchion
      | Self::LevinSword
      | Self::WoDao
      | Self::Mystletainn
      | Self::Tyrfing
      | Self::Ragnell => ItemKind::Sword,

      Self::BronzeLance
      | Self::IronLance
      | Self::SteelLance
      | Self::SilverLance
      | Self::BraveLance
      | Self::KillerLance
      | Self::Javelin
      | Self::ShortSpear
      | Self::Spear
      | Self::Horseslayer
      | Self::BeastKiller
      | Self::BlessedLance
      | Self::Gradivus => ItemKind::Lance,

      Self::BronzeAxe
      | Self::IronAxe
      | Self::SteelAxe
      | Self::SilverAxe
      | Self::BraveAxe
      | Self::KillerAxe
      | Self::HandAxe
      | Self::ShortAxe
      | Self::Tomahawk
      | Self::Hammer
      | Self::BoltAxe
      | Self::Armads
      | Self::Hauteclere
      | Self::Helswath => ItemKind::Axe,

      Self::BronzeBow
      | Self::IronBow
      | Self::SteelBow
      | Self::SilverBow
      | Self::BraveBow
      | Self::KillerBow
      | Self::ShortBow
      | Self::Longbow
      | Self::BlessedBow
      | Self::Parthia
      | Self::Yewfelle => ItemKind::Bow,

      Self::Fire
      | Self::Elfire
      | Self::Arcfire
      | Self::Bolganone
      | Self::Valflame
      | Self::Wind
      | Self::Elwind
      | Self::Arcwind
      | Self::Rexcalibur
      | Self::Forseti
      | Self::Thunder
      | Self::Elthunder
      | Self::Arcthunder
      | Self::Thoron
      | Self::Mjolnir
      | Self::Flux
      | Self::Nosferatu
      | Self::Ruin
      | Self::Waste
      | Self::Goetia
      | Self::Mire
      | Self::AversasNight
      | Self::GrimasTruth
      | Self::Slime => ItemKind::Tome,

      Self::Heal
      | Self::Mend
      | Self::Recover
      | Self::Physic
      | Self::Fortify
      | Self::Rescue
      | Self::Warp
      | Self::Hammerne => ItemKind::Staff,

      Self::Dragonstone
      | Self::DragonstonePlus
      | Self::Beaststone
      | Self::BeaststonePlus => ItemKind::Stone,

      Self::Vulnerary | Self::Concoction | Self::Elixir | Self::PureWater => {
        ItemKind::Consumable
      }

      Self::MasterSeal | Self::SecondSeal => ItemKind::Seal,

      Self::EnergyDrop
      | Self::SpiritDust
      | Self::SecretBook
      | Self::Speedwing
      | Self::GoddessIcon
      | Self::Dracoshield
      | Self::Talisman
      | Self::SeraphRobe
      | Self::Boots
      | Self::SeedOfTrust => ItemKind::StatBooster,

      Self::BullionSmall | Self::BullionMedium | Self::BullionLarge => {
        ItemKind::Valuable
      }

      Self::Unknown(_) => ItemKind::Unknown,
    }
  }

  /// Returns canonical maximum durability, or `None` if unbreakable or infinite.
  #[must_use]
  #[expect(
    clippy::too_many_lines,
    reason = "exhaustive durability mapping for all items"
  )]
  pub const fn max_durability(&self) -> Option<u8> {
    match self {
      Self::BronzeSword
      | Self::BronzeLance
      | Self::BronzeAxe
      | Self::BronzeBow
      | Self::Dragonstone
      | Self::Beaststone => Some(50),
      Self::IronSword | Self::IronLance | Self::IronAxe | Self::IronBow => {
        Some(40)
      }
      Self::SteelSword
      | Self::SteelLance
      | Self::SteelAxe
      | Self::SteelBow
      | Self::Rapier
      | Self::Elfire
      | Self::Elwind
      | Self::Elthunder => Some(35),
      Self::SilverSword
      | Self::SilverLance
      | Self::SilverAxe
      | Self::SilverBow
      | Self::BraveSword
      | Self::BraveLance
      | Self::BraveAxe
      | Self::BraveBow
      | Self::KillingEdge
      | Self::KillerLance
      | Self::KillerAxe
      | Self::KillerBow
      | Self::Wyrmslayer
      | Self::Armorslayer
      | Self::Horseslayer
      | Self::BeastKiller
      | Self::BlessedLance
      | Self::BlessedBow
      | Self::Hammer
      | Self::Arcfire
      | Self::Arcwind
      | Self::Arcthunder
      | Self::WoDao
      | Self::Heal
      | Self::DragonstonePlus
      | Self::BeaststonePlus => Some(30),

      Self::NobleRapier
      | Self::LevinSword
      | Self::Mystletainn
      | Self::Tyrfing
      | Self::Ragnell
      | Self::Javelin
      | Self::HandAxe
      | Self::BoltAxe
      | Self::Armads
      | Self::Hauteclere
      | Self::Helswath
      | Self::ShortBow
      | Self::Parthia
      | Self::Yewfelle
      | Self::Bolganone
      | Self::Rexcalibur
      | Self::Thoron
      | Self::Gradivus => Some(25),

      Self::ShortSpear
      | Self::ShortAxe
      | Self::Longbow
      | Self::Nosferatu
      | Self::Ruin
      | Self::Waste
      | Self::Goetia
      | Self::Valflame
      | Self::Forseti
      | Self::Mjolnir
      | Self::GrimasTruth
      | Self::Mend => Some(20),

      Self::Spear
      | Self::Tomahawk
      | Self::AversasNight
      | Self::Recover
      | Self::Physic => Some(15),

      Self::Fire | Self::Wind | Self::Thunder | Self::Flux => Some(45),

      Self::Mire => Some(10),
      Self::Slime | Self::Rescue | Self::Warp => Some(5),
      Self::Fortify => Some(8),
      Self::Hammerne
      | Self::Vulnerary
      | Self::Concoction
      | Self::Elixir
      | Self::PureWater => Some(3),

      Self::MasterSeal
      | Self::SecondSeal
      | Self::EnergyDrop
      | Self::SpiritDust
      | Self::SecretBook
      | Self::Speedwing
      | Self::GoddessIcon
      | Self::Dracoshield
      | Self::Talisman
      | Self::SeraphRobe
      | Self::Boots
      | Self::SeedOfTrust
      | Self::BullionSmall
      | Self::BullionMedium
      | Self::BullionLarge => Some(1),

      // Infinite/unbreakable weapons
      Self::Falchion
      | Self::ParallelFalchion
      | Self::ExaltedFalchion
      | Self::Unknown(_) => None,
    }
  }

  /// Returns true if this item has infinite durability.
  #[must_use]
  pub const fn is_unbreakable(&self) -> bool {
    matches!(
      self,
      Self::Falchion | Self::ParallelFalchion | Self::ExaltedFalchion
    )
  }

  /// Returns true if this item can be equipped as a weapon or staff.
  #[must_use]
  pub const fn is_equippable(&self) -> bool {
    self.kind().is_equippable()
  }
}

impl fmt::Display for Item {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Unknown(id) => write!(f, "Unknown Item ({id})"),
      _ => write!(f, "{}", self.name()),
    }
  }
}

/// An individual item slot within a unit's inventory or convoy storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemSlot {
  /// Base item enumeration.
  item: Item,
  /// Current remaining durability uses or stack count.
  uses: u16,
  /// Optional forge modifications.
  forge: Option<forge::Forge>,
}

impl ItemSlot {
  /// Creates a new un-forged item slot with default canonical durability.
  #[must_use]
  pub fn new(item: Item) -> Self {
    let uses = u16::from(item.max_durability().unwrap_or(1));
    Self {
      item,
      uses,
      forge: None,
    }
  }

  /// Creates an item slot with a specified durability uses count.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::DurabilityOverflow`] if `uses > 4095`.
  pub fn with_uses(item: Item, uses: u16) -> Result<Self, error::Error> {
    if uses > 0x0fff {
      return Err(error::Error::DurabilityOverflow { uses, max: 0x0fff });
    }
    Ok(Self {
      item,
      uses,
      forge: None,
    })
  }

  /// Returns the base item variant.
  #[must_use]
  pub const fn item(&self) -> Item {
    self.item
  }

  /// Returns the remaining uses count.
  #[must_use]
  pub const fn uses(&self) -> u16 {
    self.uses
  }

  /// Returns a reference to the forge modifications, if any.
  #[must_use]
  pub const fn forge(&self) -> Option<&forge::Forge> {
    self.forge.as_ref()
  }

  /// Returns a mutable reference to the forge modifications, if any.
  pub fn forge_mut(&mut self) -> Option<&mut forge::Forge> {
    self.forge.as_mut()
  }

  /// Sets the base item variant, resetting uses to canonical max if applicable.
  pub fn set_item(&mut self, item: Item) {
    self.item = item;
    if let Some(max) = item.max_durability()
      && self.uses > u16::from(max)
    {
      self.uses = u16::from(max);
    }
  }

  /// Sets the remaining uses count with bounds validation.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::DurabilityOverflow`] if `uses > 4095`.
  pub fn set_uses(&mut self, uses: u16) -> Result<(), error::Error> {
    if uses > 0x0fff {
      return Err(error::Error::DurabilityOverflow { uses, max: 0x0fff });
    }
    self.uses = uses;
    Ok(())
  }

  /// Sets the forge modifications on this item slot.
  pub fn set_forge(&mut self, forge: Option<forge::Forge>) {
    self.forge = forge;
  }

  /// Repairs the item back to its canonical maximum durability.
  pub fn repair(&mut self) {
    if let Some(max) = self.item.max_durability() {
      self.uses = u16::from(max);
    }
  }

  /// Returns true if the weapon is broken (0 uses and not unbreakable).
  #[must_use]
  pub const fn is_broken(&self) -> bool {
    self.uses == 0 && !self.item.is_unbreakable()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_item_roundtrip_and_properties() {
    let falchion = Item::Falchion;
    assert_eq!(falchion.name(), "Falchion");
    assert_eq!(falchion.kind(), ItemKind::Sword);
    assert!(falchion.is_unbreakable());
    assert!(falchion.is_equippable());
    assert_eq!(Item::from_raw(falchion.raw_id()), falchion);

    let vuln = Item::Vulnerary;
    assert_eq!(vuln.name(), "Vulnerary");
    assert_eq!(vuln.kind(), ItemKind::Consumable);
    assert!(!vuln.is_equippable());
    assert_eq!(vuln.max_durability(), Some(3));
  }

  #[test]
  fn test_unknown_item_fallback() {
    let unknown = Item::from_raw(9999);
    assert_eq!(unknown, Item::Unknown(9999));
    assert_eq!(unknown.raw_id(), 9999);
    assert_eq!(unknown.name(), "Unknown Item");
    assert_eq!(unknown.kind(), ItemKind::Unknown);
    assert!(!unknown.is_equippable());
  }
}
