//! Save file inspection command.
//!
//! Parses and validates Nintendo 3DS Fire Emblem save files, printing
//! header metadata, play statistics, and CRC-32 integrity status.

use std::fs;
use std::path;

use clap;
use fe_compress;
use fe13;
use fedit_core;

use crate::cli::error;

/// Arguments for the `inspect` subcommand.
#[derive(Debug, clap::Args)]
pub struct InspectArgs {
  /// Path to the save file or archive directory to inspect.
  pub file: path::PathBuf,
}

impl InspectArgs {
  /// Executes the inspect command.
  ///
  /// # Errors
  ///
  /// Returns an error if the save file cannot be read or validated.
  pub fn run(self) -> Result<(), error::Error> {
    if self.file.is_dir() {
      let archive = fedit_core::SaveArchive::open(&self.file)?;
      println!("Archive Root:      {}", archive.root.display());
      println!(
        "Global Status:     {}",
        if archive.global.is_some() {
          "Present"
        } else {
          "Missing"
        }
      );
      for (i, slot) in archive.chapters.iter().enumerate() {
        if let Some(s) = slot {
          let p = s.preview();
          println!(
            "Chapter {i}:         {} | {} | Gold: {}",
            p.avatar_name(),
            p.chapter_title(),
            s.gold()
          );
        }
      }
      for (i, slot) in archive.maps.iter().enumerate() {
        if let Some(s) = slot {
          println!(
            "Map {i}:             Units: {} | Convoy: {}",
            s.roster.len(),
            s.convoy.len()
          );
        }
      }
      return Ok(());
    }
    let bytes = fs::read(&self.file)?;
    if bytes.len() >= 4 && &bytes[0..4] == b"PMOC" {
      let decomp = fe_compress::unpack(&bytes)?;
      println!("Container Kind:    Global");
      println!("Decompressed Size: {} bytes", decomp.payload.len());
      println!("CRC-32:            Valid");
      return Ok(());
    }

    let slot = fe13::SaveSlot::from_bytes(&bytes)?;
    let p = slot.preview();
    let fields = [
      ("Container Kind:", "Slot Save".to_string()),
      ("Avatar:", p.avatar_name()),
      ("Chapter:", p.chapter_title()),
      (
        "Difficulty:",
        p.difficulty()
          .map_or_else(|| "Unknown".to_string(), |d| d.to_string()),
      ),
      (
        "Mode:",
        p.mode()
          .map_or_else(|| "Unknown".to_string(), |m| m.to_string()),
      ),
      ("Gold:", slot.gold().to_string()),
      ("Play Time:", p.play_time().to_string()),
      ("Roster Count:", slot.units().count().to_string()),
      ("CRC-32:", "Valid".to_string()),
    ];
    for (k, v) in fields {
      println!("{k:18} {v}");
    }
    Ok(())
  }
}
