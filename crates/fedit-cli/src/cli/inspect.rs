//! Save file inspection command.
//!
//! Parses and validates Nintendo 3DS Fire Emblem save files, printing
//! header metadata, play statistics, and CRC-32 integrity status.

use std::fs;
use std::path;

use clap;
use fe_compress;
use fe13;

use crate::cli::error;

/// Arguments for the `inspect` subcommand.
#[derive(Debug, clap::Args)]
pub struct InspectArgs {
  /// Path to the save file to inspect.
  pub file: path::PathBuf,
}

impl InspectArgs {
  /// Executes the inspect command.
  ///
  /// # Errors
  ///
  /// Returns an error if the save file cannot be read or validated.
  pub fn run(self) -> Result<(), error::Error> {
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
      ("Difficulty:", format!("{:?}", p.difficulty())),
      ("Mode:", format!("{:?}", p.mode())),
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
