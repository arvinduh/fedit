//! `SaveSlot` container aggregate and codecs.
//!
//! Owns the uncompressed save slot aggregate, parsing and repacking archives.

use fedit_compress;

use crate::error;

use super::avatar;
use super::convoy;
use super::index;
use super::paralogue;
use super::preview;
use super::roster;
use super::support;
use super::text;
use super::time;
use super::types;
use super::user;

pub use super::paralogue::ParalogueState;

/// An uncompressed, editable Fire Emblem Awakening save slot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SaveSlot {
  /// Raw 192-byte preview header buffer.
  preview: [u8; preview::PREVIEW_LEN],
  /// Decompressed EDNI save payload buffer.
  payload: Vec<u8>,
  /// Resolved subsystem block index table.
  index: index::EdniIndex,
  /// Encapsulated unit roster.
  pub roster: roster::Roster,
  /// Owned convoy storage items.
  pub convoy: Vec<convoy::ConvoyItem>,
  /// Army-wide support tracking table.
  pub supports: support::SupportTable,
}

impl SaveSlot {
  /// Parses a save slot from raw compressed container bytes.
  ///
  /// # Errors
  ///
  /// Fails on invalid container, corrupt stream, or bad EDNI index.
  pub fn from_bytes(raw: &[u8]) -> Result<Self, error::Error> {
    let container = fedit_compress::unpack(raw)?;
    let preview: [u8; preview::PREVIEW_LEN] =
      container.preview.as_slice().try_into().map_err(|_| {
        error::Error::InvalidPreviewLength {
          expected: preview::PREVIEW_LEN,
          actual: container.preview.len(),
        }
      })?;
    let index = index::EdniIndex::parse(&container.payload)?;
    let mode = preview::PreviewRef::new(&preview).mode();
    let units = roster::unit::UnitIter::collect_units(
      &container.payload[index.unit_offset..index.ifer_offset],
      mode,
    );
    let roster = roster::Roster::new(units);
    let convoy = convoy::ConvoyRef::new(
      &container.payload[index.convoy_offset..index.convoy_end_offset],
    )
    .items();
    let supports = support::SupportTable::new();
    Ok(Self {
      preview,
      payload: container.payload,
      index,
      roster,
      convoy,
      supports,
    })
  }

  /// Returns a borrowed slice of the decompressed save payload buffer.
  #[must_use]
  pub fn payload(&self) -> &[u8] {
    &self.payload
  }

  /// Returns the offset of the TINU unit block.
  #[must_use]
  pub fn unit_offset(&self) -> usize {
    self.index.unit_offset
  }

  /// Returns the offset of the IFER item block.
  #[must_use]
  pub fn ifer_offset(&self) -> usize {
    self.index.ifer_offset
  }

  /// Repacks the save slot into a compressed PMOC container.
  ///
  /// # Errors
  ///
  /// Fails on compression error.
  pub fn to_bytes(&self) -> Result<Vec<u8>, error::Error> {
    let mut payload = self.payload.clone();
    for u in &self.roster {
      let base = self.index.unit_offset + u.offset();
      if base + 60 <= payload.len() {
        u.write_back(&mut payload[base..]);
      }
    }
    for item in &self.convoy {
      let off = self.index.convoy_offset + 8 + item.slot * 4;
      if off + 4 <= payload.len() {
        payload[off..off + 2]
          .copy_from_slice(&item.item.raw_id().to_be_bytes());
        payload[off + 2..off + 4].copy_from_slice(&item.uses.to_be_bytes());
      }
    }
    Ok(fedit_compress::repack(&self.preview, &payload)?)
  }

  /// Returns a borrowed view over the preview header.
  #[must_use]
  pub fn preview(&self) -> preview::PreviewRef<'_> {
    preview::PreviewRef::new(&self.preview)
  }

  /// Returns a mutable view over the preview header.
  pub fn preview_mut(&mut self) -> preview::PreviewMut<'_> {
    preview::PreviewMut::new(&mut self.preview)
  }

  /// Returns a borrowed view over the RESU campaign user block.
  #[must_use]
  pub fn user(&self) -> user::UserRef<'_> {
    user::UserRef::new(&self.payload[self.index.user_offset..])
  }

  /// Returns a mutable view over the RESU campaign user block.
  pub fn user_mut(&mut self) -> user::UserMut<'_> {
    user::UserMut::new(&mut self.payload[self.index.user_offset..])
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

  /// Returns campaign gold.
  #[must_use]
  pub fn gold(&self) -> u32 {
    self.user().gold()
  }

  /// Sets campaign gold amount.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::GoldOutOfBounds`] if `amount > MAX_GOLD`.
  pub fn set_gold(&mut self, amount: u32) -> Result<(), error::Error> {
    if amount > user::MAX_GOLD {
      return Err(error::Error::GoldOutOfBounds {
        value: amount,
        max: user::MAX_GOLD,
      });
    }
    self.user_mut().set_gold(amount);
    Ok(())
  }

  /// Returns total play time duration.
  #[must_use]
  pub fn play_time(&self) -> time::PlayTime {
    self.preview().play_time()
  }

  /// Updates total play time across preview header and campaign user block.
  pub fn set_play_time(&mut self, time: time::PlayTime) {
    self.preview_mut().set_play_time(time);
    let secs = u32::try_from(time.as_secs()).unwrap_or(u32::MAX);
    self.user_mut().set_play_time(secs);
  }

  /// Updates unit status at `index` with Casual mode permadeath protection.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::CasualModePermadeathForbidden`] if trying to mark
  /// a unit as [`roster::UnitStatus::Dead`] in Casual mode,
  /// or [`error::Error::SlotOutOfBounds`] if `index >= self.roster.len()`.
  pub fn set_unit_status(
    &mut self,
    index: usize,
    status: roster::UnitStatus,
  ) -> Result<(), error::Error> {
    if index >= self.roster.len() {
      return Err(error::Error::SlotOutOfBounds { slot: index });
    }
    if self.preview().mode() == Some(types::GameMode::Casual)
      && status == roster::UnitStatus::Dead
    {
      return Err(error::Error::CasualModePermadeathForbidden);
    }
    self.roster[index].set_status(status)
  }

  /// Sets the save game mode, reviving any dead units if switching to Casual.
  pub fn set_mode(&mut self, mode: types::GameMode) {
    self.preview_mut().set_mode(mode);
    if mode == types::GameMode::Casual {
      for u in &mut self.roster {
        if u.status() == roster::UnitStatus::Dead {
          u.revive();
        }
      }
    }
  }

  /// Sets whether the unit at `index` is actively deployed or inactive.
  ///
  /// Automatically resolves whether to mark the unit as [`roster::UnitStatus::Retreated`]
  /// or [`roster::UnitStatus::Dead`] based on game mode and story anchor invariants:
  /// - Mandatory story force-deploys (Chrom, Robin) cannot be made inactive.
  /// - On Casual mode or for story anchors (Lucina), inactive units become `Retreated`.
  /// - On Classic mode for regular units, inactive units become `Dead`.
  /// - Activating an inactive unit automatically revives them, restoring HP and base stats.
  ///
  /// # Errors
  ///
  /// - Returns [`error::Error::AnchorUnitProtected`] if attempting to deactivate Chrom or Robin.
  /// - Returns [`error::Error::SlotOutOfBounds`] if `index >= self.roster.len()`.
  pub fn set_unit_active(
    &mut self,
    index: usize,
    active: bool,
  ) -> Result<(), error::Error> {
    if index >= self.roster.len() {
      return Err(error::Error::SlotOutOfBounds { slot: index });
    }
    let is_casual = self.preview().mode() == Some(types::GameMode::Casual);
    let unit = &mut self.roster[index];
    if active {
      unit.revive();
      Ok(())
    } else {
      if unit.character().is_force_deployed() {
        return Err(error::Error::AnchorUnitProtected {
          character: unit.character(),
        });
      }
      if is_casual || unit.character().is_story_anchor() {
        unit.set_status(roster::UnitStatus::Retreated)
      } else {
        unit.set_status(roster::UnitStatus::Dead)
      }
    }
  }

  /// Updates current story chapter with hard-block dead recruitment conflict checks.
  ///
  /// # Errors
  ///
  /// - Returns [`error::Error::ChapterOutOfBounds`] if `target_chapter > 26`.
  /// - Returns [`error::Error::DeadUnitRecruitmentConflict`] if reverting to or before
  ///   the recruitment chapter of any unit currently dead in the roster.
  pub fn set_chapter(
    &mut self,
    target_chapter: u16,
  ) -> Result<(), error::Error> {
    if target_chapter > 26 {
      return Err(error::Error::ChapterOutOfBounds {
        chapter: target_chapter,
      });
    }
    for u in &self.roster {
      if u.status() == roster::UnitStatus::Dead
        && let Some(rec) = u.character().recruitment_chapter()
        && target_chapter <= u16::from(rec)
      {
        return Err(error::Error::DeadUnitRecruitmentConflict {
          character: u.character(),
          recruitment_chapter: rec,
          target_chapter,
        });
      }
    }
    self.preview_mut().set_chapter_id(target_chapter);
    let ch_byte = u8::try_from(target_chapter).unwrap_or(26);
    self.user_mut().set_current_chapter(ch_byte);
    Ok(())
  }

  /// Resolves the decompressed payload byte offset of Robin's `LogBlock`.
  fn find_avatar_log_offset(&self) -> Result<usize, error::Error> {
    let robin_name = self.preview().avatar_name();
    let mut name_bytes = [0u8; 26];
    text::write_utf16le(&mut name_bytes, &robin_name);
    let search_prefix = if robin_name.is_empty() {
      &b"R\x00o\x00"[..]
    } else {
      let len = (robin_name.chars().count() * 2).min(26);
      &name_bytes[..len]
    };

    let start = self.index.unit_offset;
    let end = self.index.ifer_offset.min(self.payload.len());
    let min_len = avatar::AvatarCustomization::LOG_BLOCK_LEN;
    if start >= end || end.saturating_sub(start) < min_len {
      return Err(error::Error::AvatarNotFound);
    }

    for i in start..=end - min_len {
      if self.payload[i..].starts_with(search_prefix) {
        let asset = self.payload[i + 26];
        let flaw = self.payload[i + 27];
        let gender = self.payload[i + 28];
        if asset <= 7 && flaw <= 7 && gender <= 1 {
          return Ok(i);
        }
      }
    }
    Err(error::Error::AvatarNotFound)
  }

  /// Returns Robin's avatar customization attributes.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::AvatarNotFound`] if the `LogBlock` is not found.
  pub fn avatar_customization(
    &self,
  ) -> Result<avatar::AvatarCustomization, error::Error> {
    let offset = self.find_avatar_log_offset()?;
    avatar::AvatarCustomization::parse(&self.payload[offset..])
  }

  /// Sets Robin's avatar customization attributes with strict invariant checks.
  ///
  /// # Side Effects
  ///
  /// - Synchronizes the avatar display name in the preview header.
  /// - Modifies the 392-byte `LogBlock` attributes in the save payload.
  /// - If the avatar gender changes, synchronizes Robin's PID ($0 \leftrightarrow 1$)
  ///   and Morgan's PID ($39 \leftrightarrow 40$) and verifies class compatibility.
  ///
  /// # Errors
  ///
  /// - Returns [`error::Error::StatFocusConflict`] if asset and flaw are identical.
  /// - Returns [`error::Error::GenderIncompatibleClass`] if Robin's or Morgan's
  ///   current class is ineligible for the new gender.
  /// - Returns [`error::Error::AvatarNotFound`] if the `LogBlock` is not found.
  pub fn set_avatar_customization(
    &mut self,
    custom: &avatar::AvatarCustomization,
  ) -> Result<(), error::Error> {
    if custom.asset == custom.flaw {
      return Err(error::Error::StatFocusConflict);
    }

    let log_offset = self.find_avatar_log_offset()?;
    let current_custom =
      avatar::AvatarCustomization::parse(&self.payload[log_offset..])?;

    if custom.gender != current_custom.gender {
      if let Some(robin) = self
        .roster
        .iter()
        .find(|u| u.character() == roster::Character::Robin)
        && !robin.class().is_eligible_for_gender(custom.gender)
      {
        return Err(error::Error::GenderIncompatibleClass {
          character: roster::Character::Robin,
          class: robin.class(),
          gender: custom.gender,
        });
      }
      let morgan_gender = custom.gender.opposite();
      if let Some(morgan) = self
        .roster
        .iter()
        .find(|u| u.character() == roster::Character::Morgan)
        && !morgan.class().is_eligible_for_gender(morgan_gender)
      {
        return Err(error::Error::GenderIncompatibleClass {
          character: roster::Character::Morgan,
          class: morgan.class(),
          gender: morgan_gender,
        });
      }

      if let Some(robin) = self
        .roster
        .iter_mut()
        .find(|u| u.character() == roster::Character::Robin)
      {
        robin.set_gender(custom.gender)?;
      }
      if let Some(morgan) = self
        .roster
        .iter_mut()
        .find(|u| u.character() == roster::Character::Morgan)
      {
        morgan.set_gender(morgan_gender)?;
      }
    }

    self.preview_mut().set_avatar_name(&custom.name);
    custom.write_back(&mut self.payload[log_offset..])?;
    Ok(())
  }

  /// Returns the current progression state of the specified paralogue (1..=23).
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::ChapterOutOfBounds`] if `paralogue == 0 || paralogue > 23`.
  pub fn paralogue_state(
    &self,
    paralogue: u8,
  ) -> Result<ParalogueState, error::Error> {
    if paralogue == 0 || paralogue > 23 {
      return Err(error::Error::ChapterOutOfBounds {
        chapter: u16::from(paralogue),
      });
    }
    let map_id = 26 + usize::from(paralogue);
    let map_offset = self.index.pamg_offset + 62 + map_id * 29 + 1;
    if map_offset >= self.payload.len() {
      return Err(error::Error::OffsetOutOfBounds {
        offset: map_offset,
        len: self.payload.len(),
      });
    }
    Ok(ParalogueState::from_raw(self.payload[map_offset]))
  }

  /// Sets the progression state of the specified paralogue (1..=23).
  ///
  /// # Side Effects
  ///
  /// Modifies the `lockState` byte for the paralogue in the PAMG block.
  ///
  /// # Errors
  ///
  /// - Returns [`error::Error::ChapterOutOfBounds`] if `paralogue == 0 || paralogue > 23`.
  /// - Returns [`error::Error::ChildParalogueGatedByChapter13`] if attempting to unlock
  ///   a child paralogue (5..=16) before Chapter 13 is cleared.
  /// - Returns [`error::Error::ParentsNotMarried`] if attempting to unlock a child
  ///   paralogue when the child's parents have not reached S-Rank marriage.
  /// - Returns [`error::Error::ChildInRosterParalogueConflict`] if attempting to revert
  ///   a completed paralogue to Locked or Unlocked while the child unit is in the roster.
  pub fn set_paralogue_state(
    &mut self,
    paralogue: u8,
    state: ParalogueState,
  ) -> Result<(), error::Error> {
    if paralogue == 0 || paralogue > 23 {
      return Err(error::Error::ChapterOutOfBounds {
        chapter: u16::from(paralogue),
      });
    }

    if let Some(child) = paralogue::child_for_paralogue(paralogue) {
      if state == ParalogueState::Unlocked {
        if self.preview().chapter_id() < 13 {
          return Err(error::Error::ChildParalogueGatedByChapter13 {
            paralogue,
          });
        }
        if let Some(parent) = paralogue::child_primary_parent(child)
          && self.supports.spouse_of(parent).is_none()
        {
          return Err(error::Error::ParentsNotMarried { child });
        }
      }

      let current_state = self.paralogue_state(paralogue)?;
      if current_state == ParalogueState::Completed
        && state != ParalogueState::Completed
        && self.roster.iter().any(|u| u.character() == child)
      {
        return Err(error::Error::ChildInRosterParalogueConflict {
          child,
          paralogue,
        });
      }
    }

    let map_id = 26 + usize::from(paralogue);
    let map_offset = self.index.pamg_offset + 62 + map_id * 29 + 1;
    if map_offset >= self.payload.len() {
      return Err(error::Error::OffsetOutOfBounds {
        offset: map_offset,
        len: self.payload.len(),
      });
    }
    self.payload[map_offset] = state.raw_id();
    Ok(())
  }

  /// Returns the parents (mother, optional father) of a child character.
  #[must_use]
  pub fn child_parents(
    &self,
    child: roster::Character,
  ) -> (Option<roster::Character>, Option<roster::Character>) {
    let primary = paralogue::child_primary_parent(child);
    let secondary = primary.and_then(|p| self.supports.spouse_of(p));
    (primary, secondary)
  }

  /// Sets the father/spouse for a canonical child character.
  ///
  /// # Side Effects
  ///
  /// Modifies the support table to establish an S-Rank marriage between
  /// the child's primary parent and the assigned spouse.
  ///
  /// # Errors
  ///
  /// - Returns [`error::Error::NotAChildCharacter`] if the child has no variable parentage.
  /// - Returns [`error::Error::AlreadyMarried`] if the father or mother is already married.
  /// - Returns [`error::Error::IncompatibleSupportPair`] if the characters cannot support.
  pub fn set_child_father(
    &mut self,
    child: roster::Character,
    father: roster::Character,
  ) -> Result<(), error::Error> {
    let Some(mother) = paralogue::child_primary_parent(child) else {
      return Err(error::Error::NotAChildCharacter { child });
    };
    self
      .supports
      .set_rank(mother, father, support::SupportRank::S)
  }
}
