//! Error types for Fire Emblem: Awakening save domain operations.
//!
//! Owns the crate-wide [`enum@Error`] type.

use fe_compress;
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
  /// Character was not found in the roster.
  #[error("character '{name}' was not found in the roster")]
  CharacterNotFound {
    /// Character name searched for.
    name: String,
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
  Compress(#[from] fe_compress::Error),
}
