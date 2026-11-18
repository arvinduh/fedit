//! Campaign story progression, inventory economics, and social states.
//!
//! Owns story progression mutations, gold and renown economics, play time tracking,
//! chapter rollback validation, paralogue lifecycles, and child parentage resolution.

use crate::save::Error;
use crate::save::Save;
use crate::save::header;
use crate::save::header::config::GameMode;
use crate::save::roster;
use crate::save::social;
use crate::save::social::paralogue::ParalogueState;
use crate::save::social::support::SupportRank;
use crate::save::utils::time::PlayTime;

impl Save {
  /// Returns campaign gold.
  #[must_use]
  pub fn gold(&self) -> u32 {
    self.user().gold()
  }

  /// Sets campaign gold amount.
  ///
  /// # Errors
  ///
  /// Returns [`Error::GoldOutOfBounds`] if `amount > MAX_GOLD`.
  pub fn set_gold(&mut self, amount: u32) -> Result<(), Error> {
    if amount > header::user::MAX_GOLD {
      return Err(Error::GoldOutOfBounds {
        gold: amount,
        max: header::user::MAX_GOLD,
      });
    }
    self.user_mut().set_gold(amount);
    Ok(())
  }

  /// Returns campaign renown points.
  #[must_use]
  pub fn renown(&self) -> u32 {
    self.user().renown()
  }

  /// Sets campaign renown points.
  pub fn set_renown(&mut self, amount: u32) {
    self.user_mut().set_renown(amount);
  }

  /// Returns total play time duration.
  #[must_use]
  pub fn play_time(&self) -> PlayTime {
    self.preview().play_time()
  }

  /// Updates total play time across preview header and campaign user block.
  pub fn set_play_time(&mut self, time: PlayTime) {
    self.preview_mut().set_play_time(time);
    let secs = u32::try_from(time.as_secs()).unwrap_or(u32::MAX);
    self.user_mut().set_play_time(secs);
  }

  /// Updates unit status at `index` with Casual mode permadeath protection.
  ///
  /// # Errors
  ///
  /// Returns [`Error::CasualModePermadeathForbidden`] if trying to mark
  /// a unit as [`roster::UnitStatus::Dead`] in Casual mode,
  /// or [`Error::SlotOutOfBounds`] if `index >= self.roster.len()`.
  pub fn set_unit_status(
    &mut self,
    index: usize,
    status: roster::UnitStatus,
  ) -> Result<(), Error> {
    if index >= self.roster.len() {
      return Err(roster::unit::Error::SlotOutOfBounds { slot: index }.into());
    }
    if self.preview().mode() == Some(GameMode::Casual)
      && status == roster::UnitStatus::Dead
    {
      return Err(Error::CasualModePermadeathForbidden);
    }
    if self.roster[index].is_story_anchor()
      && status == roster::UnitStatus::Dead
    {
      return Err(
        roster::unit::Error::AnchorUnitProtected {
          character: self.roster[index].character(),
        }
        .into(),
      );
    }
    self.roster.set_unit_status(index, status)?;
    Ok(())
  }

  /// Sets the save game mode, reviving any dead units if switching to Casual.
  pub fn set_mode(&mut self, mode: GameMode) {
    self.preview_mut().set_mode(mode);
    if mode == GameMode::Casual {
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
  /// - Returns [`Error::AnchorUnitProtected`] if attempting to deactivate Chrom or Robin.
  /// - Returns [`Error::SlotOutOfBounds`] if `index >= self.roster.len()`.
  pub fn set_unit_active(
    &mut self,
    index: usize,
    active: bool,
  ) -> Result<(), Error> {
    if index >= self.roster.len() {
      return Err(roster::unit::Error::SlotOutOfBounds { slot: index }.into());
    }
    let is_casual = self.preview().mode() == Some(GameMode::Casual);
    let unit = &mut self.roster[index];
    if active {
      unit.revive();
      Ok(())
    } else {
      if unit.character().is_force_deployed() {
        return Err(
          roster::unit::Error::AnchorUnitProtected {
            character: unit.character(),
          }
          .into(),
        );
      }
      if is_casual || unit.character().is_story_anchor() {
        unit.set_status(roster::UnitStatus::Retreated)?;
      } else {
        unit.set_status(roster::UnitStatus::Dead)?;
      }
      Ok(())
    }
  }

  /// Updates current story chapter with hard-block dead recruitment conflict checks.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::ChapterOutOfBounds`] if `target_chapter > 26`.
  /// - Returns [`Error::DeadUnitRecruitmentConflict`] if reverting to or before
  ///   the recruitment chapter of any unit currently dead in the roster.
  pub fn set_chapter(&mut self, target_chapter: u16) -> Result<(), Error> {
    if target_chapter > 26 {
      return Err(Error::ChapterOutOfBounds {
        chapter: target_chapter,
      });
    }
    for u in &self.roster {
      if u.status() == roster::UnitStatus::Dead
        && let Some(rec) = u.character().recruitment_chapter()
        && target_chapter <= u16::from(rec)
      {
        return Err(Error::DeadUnitRecruitmentConflict {
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

  /// Returns the current progression state of the specified paralogue (1..=23).
  ///
  /// # Errors
  ///
  /// Returns [`Error::ChapterOutOfBounds`] if `paralogue == 0 || paralogue > 23`.
  pub fn paralogue_state(
    &self,
    paralogue: u8,
  ) -> Result<ParalogueState, Error> {
    if paralogue == 0 || paralogue > 23 {
      return Err(Error::ChapterOutOfBounds {
        chapter: u16::from(paralogue),
      });
    }
    let map_id = 26 + usize::from(paralogue);
    let map_offset = self.index.pamg_offset + 62 + map_id * 29 + 1;
    if map_offset >= self.payload.len() {
      return Err(
        header::edni::Error::OffsetOutOfBounds {
          offset: map_offset,
          len: self.payload.len(),
        }
        .into(),
      );
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
  /// - Returns [`Error::ChapterOutOfBounds`] if `paralogue == 0 || paralogue > 23`.
  /// - Returns [`social::paralogue::Error::ChildParalogueGatedByChapter13`] if
  ///   attempting to unlock a child paralogue (5..=16) before Chapter 13 is cleared.
  /// - Returns [`social::paralogue::Error::ParentsNotMarried`] if attempting to
  ///   unlock a child paralogue when the child's parents have not reached S-Rank marriage.
  /// - Returns [`social::paralogue::Error::ChildInRosterParalogueConflict`] if
  ///   attempting to revert a completed paralogue to Locked or Unlocked while the
  ///   child unit is in the roster.
  pub fn set_paralogue_state(
    &mut self,
    paralogue: u8,
    state: ParalogueState,
  ) -> Result<(), Error> {
    if paralogue == 0 || paralogue > 23 {
      return Err(Error::ChapterOutOfBounds {
        chapter: u16::from(paralogue),
      });
    }

    if let Some(child) = roster::Character::from_paralogue(paralogue) {
      if state == ParalogueState::Unlocked {
        if self.preview().chapter_id() < 13 {
          return Err(
            social::paralogue::Error::ChildParalogueGatedByChapter13 {
              paralogue,
            }
            .into(),
          );
        }
        if let Some(parent) = child.primary_parent()
          && self.supports.spouse_of(parent).is_none()
        {
          return Err(
            social::paralogue::Error::ParentsNotMarried { child }.into(),
          );
        }
      }

      let current_state = self.paralogue_state(paralogue)?;
      if current_state == ParalogueState::Completed
        && state != ParalogueState::Completed
        && self.roster.iter().any(|u| u.character() == child)
      {
        return Err(
          social::paralogue::Error::ChildInRosterParalogueConflict {
            child,
            paralogue,
          }
          .into(),
        );
      }
    }

    let map_id = 26 + usize::from(paralogue);
    let map_offset = self.index.pamg_offset + 62 + map_id * 29 + 1;
    if map_offset >= self.payload.len() {
      return Err(
        header::edni::Error::OffsetOutOfBounds {
          offset: map_offset,
          len: self.payload.len(),
        }
        .into(),
      );
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
    let primary = child.primary_parent();
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
  /// - Returns [`social::paralogue::Error::NotAChildCharacter`] if the child has no variable parentage.
  /// - Returns [`social::support::Error`] if the characters cannot support or are already married.
  pub fn set_child_father(
    &mut self,
    child: roster::Character,
    father: roster::Character,
  ) -> Result<(), Error> {
    let Some(mother) = child.primary_parent() else {
      return Err(
        social::paralogue::Error::NotAChildCharacter { child }.into(),
      );
    };
    self.supports.set_rank(mother, father, SupportRank::S)?;
    Ok(())
  }
}
