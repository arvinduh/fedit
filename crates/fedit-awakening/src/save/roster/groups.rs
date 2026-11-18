//! Unit groups partitioning (Groups 0..5) from the TINU subsystem block.
//!
//! Owns the partitioning of all six character groups stored in the save file:
//! Group 0 (Blue deployed), Group 1 (Red enemies), Group 2 (Green allies),
//! Group 3 (Main campaign reserves), Group 4 (Dead units), and Group 5 (Other).

use crate::save::header::config;
use crate::save::roster::unit;

/// Encapsulated unit collections across all six TINU group partitions.
#[derive(Debug)]
pub struct UnitGroups {
  /// Group 0: Deployed player combatants on battle maps.
  pub(crate) blue: Vec<unit::Unit>,
  /// Group 1: Enemy combatants on battle maps.
  pub(crate) red: Vec<unit::Unit>,
  /// Group 2: Allied/partner combatants on battle maps.
  pub(crate) green: Vec<unit::Unit>,
  /// Group 3: Main campaign army roster (camp and reserves).
  pub(crate) main: Vec<unit::Unit>,
  /// Group 4: Fallen units in Classic mode.
  pub(crate) dead: Vec<unit::Unit>,
  /// Group 5: Other mission combatants.
  pub(crate) other: Vec<unit::Unit>,
  /// Preserved TINU block header bytes (5 bytes: `b"TINU"` + version).
  pub(crate) header: Vec<u8>,
  /// Preserved trailer/footer bytes following the unit groups.
  pub(crate) footer: Vec<u8>,
}

impl UnitGroups {
  /// Parses all unit groups from a TINU block byte slice.
  ///
  /// # Errors
  ///
  /// Returns [`unit::Error`] if the magic is invalid or a unit record is corrupt.
  #[expect(
    clippy::too_many_lines,
    reason = "Comprehensive 6-group binary decoder matching Java reference layout"
  )]
  pub fn from_bytes(
    raw: &[u8],
    is_west: bool,
    mode: Option<config::GameMode>,
  ) -> Result<Self, unit::Error> {
    if raw.len() < 5 || &raw[0..4] != b"TINU" {
      return Err(unit::Error::InvalidByteLength {
        expected: 5,
        actual: raw.len(),
      });
    }

    let header = raw[0..5].to_vec();
    let mut offset = 5;
    let mut groups: [Vec<unit::Unit>; 6] = [
      Vec::new(),
      Vec::new(),
      Vec::new(),
      Vec::new(),
      Vec::new(),
      Vec::new(),
    ];

    for prefix in 0..=5u8 {
      if offset < raw.len() && raw[offset] == prefix {
        offset += 1;
        if offset >= raw.len() {
          break;
        }
        let unit_count = raw[offset] as usize;
        offset += 1;
        let lblock_size = if is_west { 0x188 } else { 0xFC };
        let cblock_size = 0x25;
        let is_battle_group = matches!(prefix, 0 | 1 | 2 | 5);

        for _ in 0..unit_count {
          if offset + 0x44 > raw.len() {
            break;
          }
          let support_size = raw[offset + 0x43] as usize;
          let unk_idx = offset + 0x43 + support_size + 1;
          let unknown_size = if unk_idx < raw.len() {
            raw[unk_idx] as usize
          } else {
            0
          };
          let gblock_len = 0xBB + support_size + unknown_size;
          if offset + gblock_len > raw.len() {
            break;
          }
          let mut lblock_len = 0;
          let mut cblock_len = 0;
          if raw[offset + gblock_len - 2] == 1
            && raw[offset + gblock_len - 1] == 6
          {
            lblock_len = lblock_size;
            if offset + gblock_len + lblock_len <= raw.len()
              && raw[offset + gblock_len + lblock_len - 1] == 1
            {
              cblock_len = cblock_size;
            }
          } else if raw[offset + gblock_len - 2] == 0
            && raw[offset + gblock_len - 1] == 1
          {
            cblock_len = cblock_size;
          }
          let total_len = gblock_len + lblock_len + cblock_len;
          if offset + total_len > raw.len() {
            break;
          }
          let unit_slice = &raw[offset..offset + total_len];
          let mut unit = unit::Unit::from_bytes(unit_slice, offset, mode)?;
          if is_battle_group {
            let hp = if unit_slice.len() > 20 {
              unit_slice[20]
            } else {
              0
            };
            let boots = if unit_slice.len() > 21 {
              unit_slice[21]
            } else {
              0
            };
            let c1 = if unit_slice.len() >= 24 {
              [unit_slice[22], unit_slice[23]]
            } else {
              [0, 0]
            };
            let c2 = if unit_slice.len() >= 26 {
              [unit_slice[24], unit_slice[25]]
            } else {
              [0, 0]
            };
            unit.set_battle_state(Some(unit::battle::BattleState::new(
              hp, boots, c1, c2,
            )));
          }
          groups[prefix as usize].push(unit);
          offset += total_len;
        }
      }
    }

    let footer = if offset < raw.len() {
      raw[offset..].to_vec()
    } else {
      Vec::new()
    };

    let [blue, red, green, main, dead, other] = groups;
    Ok(Self {
      blue,
      red,
      green,
      main,
      dead,
      other,
      header,
      footer,
    })
  }

  /// Serializes all unit groups back into a contiguous binary TINU block.
  #[must_use]
  pub fn to_bytes(&self) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&self.header);
    let all_groups = [
      &self.blue,
      &self.red,
      &self.green,
      &self.main,
      &self.dead,
      &self.other,
    ];
    for (i, grp) in all_groups.iter().enumerate() {
      if !grp.is_empty() {
        out.push(u8::try_from(i).unwrap_or(0));
        out.push(u8::try_from(grp.len()).unwrap_or(0));
        for u in *grp {
          out.extend_from_slice(&u.to_bytes());
        }
      }
    }
    out.extend_from_slice(&self.footer);
    out
  }

  /// Returns true if this unit collection represents an active mid-battle bookmark save.
  #[must_use]
  pub fn is_bookmark(&self) -> bool {
    !self.blue.is_empty() || !self.red.is_empty()
  }

  /// Checks if the given battle grid tile coordinates are occupied by any combatant.
  #[must_use]
  pub fn is_tile_occupied(&self, coords: [u8; 2]) -> bool {
    let check = |units: &[unit::Unit]| {
      units
        .iter()
        .any(|u| u.battle_state().is_some_and(|bs| bs.coordinates == coords))
    };
    check(&self.blue)
      || check(&self.red)
      || check(&self.green)
      || check(&self.other)
  }

  /// Returns a slice of deployed player combatants on battle maps.
  #[must_use]
  pub fn blue(&self) -> &[unit::Unit] {
    &self.blue
  }

  /// Returns a slice of enemy combatants on battle maps.
  #[must_use]
  pub fn red(&self) -> &[unit::Unit] {
    &self.red
  }

  /// Returns a slice of allied/partner combatants on battle maps.
  #[must_use]
  pub fn green(&self) -> &[unit::Unit] {
    &self.green
  }

  /// Returns a slice of main campaign army units.
  #[must_use]
  pub fn main(&self) -> &[unit::Unit] {
    &self.main
  }

  /// Returns a slice of fallen units in Classic mode.
  #[must_use]
  pub fn dead(&self) -> &[unit::Unit] {
    &self.dead
  }

  /// Returns a slice of other mission combatants.
  #[must_use]
  pub fn other(&self) -> &[unit::Unit] {
    &self.other
  }

  /// Deploys a unit from camp reserves onto the battle grid in a bookmark save.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::IndexOutOfBounds`] if `roster_idx >= self.main.len()`.
  /// - Returns [`Error::CoordinatesOutOfBounds`] if coordinates exceed 32.
  /// - Returns [`Error::TileCollision`] if the target tile is occupied.
  pub fn deploy_unit(
    &mut self,
    roster_idx: usize,
    coordinates: [u8; 2],
  ) -> Result<(), Error> {
    if roster_idx >= self.main.len() {
      return Err(Error::IndexOutOfBounds {
        index: roster_idx,
        len: self.main.len(),
      });
    }
    if coordinates[0] > 32 || coordinates[1] > 32 {
      return Err(Error::CoordinatesOutOfBounds {
        x: coordinates[0],
        y: coordinates[1],
      });
    }
    if self.is_tile_occupied(coordinates) {
      return Err(Error::TileCollision {
        x: coordinates[0],
        y: coordinates[1],
      });
    }
    let mut u = self.main[roster_idx].clone();
    let hp = u.hp.total();
    u.set_battle_state(Some(unit::battle::BattleState::new(
      hp,
      0,
      coordinates,
      [0, 0],
    )));
    self.blue.push(u);
    Ok(())
  }

  /// Moves a deployed player combatant back to camp reserves.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::IndexOutOfBounds`] if `blue_idx >= self.blue.len()`.
  /// - Returns [`Error::ForceDeployUnitCannotBeUndeployed`] if character is Chrom or Robin.
  pub fn undeploy_unit(&mut self, blue_idx: usize) -> Result<(), Error> {
    if blue_idx >= self.blue.len() {
      return Err(Error::IndexOutOfBounds {
        index: blue_idx,
        len: self.blue.len(),
      });
    }
    let char_id = self.blue[blue_idx].character();
    if char_id.is_force_deployed() {
      return Err(Error::ForceDeployUnitCannotBeUndeployed {
        character: char_id,
      });
    }
    self.blue.remove(blue_idx);
    Ok(())
  }

  /// Moves a deployed player combatant to new grid coordinates.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::IndexOutOfBounds`] if `blue_idx >= self.blue.len()`.
  /// - Returns [`Error::CoordinatesOutOfBounds`] if coordinates exceed 32.
  /// - Returns [`Error::TileCollision`] if the target tile is occupied.
  pub fn move_deployed_unit(
    &mut self,
    blue_idx: usize,
    coordinates: [u8; 2],
  ) -> Result<(), Error> {
    if blue_idx >= self.blue.len() {
      return Err(Error::IndexOutOfBounds {
        index: blue_idx,
        len: self.blue.len(),
      });
    }
    if coordinates[0] > 32 || coordinates[1] > 32 {
      return Err(Error::CoordinatesOutOfBounds {
        x: coordinates[0],
        y: coordinates[1],
      });
    }
    if self.is_tile_occupied(coordinates) {
      return Err(Error::TileCollision {
        x: coordinates[0],
        y: coordinates[1],
      });
    }
    let u = &mut self.blue[blue_idx];
    if let Some(bs) = u.battle_state_mut() {
      bs.coordinates = coordinates;
    }
    Ok(())
  }

  /// Spawns a new enemy combatant onto the active battle grid.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::CoordinatesOutOfBounds`] if coordinates exceed 32.
  /// - Returns [`Error::TileCollision`] if the target tile is occupied.
  pub fn add_enemy(
    &mut self,
    character: unit::Character,
    class: unit::Class,
    coordinates: [u8; 2],
  ) -> Result<usize, Error> {
    if coordinates[0] > 32 || coordinates[1] > 32 {
      return Err(Error::CoordinatesOutOfBounds {
        x: coordinates[0],
        y: coordinates[1],
      });
    }
    if self.is_tile_occupied(coordinates) {
      return Err(Error::TileCollision {
        x: coordinates[0],
        y: coordinates[1],
      });
    }
    let mut u = unit::Unit::new(character, class, 0);
    let hp = u.hp.total();
    u.set_battle_state(Some(unit::battle::BattleState::new(
      hp,
      0,
      coordinates,
      [0, 0],
    )));
    self.red.push(u);
    Ok(self.red.len() - 1)
  }

  /// Removes an enemy combatant from the active battle grid by index.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::IndexOutOfBounds`] if `index >= self.red.len()`.
  /// - Returns [`Error::CannotRemoveChapterBoss`] if character is a chapter boss.
  /// - Returns [`Error::CannotRemoveRecruitmentTarget`] if character is a recruitable enemy.
  pub fn remove_enemy(&mut self, index: usize) -> Result<(), Error> {
    if index >= self.red.len() {
      return Err(Error::IndexOutOfBounds {
        index,
        len: self.red.len(),
      });
    }
    let char_id = self.red[index].character();
    if char_id.is_story_boss() {
      return Err(Error::CannotRemoveChapterBoss { character: char_id });
    }
    if char_id.is_recruitable_enemy() {
      return Err(Error::CannotRemoveRecruitmentTarget { character: char_id });
    }
    self.red.remove(index);
    Ok(())
  }

  /// Spawns a new allied/partner combatant onto the active battle grid.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::CoordinatesOutOfBounds`] if coordinates exceed 32.
  /// - Returns [`Error::TileCollision`] if the target tile is occupied.
  pub fn add_ally(
    &mut self,
    character: unit::Character,
    class: unit::Class,
    coordinates: [u8; 2],
  ) -> Result<usize, Error> {
    if coordinates[0] > 32 || coordinates[1] > 32 {
      return Err(Error::CoordinatesOutOfBounds {
        x: coordinates[0],
        y: coordinates[1],
      });
    }
    if self.is_tile_occupied(coordinates) {
      return Err(Error::TileCollision {
        x: coordinates[0],
        y: coordinates[1],
      });
    }
    let mut u = unit::Unit::new(character, class, 0);
    let hp = u.hp.total();
    u.set_battle_state(Some(unit::battle::BattleState::new(
      hp,
      0,
      coordinates,
      [0, 0],
    )));
    self.green.push(u);
    Ok(self.green.len() - 1)
  }

  /// Removes an allied combatant from the active battle grid by index.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::IndexOutOfBounds`] if `index >= self.green.len()`.
  /// - Returns [`Error::CannotRemoveProtectedStoryUnit`] if character is a protected NPC.
  pub fn remove_ally(&mut self, index: usize) -> Result<(), Error> {
    if index >= self.green.len() {
      return Err(Error::IndexOutOfBounds {
        index,
        len: self.green.len(),
      });
    }
    let char_id = self.green[index].character();
    if char_id.is_protected_story_npc() {
      return Err(Error::CannotRemoveProtectedStoryUnit { character: char_id });
    }
    self.green.remove(index);
    Ok(())
  }

  /// Records the death or retreat of a deployed combatant in a bookmark save.
  ///
  /// # Errors
  ///
  /// - Returns [`Error::IndexOutOfBounds`] if `blue_idx >= self.blue.len()`.
  /// - Returns [`Error::CannotKillStoryAnchor`] if character is a story anchor.
  pub fn kill_deployed_unit(
    &mut self,
    blue_idx: usize,
    is_casual: bool,
  ) -> Result<(), Error> {
    if blue_idx >= self.blue.len() {
      return Err(Error::IndexOutOfBounds {
        index: blue_idx,
        len: self.blue.len(),
      });
    }
    let char_id = self.blue[blue_idx].character();
    if char_id.is_story_anchor() {
      return Err(Error::CannotKillStoryAnchor { character: char_id });
    }
    let mut fallen = self.blue.remove(blue_idx);
    if is_casual {
      // In Casual mode, unit retreats to reserves
      fallen.set_battle_state(None);
    } else {
      // In Classic mode, unit is marked dead
      fallen.set_battle_state(None);
      self.dead.push(fallen);
    }
    Ok(())
  }
}

/// Errors that can occur during battle group combatant mutations.
#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  /// Index out of bounds.
  #[error("combatant index {index} is out of bounds (len {len})")]
  IndexOutOfBounds {
    /// Target index.
    index: usize,
    /// Vector length.
    len: usize,
  },
  /// Map grid coordinate out of bounds.
  #[error("coordinates [{x}, {y}] exceed map grid limits [32, 32]")]
  CoordinatesOutOfBounds {
    /// X coordinate.
    x: u8,
    /// Y coordinate.
    y: u8,
  },
  /// Tile collision.
  #[error("tile [{x}, {y}] is already occupied by another combatant")]
  TileCollision {
    /// X coordinate.
    x: u8,
    /// Y coordinate.
    y: u8,
  },
  /// Boss removal protection.
  #[error(
    "cannot remove chapter boss '{character}' in 'Defeat Boss' objective"
  )]
  CannotRemoveChapterBoss {
    /// Boss character.
    character: unit::Character,
  },
  /// Recruitment target removal protection.
  #[error("cannot remove recruitable enemy '{character}'")]
  CannotRemoveRecruitmentTarget {
    /// Recruitable character.
    character: unit::Character,
  },
  /// Protected friendly NPC removal protection.
  #[error("cannot remove story-protected friendly unit '{character}'")]
  CannotRemoveProtectedStoryUnit {
    /// Protected NPC character.
    character: unit::Character,
  },
  /// Force-deployed unit undeploy protection.
  #[error("mandatory force-deploy unit '{character}' cannot be undeployed")]
  ForceDeployUnitCannotBeUndeployed {
    /// Force deployed character.
    character: unit::Character,
  },
  /// Story anchor permadeath protection.
  #[error("story anchor unit '{character}' cannot be killed")]
  CannotKillStoryAnchor {
    /// Story anchor character.
    character: unit::Character,
  },
}
