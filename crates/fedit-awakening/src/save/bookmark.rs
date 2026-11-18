//! Mid-battle bookmark save state operations and combatant mutations.
//!
//! Owns guarded operations that are strictly valid only on active battle grid
//! bookmark saves (Map0/Map1), guarding chapter bosses, recruitment targets,
//! and story anchors.

use crate::save::Error;
use crate::save::Save;
use crate::save::header::config::GameMode;
use crate::save::roster;

impl Save {
  /// Returns true if this save slot is an active mid-battle bookmark save.
  #[must_use]
  pub fn is_bookmark(&self) -> bool {
    self.roster.groups().is_bookmark()
  }

  /// Returns read-only deployed player combatants on the active battle grid.
  #[must_use]
  pub fn bookmark_deployed_units(&self) -> &[roster::Unit] {
    self.roster.groups().blue()
  }

  /// Returns read-only enemy combatants on the active battle grid.
  #[must_use]
  pub fn bookmark_enemy_units(&self) -> &[roster::Unit] {
    self.roster.groups().red()
  }

  /// Returns read-only allied/partner combatants on the active battle grid.
  #[must_use]
  pub fn bookmark_ally_units(&self) -> &[roster::Unit] {
    self.roster.groups().green()
  }

  /// Deploys a unit from camp reserves onto the battle grid in a bookmark save.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::NotABookmarkSave`] if invoked on a standard chapter save.
  /// - Returns [`Error::Group`] if coordinates are out of bounds or tile is occupied.
  pub fn bookmark_deploy_unit(
    &mut self,
    roster_idx: usize,
    coordinates: [u8; 2],
  ) -> Result<(), Error> {
    if !self.is_bookmark() {
      return Err(Error::NotABookmarkSave);
    }
    self
      .roster
      .groups_mut()
      .deploy_unit(roster_idx, coordinates)?;
    Ok(())
  }

  /// Moves a deployed player combatant back to camp reserves.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::NotABookmarkSave`] if invoked on a standard chapter save.
  /// - Returns [`Error::Group`] if attempting to undeploy mandatory force-deploys (Chrom, Robin).
  pub fn bookmark_undeploy_unit(
    &mut self,
    blue_idx: usize,
  ) -> Result<(), Error> {
    if !self.is_bookmark() {
      return Err(Error::NotABookmarkSave);
    }
    self.roster.groups_mut().undeploy_unit(blue_idx)?;
    Ok(())
  }

  /// Moves a deployed player combatant to new grid coordinates in a bookmark save.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::NotABookmarkSave`] if invoked on a standard chapter save.
  /// - Returns [`Error::Group`] if coordinates are out of bounds or tile is occupied.
  pub fn bookmark_move_unit(
    &mut self,
    blue_idx: usize,
    coordinates: [u8; 2],
  ) -> Result<(), Error> {
    if !self.is_bookmark() {
      return Err(Error::NotABookmarkSave);
    }
    self
      .roster
      .groups_mut()
      .move_deployed_unit(blue_idx, coordinates)?;
    Ok(())
  }

  /// Spawns a new enemy combatant on the active battle grid in a bookmark save.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::NotABookmarkSave`] if invoked on a standard chapter save.
  /// - Returns [`Error::Group`] if coordinates are out of bounds or tile is occupied.
  pub fn bookmark_spawn_enemy(
    &mut self,
    character: roster::Character,
    class: roster::Class,
    coordinates: [u8; 2],
  ) -> Result<usize, Error> {
    if !self.is_bookmark() {
      return Err(Error::NotABookmarkSave);
    }
    Ok(
      self
        .roster
        .groups_mut()
        .add_enemy(character, class, coordinates)?,
    )
  }

  /// Removes an enemy combatant from the active battle grid in a bookmark save.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::NotABookmarkSave`] if invoked on a standard chapter save.
  /// - Returns [`Error::Group`] if index is out of bounds, target is a chapter boss,
  ///   or target is a recruitable unit (preventing uncompletable chapter softlocks).
  pub fn bookmark_remove_enemy(&mut self, index: usize) -> Result<(), Error> {
    if !self.is_bookmark() {
      return Err(Error::NotABookmarkSave);
    }
    self.roster.groups_mut().remove_enemy(index)?;
    Ok(())
  }

  /// Spawns a new allied/partner combatant on the active battle grid.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::NotABookmarkSave`] if invoked on a standard chapter save.
  /// - Returns [`Error::Group`] if coordinates are out of bounds or tile is occupied.
  pub fn bookmark_spawn_ally(
    &mut self,
    character: roster::Character,
    class: roster::Class,
    coordinates: [u8; 2],
  ) -> Result<usize, Error> {
    if !self.is_bookmark() {
      return Err(Error::NotABookmarkSave);
    }
    Ok(
      self
        .roster
        .groups_mut()
        .add_ally(character, class, coordinates)?,
    )
  }

  /// Removes an allied combatant from the active battle grid.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::NotABookmarkSave`] if invoked on a standard chapter save.
  /// - Returns [`Error::Group`] if target is a story-protected friendly NPC.
  pub fn bookmark_remove_ally(&mut self, index: usize) -> Result<(), Error> {
    if !self.is_bookmark() {
      return Err(Error::NotABookmarkSave);
    }
    self.roster.groups_mut().remove_ally(index)?;
    Ok(())
  }

  /// Records the death or retreat of a deployed combatant in a bookmark save.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::NotABookmarkSave`] if invoked on a standard chapter save.
  /// - Returns [`Error::Group`] if attempting to kill a story anchor unit.
  pub fn bookmark_kill_unit(&mut self, blue_idx: usize) -> Result<(), Error> {
    if !self.is_bookmark() {
      return Err(Error::NotABookmarkSave);
    }
    let is_casual = self.preview().mode() == Some(GameMode::Casual);
    self
      .roster
      .groups_mut()
      .kill_deployed_unit(blue_idx, is_casual)?;
    Ok(())
  }
}
