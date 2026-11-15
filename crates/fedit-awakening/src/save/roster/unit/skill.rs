//! Skill domain enumerations and canonical identifiers.
//!
//! Provides the exhaustive strongly-typed enumeration of personal, class, DLC,
//! and enemy skills in Fire Emblem Awakening, mapping to and from raw 16-bit IDs.

use std::fmt;

/// Strongly typed skill enumeration for Fire Emblem: Awakening.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
pub enum Skill {
  /// Empty or unassigned skill slot.
  None,

  // Stat buffs & basics (0x01..=0x0F)
  /// HP +5.
  HpPlus5,
  /// Strength +2.
  StrengthPlus2,
  /// Magic +2.
  MagicPlus2,
  /// Skill +2.
  SkillPlus2,
  /// Speed +2.
  SpeedPlus2,
  /// Defense +2.
  DefensePlus2,
  /// Resistance +2.
  ResistancePlus2,
  /// Hit Rate +10.
  HitRatePlus10,
  /// Hit Rate +20.
  HitRatePlus20,
  /// Avoid +10.
  AvoidPlus10,
  /// Movement +1.
  MovementPlus1,
  /// Locktouch.
  Locktouch,
  /// Veteran.
  Veteran,
  /// Aptitude.
  Aptitude,
  /// Discipline.
  Discipline,

  // Class & combat skills (0x10..=0x2F)
  /// Armsthrift.
  Armsthrift,
  /// Dual Support+.
  DualSupportPlus,
  /// Dual Strike+.
  DualStrikePlus,
  /// Dual Guard+.
  DualGuardPlus,
  /// Rightful King.
  RightfulKing,
  /// Odd Rhythm.
  OddRhythm,
  /// Even Rhythm.
  EvenRhythm,
  /// Quick Burn.
  QuickBurn,
  /// Slow Burn.
  SlowBurn,
  /// Lucky Seven.
  LuckySeven,
  /// Zeal.
  Zeal,
  /// Outdoor Fighter.
  OutdoorFighter,
  /// Indoor Fighter.
  IndoorFighter,
  /// Tantivy.
  Tantivy,
  /// Focus.
  Focus,
  /// Gamble.
  Gamble,
  /// Wrath.
  Wrath,
  /// Prescience.
  Prescience,
  /// Patience.
  Patience,
  /// Underdog.
  Underdog,
  /// Charm.
  Charm,
  /// Solidarity.
  Solidarity,
  /// Demoiselle.
  Demoiselle,
  /// Hex.
  Hex,
  /// Anathema.
  Anathema,
  /// Healtouch.
  Healtouch,
  /// Relief.
  Relief,
  /// Renewal.
  Renewal,
  /// Deliverer.
  Deliverer,
  /// Defender.
  Defender,
  /// Acrobat.
  Acrobat,
  /// Pass.
  Pass,

  // Faires, rallies & breakers (0x30..=0x47)
  /// Swordfaire.
  Swordfaire,
  /// Lancefaire.
  Lancefaire,
  /// Axefaire.
  Axefaire,
  /// Bowfaire.
  Bowfaire,
  /// Tomefaire.
  Tomefaire,
  /// Luck +4.
  LuckPlus4,
  /// Special Dance.
  SpecialDance,
  /// Rally Strength.
  RallyStrength,
  /// Rally Magic.
  RallyMagic,
  /// Rally Skill.
  RallySkill,
  /// Rally Speed.
  RallySpeed,
  /// Rally Luck.
  RallyLuck,
  /// Rally Defense.
  RallyDefense,
  /// Rally Resistance.
  RallyResistance,
  /// Rally Movement.
  RallyMovement,
  /// Rally Spectrum.
  RallySpectrum,
  /// Swordbreaker.
  Swordbreaker,
  /// Lancebreaker.
  Lancebreaker,
  /// Axebreaker.
  Axebreaker,
  /// Bowbreaker.
  Bowbreaker,
  /// Tomebreaker.
  Tomebreaker,
  /// Wyrmsbane.
  Wyrmsbane,
  /// Beastbane.
  Beastbane,
  /// Lethality.
  Lethality,

  // Advanced & proc skills (0x48..=0x57)
  /// Aether.
  Aether,
  /// Astra.
  Astra,
  /// Sol.
  Sol,
  /// Luna.
  Luna,
  /// Ignis.
  Ignis,
  /// Vengeance.
  Vengeance,
  /// Vantage.
  Vantage,
  /// Pavise.
  Pavise,
  /// Aegis.
  Aegis,
  /// Counter.
  Counter,
  /// Miracle.
  Miracle,
  /// Despoil.
  Despoil,
  /// Galeforce.
  Galeforce,
  /// Lifetaker.
  Lifetaker,
  /// Conquest.
  Conquest,
  /// Shadowgift.
  Shadowgift,

  // DLC & boss skills (0x58..=0xC1)
  /// All Stats +2.
  AllStatsPlus2,
  /// Limit Breaker.
  LimitBreaker,
  /// Dragonskin.
  Dragonskin,
  /// Rightful God.
  RightfulGod,
  /// Hawkeye.
  Hawkeye,
  /// Luna+.
  LunaPlus,
  /// Vantage+.
  VantagePlus,
  /// Pavise+.
  PavisePlus,
  /// Aegis+.
  AegisPlus,
  /// Resistance +10.
  ResistancePlus10,
  /// Paragon.
  Paragon,
  /// Iote's Shield.
  IotesShield,

  /// Unknown or unrecognized skill identifier.
  Unknown(u16),
}

impl Skill {
  /// Converts a raw 16-bit identifier into a [`Skill`] variant.
  #[must_use]
  #[expect(
    clippy::too_many_lines,
    reason = "exhaustive match over all FE13 skill variants"
  )]
  pub const fn from_raw(raw: u16) -> Self {
    match raw {
      0 => Self::None,
      0x01 => Self::HpPlus5,
      0x02 => Self::StrengthPlus2,
      0x03 => Self::MagicPlus2,
      0x04 => Self::SkillPlus2,
      0x05 => Self::SpeedPlus2,
      0x06 => Self::DefensePlus2,
      0x07 => Self::ResistancePlus2,
      0x08 => Self::HitRatePlus10,
      0x09 => Self::HitRatePlus20,
      0x0A => Self::AvoidPlus10,
      0x0B => Self::MovementPlus1,
      0x0C => Self::Locktouch,
      0x0D => Self::Veteran,
      0x0E => Self::Aptitude,
      0x0F => Self::Discipline,
      0x10 => Self::Armsthrift,
      0x11 => Self::DualSupportPlus,
      0x12 => Self::DualStrikePlus,
      0x13 => Self::DualGuardPlus,
      0x14 => Self::RightfulKing,
      0x15 => Self::OddRhythm,
      0x16 => Self::EvenRhythm,
      0x17 => Self::QuickBurn,
      0x18 => Self::SlowBurn,
      0x19 => Self::LuckySeven,
      0x1A => Self::Zeal,
      0x1B => Self::OutdoorFighter,
      0x1C => Self::IndoorFighter,
      0x1D => Self::Tantivy,
      0x1E => Self::Focus,
      0x1F => Self::Gamble,
      0x20 => Self::Wrath,
      0x21 => Self::Prescience,
      0x22 => Self::Patience,
      0x23 => Self::Underdog,
      0x24 => Self::Charm,
      0x25 => Self::Solidarity,
      0x26 => Self::Demoiselle,
      0x27 => Self::Hex,
      0x28 => Self::Anathema,
      0x29 => Self::Healtouch,
      0x2A => Self::Relief,
      0x2B => Self::Renewal,
      0x2C => Self::Deliverer,
      0x2D => Self::Defender,
      0x2E => Self::Acrobat,
      0x2F => Self::Pass,
      0x30 => Self::Swordfaire,
      0x31 => Self::Lancefaire,
      0x32 => Self::Axefaire,
      0x33 => Self::Bowfaire,
      0x34 => Self::Tomefaire,
      0x35 => Self::LuckPlus4,
      0x36 => Self::SpecialDance,
      0x37 => Self::RallyStrength,
      0x38 => Self::RallyMagic,
      0x39 => Self::RallySkill,
      0x3A => Self::RallySpeed,
      0x3B => Self::RallyLuck,
      0x3C => Self::RallyDefense,
      0x3D => Self::RallyResistance,
      0x3E => Self::RallyMovement,
      0x3F => Self::RallySpectrum,
      0x40 => Self::Swordbreaker,
      0x41 => Self::Lancebreaker,
      0x42 => Self::Axebreaker,
      0x43 => Self::Bowbreaker,
      0x44 => Self::Tomebreaker,
      0x45 => Self::Wyrmsbane,
      0x46 => Self::Beastbane,
      0x47 => Self::Lethality,
      0x48 => Self::Aether,
      0x49 => Self::Astra,
      0x4A => Self::Sol,
      0x4B => Self::Luna,
      0x4C => Self::Ignis,
      0x4D => Self::Vengeance,
      0x4E => Self::Vantage,
      0x4F => Self::Pavise,
      0x50 => Self::Aegis,
      0x51 => Self::Counter,
      0x52 => Self::Miracle,
      0x53 => Self::Despoil,
      0x54 => Self::Galeforce,
      0x55 => Self::Lifetaker,
      0x56 => Self::Conquest,
      0x57 => Self::Shadowgift,
      0x58 => Self::AllStatsPlus2,
      0x5B => Self::LimitBreaker,
      0x5C => Self::Dragonskin,
      0x5D => Self::RightfulGod,
      0x5E => Self::Hawkeye,
      0x5F => Self::LunaPlus,
      0x60 => Self::VantagePlus,
      0x61 => Self::PavisePlus,
      0x62 => Self::AegisPlus,
      0x63 => Self::ResistancePlus10,
      0xC0 => Self::Paragon,
      0xC1 => Self::IotesShield,
      other => Self::Unknown(other),
    }
  }

  /// Converts a [`Skill`] variant back into its raw 16-bit identifier.
  #[must_use]
  #[expect(
    clippy::too_many_lines,
    reason = "exhaustive match over all FE13 skill variants"
  )]
  pub const fn raw_id(self) -> u16 {
    match self {
      Self::None => 0,
      Self::HpPlus5 => 0x01,
      Self::StrengthPlus2 => 0x02,
      Self::MagicPlus2 => 0x03,
      Self::SkillPlus2 => 0x04,
      Self::SpeedPlus2 => 0x05,
      Self::DefensePlus2 => 0x06,
      Self::ResistancePlus2 => 0x07,
      Self::HitRatePlus10 => 0x08,
      Self::HitRatePlus20 => 0x09,
      Self::AvoidPlus10 => 0x0A,
      Self::MovementPlus1 => 0x0B,
      Self::Locktouch => 0x0C,
      Self::Veteran => 0x0D,
      Self::Aptitude => 0x0E,
      Self::Discipline => 0x0F,
      Self::Armsthrift => 0x10,
      Self::DualSupportPlus => 0x11,
      Self::DualStrikePlus => 0x12,
      Self::DualGuardPlus => 0x13,
      Self::RightfulKing => 0x14,
      Self::OddRhythm => 0x15,
      Self::EvenRhythm => 0x16,
      Self::QuickBurn => 0x17,
      Self::SlowBurn => 0x18,
      Self::LuckySeven => 0x19,
      Self::Zeal => 0x1A,
      Self::OutdoorFighter => 0x1B,
      Self::IndoorFighter => 0x1C,
      Self::Tantivy => 0x1D,
      Self::Focus => 0x1E,
      Self::Gamble => 0x1F,
      Self::Wrath => 0x20,
      Self::Prescience => 0x21,
      Self::Patience => 0x22,
      Self::Underdog => 0x23,
      Self::Charm => 0x24,
      Self::Solidarity => 0x25,
      Self::Demoiselle => 0x26,
      Self::Hex => 0x27,
      Self::Anathema => 0x28,
      Self::Healtouch => 0x29,
      Self::Relief => 0x2A,
      Self::Renewal => 0x2B,
      Self::Deliverer => 0x2C,
      Self::Defender => 0x2D,
      Self::Acrobat => 0x2E,
      Self::Pass => 0x2F,
      Self::Swordfaire => 0x30,
      Self::Lancefaire => 0x31,
      Self::Axefaire => 0x32,
      Self::Bowfaire => 0x33,
      Self::Tomefaire => 0x34,
      Self::LuckPlus4 => 0x35,
      Self::SpecialDance => 0x36,
      Self::RallyStrength => 0x37,
      Self::RallyMagic => 0x38,
      Self::RallySkill => 0x39,
      Self::RallySpeed => 0x3A,
      Self::RallyLuck => 0x3B,
      Self::RallyDefense => 0x3C,
      Self::RallyResistance => 0x3D,
      Self::RallyMovement => 0x3E,
      Self::RallySpectrum => 0x3F,
      Self::Swordbreaker => 0x40,
      Self::Lancebreaker => 0x41,
      Self::Axebreaker => 0x42,
      Self::Bowbreaker => 0x43,
      Self::Tomebreaker => 0x44,
      Self::Wyrmsbane => 0x45,
      Self::Beastbane => 0x46,
      Self::Lethality => 0x47,
      Self::Aether => 0x48,
      Self::Astra => 0x49,
      Self::Sol => 0x4A,
      Self::Luna => 0x4B,
      Self::Ignis => 0x4C,
      Self::Vengeance => 0x4D,
      Self::Vantage => 0x4E,
      Self::Pavise => 0x4F,
      Self::Aegis => 0x50,
      Self::Counter => 0x51,
      Self::Miracle => 0x52,
      Self::Despoil => 0x53,
      Self::Galeforce => 0x54,
      Self::Lifetaker => 0x55,
      Self::Conquest => 0x56,
      Self::Shadowgift => 0x57,
      Self::AllStatsPlus2 => 0x58,
      Self::LimitBreaker => 0x5B,
      Self::Dragonskin => 0x5C,
      Self::RightfulGod => 0x5D,
      Self::Hawkeye => 0x5E,
      Self::LunaPlus => 0x5F,
      Self::VantagePlus => 0x60,
      Self::PavisePlus => 0x61,
      Self::AegisPlus => 0x62,
      Self::ResistancePlus10 => 0x63,
      Self::Paragon => 0xC0,
      Self::IotesShield => 0xC1,
      Self::Unknown(id) => id,
    }
  }

  /// Returns the human-readable canonical name of the skill.
  #[must_use]
  #[expect(
    clippy::too_many_lines,
    reason = "exhaustive match over all FE13 skill variants"
  )]
  pub const fn name(self) -> &'static str {
    match self {
      Self::None => "None",
      Self::HpPlus5 => "HP +5",
      Self::StrengthPlus2 => "Strength +2",
      Self::MagicPlus2 => "Magic +2",
      Self::SkillPlus2 => "Skill +2",
      Self::SpeedPlus2 => "Speed +2",
      Self::DefensePlus2 => "Defense +2",
      Self::ResistancePlus2 => "Resistance +2",
      Self::HitRatePlus10 => "Hit Rate +10",
      Self::HitRatePlus20 => "Hit Rate +20",
      Self::AvoidPlus10 => "Avoid +10",
      Self::MovementPlus1 => "Movement +1",
      Self::Locktouch => "Locktouch",
      Self::Veteran => "Veteran",
      Self::Aptitude => "Aptitude",
      Self::Discipline => "Discipline",
      Self::Armsthrift => "Armsthrift",
      Self::DualSupportPlus => "Dual Support+",
      Self::DualStrikePlus => "Dual Strike+",
      Self::DualGuardPlus => "Dual Guard+",
      Self::RightfulKing => "Rightful King",
      Self::OddRhythm => "Odd Rhythm",
      Self::EvenRhythm => "Even Rhythm",
      Self::QuickBurn => "Quick Burn",
      Self::SlowBurn => "Slow Burn",
      Self::LuckySeven => "Lucky Seven",
      Self::Zeal => "Zeal",
      Self::OutdoorFighter => "Outdoor Fighter",
      Self::IndoorFighter => "Indoor Fighter",
      Self::Tantivy => "Tantivy",
      Self::Focus => "Focus",
      Self::Gamble => "Gamble",
      Self::Wrath => "Wrath",
      Self::Prescience => "Prescience",
      Self::Patience => "Patience",
      Self::Underdog => "Underdog",
      Self::Charm => "Charm",
      Self::Solidarity => "Solidarity",
      Self::Demoiselle => "Demoiselle",
      Self::Hex => "Hex",
      Self::Anathema => "Anathema",
      Self::Healtouch => "Healtouch",
      Self::Relief => "Relief",
      Self::Renewal => "Renewal",
      Self::Deliverer => "Deliverer",
      Self::Defender => "Defender",
      Self::Acrobat => "Acrobat",
      Self::Pass => "Pass",
      Self::Swordfaire => "Swordfaire",
      Self::Lancefaire => "Lancefaire",
      Self::Axefaire => "Axefaire",
      Self::Bowfaire => "Bowfaire",
      Self::Tomefaire => "Tomefaire",
      Self::LuckPlus4 => "Luck +4",
      Self::SpecialDance => "Special Dance",
      Self::RallyStrength => "Rally Strength",
      Self::RallyMagic => "Rally Magic",
      Self::RallySkill => "Rally Skill",
      Self::RallySpeed => "Rally Speed",
      Self::RallyLuck => "Rally Luck",
      Self::RallyDefense => "Rally Defense",
      Self::RallyResistance => "Rally Resistance",
      Self::RallyMovement => "Rally Movement",
      Self::RallySpectrum => "Rally Spectrum",
      Self::Swordbreaker => "Swordbreaker",
      Self::Lancebreaker => "Lancebreaker",
      Self::Axebreaker => "Axebreaker",
      Self::Bowbreaker => "Bowbreaker",
      Self::Tomebreaker => "Tomebreaker",
      Self::Wyrmsbane => "Wyrmsbane",
      Self::Beastbane => "Beastbane",
      Self::Lethality => "Lethality",
      Self::Aether => "Aether",
      Self::Astra => "Astra",
      Self::Sol => "Sol",
      Self::Luna => "Luna",
      Self::Ignis => "Ignis",
      Self::Vengeance => "Vengeance",
      Self::Vantage => "Vantage",
      Self::Pavise => "Pavise",
      Self::Aegis => "Aegis",
      Self::Counter => "Counter",
      Self::Miracle => "Miracle",
      Self::Despoil => "Despoil",
      Self::Galeforce => "Galeforce",
      Self::Lifetaker => "Lifetaker",
      Self::Conquest => "Conquest",
      Self::Shadowgift => "Shadowgift",
      Self::AllStatsPlus2 => "All Stats +2",
      Self::LimitBreaker => "Limit Breaker",
      Self::Dragonskin => "Dragonskin",
      Self::RightfulGod => "Rightful God",
      Self::Hawkeye => "Hawkeye",
      Self::LunaPlus => "Luna+",
      Self::VantagePlus => "Vantage+",
      Self::PavisePlus => "Pavise+",
      Self::AegisPlus => "Aegis+",
      Self::ResistancePlus10 => "Resistance +10",
      Self::Paragon => "Paragon",
      Self::IotesShield => "Iote's Shield",
      Self::Unknown(_) => "Unknown Skill",
    }
  }

  /// Returns true if this skill represents an unassigned slot.
  #[must_use]
  pub const fn is_none(self) -> bool {
    matches!(self, Self::None)
  }
}

impl fmt::Display for Skill {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Unknown(id) => write!(f, "Unknown Skill ({id})"),
      _ => write!(f, "{}", self.name()),
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_skill_roundtrip_and_properties() {
    let aether = Skill::Aether;
    assert_eq!(aether.raw_id(), 0x48);
    assert_eq!(aether.name(), "Aether");
    assert_eq!(Skill::from_raw(0x48), aether);

    let galeforce = Skill::Galeforce;
    assert_eq!(galeforce.raw_id(), 0x54);
    assert_eq!(galeforce.name(), "Galeforce");
    assert_eq!(Skill::from_raw(0x54), galeforce);

    let limit_breaker = Skill::LimitBreaker;
    assert_eq!(limit_breaker.raw_id(), 0x5B);
    assert_eq!(limit_breaker.name(), "Limit Breaker");
    assert_eq!(Skill::from_raw(0x5B), limit_breaker);

    let unknown = Skill::from_raw(0x999);
    assert_eq!(unknown, Skill::Unknown(0x999));
    assert_eq!(unknown.raw_id(), 0x999);
  }
}
