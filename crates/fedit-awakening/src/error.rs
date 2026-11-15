//! Error types for Fire Emblem: Awakening save domain operations.
//!
//! Owns the crate-wide [`enum@Error`] type.

use fedit_compress;
use thiserror;

use crate::convoy::item;
use crate::roster::unit::character;
use crate::roster::unit::class;

/// Errors that can occur when parsing or mutating FE13 saves.
#[derive(Debug, thiserror::Error)]
pub enum Error {
  /// Preview buffer length was not 192 bytes.
  #[error("invalid preview length: expected {expected}, got {actual}")]
  InvalidPreviewLength {
    /// Expected byte length (192).
    expected: usize,
    /// Actual byte length.
    actual: usize,
  },
  /// EDNI master table magic did not match `b"EDNI"`.
  #[error("invalid EDNI master index magic")]
  InvalidEdniMagic,
  /// Target block magic tag did not match expected ASCII bytes.
  #[error("invalid block magic: expected {expected:?}, got {actual:?}")]
  InvalidBlockMagic {
    /// Expected 4-byte ASCII magic tag.
    expected: [u8; 4],
    /// Actual 4-byte tag found at offset.
    actual: [u8; 4],
  },
  /// Block offset was out of bounds for the decompressed payload.
  #[error("offset {offset} is out of bounds for payload length {len}")]
  OffsetOutOfBounds {
    /// Resolved offset.
    offset: usize,
    /// Payload length.
    len: usize,
  },
  /// Fixed-width UTF-16LE string decoding failed.
  #[error("failed to decode UTF-16LE string")]
  Utf16Decode,
  /// Combat or character stat value was out of bounds.
  #[error("stat '{stat}' value {value} is out of bounds [{min}..={max}]")]
  StatOutOfBounds {
    /// Stat name.
    stat: &'static str,
    /// Provided value.
    value: u8,
    /// Minimum allowed.
    min: u8,
    /// Maximum allowed.
    max: u8,
  },
  /// Gold value exceeded maximum allowable amount.
  #[error("gold amount {value} exceeds maximum {max}")]
  GoldOutOfBounds {
    /// Provided value.
    value: u32,
    /// Maximum allowed.
    max: u32,
  },
  /// Character is not a canonical child unit with variable parentage.
  #[error("character '{child}' does not have variable parentage")]
  NotAChildCharacter {
    /// Non-child character.
    child: character::Character,
  },
  /// Target inventory slot index was out of bounds (0..5).
  #[error("inventory slot {slot} is out of bounds [0..5]")]
  SlotOutOfBounds {
    /// Out of bounds slot index.
    slot: usize,
  },
  /// Target inventory slot is empty.
  #[error("inventory slot {slot} is empty")]
  EmptySlot {
    /// Empty slot index.
    slot: usize,
  },
  /// Item cannot be equipped because it is not a weapon or staff.
  #[error("cannot equip non-weapon item '{item}'")]
  CannotEquipNonWeapon {
    /// Item that cannot be equipped.
    item: item::Item,
  },
  /// Class does not possess weapon proficiency for the target item.
  #[error("class '{class}' cannot equip weapon '{item}'")]
  ClassCannotEquip {
    /// Incompatible class.
    class: class::Class,
    /// Weapon attempted.
    item: item::Item,
  },
  /// Characters cannot support each other.
  #[error("characters '{a}' and '{b}' cannot support each other")]
  IncompatibleSupportPair {
    /// First character.
    a: character::Character,
    /// Second character.
    b: character::Character,
  },
  /// Character is already married to another partner.
  #[error("character '{character}' is already married to '{spouse}'")]
  AlreadyMarried {
    /// Target character.
    character: character::Character,
    /// Existing spouse.
    spouse: character::Character,
  },
  /// Critical story anchor character cannot be removed or killed.
  #[error("story anchor unit '{character}' cannot be removed")]
  AnchorUnitProtected {
    /// Protected character.
    character: character::Character,
  },
  /// Permanent death is not permitted in Casual game mode.
  #[error("cannot kill or mark unit dead in casual mode")]
  CasualModePermadeathForbidden,
  /// Target class is not permitted for the character's biological gender.
  #[error("character '{character}' ({gender}) cannot reclass to '{class}'")]
  GenderIncompatibleClass {
    /// Target character.
    character: character::Character,
    /// Incompatible class.
    class: class::Class,
    /// Biological gender of the character.
    gender: character::Gender,
  },
  /// Cannot revert to or before recruitment chapter while unit is dead.
  #[error(
    "cannot revert to chapter {target_chapter}: '{character}' recruited in chapter {recruitment_chapter} is dead"
  )]
  DeadUnitRecruitmentConflict {
    /// Dead character.
    character: character::Character,
    /// Canonical recruitment chapter.
    recruitment_chapter: u8,
    /// Target rollback chapter.
    target_chapter: u16,
  },
  /// Story chapter identifier was outside valid bounds [0..=26].
  #[error("chapter {chapter} is out of bounds [0..=26]")]
  ChapterOutOfBounds {
    /// Provided chapter index.
    chapter: u16,
  },
  /// Child paralogues cannot be unlocked before Chapter 13 is cleared.
  #[error("child paralogue {paralogue} requires chapter 13 to be cleared")]
  ChildParalogueGatedByChapter13 {
    /// Target paralogue index.
    paralogue: u8,
  },
  /// Child unit parents are not married (support rank S not achieved).
  #[error(
    "cannot unlock child paralogue: parents of '{child}' are not married"
  )]
  ParentsNotMarried {
    /// Child character.
    child: character::Character,
  },
  /// Cannot relock or reset completed paralogue while child is in roster.
  #[error(
    "cannot rollback paralogue {paralogue}: child unit '{child}' is already in roster"
  )]
  ChildInRosterParalogueConflict {
    /// Recruited child character.
    child: character::Character,
    /// Target paralogue index.
    paralogue: u8,
  },
  /// Avatar `LogBlock` data was not found in the save payload.
  #[error("avatar customization block not found in save payload")]
  AvatarNotFound,
  /// Asset and flaw cannot be identical.
  #[error("avatar asset and flaw cannot be the same stat")]
  StatFocusConflict,
  /// Durability value exceeded maximum allowable bound.
  #[error("durability {uses} exceeds safe limit {max}")]
  DurabilityOverflow {
    /// Provided uses.
    uses: u16,
    /// Maximum allowed.
    max: u16,
  },
  /// Forge stat modification exceeded allowable limits.
  #[error("forge stat '{stat}' value {value} is out of bounds [{min}..={max}]")]
  ForgeOutOfBounds {
    /// Forge stat name.
    stat: &'static str,
    /// Provided value.
    value: i8,
    /// Minimum allowed.
    min: i8,
    /// Maximum allowed.
    max: i8,
  },
  /// Lower-level container or compression error.
  #[error(transparent)]
  Compress(#[from] fedit_compress::Error),
}
