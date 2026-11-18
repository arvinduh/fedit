//! `UnitBlock` character slot domain aggregate and mutators.
//!
//! Owns character entry models in the TINU roster block, coordinating combat stats,
//! equipment, skills, and biological gender resolution.

use thiserror;

use crate::save::convoy::item;
use crate::save::header::config;

pub(crate) mod battle;
pub(crate) mod character;
pub(crate) mod class;
pub(crate) mod skill;
pub(crate) mod stat;
pub(crate) mod status;

pub use battle::BattleState;
pub use character::Character;
pub use character::Gender;
pub use class::Class;
pub use skill::Skill;
pub use stat::Error as StatError;
pub use stat::Stat;
pub use stat::StatKind;
pub use status::Availability;
pub use status::UnitStatus;

/// Errors that can occur when mutating unit attributes or inventory.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  /// Level value was out of bounds [1..=40].
  #[error("level {level} is out of bounds [1..=40]")]
  LevelOutOfBounds {
    /// Provided level.
    level: u8,
  },
  /// Exp value was out of bounds [0..=99].
  #[error("exp {exp} is out of bounds [0..=99]")]
  ExpOutOfBounds {
    /// Provided exp.
    exp: u8,
  },
  /// Growth index or value was out of bounds.
  #[error("growth index {index} value {value} is out of bounds")]
  GrowthOutOfBounds {
    /// Stat index.
    index: usize,
    /// Provided value.
    value: u8,
  },
  /// Weapon rank index or value was out of bounds.
  #[error("weapon rank index {index} value {value} is out of bounds")]
  WeaponRankOutOfBounds {
    /// Rank index.
    index: usize,
    /// Provided value.
    value: u8,
  },
  /// Character is incompatible with the specified class.
  #[error("character '{character}' is incompatible with class '{class}'")]
  ClassIncompatible {
    /// Target character.
    character: Character,
    /// Target class.
    class: Class,
  },
  /// Class does not match the character's biological gender.
  #[error("class '{class}' does not match gender '{gender}'")]
  ClassGenderMismatch {
    /// Incompatible class.
    class: Class,
    /// Biological gender.
    gender: Gender,
  },
  /// Item cannot be equipped because it is not a weapon or staff.
  #[error("cannot equip non-weapon item '{item_type}'")]
  CannotEquipNonWeapon {
    /// Attempted item type.
    item_type: item::ItemType,
  },
  /// Class does not possess weapon proficiency for the target item.
  #[error("class '{class}' cannot equip weapon of kind '{kind:?}'")]
  ClassCannotEquip {
    /// Incompatible class.
    class: Class,
    /// Weapon category.
    kind: item::ItemKind,
  },
  /// Target inventory slot index was out of bounds (0..5).
  #[error("unit slot {slot} is out of bounds")]
  SlotOutOfBounds {
    /// Out of bounds slot index.
    slot: usize,
  },
  /// Target inventory slot is empty.
  #[error("unit slot {slot} is empty")]
  EmptySlot {
    /// Empty slot index.
    slot: usize,
  },
  /// Target skill slot index was out of bounds (0..5).
  #[error("skill slot {slot} is out of bounds")]
  SkillSlotOutOfBounds {
    /// Out of bounds slot index.
    slot: usize,
  },
  /// Story anchor character cannot be removed or killed.
  #[error("story anchor unit '{character}' cannot be removed or killed")]
  AnchorUnitProtected {
    /// Protected character.
    character: Character,
  },
  /// Combat stat error.
  #[error(transparent)]
  Stat(#[from] stat::Error),
  /// Movement was out of bounds [min..=max].
  #[error("movement {value} is out of bounds [{min}..={max}]")]
  MovementOutOfBounds {
    /// Provided movement value.
    value: u8,
    /// Minimum allowed.
    min: u8,
    /// Maximum allowed.
    max: u8,
  },
  /// Boots movement bonus exceeded the safe maximum.
  #[error("boots bonus {value} exceeds safe maximum {max}")]
  BootsBonusOutOfBounds {
    /// Provided boots bonus.
    value: u8,
    /// Maximum allowed.
    max: u8,
  },
  /// Coordinates were out of bounds [0..=32].
  #[error("coordinates [{x}, {y}] out of bounds [0..=32]")]
  CoordinatesOutOfBounds {
    /// Provided X coordinate.
    x: u8,
    /// Provided Y coordinate.
    y: u8,
  },
  /// Unit byte slice length was invalid.
  #[error("invalid unit byte length: expected {expected}, got {actual}")]
  InvalidByteLength {
    /// Expected length.
    expected: usize,
    /// Actual length.
    actual: usize,
  },
  /// Item error.
  #[error(transparent)]
  Item(#[from] item::Error),
}

/// Owned character unit domain representation.
#[derive(Clone, Debug)]
pub struct Unit {
  /// Canonical character enumeration.
  character: Character,
  /// Biological gender of the unit.
  gender: Gender,
  /// Unit class enumeration.
  class: Class,
  /// Character experience level (1..=40).
  level: u8,
  /// Level experience progress (0..=99).
  exp: u8,
  /// Maximum hit points attribute.
  pub hp: Stat,
  /// Physical strength attribute.
  pub strength: Stat,
  /// Magic attribute.
  pub magic: Stat,
  /// Skill attribute.
  pub skill: Stat,
  /// Speed attribute.
  pub speed: Stat,
  /// Luck attribute.
  pub luck: Stat,
  /// Physical defense attribute.
  pub defense: Stat,
  /// Magical resistance attribute.
  pub resistance: Stat,
  /// Boots permanent movement bonus attribute stored at offset 0x15.
  boots_movement: u8,
  /// Weapon proficiency ranks (Sword, Lance, Axe, Bow, Tome, Staff).
  weapon_ranks: [u8; 6],
  /// Total battles fought by this unit.
  battles: u16,
  /// Total victories won by this unit.
  victories: u16,
  /// Personal inventory equipment slots (up to 5 items).
  inventory: [Option<item::Item>; 5],
  /// Index of the currently equipped weapon slot (0..5), or None if unarmed.
  equipped_slot: Option<usize>,
  /// Equipped skills (up to 5 skills).
  skills: [Skill; 5],
  /// Learned skill repertoire.
  repertoire: Vec<Skill>,
  /// Operational deployment status.
  status: UnitStatus,
  /// Byte offset of this unit entry within the TINU block.
  offset: usize,
  /// Active mid-map battle state, if deployed on a combat grid.
  battle_state: Option<BattleState>,
  /// Backing raw bytes for lossless roundtrip preservation.
  raw_bytes: Vec<u8>,
}

impl Unit {
  /// Builds a combat [`Stat`] with character additions and class bounds.
  fn build_stat(
    character: Character,
    class: Class,
    idx: usize,
    kind: StatKind,
    growth: u8,
  ) -> Stat {
    let raw_base = i16::from(class.base_stats()[idx])
      + i16::from(character.additions()[idx]);
    let base = u8::try_from(raw_base.clamp(0, 99)).unwrap_or(0);
    let cap = if idx == 0 {
      stat::UNIVERSAL_STAT_CEILING
    } else {
      let raw_cap = i16::from(class.max_stats()[idx])
        + i16::from(character.modifiers()[idx]);
      u8::try_from(raw_cap.clamp(0, 99)).unwrap_or(stat::UNIVERSAL_STAT_CEILING)
    };
    Stat::new(kind, base, cap, growth)
  }

  /// Constructs a new unit with default empty inventory and active status.
  #[must_use]
  pub(crate) fn new(character: Character, class: Class, offset: usize) -> Self {
    let gender = character.gender();
    Self {
      character,
      gender,
      class,
      level: 1,
      exp: 0,
      hp: Self::build_stat(character, class, 0, StatKind::Hp, 0),
      strength: Self::build_stat(character, class, 1, StatKind::Strength, 0),
      magic: Self::build_stat(character, class, 2, StatKind::Magic, 0),
      skill: Self::build_stat(character, class, 3, StatKind::Skill, 0),
      speed: Self::build_stat(character, class, 4, StatKind::Speed, 0),
      luck: Self::build_stat(character, class, 5, StatKind::Luck, 0),
      defense: Self::build_stat(character, class, 6, StatKind::Defense, 0),
      resistance: Self::build_stat(character, class, 7, StatKind::Resist, 0),
      boots_movement: 0,
      weapon_ranks: [0; 6],
      battles: 0,
      victories: 0,
      inventory: [None, None, None, None, None],
      equipped_slot: None,
      skills: [Skill::None; 5],
      repertoire: Vec::new(),
      status: UnitStatus::Active,
      offset,
      battle_state: None,
      raw_bytes: {
        let mut v = vec![0u8; 187];
        for slot in 0..5 {
          v[26 + slot * 5] = 0x04;
        }
        v
      },
    }
  }

  /// Parses an owned [`Unit`] from a TINU unit record byte slice.
  ///
  /// # Errors
  ///
  /// Returns [`Error::InvalidByteLength`] if `raw.len() < 67`.
  #[expect(
    clippy::too_many_lines,
    reason = "Comprehensive binary unit record parser matching Awakening TINU layout"
  )]
  pub(crate) fn from_bytes(
    raw: &[u8],
    offset: usize,
    mode: Option<config::GameMode>,
  ) -> Result<Self, Error> {
    if raw.len() < 67 {
      return Err(Error::InvalidByteLength {
        expected: 67,
        actual: raw.len(),
      });
    }
    let char_id = Character::from_raw(u16::from_le_bytes([raw[1], raw[2]]));
    let class_id = Class::from_raw(raw[3]);
    let mut growths = [0u8; 8];
    growths.copy_from_slice(&raw[10..18]);

    let level = raw[18].max(1);
    let exp = raw[19];
    let current_hp = raw[20];
    let boots_movement = raw[21];
    let coords1 = [raw[22], raw[23]];
    let coords2 = [raw[24], raw[25]];

    let mut inventory = [None, None, None, None, None];
    let mut equipped_slot = None;
    for (slot, inv_slot) in inventory.iter_mut().enumerate() {
      let off = 26 + slot * 5;
      if off + 5 <= raw.len() {
        let id = u16::from_le_bytes([raw[off + 1], raw[off + 2]]);
        if id != 0 && id != 0xFFFF {
          let raw_uses = u16::from_le_bytes([raw[off + 3], raw[off + 4]]);
          let uses = raw_uses & 0x0FFF;
          let item_type = item::ItemType::from_raw(id);
          if let Ok(it) = item::Item::with_durability(item_type, uses) {
            *inv_slot = Some(it);
          }
          if (raw_uses & 0x1000) != 0 {
            equipped_slot = Some(slot);
          }
        }
      }
    }

    let mut skills = [Skill::None; 5];
    for (slot, skill_slot) in skills.iter_mut().enumerate() {
      let off = 51 + slot * 2;
      if off + 2 <= raw.len() {
        *skill_slot =
          Skill::from_raw(u16::from_le_bytes([raw[off], raw[off + 1]]));
      }
    }

    let mut weapon_ranks = [0u8; 6];
    weapon_ranks.copy_from_slice(&raw[61..67]);

    let support_size = usize::from(raw[67]);
    let unk_idx = 67 + 1 + support_size;
    let unk_size = if unk_idx < raw.len() {
      usize::from(raw[unk_idx])
    } else {
      0
    };
    let flags_off = unk_idx + 1 + unk_size;
    let skill_off = flags_off + 0x2A;
    let end_off = skill_off + 0xD;

    let mut repertoire = Vec::new();
    if skill_off + 0xD <= raw.len() {
      let skill_bytes = &raw[skill_off..skill_off + 0xD];
      for sk_id in 1..=0x66 {
        let byte_idx = sk_id / 8;
        let bit_idx = sk_id % 8;
        if byte_idx < skill_bytes.len()
          && (skill_bytes[byte_idx] & (1 << bit_idx)) != 0
          && let Ok(sk_u16) = u16::try_from(sk_id)
        {
          repertoire.push(Skill::from_raw(sk_u16));
        }
      }
    }

    let battles = if end_off + 0x33 <= raw.len() {
      u16::from_le_bytes([raw[end_off + 0x31], raw[end_off + 0x32]])
    } else {
      0
    };
    let victories = if end_off + 0x35 <= raw.len() {
      u16::from_le_bytes([raw[end_off + 0x33], raw[end_off + 0x34]])
    } else {
      0
    };

    let status = if mode == Some(config::GameMode::Casual) {
      UnitStatus::Active
    } else if current_hp == 0 {
      if char_id.is_story_anchor() {
        UnitStatus::Retreated
      } else {
        UnitStatus::Dead
      }
    } else {
      UnitStatus::Active
    };

    let battle_state = if coords1 != [0, 0] || coords2 != [0, 0] {
      Some(BattleState::new(
        current_hp,
        boots_movement,
        coords1,
        coords2,
      ))
    } else {
      None
    };

    Ok(Self {
      character: char_id,
      gender: char_id.gender(),
      class: class_id,
      level,
      exp,
      hp: Self::build_stat(char_id, class_id, 0, StatKind::Hp, growths[0]),
      strength: Self::build_stat(
        char_id,
        class_id,
        1,
        StatKind::Strength,
        growths[1],
      ),
      magic: Self::build_stat(
        char_id,
        class_id,
        2,
        StatKind::Magic,
        growths[2],
      ),
      skill: Self::build_stat(
        char_id,
        class_id,
        3,
        StatKind::Skill,
        growths[3],
      ),
      speed: Self::build_stat(
        char_id,
        class_id,
        4,
        StatKind::Speed,
        growths[4],
      ),
      luck: Self::build_stat(char_id, class_id, 5, StatKind::Luck, growths[5]),
      defense: Self::build_stat(
        char_id,
        class_id,
        6,
        StatKind::Defense,
        growths[6],
      ),
      resistance: Self::build_stat(
        char_id,
        class_id,
        7,
        StatKind::Resist,
        growths[7],
      ),
      boots_movement,
      weapon_ranks,
      battles,
      victories,
      inventory,
      equipped_slot,
      skills,
      repertoire,
      status,
      offset,
      battle_state,
      raw_bytes: raw.to_vec(),
    })
  }

  /// Serializes modified unit fields back into a binary unit record.
  #[must_use]
  pub(crate) fn to_bytes(&self) -> Vec<u8> {
    let mut bytes = if self.raw_bytes.is_empty() {
      let mut v = vec![0u8; 187];
      for slot in 0..5 {
        v[26 + slot * 5] = 0x04;
      }
      v
    } else {
      self.raw_bytes.clone()
    };
    self.write_back(&mut bytes);
    bytes
  }

  /// Returns the byte offset of this unit entry within the TINU block.
  #[must_use]
  pub const fn offset(&self) -> usize {
    self.offset
  }

  /// Returns the canonical character identity.
  #[must_use]
  pub const fn character(&self) -> Character {
    self.character
  }

  /// Returns the concrete 16-bit identifier of this unit in the save file.
  #[must_use]
  pub(crate) const fn raw_id(&self) -> u16 {
    self.character.raw_id()
  }

  /// Returns the concrete 8-bit class identifier in the save file,
  /// resolved against the unit's biological gender.
  #[must_use]
  pub(crate) const fn class_id(&self) -> u8 {
    self.class.raw_id(self.gender)
  }

  /// Updates the unit's canonical character identity with strict validation.
  ///
  /// # Errors
  ///
  /// Returns [`Error::ClassIncompatible`] if the current class is ineligible
  /// for the new character's biological gender. Never silently auto-reclasses!
  pub fn set_character(&mut self, character: Character) -> Result<(), Error> {
    let gender = character.gender();
    if !self.class.is_eligible(gender) {
      return Err(Error::ClassIncompatible {
        character,
        class: self.class,
      });
    }
    self.character = character;
    self.gender = gender;
    Ok(())
  }

  /// Returns the canonical character display name.
  #[must_use]
  pub fn name(&self) -> &'static str {
    self.character.name()
  }

  /// Returns true if this unit is an essential story anchor.
  #[must_use]
  pub fn is_story_anchor(&self) -> bool {
    self.character.is_story_anchor()
  }

  /// Returns the current class.
  #[must_use]
  pub const fn class(&self) -> Class {
    self.class
  }

  /// Returns the biological gender of this unit.
  #[must_use]
  pub const fn gender(&self) -> Gender {
    self.gender
  }

  /// Sets the unit's biological gender with class compatibility checks.
  ///
  /// # Errors
  ///
  /// Returns [`Error::ClassGenderMismatch`] if the current class is incompatible
  /// with the requested gender.
  pub fn set_gender(&mut self, gender: Gender) -> Result<(), Error> {
    if !self.class.is_eligible(gender) {
      return Err(Error::ClassGenderMismatch {
        class: self.class,
        gender,
      });
    }
    self.gender = gender;
    if self.character.is_robin() {
      self.character = match gender {
        Gender::Male => Character::RobinM,
        Gender::Female => Character::RobinF,
      };
    } else if self.character.is_morgan() {
      self.character = match gender {
        Gender::Male => Character::MorganM,
        Gender::Female => Character::MorganF,
      };
    }
    Ok(())
  }

  /// Returns an iterator over references to the combat attributes (HP, Str, Mag, Skl, Spd, Lck, Def, Res).
  pub fn stats(&self) -> impl Iterator<Item = &Stat> {
    [
      &self.hp,
      &self.strength,
      &self.magic,
      &self.skill,
      &self.speed,
      &self.luck,
      &self.defense,
      &self.resistance,
    ]
    .into_iter()
  }

  /// Returns an iterator over mutable references to the combat attributes.
  pub(crate) fn stats_mut(&mut self) -> impl Iterator<Item = &mut Stat> {
    [
      &mut self.hp,
      &mut self.strength,
      &mut self.magic,
      &mut self.skill,
      &mut self.speed,
      &mut self.luck,
      &mut self.defense,
      &mut self.resistance,
    ]
    .into_iter()
  }

  /// Returns a reference to the combat stat corresponding to `kind`.
  #[must_use]
  pub const fn stat(&self, kind: StatKind) -> &Stat {
    match kind {
      StatKind::Hp => &self.hp,
      StatKind::Strength => &self.strength,
      StatKind::Magic => &self.magic,
      StatKind::Skill => &self.skill,
      StatKind::Speed => &self.speed,
      StatKind::Luck => &self.luck,
      StatKind::Defense => &self.defense,
      StatKind::Resist => &self.resistance,
    }
  }

  /// Returns the level-up stat growth bonuses.
  #[must_use]
  pub fn growths(&self) -> [u8; 8] {
    [
      self.hp.growth,
      self.strength.growth,
      self.magic.growth,
      self.skill.growth,
      self.speed.growth,
      self.luck.growth,
      self.defense.growth,
      self.resistance.growth,
    ]
  }

  /// Total movement tiles (class base + boots bonus).
  #[must_use]
  pub fn movement(&self) -> u8 {
    self
      .class
      .base_movement()
      .saturating_add(self.boots_movement)
  }

  /// Permanent movement bonus granted by Boots (stored at offset 0x15).
  #[must_use]
  pub const fn boots_movement(&self) -> u8 {
    self.boots_movement
  }

  /// Sets total movement tiles, adjusting the boots bonus accordingly.
  ///
  /// # Errors
  ///
  /// Returns [`Error::MovementOutOfBounds`] if `total < base` or `total > 99`.
  pub fn set_movement(&mut self, total_movement: u8) -> Result<(), Error> {
    let base = self.class.base_movement();
    if total_movement < base || total_movement > stat::UNIVERSAL_STAT_CEILING {
      return Err(Error::MovementOutOfBounds {
        value: total_movement,
        min: base,
        max: stat::UNIVERSAL_STAT_CEILING,
      });
    }
    let bonus = total_movement - base;
    if bonus > stat::MAX_SAFE_BOOTS_BONUS {
      return Err(Error::MovementOutOfBounds {
        value: total_movement,
        min: base,
        max: base.saturating_add(stat::MAX_SAFE_BOOTS_BONUS),
      });
    }
    self.boots_movement = bonus;
    Ok(())
  }

  /// Sets the boots movement bonus directly with safe ceiling bounds.
  ///
  /// # Errors
  ///
  /// Returns [`Error::BootsBonusOutOfBounds`] if `bonus > 91`.
  pub fn set_boots_movement(&mut self, bonus: u8) -> Result<(), Error> {
    if bonus > stat::MAX_SAFE_BOOTS_BONUS {
      return Err(Error::BootsBonusOutOfBounds {
        value: bonus,
        max: stat::MAX_SAFE_BOOTS_BONUS,
      });
    }
    self.boots_movement = bonus;
    Ok(())
  }

  /// Calculates the displayed combat rating.
  ///
  /// Rating = Str + Mag + Skl + Spd + Lck + Def + Res + Mov.
  #[must_use]
  pub fn rating(&self) -> u16 {
    u16::from(self.strength.total())
      + u16::from(self.magic.total())
      + u16::from(self.skill.total())
      + u16::from(self.speed.total())
      + u16::from(self.luck.total())
      + u16::from(self.defense.total())
      + u16::from(self.resistance.total())
      + u16::from(self.movement())
  }

  /// Returns the weapon proficiency ranks.
  #[must_use]
  pub const fn weapon_ranks(&self) -> &[u8; 6] {
    &self.weapon_ranks
  }

  /// Returns total battles fought by this unit.
  #[must_use]
  pub const fn battles(&self) -> u16 {
    self.battles
  }

  /// Sets total battles fought by this unit.
  pub fn set_battles(&mut self, battles: u16) {
    self.battles = battles;
  }

  /// Returns total victories won by this unit.
  #[must_use]
  pub const fn victories(&self) -> u16 {
    self.victories
  }

  /// Sets total victories won by this unit.
  pub fn set_victories(&mut self, victories: u16) {
    self.victories = victories;
  }

  /// Returns character experience level (1..=40).
  #[must_use]
  pub const fn level(&self) -> u8 {
    self.level
  }

  /// Sets experience level with bounds checking `[1..=40]`.
  ///
  /// # Errors
  ///
  /// Returns [`Error::LevelOutOfBounds`] if `level` is outside `[1, 40]`.
  pub fn set_level(&mut self, level: u8) -> Result<(), Error> {
    if level == 0 || level > 40 {
      return Err(Error::LevelOutOfBounds { level });
    }
    self.level = level;
    Ok(())
  }

  /// Returns current level experience progress (0..=99).
  #[must_use]
  pub const fn exp(&self) -> u8 {
    self.exp
  }

  /// Sets level experience progress with bounds checking `[0..=99]`.
  ///
  /// # Errors
  ///
  /// Returns [`Error::ExpOutOfBounds`] if `exp > 99`.
  pub fn set_exp(&mut self, exp: u8) -> Result<(), Error> {
    if exp > 99 {
      return Err(Error::ExpOutOfBounds { exp });
    }
    self.exp = exp;
    Ok(())
  }

  /// Returns equipped skills.
  #[must_use]
  pub const fn skills(&self) -> [Skill; 5] {
    self.skills
  }

  /// Sets the skill at the specified slot index (0..5).
  ///
  /// Also ensures the assigned skill is added to the learned skill repertoire.
  ///
  /// # Errors
  ///
  /// Returns [`Error::SkillSlotOutOfBounds`] if `slot >= 5`.
  pub fn set_skill(&mut self, slot: usize, skill: Skill) -> Result<(), Error> {
    if slot >= 5 {
      return Err(Error::SkillSlotOutOfBounds { slot });
    }
    self.skills[slot] = skill;
    if skill != Skill::None && !self.repertoire.contains(&skill) {
      self.repertoire.push(skill);
    }
    Ok(())
  }

  /// Returns the learned skill repertoire.
  #[must_use]
  pub fn repertoire(&self) -> &[Skill] {
    &self.repertoire
  }

  /// Adds a skill directly to the unit's learned skill repertoire.
  pub fn learn_skill(&mut self, skill: Skill) {
    if skill != Skill::None && !self.repertoire.contains(&skill) {
      self.repertoire.push(skill);
    }
  }

  /// Removes a skill from the learned repertoire and unequips it if active.
  pub fn unlearn_skill(&mut self, skill: Skill) {
    self.repertoire.retain(|&s| s != skill);
    for slot in &mut self.skills {
      if *slot == skill {
        *slot = Skill::None;
      }
    }
  }

  /// Returns true if the skill is currently learned in the repertoire.
  #[must_use]
  pub fn has_learned(&self, skill: Skill) -> bool {
    self.repertoire.contains(&skill)
  }

  /// Changes the unit's class with biological gender eligibility enforcement.
  ///
  /// Automatically unequips the currently active weapon if the new class
  /// is not proficient in using it, preventing battle animation crashes.
  ///
  /// # Errors
  ///
  /// Returns [`Error::ClassGenderMismatch`] if the unit's biological
  /// gender cannot reclass to the requested class.
  pub fn set_class(&mut self, class: Class) -> Result<(), Error> {
    if !class.is_eligible(self.gender) {
      return Err(Error::ClassGenderMismatch {
        class,
        gender: self.gender,
      });
    }
    self.class = class;
    if let Some(eq_slot) = self.equipped_slot
      && let Some(item) = &self.inventory[eq_slot]
      && !class.can_use(item.item_type().kind())
    {
      self.equipped_slot = None;
    }
    let additions = self.character.additions();
    let modifiers = self.character.modifiers();
    for (idx, stat) in self.stats_mut().enumerate() {
      let raw_base =
        i16::from(class.base_stats()[idx]) + i16::from(additions[idx]);
      let new_base = u8::try_from(raw_base.clamp(0, 99)).unwrap_or(0);
      let new_cap = if idx == 0 {
        stat::UNIVERSAL_STAT_CEILING
      } else {
        let raw_cap =
          i16::from(class.max_stats()[idx]) + i16::from(modifiers[idx]);
        u8::try_from(raw_cap.clamp(0, 99))
          .unwrap_or(stat::UNIVERSAL_STAT_CEILING)
      };
      stat.clamp_on_reclass(new_base, new_cap);
    }
    Ok(())
  }

  /// Returns a borrowed slice of the 5 inventory item slots.
  #[must_use]
  pub const fn inventory(&self) -> &[Option<item::Item>; 5] {
    &self.inventory
  }

  /// Returns a reference to the item at index `slot` (0..5).
  #[must_use]
  pub fn item(&self, slot: usize) -> Option<&item::Item> {
    if slot < 5 {
      self.inventory[slot].as_ref()
    } else {
      None
    }
  }

  /// Returns the index of the currently equipped inventory slot, if any.
  #[must_use]
  pub const fn equipped_slot(&self) -> Option<usize> {
    self.equipped_slot
  }

  /// Returns true if the specified slot is the currently equipped weapon.
  #[must_use]
  pub fn is_slot_equipped(&self, slot: usize) -> bool {
    self.equipped_slot == Some(slot)
  }

  /// Equips the weapon in the specified inventory slot (0..5).
  ///
  /// Guarantees that at most one item is equipped at any time.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::SlotOutOfBounds`] if `slot >= 5`.
  /// - Returns [`Error::EmptySlot`] if the slot has no item.
  /// - Returns [`Error::CannotEquipNonWeapon`] if the item is not equippable.
  /// - Returns [`Error::ClassCannotEquip`] if the unit's class cannot use it.
  pub fn equip(&mut self, slot: usize) -> Result<(), Error> {
    if slot >= 5 {
      return Err(Error::SlotOutOfBounds { slot });
    }
    let Some(it) = &self.inventory[slot] else {
      return Err(Error::EmptySlot { slot });
    };
    if !it.item_type().is_equippable() {
      return Err(Error::CannotEquipNonWeapon {
        item_type: it.item_type(),
      });
    }
    if !self.class.can_use(it.item_type().kind()) {
      return Err(Error::ClassCannotEquip {
        class: self.class,
        kind: it.item_type().kind(),
      });
    }
    self.equipped_slot = Some(slot);
    Ok(())
  }

  /// Unequips the currently equipped weapon.
  pub fn unequip(&mut self) {
    self.equipped_slot = None;
  }

  /// Sets an item in a specific inventory slot (0..5).
  ///
  /// # Errors
  ///
  /// Returns [`Error::SlotOutOfBounds`] if `slot >= 5`.
  pub fn set_item(
    &mut self,
    slot: usize,
    item: item::Item,
  ) -> Result<(), Error> {
    if slot >= 5 {
      return Err(Error::SlotOutOfBounds { slot });
    }
    let item_type = item.item_type();
    self.inventory[slot] = Some(item);
    if self.equipped_slot == Some(slot)
      && (!item_type.is_equippable() || !self.class.can_use(item_type.kind()))
    {
      self.equipped_slot = None;
    }
    Ok(())
  }

  /// Clears an item slot (0..5), unequipping it if currently active.
  ///
  /// # Errors
  ///
  /// Returns [`Error::SlotOutOfBounds`] if `slot >= 5`.
  pub fn clear_item(&mut self, slot: usize) -> Result<(), Error> {
    if slot >= 5 {
      return Err(Error::SlotOutOfBounds { slot });
    }
    self.inventory[slot] = None;
    if self.equipped_slot == Some(slot) {
      self.equipped_slot = None;
    }
    Ok(())
  }

  /// Sets a specific level-up stat growth bonus.
  ///
  /// # Errors
  ///
  /// Returns [`Error::GrowthOutOfBounds`] if `stat_index >= 8`.
  pub fn set_growth(
    &mut self,
    stat_index: usize,
    value: u8,
  ) -> Result<(), Error> {
    match stat_index {
      0 => self.hp.set_growth(value).map_err(Into::into),
      1 => self.strength.set_growth(value).map_err(Into::into),
      2 => self.magic.set_growth(value).map_err(Into::into),
      3 => self.skill.set_growth(value).map_err(Into::into),
      4 => self.speed.set_growth(value).map_err(Into::into),
      5 => self.luck.set_growth(value).map_err(Into::into),
      6 => self.defense.set_growth(value).map_err(Into::into),
      7 => self.resistance.set_growth(value).map_err(Into::into),
      _ => Err(Error::GrowthOutOfBounds {
        index: stat_index,
        value,
      }),
    }
  }

  /// Sets all level-up growth bonuses to maximum (99 - base).
  pub fn max_growths(&mut self) {
    for stat in self.stats_mut() {
      let _ = stat.set_total(stat::UNIVERSAL_STAT_CEILING);
    }
  }

  /// Sets a specific weapon proficiency rank.
  ///
  /// # Errors
  ///
  /// Returns [`Error::WeaponRankOutOfBounds`] if `rank_index >= 6`.
  pub fn set_weapon_rank(
    &mut self,
    rank_index: usize,
    value: u8,
  ) -> Result<(), Error> {
    if rank_index >= 6 {
      return Err(Error::WeaponRankOutOfBounds {
        index: rank_index,
        value,
      });
    }
    self.weapon_ranks[rank_index] = value;
    Ok(())
  }

  /// Sets all weapon proficiency ranks to S rank (90 `WExp`).
  pub fn max_weapon_ranks(&mut self) {
    for i in 0..6 {
      let _ = self.set_weapon_rank(i, 90);
    }
  }

  /// Returns current operational combat status.
  #[must_use]
  pub const fn status(&self) -> UnitStatus {
    self.status
  }

  /// Returns high-level deployment availability status.
  #[must_use]
  pub const fn availability(&self) -> Availability {
    match self.status {
      UnitStatus::Active => Availability::Active,
      UnitStatus::Retreated | UnitStatus::Dead => Availability::Inactive,
    }
  }

  /// Returns true if unit is active and eligible for combat deployment.
  #[must_use]
  pub const fn is_active(&self) -> bool {
    match self.status {
      UnitStatus::Active => true,
      UnitStatus::Retreated | UnitStatus::Dead => false,
    }
  }

  /// Sets operational combat status.
  ///
  /// # Errors
  ///
  /// Returns [`Error::AnchorUnitProtected`] if attempting to set a story
  /// anchor unit to [`UnitStatus::Dead`].
  pub fn set_status(&mut self, status: UnitStatus) -> Result<(), Error> {
    if self.is_story_anchor() && status == UnitStatus::Dead {
      return Err(Error::AnchorUnitProtected {
        character: self.character,
      });
    }
    self.status = status;
    if status == UnitStatus::Dead {
      self.hp.growth = 0;
      self.strength.growth = 0;
      self.magic.growth = 0;
      self.skill.growth = 0;
      self.speed.growth = 0;
      self.luck.growth = 0;
      self.defense.growth = 0;
      self.resistance.growth = 0;
    }
    Ok(())
  }

  /// Revives a dead or retreated unit to active status with restored hit points.
  pub fn revive(&mut self) {
    self.status = UnitStatus::Active;
  }

  /// Returns a reference to the active battle state, if deployed on a battle map.
  #[must_use]
  pub const fn battle_state(&self) -> Option<&BattleState> {
    self.battle_state.as_ref()
  }

  /// Returns a mutable reference to the active battle state, if deployed on a battle map.
  pub(crate) fn battle_state_mut(&mut self) -> Option<&mut BattleState> {
    self.battle_state.as_mut()
  }

  /// Sets or clears the active battle state.
  pub fn set_battle_state(&mut self, state: Option<BattleState>) {
    self.battle_state = state;
  }

  /// Returns current live hit points.
  #[must_use]
  pub const fn current_hp(&self) -> u8 {
    if let Some(bs) = &self.battle_state {
      bs.current_hp
    } else {
      self.hp.total()
    }
  }

  /// Sets current live hit points.
  ///
  /// # Errors
  ///
  /// Returns [`Error::Stat`] if `hp > 99`.
  pub fn set_current_hp(&mut self, hp: u8) -> Result<(), Error> {
    if hp > stat::UNIVERSAL_STAT_CEILING {
      return Err(
        stat::Error::StatOutOfBounds {
          stat: StatKind::Hp,
          value: hp,
          min: 0,
          max: stat::UNIVERSAL_STAT_CEILING,
        }
        .into(),
      );
    }
    if let Some(bs) = &mut self.battle_state {
      bs.current_hp = hp;
    }
    Ok(())
  }

  /// Returns battle map coordinates `[x, y]` if deployed.
  #[must_use]
  pub const fn coordinates(&self) -> Option<[u8; 2]> {
    if let Some(bs) = &self.battle_state {
      Some(bs.coordinates)
    } else {
      None
    }
  }

  /// Sets battle map coordinates `[x, y]` with bounds check `[0..=32]`.
  ///
  /// # Errors
  ///
  /// Returns [`Error::CoordinatesOutOfBounds`] if coordinates exceed 32.
  pub fn set_coordinates(&mut self, x: u8, y: u8) -> Result<(), Error> {
    if x > 32 || y > 32 {
      return Err(Error::CoordinatesOutOfBounds { x, y });
    }
    if let Some(bs) = &mut self.battle_state {
      bs.coordinates = [x, y];
    }
    Ok(())
  }

  /// Returns pair-up partner coordinates `[x, y]` if deployed.
  #[must_use]
  pub const fn pair_up_coordinates(&self) -> Option<[u8; 2]> {
    if let Some(bs) = &self.battle_state {
      Some(bs.pair_up_coordinates)
    } else {
      None
    }
  }

  /// Sets pair-up partner coordinates `[x, y]` with bounds check `[0..=32]`.
  ///
  /// # Errors
  ///
  /// Returns [`Error::CoordinatesOutOfBounds`] if coordinates exceed 32.
  pub fn set_pair_up_coordinates(&mut self, x: u8, y: u8) -> Result<(), Error> {
    if x > 32 || y > 32 {
      return Err(Error::CoordinatesOutOfBounds { x, y });
    }
    if let Some(bs) = &mut self.battle_state {
      bs.pair_up_coordinates = [x, y];
    }
    Ok(())
  }

  /// Writes modified unit fields back into a raw unit slice.
  #[expect(
    clippy::too_many_lines,
    reason = "Comprehensive binary encoder supporting canonical and legacy layouts"
  )]
  pub(crate) fn write_back(&self, slice: &mut [u8]) {
    if slice.len() < 26 {
      return;
    }
    let is_canonical = slice.len() > 26 && slice[26] == 0x04;
    let base = usize::from(is_canonical);

    let pid = self.raw_id();
    slice[base..base + 2].copy_from_slice(&pid.to_le_bytes());
    if base + 2 < slice.len() {
      slice[base + 2] = self.class_id();
    }
    let growths = self.growths();
    let g_start = if is_canonical { 10 } else { 9 };
    for (i, &g) in growths.iter().enumerate() {
      if g_start + i < slice.len() {
        slice[g_start + i] = g;
      }
    }
    let lvl_off = if is_canonical { 18 } else { 17 };
    if lvl_off + 2 <= slice.len() {
      slice[lvl_off] = self.level;
      slice[lvl_off + 1] = self.exp;
    }
    let hp_off = if is_canonical { 20 } else { 19 };
    if hp_off < slice.len() {
      slice[hp_off] = if let Some(bs) = &self.battle_state {
        bs.current_hp
      } else {
        self.hp.total()
      };
    }
    let boots_off = 21;
    if boots_off < slice.len() {
      slice[boots_off] = if let Some(bs) = &self.battle_state {
        bs.boots_movement
      } else {
        self.boots_movement
      };
    }
    if is_canonical
      && slice.len() >= 26
      && let Some(bs) = &self.battle_state
    {
      slice[22..24].copy_from_slice(&bs.coordinates);
      slice[24..26].copy_from_slice(&bs.pair_up_coordinates);
    }

    let inv_start = if is_canonical { 26 } else { 25 };
    for (i, slot) in self.inventory.iter().enumerate() {
      let off = inv_start + i * 5;
      if off + 5 <= slice.len() {
        if let Some(it) = slot {
          slice[off] = 0x04;
          slice[off + 1..off + 3]
            .copy_from_slice(&it.item_type().raw_id().to_le_bytes());
          let equipped_flag = if self.equipped_slot == Some(i) {
            0x1000
          } else {
            0
          };
          let raw_uses = (it.durability() & 0x0fff) | equipped_flag;
          slice[off + 3..off + 5].copy_from_slice(&raw_uses.to_le_bytes());
        } else {
          slice[off] = 0x04;
          slice[off + 1..off + 5].fill(0);
        }
      }
    }

    let skl_start = if is_canonical { 51 } else { 50 };
    for (i, slot) in self.skills.iter().enumerate() {
      let off = skl_start + i * 2;
      if off + 2 <= slice.len() {
        slice[off..off + 2].copy_from_slice(&slot.raw_id().to_le_bytes());
      }
    }

    let wr_start = if is_canonical { 61 } else { 60 };
    if wr_start + 6 <= slice.len() {
      slice[wr_start..wr_start + 6].copy_from_slice(&self.weapon_ranks);
    }

    if is_canonical && slice.len() > 67 {
      let support_size = slice[67] as usize;
      let unk_idx = 67 + 1 + support_size;
      let unk_size = if unk_idx < slice.len() {
        slice[unk_idx] as usize
      } else {
        0
      };
      let skill_off = unk_idx + 1 + unk_size + 0x2A;
      let end_off = skill_off + 0xD;
      if end_off + 0x35 <= slice.len() {
        slice[end_off + 0x31..end_off + 0x33]
          .copy_from_slice(&self.battles.to_le_bytes());
        slice[end_off + 0x33..end_off + 0x35]
          .copy_from_slice(&self.victories.to_le_bytes());
      }
      if skill_off + 0xD <= slice.len() {
        slice[skill_off..skill_off + 0xD].fill(0);
        for &sk in &self.repertoire {
          let id = usize::from(sk.raw_id());
          if id > 0 && id <= 0x66 {
            let byte_idx = skill_off + (id / 8);
            if byte_idx < slice.len() {
              slice[byte_idx] |= 1 << (id % 8);
            }
          }
        }
      }
    } else if slice.len() >= 70 {
      slice[66..68].copy_from_slice(&self.battles.to_le_bytes());
      slice[68..70].copy_from_slice(&self.victories.to_le_bytes());
      for &sk in &self.repertoire {
        let id = usize::from(sk.raw_id());
        if id > 0 {
          let byte_idx = 70 + (id / 8);
          if byte_idx < slice.len() {
            slice[byte_idx] |= 1 << (id % 8);
          }
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_unit_equipment_and_single_equipped_invariant() {
    let mut unit = Unit::new(Character::Chrom, Class::Lord, 0);

    // Lord can use swords, not axes
    assert!(
      unit
        .set_item(0, item::Item::new(item::ItemType::Falchion))
        .is_ok()
    );
    assert!(
      unit
        .set_item(
          1,
          item::Item::with_durability(item::ItemType::SteelAxe, 35)
            .expect("steel axe")
        )
        .is_ok()
    );
    assert!(
      unit
        .set_item(
          2,
          item::Item::with_durability(item::ItemType::Vulnerary, 3)
            .expect("vulnerary")
        )
        .is_ok()
    );

    // Equip Falchion
    assert!(unit.equip(0).is_ok());
    assert_eq!(unit.equipped_slot(), Some(0));
    assert!(unit.is_slot_equipped(0));

    // Equipping steel axe on Lord fails
    assert!(unit.equip(1).is_err());
    // Falchion remains equipped
    assert_eq!(unit.equipped_slot(), Some(0));

    // Equipping non-weapon fails
    assert!(unit.equip(2).is_err());

    // Unequip
    unit.unequip();
    assert_eq!(unit.equipped_slot(), None);
  }

  #[test]
  fn test_class_change_unequips_incompatible_weapon() {
    let mut unit = Unit::new(Character::Chrom, Class::Lord, 0);
    assert!(
      unit
        .set_item(0, item::Item::new(item::ItemType::Falchion))
        .is_ok()
    );
    assert!(unit.equip(0).is_ok());
    assert_eq!(unit.equipped_slot(), Some(0));

    // Reclass to Archer (which cannot use Swords)
    assert!(unit.set_class(Class::Archer).is_ok());
    assert_eq!(unit.equipped_slot(), None);
  }

  #[test]
  fn test_class_gender_restrictions_and_resolution() {
    let mut chrom = Unit::new(Character::Chrom, Class::Lord, 0);
    // Chrom cannot become a Bride or Pegasus Knight
    assert!(chrom.set_class(Class::Bride).is_err());
    assert!(chrom.set_class(Class::PegasusKnight).is_err());
    assert!(chrom.set_class(Class::DarkFlier).is_err());

    // Chrom can become a Dread Fighter or Berserker
    assert!(chrom.set_class(Class::DreadFighter).is_ok());
    assert!(chrom.set_class(Class::Berserker).is_ok());

    // Reverting to Lord gives ID 0 (Lord M)
    assert!(chrom.set_class(Class::Lord).is_ok());
    let mut slice = [0u8; 32];
    chrom.write_back(&mut slice);
    assert_eq!(slice[2], 0);

    // Lucina
    let mut lucina = Unit::new(Character::Lucina, Class::Lord, 0);
    // Lucina as Lord gives ID 1 (Lord F)
    let mut lucina_slice = [0u8; 32];
    lucina.write_back(&mut lucina_slice);
    assert_eq!(lucina_slice[2], 1);

    // Lucina can become a Bride or Dark Flier, but not a Berserker
    assert!(lucina.set_class(Class::Bride).is_ok());
    assert!(lucina.set_class(Class::DarkFlier).is_ok());
    assert!(lucina.set_class(Class::Berserker).is_err());
  }

  #[test]
  fn test_unit_availability_abstractions() {
    let mut cordelia = Unit::new(Character::Cordelia, Class::DarkFlier, 0);
    assert_eq!(cordelia.availability(), Availability::Active);
    assert!(cordelia.is_active());

    assert!(cordelia.set_status(UnitStatus::Dead).is_ok());
    assert_eq!(cordelia.availability(), Availability::Inactive);
    assert!(!cordelia.is_active());

    cordelia.revive();
    assert_eq!(cordelia.availability(), Availability::Active);
    assert!(cordelia.is_active());
  }

  #[test]
  fn test_story_anchor_unit_protection() {
    let mut chrom = Unit::new(Character::Chrom, Class::Lord, 0);
    assert!(chrom.is_story_anchor());
    assert!(chrom.set_status(UnitStatus::Dead).is_err());
    assert!(chrom.set_status(UnitStatus::Retreated).is_ok());
    assert_eq!(chrom.status(), UnitStatus::Retreated);

    let mut donnel = Unit::new(Character::Donnel, Class::Villager, 180);
    assert!(!donnel.is_story_anchor());
    assert!(donnel.set_status(UnitStatus::Dead).is_ok());
    assert_eq!(donnel.status(), UnitStatus::Dead);
    assert_eq!(donnel.hp.growth, 0);

    donnel.revive();
    assert_eq!(donnel.status(), UnitStatus::Active);
  }

  #[test]
  fn test_unit_skills_and_raw_id() {
    let mut robin = Unit::new(Character::RobinM, Class::Tactician, 0);
    assert_eq!(robin.raw_id(), 0);
    assert!(robin.set_gender(Gender::Female).is_ok());
    assert_eq!(robin.raw_id(), 1);

    assert_eq!(robin.skills()[0], Skill::None);
    assert!(robin.set_skill(0, Skill::Veteran).is_ok());
    assert!(robin.set_skill(1, Skill::Solidarity).is_ok());
    assert!(robin.set_skill(2, Skill::Ignis).is_ok());
    assert!(robin.set_skill(3, Skill::RallySpectrum).is_ok());
    assert!(robin.set_skill(4, Skill::LimitBreaker).is_ok());
    assert_eq!(robin.skills()[0], Skill::Veteran);
    assert_eq!(robin.skills()[4], Skill::LimitBreaker);
    assert!(robin.has_learned(Skill::Veteran));
    assert!(robin.has_learned(Skill::LimitBreaker));
    assert_eq!(robin.repertoire().len(), 5);

    robin.learn_skill(Skill::Aether);
    assert!(robin.has_learned(Skill::Aether));
    assert_eq!(robin.repertoire().len(), 6);

    robin.unlearn_skill(Skill::Veteran);
    assert!(!robin.has_learned(Skill::Veteran));
    assert_eq!(robin.skills()[0], Skill::None);
    assert_eq!(robin.repertoire().len(), 5);

    let mut slice = [0u8; 80];
    robin.write_back(&mut slice);
    assert_eq!(
      u16::from_le_bytes([slice[50], slice[51]]),
      Skill::None.raw_id()
    );
    assert_eq!(
      u16::from_le_bytes([slice[58], slice[59]]),
      Skill::LimitBreaker.raw_id()
    );
  }

  #[test]
  fn test_unit_stat_mutations_and_battles() {
    let mut chrom = Unit::new(Character::Chrom, Class::Lord, 0);
    assert!(chrom.hp.set_total(99).is_ok());
    assert_eq!(chrom.hp.total(), 99);
    assert!(chrom.hp.set_total(0).is_err());
    assert!(chrom.strength.set_total(50).is_ok());
    assert_eq!(chrom.strength.total(), 50);
    assert!(chrom.strength.set_total(100).is_err());

    chrom.set_battles(42);
    assert_eq!(chrom.battles(), 42);
    chrom.set_victories(30);
    assert_eq!(chrom.victories(), 30);

    // Direct StatKind query & mutation
    assert_eq!(chrom.stat(StatKind::Hp).total(), 99);
    assert!(chrom.strength.set_total(45).is_ok());
    assert_eq!(chrom.stat(StatKind::Strength).total(), 45);
    let totals: Vec<u8> = chrom.stats().map(Stat::total).collect();
    assert_eq!(totals[0], 99); // HP
    assert_eq!(totals[1], 45); // Str

    let mut slice = [0u8; 80];
    chrom.write_back(&mut slice);
    assert_eq!(u16::from_le_bytes([slice[66], slice[67]]), 42);
    assert_eq!(u16::from_le_bytes([slice[68], slice[69]]), 30);
  }

  #[test]
  fn test_unit_movement_and_boots() {
    let mut paladin = Unit::new(Character::Chrom, Class::Paladin, 0);
    assert_eq!(paladin.movement(), 8);
    assert_eq!(paladin.boots_movement(), 0);

    // Increase total movement to 12 (4 boots points)
    assert!(paladin.set_movement(12).is_ok());
    assert_eq!(paladin.boots_movement(), 4);
    assert_eq!(paladin.movement(), 12);

    // Boots bonus up to max safe 91
    assert!(paladin.set_boots_movement(91).is_ok());
    assert_eq!(paladin.movement(), 99);

    // Beyond max safe bonus 91 fails
    assert!(paladin.set_boots_movement(92).is_err());

    // Reclassing to Tactician (base 5) preserves bonus: 5 + 91 = 96 <= 99
    assert!(paladin.set_class(Class::Tactician).is_ok());
    assert_eq!(paladin.movement(), 96);
  }
}
