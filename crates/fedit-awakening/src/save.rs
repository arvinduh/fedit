//! Save campaign structures, unit roster, convoy storage, and support tracking.
//!
//! Provides root abstractions for deserializing, modifying, and serializing
//! save campaign archives and nested subsystems.

use fedit_compress;
use thiserror;

pub mod avatar;
mod bookmark;
mod campaign;
pub mod convoy;
mod forge;
pub mod header;
pub mod roster;
pub mod social;
pub(crate) mod utils;

pub use avatar::AvatarCustomization;
pub use header::Difficulty;
pub use header::GameMode;
pub use roster::Roster;
pub use social::ParalogueState;
pub use social::SupportRank;
pub use social::SupportTable;
pub use utils::time::PlayTime;

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
  /// Campaign gold amount exceeded maximum allowable limit.
  #[error("gold amount {gold} exceeds maximum {max}")]
  GoldOutOfBounds {
    /// Provided gold amount.
    gold: u32,
    /// Maximum allowed gold amount.
    max: u32,
  },
  /// Permanent death is not permitted in Casual game mode.
  #[error("cannot kill or mark unit dead in casual mode")]
  CasualModePermadeathForbidden,
  /// Story chapter identifier was outside valid bounds [0..=26].
  #[error("chapter {chapter} is out of bounds [0..=26]")]
  ChapterOutOfBounds {
    /// Provided chapter index.
    chapter: u16,
  },
  /// Cannot revert to or before recruitment chapter while unit is dead.
  #[error(
    "cannot revert to chapter {target_chapter}: '{character}' recruited in chapter {recruitment_chapter} is dead"
  )]
  DeadUnitRecruitmentConflict {
    /// Dead character.
    character: roster::unit::Character,
    /// Canonical recruitment chapter.
    recruitment_chapter: u8,
    /// Target rollback chapter.
    target_chapter: u16,
  },
  /// Avatar `LogBlock` data was not found in the save payload.
  #[error("avatar customization block not found in save payload")]
  AvatarNotFound,
  /// Operation is only valid for bookmark (in-battle) saves.
  #[error("operation requires a bookmark (mid-battle) save")]
  NotABookmarkSave,
  /// Forged weapon at position cannot be deleted because it is currently equipped or stored.
  #[error("forged weapon at position {position} is currently in use")]
  ForgeInUse {
    /// Forged weapon slot index in IFER table.
    position: usize,
  },
  /// Unit group / battle map operation error.
  #[error(transparent)]
  Group(#[from] roster::groups::Error),
  /// EDNI index table parsing error.
  #[error(transparent)]
  Edni(#[from] header::edni::Error),
  /// Unit roster manipulation error.
  #[error(transparent)]
  Roster(#[from] roster::Error),
  /// Unit manipulation error.
  #[error(transparent)]
  Unit(#[from] roster::unit::Error),
  /// Convoy storage error.
  #[error(transparent)]
  Convoy(#[from] convoy::Error),
  /// Forged weapons table error.
  #[error(transparent)]
  Forge(#[from] convoy::forge::Error),
  /// Avatar customization error.
  #[error(transparent)]
  Avatar(#[from] avatar::Error),
  /// Support table error.
  #[error(transparent)]
  Support(#[from] social::support::Error),
  /// Paralogue lifecycle error.
  #[error(transparent)]
  Paralogue(#[from] social::paralogue::Error),
  /// Container decompression or packing error.
  #[error(transparent)]
  Compress(#[from] fedit_compress::Error),
}

/// An uncompressed, editable Fire Emblem Awakening save.
#[derive(Debug)]
pub struct Save {
  /// Raw 192-byte preview header buffer.
  preview: [u8; header::preview::PREVIEW_LEN],
  /// Decompressed EDNI save payload buffer.
  payload: Vec<u8>,
  /// Resolved subsystem block index table.
  index: header::edni::EdniIndex,
  /// Encapsulated unit roster.
  pub roster: Roster,
  /// Refined and forged weapons table (IFER).
  pub forges: convoy::ForgeTable,
  /// Owned convoy storage items.
  pub convoy: convoy::Convoy,
  /// Army-wide support tracking table.
  pub supports: SupportTable,
}

impl Save {
  /// Parses a save slot from raw compressed container bytes.
  ///
  /// # Errors
  ///
  /// Fails on invalid container, corrupt stream, or bad EDNI index.
  pub fn from_bytes(raw: &[u8]) -> Result<Self, Error> {
    let container = fedit_compress::unpack(raw)?;
    let preview: [u8; header::preview::PREVIEW_LEN] =
      container.preview.as_slice().try_into().map_err(|_| {
        Error::InvalidPreviewLength {
          expected: header::preview::PREVIEW_LEN,
          actual: container.preview.len(),
        }
      })?;
    let index = header::edni::EdniIndex::from_bytes(&container.payload)?;
    let mode = header::preview::PreviewRef::from_bytes(&preview).mode();
    let is_west = preview.len() >= 0xC0;
    let roster = Roster::from_bytes(
      &container.payload[index.unit_offset..index.ifer_offset],
      is_west,
      mode,
    )?;
    let forges = convoy::ForgeTable::from_bytes(
      &container.payload[index.ifer_offset..index.convoy_offset],
      is_west,
    )?;
    let convoy = convoy::Convoy::from_bytes(
      &container.payload[index.convoy_offset..index.convoy_end_offset],
    );
    let supports =
      SupportTable::from_bytes(&container.payload[index.d62_offset..]);
    Ok(Self {
      preview,
      payload: container.payload,
      index,
      roster,
      forges,
      convoy,
      supports,
    })
  }

  /// Repacks the save slot into a compressed PMOC container.
  ///
  /// Dynamically resizes subsystem blocks and recalculates EDNI offset tables
  /// when units or forged weapons are added or removed.
  ///
  /// # Errors
  ///
  /// Fails on compression error.
  pub fn to_bytes(&self) -> Result<Vec<u8>, Error> {
    let mut head = self.payload[..self.index.unit_offset].to_vec();
    let tinu_bytes = self.roster.to_bytes();
    let ifer_bytes = self.forges.to_bytes();

    let mut nart_bytes =
      self.payload[self.index.convoy_offset..self.index.d62_offset].to_vec();
    self.convoy.write_back(&mut nart_bytes);

    let mut d62_bytes =
      self.payload[self.index.d62_offset..self.index.tsve_offset].to_vec();
    self.supports.write_back(&mut d62_bytes);

    let tsve_bytes = &self.payload[self.index.tsve_offset..];

    let new_unit_offset = self.index.unit_offset;
    let new_ifer_offset = new_unit_offset + tinu_bytes.len();
    let new_convoy_offset = new_ifer_offset + ifer_bytes.len();
    let new_d62_offset = new_convoy_offset + nart_bytes.len();
    let new_tsve_offset = new_d62_offset + d62_bytes.len();
    let delta =
      new_tsve_offset.cast_signed() - self.index.tsve_offset.cast_signed();

    let mut idx = 1;
    while idx * 4 + 4 <= head.len() {
      let b = &head[idx * 4..idx * 4 + 4];
      let raw = u32::from_le_bytes([b[0], b[1], b[2], b[3]]) as usize;
      if raw == 0 {
        break;
      }
      let old_off = raw.saturating_sub(header::edni::PREVIEW_OFFSET);
      let new_off = if old_off == self.index.user_offset {
        self.index.user_offset
      } else if old_off == self.index.pamg_offset {
        self.index.pamg_offset
      } else if old_off == self.index.unit_offset {
        new_unit_offset
      } else if old_off == self.index.ifer_offset {
        new_ifer_offset
      } else if old_off == self.index.convoy_offset {
        new_convoy_offset
      } else if old_off == self.index.d62_offset {
        new_d62_offset
      } else if old_off == self.index.tsve_offset {
        new_tsve_offset
      } else if old_off > self.index.tsve_offset {
        old_off.checked_add_signed(delta).unwrap_or(old_off)
      } else {
        old_off
      };
      let new_raw =
        u32::try_from(new_off + header::edni::PREVIEW_OFFSET).unwrap_or(0);
      head[idx * 4..idx * 4 + 4].copy_from_slice(&new_raw.to_le_bytes());
      idx += 1;
    }

    let mut payload = Vec::with_capacity(
      head.len()
        + tinu_bytes.len()
        + ifer_bytes.len()
        + nart_bytes.len()
        + d62_bytes.len()
        + tsve_bytes.len(),
    );
    payload.extend_from_slice(&head);
    payload.extend_from_slice(&tinu_bytes);
    payload.extend_from_slice(&ifer_bytes);
    payload.extend_from_slice(&nart_bytes);
    payload.extend_from_slice(&d62_bytes);
    payload.extend_from_slice(tsve_bytes);

    Ok(fedit_compress::repack(&self.preview, &payload)?)
  }

  /// Returns a borrowed view over the preview header.
  #[must_use]
  pub fn preview(&self) -> header::preview::PreviewRef<'_> {
    header::preview::PreviewRef::from_bytes(&self.preview)
  }

  /// Returns a mutable view over the preview header.
  pub fn preview_mut(&mut self) -> header::preview::PreviewMut<'_> {
    header::preview::PreviewMut::from_bytes(&mut self.preview)
  }

  /// Returns a borrowed view over the RESU campaign user block.
  #[must_use]
  pub fn user(&self) -> header::user::UserRef<'_> {
    header::user::UserRef::new(
      &self.payload[self.index.user_offset..self.index.pamg_offset],
    )
  }

  /// Returns a mutable view over the RESU campaign user block.
  pub(crate) fn user_mut(&mut self) -> header::user::UserMut<'_> {
    header::user::UserMut::new(
      &mut self.payload[self.index.user_offset..self.index.pamg_offset],
    )
  }

  /// Returns a reference to the forged weapons table.
  #[must_use]
  pub fn forges(&self) -> &convoy::ForgeTable {
    &self.forges
  }

  /// Returns a reference to the convoy storage subsystem.
  #[must_use]
  pub fn convoy(&self) -> &convoy::Convoy {
    &self.convoy
  }

  /// Returns a reference to all unit groups across the save file.
  #[must_use]
  pub fn groups(&self) -> &roster::UnitGroups {
    self.roster.groups()
  }

  /// Returns deployed player combatants on battle maps.
  #[must_use]
  pub fn deployed_units(&self) -> &[roster::Unit] {
    self.roster.groups().blue()
  }

  /// Returns enemy combatants on battle maps.
  #[must_use]
  pub fn enemy_units(&self) -> &[roster::Unit] {
    self.roster.groups().red()
  }

  /// Returns allied/partner combatants on battle maps.
  #[must_use]
  pub fn ally_units(&self) -> &[roster::Unit] {
    self.roster.groups().green()
  }

  /// Returns dead units fallen in battle.
  #[must_use]
  pub fn dead_units(&self) -> &[roster::Unit] {
    self.roster.groups().dead()
  }

  /// Returns an iterator over all player units across deployed battle units and reserve camp roster.
  pub fn all_player_units(&self) -> impl Iterator<Item = &roster::Unit> {
    self.roster.groups().blue().iter().chain(self.roster.iter())
  }

  /// Finds a unit in the roster by character name (case-insensitive).
  pub fn find_unit_by_name_mut(
    &mut self,
    name: &str,
  ) -> Option<&mut roster::Unit> {
    self.roster.find_by_name_mut(name)
  }

  /// Finds a unit in the roster by character name (case-insensitive).
  #[must_use]
  pub fn find_unit_by_name(&self, name: &str) -> Option<&roster::Unit> {
    self.roster.find_by_name(name)
  }
}
