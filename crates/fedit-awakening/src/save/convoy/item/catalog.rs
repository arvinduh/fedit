//! Canonical Fire Emblem Awakening item definitions and classifications.
//!
//! Provides the exhaustive strongly-typed enumeration of weapons, staves,
//! stones, consumables, and seals, mapping to and from raw game item IDs.

use std::fmt;

use crate::save::convoy::item::kind;

/// Static metadata descriptor for an item variant.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ItemInfo {
  /// Canonical item display name.
  pub name: &'static str,
  /// 16-bit raw game item identifier.
  pub raw_id: u16,
  /// Broad equipment category.
  pub kind: kind::ItemKind,
  /// Base weapon might (attack power).
  pub might: u8,
  /// Base weapon accuracy hit rate percentage.
  pub hit: u8,
  /// Base weapon critical hit rate percentage.
  pub crit: u8,
  /// Canonical maximum durability, or `None` if unbreakable.
  pub max_durability: Option<u8>,
}

/// Strongly typed canonical item enumeration for Fire Emblem: Awakening.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ItemType {
  /// Bronze Sword (50 uses).
  BronzeSword,
  /// Iron Sword (40 uses).
  IronSword,
  /// Steel Sword (35 uses).
  SteelSword,
  /// Silver Sword (30 uses).
  SilverSword,
  /// Brave Sword (30 uses).
  BraveSword,
  /// Armorslayer (25 uses).
  Armorslayer,
  /// Wyrmslayer (25 uses).
  Wyrmslayer,
  /// Killing Edge (30 uses).
  KillingEdge,
  /// Levin Sword (25 uses).
  LevinSword,
  /// Rapier (35 uses).
  Rapier,
  /// Noble Rapier (25 uses).
  NobleRapier,
  /// Missiletainn (35 uses).
  Missiletainn,
  /// Sol (30 uses).
  Sol,
  /// Amatsu (30 uses).
  Amatsu,
  /// Falchion (0 uses).
  Falchion,
  /// Exalted Falchion (0 uses).
  ExaltedFalchion,
  /// Parallel Falchion (0 uses).
  ParallelFalchion,
  /// Mercurius (25 uses).
  Mercurius,
  /// Tyrfing (25 uses).
  Tyrfing,
  /// Mystletainn (25 uses).
  Mystletainn,
  /// Balmung (25 uses).
  Balmung,
  /// Sol Katti (25 uses).
  SolKatti,
  /// Ragnell (25 uses).
  Ragnell,
  /// Ragnell (Inf. Uses) (0 uses).
  RagnellInfinite,
  /// Tree Branch (20 uses).
  TreeBranch,
  /// Soothing Sword (10 uses).
  SoothingSword,
  /// Glass Sword (3 uses).
  GlassSword,
  /// Superior Edge (10 uses).
  SuperiorEdge,
  /// Eliwood's Blade (20 uses).
  EliwoodsBlade,
  /// Roy's Blade (25 uses).
  RoysBlade,
  /// Alm's Blade (10 uses).
  AlmsBlade,
  /// Leif's Blade (20 uses).
  LeifsBlade,
  /// Eirika's Blade (20 uses).
  EirikasBlade,
  /// Seliph's Blade (15 uses).
  SeliphsBlade,
  /// Bronze Lance (50 uses).
  BronzeLance,
  /// Iron Lance (40 uses).
  IronLance,
  /// Steel Lance (35 uses).
  SteelLance,
  /// Silver Lance (30 uses).
  SilverLance,
  /// Brave Lance (30 uses).
  BraveLance,
  /// Javelin (25 uses).
  Javelin,
  /// Short Spear (25 uses).
  ShortSpear,
  /// Spear (25 uses).
  Spear,
  /// Beast Killer (25 uses).
  BeastKiller,
  /// Blessed Lance (35 uses).
  BlessedLance,
  /// Killer Lance (30 uses).
  KillerLance,
  /// Luna (30 uses).
  Luna,
  /// Gradivus (25 uses).
  Gradivus,
  /// Gungnir (25 uses).
  Gungnir,
  /// Gae Bolg (25 uses).
  GaeBolg,
  /// Log (20 uses).
  Log,
  /// Miniature Lance (10 uses).
  MiniatureLance,
  /// Shockstick (20 uses).
  Shockstick,
  /// Glass Lance (3 uses).
  GlassLance,
  /// Superior Lance (10 uses).
  SuperiorLance,
  /// Sigurd's Lance (15 uses).
  SigurdsLance,
  /// Ephraim's Lance (20 uses).
  EphraimsLance,
  /// Finn's Lance (25 uses).
  FinnsLance,
  /// Bronze Axe (50 uses).
  BronzeAxe,
  /// Iron Axe (40 uses).
  IronAxe,
  /// Steel Axe (35 uses).
  SteelAxe,
  /// Silver Axe (30 uses).
  SilverAxe,
  /// Brave Axe (30 uses).
  BraveAxe,
  /// Hand Axe (25 uses).
  HandAxe,
  /// Short Axe (25 uses).
  ShortAxe,
  /// Tomahawk (25 uses).
  Tomahawk,
  /// Hammer (25 uses).
  Hammer,
  /// Bolt Axe (30 uses).
  BoltAxe,
  /// Killer Axe (30 uses).
  KillerAxe,
  /// Vengeance (30 uses).
  Vengeance,
  /// Wolf Berg (35 uses).
  WolfBerg,
  /// Hauteclere (25 uses).
  Hauteclere,
  /// Helswath (25 uses).
  Helswath,
  /// Armads (25 uses).
  Armads,
  /// Ladle (20 uses).
  Ladle,
  /// Imposing Axe (10 uses).
  ImposingAxe,
  /// Volant Axe (10 uses).
  VolantAxe,
  /// Glass Axe (3 uses).
  GlassAxe,
  /// Superior Axe (10 uses).
  SuperiorAxe,
  /// Titania's Axe (20 uses).
  TitaniasAxe,
  /// Orsin's Hatchet (20 uses).
  OrsinsHatchet,
  /// Hector's Axe (15 uses).
  HectorsAxe,
  /// Bronze Bow (50 uses).
  BronzeBow,
  /// Iron Bow (40 uses).
  IronBow,
  /// Steel Bow (35 uses).
  SteelBow,
  /// Silver Bow (30 uses).
  SilverBow,
  /// Brave Bow (30 uses).
  BraveBow,
  /// Blessed Bow (35 uses).
  BlessedBow,
  /// Killer Bow (30 uses).
  KillerBow,
  /// Long Bow (25 uses).
  LongBow,
  /// Astra (30 uses).
  Astra,
  /// Parthia (25 uses).
  Parthia,
  /// Yewfelle (25 uses).
  Yewfelle,
  /// Nidhogg (25 uses).
  Nidhogg,
  /// Double Bow (25 uses).
  DoubleBow,
  /// Slack Bow (20 uses).
  SlackBow,
  /// Towering Bow (10 uses).
  ToweringBow,
  /// Underdog Bow (15 uses).
  UnderdogBow,
  /// Glass Bow (3 uses).
  GlassBow,
  /// Superior Bow (10 uses).
  SuperiorBow,
  /// Wolt's Bow (25 uses).
  WoltsBow,
  /// Innes' Bow (15 uses).
  InnesBow,
  /// Fire (45 uses).
  Fire,
  /// Elfire (35 uses).
  Elfire,
  /// Arcfire (30 uses).
  Arcfire,
  /// Bolganone (25 uses).
  Bolganone,
  /// Valflame (25 uses).
  Valflame,
  /// Thunder (45 uses).
  Thunder,
  /// Elthunder (35 uses).
  Elthunder,
  /// Arcthunder (30 uses).
  Arcthunder,
  /// Thoron (25 uses).
  Thoron,
  /// Mjollnir (25 uses).
  Mjollnir,
  /// Wind (45 uses).
  Wind,
  /// Elwind (35 uses).
  Elwind,
  /// Arcwind (30 uses).
  Arcwind,
  /// Rexcalibur (25 uses).
  Rexcalibur,
  /// Forseti (25 uses).
  Forseti,
  /// Excalibur (25 uses).
  Excalibur,
  /// Book of Naga (25 uses).
  BookOfNaga,
  /// Flux (45 uses).
  Flux,
  /// Nosferatu (20 uses).
  Nosferatu,
  /// Ruin (20 uses).
  Ruin,
  /// Waste (30 uses).
  Waste,
  /// Goetia (25 uses).
  Goetia,
  /// Grima's Truth (0 uses).
  GrimasTruth,
  /// Mire (10 uses).
  Mire,
  /// Dying Blaze (3 uses).
  DyingBlaze,
  /// Micaiah's Pyre (15 uses).
  MicaiahsPyre,
  /// Superior Jolt (10 uses).
  SuperiorJolt,
  /// Katarina's Bolt (20 uses).
  KatarinasBolt,
  /// Wilderwind (5 uses).
  Wilderwind,
  /// Celica's Gale (20 uses).
  CelicasGale,
  /// Aversa's Night (10 uses).
  AversasNight,
  /// Heal (30 uses).
  Heal,
  /// Mend (20 uses).
  Mend,
  /// Physic (10 uses).
  Physic,
  /// Recover (15 uses).
  Recover,
  /// Fortify (5 uses).
  Fortify,
  /// Goddess Staff (1 uses).
  GoddessStaff,
  /// Rescue (5 uses).
  Rescue,
  /// Ward (5 uses).
  Ward,
  /// Hammerne (1 uses).
  Hammerne,
  /// Kneader (20 uses).
  Kneader,
  /// Balmwood Staff (15 uses).
  BalmwoodStaff,
  /// Catharsis (5 uses).
  Catharsis,
  /// Dragonstone (50 uses).
  Dragonstone,
  /// Dragonstone+ (35 uses).
  Dragonstoneplus,
  /// Beaststone (50 uses).
  Beaststone,
  /// Beaststone+ (35 uses).
  Beaststoneplus,
  /// Blighted Claws (0 uses).
  BlightedClaws,
  /// Blighted Talons (0 uses).
  BlightedTalons,
  /// Expiration (0 uses).
  Expiration,
  /// Vulnerary (3 uses).
  Vulnerary,
  /// Concoction (3 uses).
  Concoction,
  /// Elixer (3 uses).
  Elixer,
  /// Pure Water (3 uses).
  PureWater,
  /// HP Tonic (1 uses).
  HpTonic,
  /// Strength Tonic (1 uses).
  StrengthTonic,
  /// Magic Tonic (1 uses).
  MagicTonic,
  /// Skill Tonic (1 uses).
  SkillTonic,
  /// Speed Tonic (1 uses).
  SpeedTonic,
  /// Luck Tonic (1 uses).
  LuckTonic,
  /// Defense Tonic (1 uses).
  DefenseTonic,
  /// Resistance Tonic (1 uses).
  ResistanceTonic,
  /// Door Key (1 uses).
  DoorKey,
  /// Chest Key (1 uses).
  ChestKey,
  /// Master Key (1 uses).
  MasterKey,
  /// Seraph Robe (1 uses).
  SeraphRobe,
  /// Energy Drop (1 uses).
  EnergyDrop,
  /// Spirit Dust (1 uses).
  SpiritDust,
  /// Secret Book (1 uses).
  SecretBook,
  /// Speedwing (1 uses).
  Speedwing,
  /// Goddess Icon (1 uses).
  GoddessIcon,
  /// Dracoshield (1 uses).
  Dracoshield,
  /// Talisman (1 uses).
  Talisman,
  /// Naga's Tear (1 uses).
  NagasTear,
  /// Boots (1 uses).
  Boots,
  /// Arms Scroll (1 uses).
  ArmsScroll,
  /// Master Seal (1 uses).
  MasterSeal,
  /// Second Seal (1 uses).
  SecondSeal,
  /// Bullion (S) (0 uses).
  BullionS,
  /// Bullion (M) (0 uses).
  BullionM,
  /// Bullion (L) (0 uses).
  BullionL,
  /// Sweet Tincture (3 uses).
  SweetTincture,
  /// Gaius' Confect (5 uses).
  GaiusConfect,
  /// Kris' Confect (5 uses).
  KrisConfect,
  /// Tiki's Tear (1 uses).
  TikisTear,
  /// Seed of Trust (1 uses).
  SeedOfTrust,
  /// Reeking Box (1 uses).
  ReekingBox,
  /// Rift Door (1 uses).
  RiftDoor,
  /// Supreme Emblem (0 uses).
  SupremeEmblem,
  /// All Stats +2 (1 uses).
  AllStatsPlus2,
  /// Paragon (1 uses).
  Paragon,
  /// Iote's Shield (1 uses).
  IotesShield,
  /// Limit Breaker (1 uses).
  LimitBreaker,
  /// Silver Card (0 uses).
  SilverCard,
  /// Dread Scroll (1 uses).
  DreadScroll,
  /// Wedding Bouquet (1 uses).
  WeddingBouquet,
  /// 1,000 Gold (0 uses).
  Gold1000,
  /// 3,000 Gold (0 uses).
  Gold3000,
  /// 5,000 Gold (0 uses).
  Gold5000,
  /// 7,000 Gold (0 uses).
  Gold7000,
  /// Unknown or custom DLC item identifier.
  Unknown(u16),
}

/// Static descriptor for unknown or uncatalogued items.
static UNKNOWN_INFO: ItemInfo = ItemInfo {
  name: "Unknown Item",
  raw_id: 0,
  kind: kind::ItemKind::Unknown,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for Bronze Sword.
static BRONZE_SWORD_INFO: ItemInfo = ItemInfo {
  name: "Bronze Sword",
  raw_id: 1,
  kind: kind::ItemKind::Sword,
  might: 3,
  hit: 100,
  crit: 0,
  max_durability: Some(50),
};

/// Static descriptor for Iron Sword.
static IRON_SWORD_INFO: ItemInfo = ItemInfo {
  name: "Iron Sword",
  raw_id: 2,
  kind: kind::ItemKind::Sword,
  might: 5,
  hit: 95,
  crit: 0,
  max_durability: Some(40),
};

/// Static descriptor for Steel Sword.
static STEEL_SWORD_INFO: ItemInfo = ItemInfo {
  name: "Steel Sword",
  raw_id: 3,
  kind: kind::ItemKind::Sword,
  might: 8,
  hit: 90,
  crit: 0,
  max_durability: Some(35),
};

/// Static descriptor for Silver Sword.
static SILVER_SWORD_INFO: ItemInfo = ItemInfo {
  name: "Silver Sword",
  raw_id: 4,
  kind: kind::ItemKind::Sword,
  might: 11,
  hit: 85,
  crit: 0,
  max_durability: Some(30),
};

/// Static descriptor for Brave Sword.
static BRAVE_SWORD_INFO: ItemInfo = ItemInfo {
  name: "Brave Sword",
  raw_id: 5,
  kind: kind::ItemKind::Sword,
  might: 9,
  hit: 80,
  crit: 0,
  max_durability: Some(30),
};

/// Static descriptor for Armorslayer.
static ARMORSLAYER_INFO: ItemInfo = ItemInfo {
  name: "Armorslayer",
  raw_id: 6,
  kind: kind::ItemKind::Sword,
  might: 8,
  hit: 80,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Wyrmslayer.
static WYRMSLAYER_INFO: ItemInfo = ItemInfo {
  name: "Wyrmslayer",
  raw_id: 7,
  kind: kind::ItemKind::Sword,
  might: 8,
  hit: 80,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Killing Edge.
static KILLING_EDGE_INFO: ItemInfo = ItemInfo {
  name: "Killing Edge",
  raw_id: 8,
  kind: kind::ItemKind::Sword,
  might: 9,
  hit: 90,
  crit: 30,
  max_durability: Some(30),
};

/// Static descriptor for Levin Sword.
static LEVIN_SWORD_INFO: ItemInfo = ItemInfo {
  name: "Levin Sword",
  raw_id: 9,
  kind: kind::ItemKind::Sword,
  might: 10,
  hit: 80,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Rapier.
static RAPIER_INFO: ItemInfo = ItemInfo {
  name: "Rapier",
  raw_id: 10,
  kind: kind::ItemKind::Sword,
  might: 5,
  hit: 90,
  crit: 10,
  max_durability: Some(35),
};

/// Static descriptor for Noble Rapier.
static NOBLE_RAPIER_INFO: ItemInfo = ItemInfo {
  name: "Noble Rapier",
  raw_id: 11,
  kind: kind::ItemKind::Sword,
  might: 8,
  hit: 85,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Missiletainn.
static MISSILETAINN_INFO: ItemInfo = ItemInfo {
  name: "Missiletainn",
  raw_id: 12,
  kind: kind::ItemKind::Sword,
  might: 8,
  hit: 85,
  crit: 10,
  max_durability: Some(35),
};

/// Static descriptor for Sol.
static SOL_INFO: ItemInfo = ItemInfo {
  name: "Sol",
  raw_id: 13,
  kind: kind::ItemKind::Sword,
  might: 12,
  hit: 85,
  crit: 5,
  max_durability: Some(30),
};

/// Static descriptor for Amatsu.
static AMATSU_INFO: ItemInfo = ItemInfo {
  name: "Amatsu",
  raw_id: 14,
  kind: kind::ItemKind::Sword,
  might: 12,
  hit: 60,
  crit: 5,
  max_durability: Some(30),
};

/// Static descriptor for Falchion.
static FALCHION_INFO: ItemInfo = ItemInfo {
  name: "Falchion",
  raw_id: 15,
  kind: kind::ItemKind::Sword,
  might: 5,
  hit: 80,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for Exalted Falchion.
static EXALTED_FALCHION_INFO: ItemInfo = ItemInfo {
  name: "Exalted Falchion",
  raw_id: 16,
  kind: kind::ItemKind::Sword,
  might: 15,
  hit: 80,
  crit: 10,
  max_durability: None,
};

/// Static descriptor for Parallel Falchion.
static PARALLEL_FALCHION_INFO: ItemInfo = ItemInfo {
  name: "Parallel Falchion",
  raw_id: 17,
  kind: kind::ItemKind::Sword,
  might: 12,
  hit: 80,
  crit: 5,
  max_durability: None,
};

/// Static descriptor for Mercurius.
static MERCURIUS_INFO: ItemInfo = ItemInfo {
  name: "Mercurius",
  raw_id: 18,
  kind: kind::ItemKind::Sword,
  might: 17,
  hit: 95,
  crit: 5,
  max_durability: Some(25),
};

/// Static descriptor for Tyrfing.
static TYRFING_INFO: ItemInfo = ItemInfo {
  name: "Tyrfing",
  raw_id: 19,
  kind: kind::ItemKind::Sword,
  might: 15,
  hit: 85,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Mystletainn.
static MYSTLETAINN_INFO: ItemInfo = ItemInfo {
  name: "Mystletainn",
  raw_id: 20,
  kind: kind::ItemKind::Sword,
  might: 14,
  hit: 85,
  crit: 15,
  max_durability: Some(25),
};

/// Static descriptor for Balmung.
static BALMUNG_INFO: ItemInfo = ItemInfo {
  name: "Balmung",
  raw_id: 21,
  kind: kind::ItemKind::Sword,
  might: 13,
  hit: 90,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Sol Katti.
static SOL_KATTI_INFO: ItemInfo = ItemInfo {
  name: "Sol Katti",
  raw_id: 22,
  kind: kind::ItemKind::Sword,
  might: 8,
  hit: 100,
  crit: 50,
  max_durability: Some(25),
};

/// Static descriptor for Ragnell.
static RAGNELL_INFO: ItemInfo = ItemInfo {
  name: "Ragnell",
  raw_id: 23,
  kind: kind::ItemKind::Sword,
  might: 15,
  hit: 70,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Ragnell (Inf. Uses).
static RAGNELL_INFINITE_INFO: ItemInfo = ItemInfo {
  name: "Ragnell (Inf. Uses)",
  raw_id: 24,
  kind: kind::ItemKind::Sword,
  might: 15,
  hit: 70,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for Tree Branch.
static TREE_BRANCH_INFO: ItemInfo = ItemInfo {
  name: "Tree Branch",
  raw_id: 25,
  kind: kind::ItemKind::Sword,
  might: 1,
  hit: 100,
  crit: 0,
  max_durability: Some(20),
};

/// Static descriptor for Soothing Sword.
static SOOTHING_SWORD_INFO: ItemInfo = ItemInfo {
  name: "Soothing Sword",
  raw_id: 26,
  kind: kind::ItemKind::Sword,
  might: 8,
  hit: 85,
  crit: 0,
  max_durability: Some(10),
};

/// Static descriptor for Glass Sword.
static GLASS_SWORD_INFO: ItemInfo = ItemInfo {
  name: "Glass Sword",
  raw_id: 27,
  kind: kind::ItemKind::Sword,
  might: 11,
  hit: 85,
  crit: 0,
  max_durability: Some(3),
};

/// Static descriptor for Superior Edge.
static SUPERIOR_EDGE_INFO: ItemInfo = ItemInfo {
  name: "Superior Edge",
  raw_id: 28,
  kind: kind::ItemKind::Sword,
  might: 11,
  hit: 80,
  crit: 0,
  max_durability: Some(10),
};

/// Static descriptor for Eliwood's Blade.
static ELIWOODS_BLADE_INFO: ItemInfo = ItemInfo {
  name: "Eliwood's Blade",
  raw_id: 29,
  kind: kind::ItemKind::Sword,
  might: 10,
  hit: 85,
  crit: 5,
  max_durability: Some(20),
};

/// Static descriptor for Roy's Blade.
static ROYS_BLADE_INFO: ItemInfo = ItemInfo {
  name: "Roy's Blade",
  raw_id: 30,
  kind: kind::ItemKind::Sword,
  might: 8,
  hit: 95,
  crit: 5,
  max_durability: Some(25),
};

/// Static descriptor for Alm's Blade.
static ALMS_BLADE_INFO: ItemInfo = ItemInfo {
  name: "Alm's Blade",
  raw_id: 31,
  kind: kind::ItemKind::Sword,
  might: 15,
  hit: 75,
  crit: 10,
  max_durability: Some(10),
};

/// Static descriptor for Leif's Blade.
static LEIFS_BLADE_INFO: ItemInfo = ItemInfo {
  name: "Leif's Blade",
  raw_id: 32,
  kind: kind::ItemKind::Sword,
  might: 4,
  hit: 95,
  crit: 30,
  max_durability: Some(20),
};

/// Static descriptor for Eirika's Blade.
static EIRIKAS_BLADE_INFO: ItemInfo = ItemInfo {
  name: "Eirika's Blade",
  raw_id: 33,
  kind: kind::ItemKind::Sword,
  might: 6,
  hit: 95,
  crit: 10,
  max_durability: Some(20),
};

/// Static descriptor for Seliph's Blade.
static SELIPHS_BLADE_INFO: ItemInfo = ItemInfo {
  name: "Seliph's Blade",
  raw_id: 34,
  kind: kind::ItemKind::Sword,
  might: 12,
  hit: 90,
  crit: 15,
  max_durability: Some(15),
};

/// Static descriptor for Bronze Lance.
static BRONZE_LANCE_INFO: ItemInfo = ItemInfo {
  name: "Bronze Lance",
  raw_id: 35,
  kind: kind::ItemKind::Lance,
  might: 3,
  hit: 90,
  crit: 0,
  max_durability: Some(50),
};

/// Static descriptor for Iron Lance.
static IRON_LANCE_INFO: ItemInfo = ItemInfo {
  name: "Iron Lance",
  raw_id: 36,
  kind: kind::ItemKind::Lance,
  might: 6,
  hit: 85,
  crit: 0,
  max_durability: Some(40),
};

/// Static descriptor for Steel Lance.
static STEEL_LANCE_INFO: ItemInfo = ItemInfo {
  name: "Steel Lance",
  raw_id: 37,
  kind: kind::ItemKind::Lance,
  might: 9,
  hit: 80,
  crit: 0,
  max_durability: Some(35),
};

/// Static descriptor for Silver Lance.
static SILVER_LANCE_INFO: ItemInfo = ItemInfo {
  name: "Silver Lance",
  raw_id: 38,
  kind: kind::ItemKind::Lance,
  might: 13,
  hit: 75,
  crit: 0,
  max_durability: Some(30),
};

/// Static descriptor for Brave Lance.
static BRAVE_LANCE_INFO: ItemInfo = ItemInfo {
  name: "Brave Lance",
  raw_id: 39,
  kind: kind::ItemKind::Lance,
  might: 10,
  hit: 70,
  crit: 0,
  max_durability: Some(30),
};

/// Static descriptor for Javelin.
static JAVELIN_INFO: ItemInfo = ItemInfo {
  name: "Javelin",
  raw_id: 40,
  kind: kind::ItemKind::Lance,
  might: 2,
  hit: 80,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Short Spear.
static SHORT_SPEAR_INFO: ItemInfo = ItemInfo {
  name: "Short Spear",
  raw_id: 41,
  kind: kind::ItemKind::Lance,
  might: 5,
  hit: 75,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Spear.
static SPEAR_INFO: ItemInfo = ItemInfo {
  name: "Spear",
  raw_id: 42,
  kind: kind::ItemKind::Lance,
  might: 8,
  hit: 70,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Beast Killer.
static BEAST_KILLER_INFO: ItemInfo = ItemInfo {
  name: "Beast Killer",
  raw_id: 43,
  kind: kind::ItemKind::Lance,
  might: 9,
  hit: 70,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Blessed Lance.
static BLESSED_LANCE_INFO: ItemInfo = ItemInfo {
  name: "Blessed Lance",
  raw_id: 44,
  kind: kind::ItemKind::Lance,
  might: 11,
  hit: 70,
  crit: 0,
  max_durability: Some(35),
};

/// Static descriptor for Killer Lance.
static KILLER_LANCE_INFO: ItemInfo = ItemInfo {
  name: "Killer Lance",
  raw_id: 45,
  kind: kind::ItemKind::Lance,
  might: 10,
  hit: 80,
  crit: 30,
  max_durability: Some(30),
};

/// Static descriptor for Luna.
static LUNA_INFO: ItemInfo = ItemInfo {
  name: "Luna",
  raw_id: 46,
  kind: kind::ItemKind::Lance,
  might: 14,
  hit: 80,
  crit: 5,
  max_durability: Some(30),
};

/// Static descriptor for Gradivus.
static GRADIVUS_INFO: ItemInfo = ItemInfo {
  name: "Gradivus",
  raw_id: 47,
  kind: kind::ItemKind::Lance,
  might: 19,
  hit: 85,
  crit: 5,
  max_durability: Some(25),
};

/// Static descriptor for Gungnir.
static GUNGNIR_INFO: ItemInfo = ItemInfo {
  name: "Gungnir",
  raw_id: 48,
  kind: kind::ItemKind::Lance,
  might: 16,
  hit: 70,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Gae Bolg.
static GAE_BOLG_INFO: ItemInfo = ItemInfo {
  name: "Gae Bolg",
  raw_id: 49,
  kind: kind::ItemKind::Lance,
  might: 15,
  hit: 75,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Log.
static LOG_INFO: ItemInfo = ItemInfo {
  name: "Log",
  raw_id: 50,
  kind: kind::ItemKind::Lance,
  might: 1,
  hit: 90,
  crit: 0,
  max_durability: Some(20),
};

/// Static descriptor for Miniature Lance.
static MINIATURE_LANCE_INFO: ItemInfo = ItemInfo {
  name: "Miniature Lance",
  raw_id: 51,
  kind: kind::ItemKind::Lance,
  might: 1,
  hit: 55,
  crit: 35,
  max_durability: Some(10),
};

/// Static descriptor for Shockstick.
static SHOCKSTICK_INFO: ItemInfo = ItemInfo {
  name: "Shockstick",
  raw_id: 52,
  kind: kind::ItemKind::Lance,
  might: 11,
  hit: 85,
  crit: 10,
  max_durability: Some(20),
};

/// Static descriptor for Glass Lance.
static GLASS_LANCE_INFO: ItemInfo = ItemInfo {
  name: "Glass Lance",
  raw_id: 53,
  kind: kind::ItemKind::Lance,
  might: 13,
  hit: 75,
  crit: 0,
  max_durability: Some(3),
};

/// Static descriptor for Superior Lance.
static SUPERIOR_LANCE_INFO: ItemInfo = ItemInfo {
  name: "Superior Lance",
  raw_id: 54,
  kind: kind::ItemKind::Lance,
  might: 13,
  hit: 70,
  crit: 0,
  max_durability: Some(10),
};

/// Static descriptor for Sigurd's Lance.
static SIGURDS_LANCE_INFO: ItemInfo = ItemInfo {
  name: "Sigurd's Lance",
  raw_id: 55,
  kind: kind::ItemKind::Lance,
  might: 14,
  hit: 85,
  crit: 15,
  max_durability: Some(15),
};

/// Static descriptor for Ephraim's Lance.
static EPHRAIMS_LANCE_INFO: ItemInfo = ItemInfo {
  name: "Ephraim's Lance",
  raw_id: 56,
  kind: kind::ItemKind::Lance,
  might: 11,
  hit: 80,
  crit: 10,
  max_durability: Some(20),
};

/// Static descriptor for Finn's Lance.
static FINNS_LANCE_INFO: ItemInfo = ItemInfo {
  name: "Finn's Lance",
  raw_id: 57,
  kind: kind::ItemKind::Lance,
  might: 8,
  hit: 85,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Bronze Axe.
static BRONZE_AXE_INFO: ItemInfo = ItemInfo {
  name: "Bronze Axe",
  raw_id: 58,
  kind: kind::ItemKind::Axe,
  might: 4,
  hit: 80,
  crit: 0,
  max_durability: Some(50),
};

/// Static descriptor for Iron Axe.
static IRON_AXE_INFO: ItemInfo = ItemInfo {
  name: "Iron Axe",
  raw_id: 59,
  kind: kind::ItemKind::Axe,
  might: 7,
  hit: 75,
  crit: 0,
  max_durability: Some(40),
};

/// Static descriptor for Steel Axe.
static STEEL_AXE_INFO: ItemInfo = ItemInfo {
  name: "Steel Axe",
  raw_id: 60,
  kind: kind::ItemKind::Axe,
  might: 11,
  hit: 70,
  crit: 0,
  max_durability: Some(35),
};

/// Static descriptor for Silver Axe.
static SILVER_AXE_INFO: ItemInfo = ItemInfo {
  name: "Silver Axe",
  raw_id: 61,
  kind: kind::ItemKind::Axe,
  might: 15,
  hit: 65,
  crit: 0,
  max_durability: Some(30),
};

/// Static descriptor for Brave Axe.
static BRAVE_AXE_INFO: ItemInfo = ItemInfo {
  name: "Brave Axe",
  raw_id: 62,
  kind: kind::ItemKind::Axe,
  might: 12,
  hit: 60,
  crit: 0,
  max_durability: Some(30),
};

/// Static descriptor for Hand Axe.
static HAND_AXE_INFO: ItemInfo = ItemInfo {
  name: "Hand Axe",
  raw_id: 63,
  kind: kind::ItemKind::Axe,
  might: 3,
  hit: 70,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Short Axe.
static SHORT_AXE_INFO: ItemInfo = ItemInfo {
  name: "Short Axe",
  raw_id: 64,
  kind: kind::ItemKind::Axe,
  might: 7,
  hit: 65,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Tomahawk.
static TOMAHAWK_INFO: ItemInfo = ItemInfo {
  name: "Tomahawk",
  raw_id: 65,
  kind: kind::ItemKind::Axe,
  might: 10,
  hit: 60,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Hammer.
static HAMMER_INFO: ItemInfo = ItemInfo {
  name: "Hammer",
  raw_id: 66,
  kind: kind::ItemKind::Axe,
  might: 10,
  hit: 60,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Bolt Axe.
static BOLT_AXE_INFO: ItemInfo = ItemInfo {
  name: "Bolt Axe",
  raw_id: 67,
  kind: kind::ItemKind::Axe,
  might: 14,
  hit: 70,
  crit: 5,
  max_durability: Some(30),
};

/// Static descriptor for Killer Axe.
static KILLER_AXE_INFO: ItemInfo = ItemInfo {
  name: "Killer Axe",
  raw_id: 68,
  kind: kind::ItemKind::Axe,
  might: 12,
  hit: 70,
  crit: 30,
  max_durability: Some(30),
};

/// Static descriptor for Vengeance.
static VENGEANCE_INFO: ItemInfo = ItemInfo {
  name: "Vengeance",
  raw_id: 69,
  kind: kind::ItemKind::Axe,
  might: 16,
  hit: 75,
  crit: 5,
  max_durability: Some(30),
};

/// Static descriptor for Wolf Berg.
static WOLF_BERG_INFO: ItemInfo = ItemInfo {
  name: "Wolf Berg",
  raw_id: 70,
  kind: kind::ItemKind::Axe,
  might: 18,
  hit: 75,
  crit: 5,
  max_durability: Some(35),
};

/// Static descriptor for Hauteclere.
static HAUTECLERE_INFO: ItemInfo = ItemInfo {
  name: "Hauteclere",
  raw_id: 71,
  kind: kind::ItemKind::Axe,
  might: 21,
  hit: 70,
  crit: 5,
  max_durability: Some(25),
};

/// Static descriptor for Helswath.
static HELSWATH_INFO: ItemInfo = ItemInfo {
  name: "Helswath",
  raw_id: 72,
  kind: kind::ItemKind::Axe,
  might: 18,
  hit: 70,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Armads.
static ARMADS_INFO: ItemInfo = ItemInfo {
  name: "Armads",
  raw_id: 73,
  kind: kind::ItemKind::Axe,
  might: 17,
  hit: 80,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Ladle.
static LADLE_INFO: ItemInfo = ItemInfo {
  name: "Ladle",
  raw_id: 74,
  kind: kind::ItemKind::Axe,
  might: 1,
  hit: 80,
  crit: 0,
  max_durability: Some(20),
};

/// Static descriptor for Imposing Axe.
static IMPOSING_AXE_INFO: ItemInfo = ItemInfo {
  name: "Imposing Axe",
  raw_id: 75,
  kind: kind::ItemKind::Axe,
  might: 14,
  hit: 35,
  crit: 10,
  max_durability: Some(10),
};

/// Static descriptor for Volant Axe.
static VOLANT_AXE_INFO: ItemInfo = ItemInfo {
  name: "Volant Axe",
  raw_id: 76,
  kind: kind::ItemKind::Axe,
  might: 8,
  hit: 55,
  crit: 0,
  max_durability: Some(10),
};

/// Static descriptor for Glass Axe.
static GLASS_AXE_INFO: ItemInfo = ItemInfo {
  name: "Glass Axe",
  raw_id: 77,
  kind: kind::ItemKind::Axe,
  might: 15,
  hit: 65,
  crit: 0,
  max_durability: Some(3),
};

/// Static descriptor for Superior Axe.
static SUPERIOR_AXE_INFO: ItemInfo = ItemInfo {
  name: "Superior Axe",
  raw_id: 78,
  kind: kind::ItemKind::Axe,
  might: 15,
  hit: 60,
  crit: 0,
  max_durability: Some(10),
};

/// Static descriptor for Titania's Axe.
static TITANIAS_AXE_INFO: ItemInfo = ItemInfo {
  name: "Titania's Axe",
  raw_id: 79,
  kind: kind::ItemKind::Axe,
  might: 12,
  hit: 80,
  crit: 10,
  max_durability: Some(20),
};

/// Static descriptor for Orsin's Hatchet.
static ORSINS_HATCHET_INFO: ItemInfo = ItemInfo {
  name: "Orsin's Hatchet",
  raw_id: 80,
  kind: kind::ItemKind::Axe,
  might: 4,
  hit: 85,
  crit: 5,
  max_durability: Some(20),
};

/// Static descriptor for Hector's Axe.
static HECTORS_AXE_INFO: ItemInfo = ItemInfo {
  name: "Hector's Axe",
  raw_id: 81,
  kind: kind::ItemKind::Axe,
  might: 15,
  hit: 75,
  crit: 15,
  max_durability: Some(15),
};

/// Static descriptor for Bronze Bow.
static BRONZE_BOW_INFO: ItemInfo = ItemInfo {
  name: "Bronze Bow",
  raw_id: 82,
  kind: kind::ItemKind::Bow,
  might: 3,
  hit: 90,
  crit: 0,
  max_durability: Some(50),
};

/// Static descriptor for Iron Bow.
static IRON_BOW_INFO: ItemInfo = ItemInfo {
  name: "Iron Bow",
  raw_id: 83,
  kind: kind::ItemKind::Bow,
  might: 6,
  hit: 85,
  crit: 0,
  max_durability: Some(40),
};

/// Static descriptor for Steel Bow.
static STEEL_BOW_INFO: ItemInfo = ItemInfo {
  name: "Steel Bow",
  raw_id: 84,
  kind: kind::ItemKind::Bow,
  might: 9,
  hit: 80,
  crit: 0,
  max_durability: Some(35),
};

/// Static descriptor for Silver Bow.
static SILVER_BOW_INFO: ItemInfo = ItemInfo {
  name: "Silver Bow",
  raw_id: 85,
  kind: kind::ItemKind::Bow,
  might: 13,
  hit: 75,
  crit: 0,
  max_durability: Some(30),
};

/// Static descriptor for Brave Bow.
static BRAVE_BOW_INFO: ItemInfo = ItemInfo {
  name: "Brave Bow",
  raw_id: 86,
  kind: kind::ItemKind::Bow,
  might: 10,
  hit: 70,
  crit: 0,
  max_durability: Some(30),
};

/// Static descriptor for Blessed Bow.
static BLESSED_BOW_INFO: ItemInfo = ItemInfo {
  name: "Blessed Bow",
  raw_id: 87,
  kind: kind::ItemKind::Bow,
  might: 11,
  hit: 70,
  crit: 0,
  max_durability: Some(35),
};

/// Static descriptor for Killer Bow.
static KILLER_BOW_INFO: ItemInfo = ItemInfo {
  name: "Killer Bow",
  raw_id: 88,
  kind: kind::ItemKind::Bow,
  might: 10,
  hit: 80,
  crit: 30,
  max_durability: Some(30),
};

/// Static descriptor for Long Bow.
static LONG_BOW_INFO: ItemInfo = ItemInfo {
  name: "Long Bow",
  raw_id: 89,
  kind: kind::ItemKind::Bow,
  might: 9,
  hit: 70,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Astra.
static ASTRA_INFO: ItemInfo = ItemInfo {
  name: "Astra",
  raw_id: 90,
  kind: kind::ItemKind::Bow,
  might: 14,
  hit: 75,
  crit: 5,
  max_durability: Some(30),
};

/// Static descriptor for Parthia.
static PARTHIA_INFO: ItemInfo = ItemInfo {
  name: "Parthia",
  raw_id: 91,
  kind: kind::ItemKind::Bow,
  might: 19,
  hit: 95,
  crit: 5,
  max_durability: Some(25),
};

/// Static descriptor for Yewfelle.
static YEWFELLE_INFO: ItemInfo = ItemInfo {
  name: "Yewfelle",
  raw_id: 92,
  kind: kind::ItemKind::Bow,
  might: 15,
  hit: 85,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Nidhogg.
static NIDHOGG_INFO: ItemInfo = ItemInfo {
  name: "Nidhogg",
  raw_id: 93,
  kind: kind::ItemKind::Bow,
  might: 16,
  hit: 75,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Double Bow.
static DOUBLE_BOW_INFO: ItemInfo = ItemInfo {
  name: "Double Bow",
  raw_id: 94,
  kind: kind::ItemKind::Bow,
  might: 13,
  hit: 70,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Slack Bow.
static SLACK_BOW_INFO: ItemInfo = ItemInfo {
  name: "Slack Bow",
  raw_id: 95,
  kind: kind::ItemKind::Bow,
  might: 1,
  hit: 90,
  crit: 0,
  max_durability: Some(20),
};

/// Static descriptor for Towering Bow.
static TOWERING_BOW_INFO: ItemInfo = ItemInfo {
  name: "Towering Bow",
  raw_id: 96,
  kind: kind::ItemKind::Bow,
  might: 3,
  hit: 80,
  crit: 0,
  max_durability: Some(10),
};

/// Static descriptor for Underdog Bow.
static UNDERDOG_BOW_INFO: ItemInfo = ItemInfo {
  name: "Underdog Bow",
  raw_id: 97,
  kind: kind::ItemKind::Bow,
  might: 9,
  hit: 75,
  crit: 5,
  max_durability: Some(15),
};

/// Static descriptor for Glass Bow.
static GLASS_BOW_INFO: ItemInfo = ItemInfo {
  name: "Glass Bow",
  raw_id: 98,
  kind: kind::ItemKind::Bow,
  might: 13,
  hit: 75,
  crit: 0,
  max_durability: Some(3),
};

/// Static descriptor for Superior Bow.
static SUPERIOR_BOW_INFO: ItemInfo = ItemInfo {
  name: "Superior Bow",
  raw_id: 99,
  kind: kind::ItemKind::Bow,
  might: 13,
  hit: 70,
  crit: 0,
  max_durability: Some(10),
};

/// Static descriptor for Wolt's Bow.
static WOLTS_BOW_INFO: ItemInfo = ItemInfo {
  name: "Wolt's Bow",
  raw_id: 100,
  kind: kind::ItemKind::Bow,
  might: 10,
  hit: 85,
  crit: 5,
  max_durability: Some(25),
};

/// Static descriptor for Innes' Bow.
static INNES_BOW_INFO: ItemInfo = ItemInfo {
  name: "Innes' Bow",
  raw_id: 101,
  kind: kind::ItemKind::Bow,
  might: 13,
  hit: 115,
  crit: 10,
  max_durability: Some(15),
};

/// Static descriptor for Fire.
static FIRE_INFO: ItemInfo = ItemInfo {
  name: "Fire",
  raw_id: 102,
  kind: kind::ItemKind::Tome,
  might: 2,
  hit: 90,
  crit: 0,
  max_durability: Some(45),
};

/// Static descriptor for Elfire.
static ELFIRE_INFO: ItemInfo = ItemInfo {
  name: "Elfire",
  raw_id: 103,
  kind: kind::ItemKind::Tome,
  might: 5,
  hit: 85,
  crit: 0,
  max_durability: Some(35),
};

/// Static descriptor for Arcfire.
static ARCFIRE_INFO: ItemInfo = ItemInfo {
  name: "Arcfire",
  raw_id: 104,
  kind: kind::ItemKind::Tome,
  might: 8,
  hit: 80,
  crit: 0,
  max_durability: Some(30),
};

/// Static descriptor for Bolganone.
static BOLGANONE_INFO: ItemInfo = ItemInfo {
  name: "Bolganone",
  raw_id: 105,
  kind: kind::ItemKind::Tome,
  might: 12,
  hit: 75,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Valflame.
static VALFLAME_INFO: ItemInfo = ItemInfo {
  name: "Valflame",
  raw_id: 106,
  kind: kind::ItemKind::Tome,
  might: 16,
  hit: 80,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Thunder.
static THUNDER_INFO: ItemInfo = ItemInfo {
  name: "Thunder",
  raw_id: 107,
  kind: kind::ItemKind::Tome,
  might: 3,
  hit: 80,
  crit: 5,
  max_durability: Some(45),
};

/// Static descriptor for Elthunder.
static ELTHUNDER_INFO: ItemInfo = ItemInfo {
  name: "Elthunder",
  raw_id: 108,
  kind: kind::ItemKind::Tome,
  might: 6,
  hit: 75,
  crit: 5,
  max_durability: Some(35),
};

/// Static descriptor for Arcthunder.
static ARCTHUNDER_INFO: ItemInfo = ItemInfo {
  name: "Arcthunder",
  raw_id: 109,
  kind: kind::ItemKind::Tome,
  might: 10,
  hit: 70,
  crit: 10,
  max_durability: Some(30),
};

/// Static descriptor for Thoron.
static THORON_INFO: ItemInfo = ItemInfo {
  name: "Thoron",
  raw_id: 110,
  kind: kind::ItemKind::Tome,
  might: 14,
  hit: 65,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Mjollnir.
static MJOLLNIR_INFO: ItemInfo = ItemInfo {
  name: "Mjollnir",
  raw_id: 111,
  kind: kind::ItemKind::Tome,
  might: 18,
  hit: 70,
  crit: 20,
  max_durability: Some(25),
};

/// Static descriptor for Wind.
static WIND_INFO: ItemInfo = ItemInfo {
  name: "Wind",
  raw_id: 112,
  kind: kind::ItemKind::Tome,
  might: 1,
  hit: 100,
  crit: 0,
  max_durability: Some(45),
};

/// Static descriptor for Elwind.
static ELWIND_INFO: ItemInfo = ItemInfo {
  name: "Elwind",
  raw_id: 113,
  kind: kind::ItemKind::Tome,
  might: 4,
  hit: 95,
  crit: 0,
  max_durability: Some(35),
};

/// Static descriptor for Arcwind.
static ARCWIND_INFO: ItemInfo = ItemInfo {
  name: "Arcwind",
  raw_id: 114,
  kind: kind::ItemKind::Tome,
  might: 6,
  hit: 90,
  crit: 0,
  max_durability: Some(30),
};

/// Static descriptor for Rexcalibur.
static REXCALIBUR_INFO: ItemInfo = ItemInfo {
  name: "Rexcalibur",
  raw_id: 115,
  kind: kind::ItemKind::Tome,
  might: 10,
  hit: 85,
  crit: 0,
  max_durability: Some(25),
};

/// Static descriptor for Forseti.
static FORSETI_INFO: ItemInfo = ItemInfo {
  name: "Forseti",
  raw_id: 116,
  kind: kind::ItemKind::Tome,
  might: 14,
  hit: 90,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Excalibur.
static EXCALIBUR_INFO: ItemInfo = ItemInfo {
  name: "Excalibur",
  raw_id: 117,
  kind: kind::ItemKind::Tome,
  might: 13,
  hit: 100,
  crit: 30,
  max_durability: Some(25),
};

/// Static descriptor for Book of Naga.
static BOOK_OF_NAGA_INFO: ItemInfo = ItemInfo {
  name: "Book of Naga",
  raw_id: 118,
  kind: kind::ItemKind::Tome,
  might: 15,
  hit: 80,
  crit: 15,
  max_durability: Some(25),
};

/// Static descriptor for Flux.
static FLUX_INFO: ItemInfo = ItemInfo {
  name: "Flux",
  raw_id: 119,
  kind: kind::ItemKind::Tome,
  might: 5,
  hit: 70,
  crit: 0,
  max_durability: Some(45),
};

/// Static descriptor for Nosferatu.
static NOSFERATU_INFO: ItemInfo = ItemInfo {
  name: "Nosferatu",
  raw_id: 120,
  kind: kind::ItemKind::Tome,
  might: 7,
  hit: 65,
  crit: 0,
  max_durability: Some(20),
};

/// Static descriptor for Ruin.
static RUIN_INFO: ItemInfo = ItemInfo {
  name: "Ruin",
  raw_id: 121,
  kind: kind::ItemKind::Tome,
  might: 4,
  hit: 60,
  crit: 50,
  max_durability: Some(20),
};

/// Static descriptor for Waste.
static WASTE_INFO: ItemInfo = ItemInfo {
  name: "Waste",
  raw_id: 122,
  kind: kind::ItemKind::Tome,
  might: 10,
  hit: 45,
  crit: 0,
  max_durability: Some(30),
};

/// Static descriptor for Goetia.
static GOETIA_INFO: ItemInfo = ItemInfo {
  name: "Goetia",
  raw_id: 123,
  kind: kind::ItemKind::Tome,
  might: 19,
  hit: 75,
  crit: 10,
  max_durability: Some(25),
};

/// Static descriptor for Grima's Truth.
static GRIMAS_TRUTH_INFO: ItemInfo = ItemInfo {
  name: "Grima's Truth",
  raw_id: 124,
  kind: kind::ItemKind::Tome,
  might: 12,
  hit: 80,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for Mire.
static MIRE_INFO: ItemInfo = ItemInfo {
  name: "Mire",
  raw_id: 125,
  kind: kind::ItemKind::Tome,
  might: 10,
  hit: 65,
  crit: 0,
  max_durability: Some(10),
};

/// Static descriptor for Dying Blaze.
static DYING_BLAZE_INFO: ItemInfo = ItemInfo {
  name: "Dying Blaze",
  raw_id: 126,
  kind: kind::ItemKind::Tome,
  might: 10,
  hit: 75,
  crit: 0,
  max_durability: Some(3),
};

/// Static descriptor for Micaiah's Pyre.
static MICAIAHS_PYRE_INFO: ItemInfo = ItemInfo {
  name: "Micaiah's Pyre",
  raw_id: 127,
  kind: kind::ItemKind::Tome,
  might: 13,
  hit: 85,
  crit: 5,
  max_durability: Some(15),
};

/// Static descriptor for Superior Jolt.
static SUPERIOR_JOLT_INFO: ItemInfo = ItemInfo {
  name: "Superior Jolt",
  raw_id: 128,
  kind: kind::ItemKind::Tome,
  might: 14,
  hit: 60,
  crit: 15,
  max_durability: Some(10),
};

/// Static descriptor for Katarina's Bolt.
static KATARINAS_BOLT_INFO: ItemInfo = ItemInfo {
  name: "Katarina's Bolt",
  raw_id: 129,
  kind: kind::ItemKind::Tome,
  might: 11,
  hit: 75,
  crit: 30,
  max_durability: Some(20),
};

/// Static descriptor for Wilderwind.
static WILDERWIND_INFO: ItemInfo = ItemInfo {
  name: "Wilderwind",
  raw_id: 130,
  kind: kind::ItemKind::Tome,
  might: 2,
  hit: 70,
  crit: 35,
  max_durability: Some(5),
};

/// Static descriptor for Celica's Gale.
static CELICAS_GALE_INFO: ItemInfo = ItemInfo {
  name: "Celica's Gale",
  raw_id: 131,
  kind: kind::ItemKind::Tome,
  might: 4,
  hit: 80,
  crit: 5,
  max_durability: Some(20),
};

/// Static descriptor for Aversa's Night.
static AVERSAS_NIGHT_INFO: ItemInfo = ItemInfo {
  name: "Aversa's Night",
  raw_id: 132,
  kind: kind::ItemKind::Tome,
  might: 15,
  hit: 75,
  crit: 0,
  max_durability: Some(10),
};

/// Static descriptor for Heal.
static HEAL_INFO: ItemInfo = ItemInfo {
  name: "Heal",
  raw_id: 133,
  kind: kind::ItemKind::Staff,
  might: 8,
  hit: 0,
  crit: 0,
  max_durability: Some(30),
};

/// Static descriptor for Mend.
static MEND_INFO: ItemInfo = ItemInfo {
  name: "Mend",
  raw_id: 134,
  kind: kind::ItemKind::Staff,
  might: 15,
  hit: 0,
  crit: 0,
  max_durability: Some(20),
};

/// Static descriptor for Physic.
static PHYSIC_INFO: ItemInfo = ItemInfo {
  name: "Physic",
  raw_id: 135,
  kind: kind::ItemKind::Staff,
  might: 8,
  hit: 0,
  crit: 0,
  max_durability: Some(10),
};

/// Static descriptor for Recover.
static RECOVER_INFO: ItemInfo = ItemInfo {
  name: "Recover",
  raw_id: 136,
  kind: kind::ItemKind::Staff,
  might: 99,
  hit: 0,
  crit: 0,
  max_durability: Some(15),
};

/// Static descriptor for Fortify.
static FORTIFY_INFO: ItemInfo = ItemInfo {
  name: "Fortify",
  raw_id: 137,
  kind: kind::ItemKind::Staff,
  might: 8,
  hit: 0,
  crit: 0,
  max_durability: Some(5),
};

/// Static descriptor for Goddess Staff.
static GODDESS_STAFF_INFO: ItemInfo = ItemInfo {
  name: "Goddess Staff",
  raw_id: 138,
  kind: kind::ItemKind::Staff,
  might: 99,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Rescue.
static RESCUE_INFO: ItemInfo = ItemInfo {
  name: "Rescue",
  raw_id: 139,
  kind: kind::ItemKind::Staff,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(5),
};

/// Static descriptor for Ward.
static WARD_INFO: ItemInfo = ItemInfo {
  name: "Ward",
  raw_id: 140,
  kind: kind::ItemKind::Staff,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(5),
};

/// Static descriptor for Hammerne.
static HAMMERNE_INFO: ItemInfo = ItemInfo {
  name: "Hammerne",
  raw_id: 141,
  kind: kind::ItemKind::Staff,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Kneader.
static KNEADER_INFO: ItemInfo = ItemInfo {
  name: "Kneader",
  raw_id: 142,
  kind: kind::ItemKind::Staff,
  might: 1,
  hit: 0,
  crit: 0,
  max_durability: Some(20),
};

/// Static descriptor for Balmwood Staff.
static BALMWOOD_STAFF_INFO: ItemInfo = ItemInfo {
  name: "Balmwood Staff",
  raw_id: 143,
  kind: kind::ItemKind::Staff,
  might: 15,
  hit: 0,
  crit: 0,
  max_durability: Some(15),
};

/// Static descriptor for Catharsis.
static CATHARSIS_INFO: ItemInfo = ItemInfo {
  name: "Catharsis",
  raw_id: 144,
  kind: kind::ItemKind::Staff,
  might: 15,
  hit: 0,
  crit: 0,
  max_durability: Some(5),
};

/// Static descriptor for Dragonstone.
static DRAGONSTONE_INFO: ItemInfo = ItemInfo {
  name: "Dragonstone",
  raw_id: 145,
  kind: kind::ItemKind::Stone,
  might: 8,
  hit: 80,
  crit: 0,
  max_durability: Some(50),
};

/// Static descriptor for Dragonstone+.
static DRAGONSTONEPLUS_INFO: ItemInfo = ItemInfo {
  name: "Dragonstone+",
  raw_id: 146,
  kind: kind::ItemKind::Stone,
  might: 12,
  hit: 70,
  crit: 0,
  max_durability: Some(35),
};

/// Static descriptor for Beaststone.
static BEASTSTONE_INFO: ItemInfo = ItemInfo {
  name: "Beaststone",
  raw_id: 147,
  kind: kind::ItemKind::Stone,
  might: 6,
  hit: 80,
  crit: 0,
  max_durability: Some(50),
};

/// Static descriptor for Beaststone+.
static BEASTSTONEPLUS_INFO: ItemInfo = ItemInfo {
  name: "Beaststone+",
  raw_id: 148,
  kind: kind::ItemKind::Stone,
  might: 10,
  hit: 70,
  crit: 0,
  max_durability: Some(35),
};

/// Static descriptor for Blighted Claws.
static BLIGHTED_CLAWS_INFO: ItemInfo = ItemInfo {
  name: "Blighted Claws",
  raw_id: 149,
  kind: kind::ItemKind::Consumable,
  might: 12,
  hit: 75,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for Blighted Talons.
static BLIGHTED_TALONS_INFO: ItemInfo = ItemInfo {
  name: "Blighted Talons",
  raw_id: 150,
  kind: kind::ItemKind::Consumable,
  might: 16,
  hit: 75,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for Expiration.
static EXPIRATION_INFO: ItemInfo = ItemInfo {
  name: "Expiration",
  raw_id: 151,
  kind: kind::ItemKind::Consumable,
  might: 20,
  hit: 80,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for Vulnerary.
static VULNERARY_INFO: ItemInfo = ItemInfo {
  name: "Vulnerary",
  raw_id: 152,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(3),
};

/// Static descriptor for Concoction.
static CONCOCTION_INFO: ItemInfo = ItemInfo {
  name: "Concoction",
  raw_id: 153,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(3),
};

/// Static descriptor for Elixer.
static ELIXER_INFO: ItemInfo = ItemInfo {
  name: "Elixer",
  raw_id: 154,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(3),
};

/// Static descriptor for Pure Water.
static PURE_WATER_INFO: ItemInfo = ItemInfo {
  name: "Pure Water",
  raw_id: 155,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(3),
};

/// Static descriptor for HP Tonic.
static HP_TONIC_INFO: ItemInfo = ItemInfo {
  name: "HP Tonic",
  raw_id: 156,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Strength Tonic.
static STRENGTH_TONIC_INFO: ItemInfo = ItemInfo {
  name: "Strength Tonic",
  raw_id: 157,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Magic Tonic.
static MAGIC_TONIC_INFO: ItemInfo = ItemInfo {
  name: "Magic Tonic",
  raw_id: 158,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Skill Tonic.
static SKILL_TONIC_INFO: ItemInfo = ItemInfo {
  name: "Skill Tonic",
  raw_id: 159,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Speed Tonic.
static SPEED_TONIC_INFO: ItemInfo = ItemInfo {
  name: "Speed Tonic",
  raw_id: 160,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Luck Tonic.
static LUCK_TONIC_INFO: ItemInfo = ItemInfo {
  name: "Luck Tonic",
  raw_id: 161,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Defense Tonic.
static DEFENSE_TONIC_INFO: ItemInfo = ItemInfo {
  name: "Defense Tonic",
  raw_id: 162,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Resistance Tonic.
static RESISTANCE_TONIC_INFO: ItemInfo = ItemInfo {
  name: "Resistance Tonic",
  raw_id: 163,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Door Key.
static DOOR_KEY_INFO: ItemInfo = ItemInfo {
  name: "Door Key",
  raw_id: 164,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Chest Key.
static CHEST_KEY_INFO: ItemInfo = ItemInfo {
  name: "Chest Key",
  raw_id: 165,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Master Key.
static MASTER_KEY_INFO: ItemInfo = ItemInfo {
  name: "Master Key",
  raw_id: 166,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Seraph Robe.
static SERAPH_ROBE_INFO: ItemInfo = ItemInfo {
  name: "Seraph Robe",
  raw_id: 167,
  kind: kind::ItemKind::StatBooster,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Energy Drop.
static ENERGY_DROP_INFO: ItemInfo = ItemInfo {
  name: "Energy Drop",
  raw_id: 168,
  kind: kind::ItemKind::StatBooster,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Spirit Dust.
static SPIRIT_DUST_INFO: ItemInfo = ItemInfo {
  name: "Spirit Dust",
  raw_id: 169,
  kind: kind::ItemKind::StatBooster,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Secret Book.
static SECRET_BOOK_INFO: ItemInfo = ItemInfo {
  name: "Secret Book",
  raw_id: 170,
  kind: kind::ItemKind::StatBooster,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Speedwing.
static SPEEDWING_INFO: ItemInfo = ItemInfo {
  name: "Speedwing",
  raw_id: 171,
  kind: kind::ItemKind::StatBooster,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Goddess Icon.
static GODDESS_ICON_INFO: ItemInfo = ItemInfo {
  name: "Goddess Icon",
  raw_id: 172,
  kind: kind::ItemKind::StatBooster,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Dracoshield.
static DRACOSHIELD_INFO: ItemInfo = ItemInfo {
  name: "Dracoshield",
  raw_id: 173,
  kind: kind::ItemKind::StatBooster,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Talisman.
static TALISMAN_INFO: ItemInfo = ItemInfo {
  name: "Talisman",
  raw_id: 174,
  kind: kind::ItemKind::StatBooster,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Naga's Tear.
static NAGAS_TEAR_INFO: ItemInfo = ItemInfo {
  name: "Naga's Tear",
  raw_id: 175,
  kind: kind::ItemKind::StatBooster,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Boots.
static BOOTS_INFO: ItemInfo = ItemInfo {
  name: "Boots",
  raw_id: 176,
  kind: kind::ItemKind::StatBooster,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Arms Scroll.
static ARMS_SCROLL_INFO: ItemInfo = ItemInfo {
  name: "Arms Scroll",
  raw_id: 177,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Master Seal.
static MASTER_SEAL_INFO: ItemInfo = ItemInfo {
  name: "Master Seal",
  raw_id: 178,
  kind: kind::ItemKind::Seal,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Second Seal.
static SECOND_SEAL_INFO: ItemInfo = ItemInfo {
  name: "Second Seal",
  raw_id: 179,
  kind: kind::ItemKind::Seal,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Bullion (S).
static BULLION_S_INFO: ItemInfo = ItemInfo {
  name: "Bullion (S)",
  raw_id: 180,
  kind: kind::ItemKind::Valuable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for Bullion (M).
static BULLION_M_INFO: ItemInfo = ItemInfo {
  name: "Bullion (M)",
  raw_id: 181,
  kind: kind::ItemKind::Valuable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for Bullion (L).
static BULLION_L_INFO: ItemInfo = ItemInfo {
  name: "Bullion (L)",
  raw_id: 182,
  kind: kind::ItemKind::Valuable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for Sweet Tincture.
static SWEET_TINCTURE_INFO: ItemInfo = ItemInfo {
  name: "Sweet Tincture",
  raw_id: 183,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(3),
};

/// Static descriptor for Gaius' Confect.
static GAIUS_CONFECT_INFO: ItemInfo = ItemInfo {
  name: "Gaius' Confect",
  raw_id: 184,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(5),
};

/// Static descriptor for Kris' Confect.
static KRIS_CONFECT_INFO: ItemInfo = ItemInfo {
  name: "Kris' Confect",
  raw_id: 185,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(5),
};

/// Static descriptor for Tiki's Tear.
static TIKIS_TEAR_INFO: ItemInfo = ItemInfo {
  name: "Tiki's Tear",
  raw_id: 186,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Seed of Trust.
static SEED_OF_TRUST_INFO: ItemInfo = ItemInfo {
  name: "Seed of Trust",
  raw_id: 187,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Reeking Box.
static REEKING_BOX_INFO: ItemInfo = ItemInfo {
  name: "Reeking Box",
  raw_id: 188,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Rift Door.
static RIFT_DOOR_INFO: ItemInfo = ItemInfo {
  name: "Rift Door",
  raw_id: 189,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Supreme Emblem.
static SUPREME_EMBLEM_INFO: ItemInfo = ItemInfo {
  name: "Supreme Emblem",
  raw_id: 190,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for All Stats +2.
static ALL_STATS_PLUS2_INFO: ItemInfo = ItemInfo {
  name: "All Stats +2",
  raw_id: 191,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Paragon.
static PARAGON_INFO: ItemInfo = ItemInfo {
  name: "Paragon",
  raw_id: 192,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Iote's Shield.
static IOTES_SHIELD_INFO: ItemInfo = ItemInfo {
  name: "Iote's Shield",
  raw_id: 193,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Limit Breaker.
static LIMIT_BREAKER_INFO: ItemInfo = ItemInfo {
  name: "Limit Breaker",
  raw_id: 194,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Silver Card.
static SILVER_CARD_INFO: ItemInfo = ItemInfo {
  name: "Silver Card",
  raw_id: 195,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for Dread Scroll.
static DREAD_SCROLL_INFO: ItemInfo = ItemInfo {
  name: "Dread Scroll",
  raw_id: 196,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for Wedding Bouquet.
static WEDDING_BOUQUET_INFO: ItemInfo = ItemInfo {
  name: "Wedding Bouquet",
  raw_id: 197,
  kind: kind::ItemKind::Consumable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: Some(1),
};

/// Static descriptor for 1,000 Gold.
static GOLD_1000_INFO: ItemInfo = ItemInfo {
  name: "1,000 Gold",
  raw_id: 198,
  kind: kind::ItemKind::Valuable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for 3,000 Gold.
static GOLD_3000_INFO: ItemInfo = ItemInfo {
  name: "3,000 Gold",
  raw_id: 199,
  kind: kind::ItemKind::Valuable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for 5,000 Gold.
static GOLD_5000_INFO: ItemInfo = ItemInfo {
  name: "5,000 Gold",
  raw_id: 200,
  kind: kind::ItemKind::Valuable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: None,
};

/// Static descriptor for 7,000 Gold.
static GOLD_7000_INFO: ItemInfo = ItemInfo {
  name: "7,000 Gold",
  raw_id: 201,
  kind: kind::ItemKind::Valuable,
  might: 0,
  hit: 0,
  crit: 0,
  max_durability: None,
};

/// Slice containing all canonical non-unknown item variants.
pub static ALL_ITEMS: [ItemType; 201] = [
  ItemType::BronzeSword,
  ItemType::IronSword,
  ItemType::SteelSword,
  ItemType::SilverSword,
  ItemType::BraveSword,
  ItemType::Armorslayer,
  ItemType::Wyrmslayer,
  ItemType::KillingEdge,
  ItemType::LevinSword,
  ItemType::Rapier,
  ItemType::NobleRapier,
  ItemType::Missiletainn,
  ItemType::Sol,
  ItemType::Amatsu,
  ItemType::Falchion,
  ItemType::ExaltedFalchion,
  ItemType::ParallelFalchion,
  ItemType::Mercurius,
  ItemType::Tyrfing,
  ItemType::Mystletainn,
  ItemType::Balmung,
  ItemType::SolKatti,
  ItemType::Ragnell,
  ItemType::RagnellInfinite,
  ItemType::TreeBranch,
  ItemType::SoothingSword,
  ItemType::GlassSword,
  ItemType::SuperiorEdge,
  ItemType::EliwoodsBlade,
  ItemType::RoysBlade,
  ItemType::AlmsBlade,
  ItemType::LeifsBlade,
  ItemType::EirikasBlade,
  ItemType::SeliphsBlade,
  ItemType::BronzeLance,
  ItemType::IronLance,
  ItemType::SteelLance,
  ItemType::SilverLance,
  ItemType::BraveLance,
  ItemType::Javelin,
  ItemType::ShortSpear,
  ItemType::Spear,
  ItemType::BeastKiller,
  ItemType::BlessedLance,
  ItemType::KillerLance,
  ItemType::Luna,
  ItemType::Gradivus,
  ItemType::Gungnir,
  ItemType::GaeBolg,
  ItemType::Log,
  ItemType::MiniatureLance,
  ItemType::Shockstick,
  ItemType::GlassLance,
  ItemType::SuperiorLance,
  ItemType::SigurdsLance,
  ItemType::EphraimsLance,
  ItemType::FinnsLance,
  ItemType::BronzeAxe,
  ItemType::IronAxe,
  ItemType::SteelAxe,
  ItemType::SilverAxe,
  ItemType::BraveAxe,
  ItemType::HandAxe,
  ItemType::ShortAxe,
  ItemType::Tomahawk,
  ItemType::Hammer,
  ItemType::BoltAxe,
  ItemType::KillerAxe,
  ItemType::Vengeance,
  ItemType::WolfBerg,
  ItemType::Hauteclere,
  ItemType::Helswath,
  ItemType::Armads,
  ItemType::Ladle,
  ItemType::ImposingAxe,
  ItemType::VolantAxe,
  ItemType::GlassAxe,
  ItemType::SuperiorAxe,
  ItemType::TitaniasAxe,
  ItemType::OrsinsHatchet,
  ItemType::HectorsAxe,
  ItemType::BronzeBow,
  ItemType::IronBow,
  ItemType::SteelBow,
  ItemType::SilverBow,
  ItemType::BraveBow,
  ItemType::BlessedBow,
  ItemType::KillerBow,
  ItemType::LongBow,
  ItemType::Astra,
  ItemType::Parthia,
  ItemType::Yewfelle,
  ItemType::Nidhogg,
  ItemType::DoubleBow,
  ItemType::SlackBow,
  ItemType::ToweringBow,
  ItemType::UnderdogBow,
  ItemType::GlassBow,
  ItemType::SuperiorBow,
  ItemType::WoltsBow,
  ItemType::InnesBow,
  ItemType::Fire,
  ItemType::Elfire,
  ItemType::Arcfire,
  ItemType::Bolganone,
  ItemType::Valflame,
  ItemType::Thunder,
  ItemType::Elthunder,
  ItemType::Arcthunder,
  ItemType::Thoron,
  ItemType::Mjollnir,
  ItemType::Wind,
  ItemType::Elwind,
  ItemType::Arcwind,
  ItemType::Rexcalibur,
  ItemType::Forseti,
  ItemType::Excalibur,
  ItemType::BookOfNaga,
  ItemType::Flux,
  ItemType::Nosferatu,
  ItemType::Ruin,
  ItemType::Waste,
  ItemType::Goetia,
  ItemType::GrimasTruth,
  ItemType::Mire,
  ItemType::DyingBlaze,
  ItemType::MicaiahsPyre,
  ItemType::SuperiorJolt,
  ItemType::KatarinasBolt,
  ItemType::Wilderwind,
  ItemType::CelicasGale,
  ItemType::AversasNight,
  ItemType::Heal,
  ItemType::Mend,
  ItemType::Physic,
  ItemType::Recover,
  ItemType::Fortify,
  ItemType::GoddessStaff,
  ItemType::Rescue,
  ItemType::Ward,
  ItemType::Hammerne,
  ItemType::Kneader,
  ItemType::BalmwoodStaff,
  ItemType::Catharsis,
  ItemType::Dragonstone,
  ItemType::Dragonstoneplus,
  ItemType::Beaststone,
  ItemType::Beaststoneplus,
  ItemType::BlightedClaws,
  ItemType::BlightedTalons,
  ItemType::Expiration,
  ItemType::Vulnerary,
  ItemType::Concoction,
  ItemType::Elixer,
  ItemType::PureWater,
  ItemType::HpTonic,
  ItemType::StrengthTonic,
  ItemType::MagicTonic,
  ItemType::SkillTonic,
  ItemType::SpeedTonic,
  ItemType::LuckTonic,
  ItemType::DefenseTonic,
  ItemType::ResistanceTonic,
  ItemType::DoorKey,
  ItemType::ChestKey,
  ItemType::MasterKey,
  ItemType::SeraphRobe,
  ItemType::EnergyDrop,
  ItemType::SpiritDust,
  ItemType::SecretBook,
  ItemType::Speedwing,
  ItemType::GoddessIcon,
  ItemType::Dracoshield,
  ItemType::Talisman,
  ItemType::NagasTear,
  ItemType::Boots,
  ItemType::ArmsScroll,
  ItemType::MasterSeal,
  ItemType::SecondSeal,
  ItemType::BullionS,
  ItemType::BullionM,
  ItemType::BullionL,
  ItemType::SweetTincture,
  ItemType::GaiusConfect,
  ItemType::KrisConfect,
  ItemType::TikisTear,
  ItemType::SeedOfTrust,
  ItemType::ReekingBox,
  ItemType::RiftDoor,
  ItemType::SupremeEmblem,
  ItemType::AllStatsPlus2,
  ItemType::Paragon,
  ItemType::IotesShield,
  ItemType::LimitBreaker,
  ItemType::SilverCard,
  ItemType::DreadScroll,
  ItemType::WeddingBouquet,
  ItemType::Gold1000,
  ItemType::Gold3000,
  ItemType::Gold5000,
  ItemType::Gold7000,
];

impl ItemType {
  /// Returns static descriptor row for this item type.
  #[must_use]
  #[expect(
    clippy::too_many_lines,
    reason = "Comprehensive FE13 item catalog maps all in-game item variants"
  )]
  pub const fn info(self) -> &'static ItemInfo {
    match self {
      Self::BronzeSword => &BRONZE_SWORD_INFO,
      Self::IronSword => &IRON_SWORD_INFO,
      Self::SteelSword => &STEEL_SWORD_INFO,
      Self::SilverSword => &SILVER_SWORD_INFO,
      Self::BraveSword => &BRAVE_SWORD_INFO,
      Self::Armorslayer => &ARMORSLAYER_INFO,
      Self::Wyrmslayer => &WYRMSLAYER_INFO,
      Self::KillingEdge => &KILLING_EDGE_INFO,
      Self::LevinSword => &LEVIN_SWORD_INFO,
      Self::Rapier => &RAPIER_INFO,
      Self::NobleRapier => &NOBLE_RAPIER_INFO,
      Self::Missiletainn => &MISSILETAINN_INFO,
      Self::Sol => &SOL_INFO,
      Self::Amatsu => &AMATSU_INFO,
      Self::Falchion => &FALCHION_INFO,
      Self::ExaltedFalchion => &EXALTED_FALCHION_INFO,
      Self::ParallelFalchion => &PARALLEL_FALCHION_INFO,
      Self::Mercurius => &MERCURIUS_INFO,
      Self::Tyrfing => &TYRFING_INFO,
      Self::Mystletainn => &MYSTLETAINN_INFO,
      Self::Balmung => &BALMUNG_INFO,
      Self::SolKatti => &SOL_KATTI_INFO,
      Self::Ragnell => &RAGNELL_INFO,
      Self::RagnellInfinite => &RAGNELL_INFINITE_INFO,
      Self::TreeBranch => &TREE_BRANCH_INFO,
      Self::SoothingSword => &SOOTHING_SWORD_INFO,
      Self::GlassSword => &GLASS_SWORD_INFO,
      Self::SuperiorEdge => &SUPERIOR_EDGE_INFO,
      Self::EliwoodsBlade => &ELIWOODS_BLADE_INFO,
      Self::RoysBlade => &ROYS_BLADE_INFO,
      Self::AlmsBlade => &ALMS_BLADE_INFO,
      Self::LeifsBlade => &LEIFS_BLADE_INFO,
      Self::EirikasBlade => &EIRIKAS_BLADE_INFO,
      Self::SeliphsBlade => &SELIPHS_BLADE_INFO,
      Self::BronzeLance => &BRONZE_LANCE_INFO,
      Self::IronLance => &IRON_LANCE_INFO,
      Self::SteelLance => &STEEL_LANCE_INFO,
      Self::SilverLance => &SILVER_LANCE_INFO,
      Self::BraveLance => &BRAVE_LANCE_INFO,
      Self::Javelin => &JAVELIN_INFO,
      Self::ShortSpear => &SHORT_SPEAR_INFO,
      Self::Spear => &SPEAR_INFO,
      Self::BeastKiller => &BEAST_KILLER_INFO,
      Self::BlessedLance => &BLESSED_LANCE_INFO,
      Self::KillerLance => &KILLER_LANCE_INFO,
      Self::Luna => &LUNA_INFO,
      Self::Gradivus => &GRADIVUS_INFO,
      Self::Gungnir => &GUNGNIR_INFO,
      Self::GaeBolg => &GAE_BOLG_INFO,
      Self::Log => &LOG_INFO,
      Self::MiniatureLance => &MINIATURE_LANCE_INFO,
      Self::Shockstick => &SHOCKSTICK_INFO,
      Self::GlassLance => &GLASS_LANCE_INFO,
      Self::SuperiorLance => &SUPERIOR_LANCE_INFO,
      Self::SigurdsLance => &SIGURDS_LANCE_INFO,
      Self::EphraimsLance => &EPHRAIMS_LANCE_INFO,
      Self::FinnsLance => &FINNS_LANCE_INFO,
      Self::BronzeAxe => &BRONZE_AXE_INFO,
      Self::IronAxe => &IRON_AXE_INFO,
      Self::SteelAxe => &STEEL_AXE_INFO,
      Self::SilverAxe => &SILVER_AXE_INFO,
      Self::BraveAxe => &BRAVE_AXE_INFO,
      Self::HandAxe => &HAND_AXE_INFO,
      Self::ShortAxe => &SHORT_AXE_INFO,
      Self::Tomahawk => &TOMAHAWK_INFO,
      Self::Hammer => &HAMMER_INFO,
      Self::BoltAxe => &BOLT_AXE_INFO,
      Self::KillerAxe => &KILLER_AXE_INFO,
      Self::Vengeance => &VENGEANCE_INFO,
      Self::WolfBerg => &WOLF_BERG_INFO,
      Self::Hauteclere => &HAUTECLERE_INFO,
      Self::Helswath => &HELSWATH_INFO,
      Self::Armads => &ARMADS_INFO,
      Self::Ladle => &LADLE_INFO,
      Self::ImposingAxe => &IMPOSING_AXE_INFO,
      Self::VolantAxe => &VOLANT_AXE_INFO,
      Self::GlassAxe => &GLASS_AXE_INFO,
      Self::SuperiorAxe => &SUPERIOR_AXE_INFO,
      Self::TitaniasAxe => &TITANIAS_AXE_INFO,
      Self::OrsinsHatchet => &ORSINS_HATCHET_INFO,
      Self::HectorsAxe => &HECTORS_AXE_INFO,
      Self::BronzeBow => &BRONZE_BOW_INFO,
      Self::IronBow => &IRON_BOW_INFO,
      Self::SteelBow => &STEEL_BOW_INFO,
      Self::SilverBow => &SILVER_BOW_INFO,
      Self::BraveBow => &BRAVE_BOW_INFO,
      Self::BlessedBow => &BLESSED_BOW_INFO,
      Self::KillerBow => &KILLER_BOW_INFO,
      Self::LongBow => &LONG_BOW_INFO,
      Self::Astra => &ASTRA_INFO,
      Self::Parthia => &PARTHIA_INFO,
      Self::Yewfelle => &YEWFELLE_INFO,
      Self::Nidhogg => &NIDHOGG_INFO,
      Self::DoubleBow => &DOUBLE_BOW_INFO,
      Self::SlackBow => &SLACK_BOW_INFO,
      Self::ToweringBow => &TOWERING_BOW_INFO,
      Self::UnderdogBow => &UNDERDOG_BOW_INFO,
      Self::GlassBow => &GLASS_BOW_INFO,
      Self::SuperiorBow => &SUPERIOR_BOW_INFO,
      Self::WoltsBow => &WOLTS_BOW_INFO,
      Self::InnesBow => &INNES_BOW_INFO,
      Self::Fire => &FIRE_INFO,
      Self::Elfire => &ELFIRE_INFO,
      Self::Arcfire => &ARCFIRE_INFO,
      Self::Bolganone => &BOLGANONE_INFO,
      Self::Valflame => &VALFLAME_INFO,
      Self::Thunder => &THUNDER_INFO,
      Self::Elthunder => &ELTHUNDER_INFO,
      Self::Arcthunder => &ARCTHUNDER_INFO,
      Self::Thoron => &THORON_INFO,
      Self::Mjollnir => &MJOLLNIR_INFO,
      Self::Wind => &WIND_INFO,
      Self::Elwind => &ELWIND_INFO,
      Self::Arcwind => &ARCWIND_INFO,
      Self::Rexcalibur => &REXCALIBUR_INFO,
      Self::Forseti => &FORSETI_INFO,
      Self::Excalibur => &EXCALIBUR_INFO,
      Self::BookOfNaga => &BOOK_OF_NAGA_INFO,
      Self::Flux => &FLUX_INFO,
      Self::Nosferatu => &NOSFERATU_INFO,
      Self::Ruin => &RUIN_INFO,
      Self::Waste => &WASTE_INFO,
      Self::Goetia => &GOETIA_INFO,
      Self::GrimasTruth => &GRIMAS_TRUTH_INFO,
      Self::Mire => &MIRE_INFO,
      Self::DyingBlaze => &DYING_BLAZE_INFO,
      Self::MicaiahsPyre => &MICAIAHS_PYRE_INFO,
      Self::SuperiorJolt => &SUPERIOR_JOLT_INFO,
      Self::KatarinasBolt => &KATARINAS_BOLT_INFO,
      Self::Wilderwind => &WILDERWIND_INFO,
      Self::CelicasGale => &CELICAS_GALE_INFO,
      Self::AversasNight => &AVERSAS_NIGHT_INFO,
      Self::Heal => &HEAL_INFO,
      Self::Mend => &MEND_INFO,
      Self::Physic => &PHYSIC_INFO,
      Self::Recover => &RECOVER_INFO,
      Self::Fortify => &FORTIFY_INFO,
      Self::GoddessStaff => &GODDESS_STAFF_INFO,
      Self::Rescue => &RESCUE_INFO,
      Self::Ward => &WARD_INFO,
      Self::Hammerne => &HAMMERNE_INFO,
      Self::Kneader => &KNEADER_INFO,
      Self::BalmwoodStaff => &BALMWOOD_STAFF_INFO,
      Self::Catharsis => &CATHARSIS_INFO,
      Self::Dragonstone => &DRAGONSTONE_INFO,
      Self::Dragonstoneplus => &DRAGONSTONEPLUS_INFO,
      Self::Beaststone => &BEASTSTONE_INFO,
      Self::Beaststoneplus => &BEASTSTONEPLUS_INFO,
      Self::BlightedClaws => &BLIGHTED_CLAWS_INFO,
      Self::BlightedTalons => &BLIGHTED_TALONS_INFO,
      Self::Expiration => &EXPIRATION_INFO,
      Self::Vulnerary => &VULNERARY_INFO,
      Self::Concoction => &CONCOCTION_INFO,
      Self::Elixer => &ELIXER_INFO,
      Self::PureWater => &PURE_WATER_INFO,
      Self::HpTonic => &HP_TONIC_INFO,
      Self::StrengthTonic => &STRENGTH_TONIC_INFO,
      Self::MagicTonic => &MAGIC_TONIC_INFO,
      Self::SkillTonic => &SKILL_TONIC_INFO,
      Self::SpeedTonic => &SPEED_TONIC_INFO,
      Self::LuckTonic => &LUCK_TONIC_INFO,
      Self::DefenseTonic => &DEFENSE_TONIC_INFO,
      Self::ResistanceTonic => &RESISTANCE_TONIC_INFO,
      Self::DoorKey => &DOOR_KEY_INFO,
      Self::ChestKey => &CHEST_KEY_INFO,
      Self::MasterKey => &MASTER_KEY_INFO,
      Self::SeraphRobe => &SERAPH_ROBE_INFO,
      Self::EnergyDrop => &ENERGY_DROP_INFO,
      Self::SpiritDust => &SPIRIT_DUST_INFO,
      Self::SecretBook => &SECRET_BOOK_INFO,
      Self::Speedwing => &SPEEDWING_INFO,
      Self::GoddessIcon => &GODDESS_ICON_INFO,
      Self::Dracoshield => &DRACOSHIELD_INFO,
      Self::Talisman => &TALISMAN_INFO,
      Self::NagasTear => &NAGAS_TEAR_INFO,
      Self::Boots => &BOOTS_INFO,
      Self::ArmsScroll => &ARMS_SCROLL_INFO,
      Self::MasterSeal => &MASTER_SEAL_INFO,
      Self::SecondSeal => &SECOND_SEAL_INFO,
      Self::BullionS => &BULLION_S_INFO,
      Self::BullionM => &BULLION_M_INFO,
      Self::BullionL => &BULLION_L_INFO,
      Self::SweetTincture => &SWEET_TINCTURE_INFO,
      Self::GaiusConfect => &GAIUS_CONFECT_INFO,
      Self::KrisConfect => &KRIS_CONFECT_INFO,
      Self::TikisTear => &TIKIS_TEAR_INFO,
      Self::SeedOfTrust => &SEED_OF_TRUST_INFO,
      Self::ReekingBox => &REEKING_BOX_INFO,
      Self::RiftDoor => &RIFT_DOOR_INFO,
      Self::SupremeEmblem => &SUPREME_EMBLEM_INFO,
      Self::AllStatsPlus2 => &ALL_STATS_PLUS2_INFO,
      Self::Paragon => &PARAGON_INFO,
      Self::IotesShield => &IOTES_SHIELD_INFO,
      Self::LimitBreaker => &LIMIT_BREAKER_INFO,
      Self::SilverCard => &SILVER_CARD_INFO,
      Self::DreadScroll => &DREAD_SCROLL_INFO,
      Self::WeddingBouquet => &WEDDING_BOUQUET_INFO,
      Self::Gold1000 => &GOLD_1000_INFO,
      Self::Gold3000 => &GOLD_3000_INFO,
      Self::Gold5000 => &GOLD_5000_INFO,
      Self::Gold7000 => &GOLD_7000_INFO,
      Self::Unknown(_) => &UNKNOWN_INFO,
    }
  }

  /// Converts a raw 16-bit game item identifier to an `ItemType` variant.
  #[must_use]
  #[expect(
    clippy::too_many_lines,
    reason = "Comprehensive FE13 item catalog maps all in-game item variants"
  )]
  pub(crate) const fn from_raw(raw: u16) -> Self {
    match raw {
      1 => Self::BronzeSword,
      2 => Self::IronSword,
      3 => Self::SteelSword,
      4 => Self::SilverSword,
      5 => Self::BraveSword,
      6 => Self::Armorslayer,
      7 => Self::Wyrmslayer,
      8 => Self::KillingEdge,
      9 => Self::LevinSword,
      10 => Self::Rapier,
      11 => Self::NobleRapier,
      12 => Self::Missiletainn,
      13 => Self::Sol,
      14 => Self::Amatsu,
      15 => Self::Falchion,
      16 => Self::ExaltedFalchion,
      17 => Self::ParallelFalchion,
      18 => Self::Mercurius,
      19 => Self::Tyrfing,
      20 => Self::Mystletainn,
      21 => Self::Balmung,
      22 => Self::SolKatti,
      23 => Self::Ragnell,
      24 => Self::RagnellInfinite,
      25 => Self::TreeBranch,
      26 => Self::SoothingSword,
      27 => Self::GlassSword,
      28 => Self::SuperiorEdge,
      29 => Self::EliwoodsBlade,
      30 => Self::RoysBlade,
      31 => Self::AlmsBlade,
      32 => Self::LeifsBlade,
      33 => Self::EirikasBlade,
      34 => Self::SeliphsBlade,
      35 => Self::BronzeLance,
      36 => Self::IronLance,
      37 => Self::SteelLance,
      38 => Self::SilverLance,
      39 => Self::BraveLance,
      40 => Self::Javelin,
      41 => Self::ShortSpear,
      42 => Self::Spear,
      43 => Self::BeastKiller,
      44 => Self::BlessedLance,
      45 => Self::KillerLance,
      46 => Self::Luna,
      47 => Self::Gradivus,
      48 => Self::Gungnir,
      49 => Self::GaeBolg,
      50 => Self::Log,
      51 => Self::MiniatureLance,
      52 => Self::Shockstick,
      53 => Self::GlassLance,
      54 => Self::SuperiorLance,
      55 => Self::SigurdsLance,
      56 => Self::EphraimsLance,
      57 => Self::FinnsLance,
      58 => Self::BronzeAxe,
      59 => Self::IronAxe,
      60 => Self::SteelAxe,
      61 => Self::SilverAxe,
      62 => Self::BraveAxe,
      63 => Self::HandAxe,
      64 => Self::ShortAxe,
      65 => Self::Tomahawk,
      66 => Self::Hammer,
      67 => Self::BoltAxe,
      68 => Self::KillerAxe,
      69 => Self::Vengeance,
      70 => Self::WolfBerg,
      71 => Self::Hauteclere,
      72 => Self::Helswath,
      73 => Self::Armads,
      74 => Self::Ladle,
      75 => Self::ImposingAxe,
      76 => Self::VolantAxe,
      77 => Self::GlassAxe,
      78 => Self::SuperiorAxe,
      79 => Self::TitaniasAxe,
      80 => Self::OrsinsHatchet,
      81 => Self::HectorsAxe,
      82 => Self::BronzeBow,
      83 => Self::IronBow,
      84 => Self::SteelBow,
      85 => Self::SilverBow,
      86 => Self::BraveBow,
      87 => Self::BlessedBow,
      88 => Self::KillerBow,
      89 => Self::LongBow,
      90 => Self::Astra,
      91 => Self::Parthia,
      92 => Self::Yewfelle,
      93 => Self::Nidhogg,
      94 => Self::DoubleBow,
      95 => Self::SlackBow,
      96 => Self::ToweringBow,
      97 => Self::UnderdogBow,
      98 => Self::GlassBow,
      99 => Self::SuperiorBow,
      100 => Self::WoltsBow,
      101 => Self::InnesBow,
      102 => Self::Fire,
      103 => Self::Elfire,
      104 => Self::Arcfire,
      105 => Self::Bolganone,
      106 => Self::Valflame,
      107 => Self::Thunder,
      108 => Self::Elthunder,
      109 => Self::Arcthunder,
      110 => Self::Thoron,
      111 => Self::Mjollnir,
      112 => Self::Wind,
      113 => Self::Elwind,
      114 => Self::Arcwind,
      115 => Self::Rexcalibur,
      116 => Self::Forseti,
      117 => Self::Excalibur,
      118 => Self::BookOfNaga,
      119 => Self::Flux,
      120 => Self::Nosferatu,
      121 => Self::Ruin,
      122 => Self::Waste,
      123 => Self::Goetia,
      124 => Self::GrimasTruth,
      125 => Self::Mire,
      126 => Self::DyingBlaze,
      127 => Self::MicaiahsPyre,
      128 => Self::SuperiorJolt,
      129 => Self::KatarinasBolt,
      130 => Self::Wilderwind,
      131 => Self::CelicasGale,
      132 => Self::AversasNight,
      133 => Self::Heal,
      134 => Self::Mend,
      135 => Self::Physic,
      136 => Self::Recover,
      137 => Self::Fortify,
      138 => Self::GoddessStaff,
      139 => Self::Rescue,
      140 => Self::Ward,
      141 => Self::Hammerne,
      142 => Self::Kneader,
      143 => Self::BalmwoodStaff,
      144 => Self::Catharsis,
      145 => Self::Dragonstone,
      146 => Self::Dragonstoneplus,
      147 => Self::Beaststone,
      148 => Self::Beaststoneplus,
      149 => Self::BlightedClaws,
      150 => Self::BlightedTalons,
      151 => Self::Expiration,
      152 => Self::Vulnerary,
      153 => Self::Concoction,
      154 => Self::Elixer,
      155 => Self::PureWater,
      156 => Self::HpTonic,
      157 => Self::StrengthTonic,
      158 => Self::MagicTonic,
      159 => Self::SkillTonic,
      160 => Self::SpeedTonic,
      161 => Self::LuckTonic,
      162 => Self::DefenseTonic,
      163 => Self::ResistanceTonic,
      164 => Self::DoorKey,
      165 => Self::ChestKey,
      166 => Self::MasterKey,
      167 => Self::SeraphRobe,
      168 => Self::EnergyDrop,
      169 => Self::SpiritDust,
      170 => Self::SecretBook,
      171 => Self::Speedwing,
      172 => Self::GoddessIcon,
      173 => Self::Dracoshield,
      174 => Self::Talisman,
      175 => Self::NagasTear,
      176 => Self::Boots,
      177 => Self::ArmsScroll,
      178 => Self::MasterSeal,
      179 => Self::SecondSeal,
      180 => Self::BullionS,
      181 => Self::BullionM,
      182 => Self::BullionL,
      183 => Self::SweetTincture,
      184 => Self::GaiusConfect,
      185 => Self::KrisConfect,
      186 => Self::TikisTear,
      187 => Self::SeedOfTrust,
      188 => Self::ReekingBox,
      189 => Self::RiftDoor,
      190 => Self::SupremeEmblem,
      191 => Self::AllStatsPlus2,
      192 => Self::Paragon,
      193 => Self::IotesShield,
      194 => Self::LimitBreaker,
      195 => Self::SilverCard,
      196 => Self::DreadScroll,
      197 => Self::WeddingBouquet,
      198 => Self::Gold1000,
      199 => Self::Gold3000,
      200 => Self::Gold5000,
      201 => Self::Gold7000,
      other => Self::Unknown(other),
    }
  }

  /// Returns all canonical non-unknown item variants.
  #[must_use]
  pub const fn all() -> &'static [Self] {
    &ALL_ITEMS
  }

  /// Resolves an item type from its canonical numeric identifier.
  #[must_use]
  pub const fn from_id(id: u16) -> Self {
    Self::from_raw(id)
  }

  /// Canonical item identifier.
  #[must_use]
  pub const fn id(self) -> u16 {
    self.raw_id()
  }

  /// Raw 16-bit item identifier.
  #[must_use]
  pub(crate) const fn raw_id(self) -> u16 {
    match self {
      Self::Unknown(id) => id,
      other => other.info().raw_id,
    }
  }

  /// Canonical display name.
  #[must_use]
  pub const fn name(self) -> &'static str {
    self.info().name
  }

  /// Broad equipment category.
  #[must_use]
  pub const fn kind(self) -> kind::ItemKind {
    self.info().kind
  }

  /// Base weapon might.
  #[must_use]
  pub const fn might(self) -> u8 {
    self.info().might
  }

  /// Base accuracy hit rate.
  #[must_use]
  pub const fn hit(self) -> u8 {
    self.info().hit
  }

  /// Base critical hit rate.
  #[must_use]
  pub const fn crit(self) -> u8 {
    self.info().crit
  }

  /// Canonical max durability uses, or `None` if unbreakable.
  #[must_use]
  pub const fn max_durability(self) -> Option<u8> {
    self.info().max_durability
  }

  /// Whether this item variant is unbreakable (has infinite durability).
  #[must_use]
  pub const fn is_unbreakable(self) -> bool {
    self.max_durability().is_none()
  }

  /// Whether this item variant can be equipped by a unit.
  #[must_use]
  pub const fn is_equippable(self) -> bool {
    self.kind().is_equippable()
  }

  /// Whether this item variant supports weapon forging.
  #[must_use]
  pub const fn is_forgeable(self) -> bool {
    if self.is_unbreakable() {
      return false;
    }
    match self.kind() {
      kind::ItemKind::Sword
      | kind::ItemKind::Lance
      | kind::ItemKind::Axe
      | kind::ItemKind::Bow
      | kind::ItemKind::Tome => true,
      kind::ItemKind::Staff
      | kind::ItemKind::Stone
      | kind::ItemKind::Consumable
      | kind::ItemKind::Seal
      | kind::ItemKind::StatBooster
      | kind::ItemKind::Valuable
      | kind::ItemKind::Unknown => false,
    }
  }
}

impl fmt::Display for ItemType {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Unknown(id) => write!(f, "Unknown Item ({id})"),
      other => write!(f, "{}", other.name()),
    }
  }
}
