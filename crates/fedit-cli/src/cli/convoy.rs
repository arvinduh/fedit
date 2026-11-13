//! Convoy storage inspection command.

use std::fs;
use std::path;

use clap;
use fe13;

use crate::cli::error;

/// Arguments for the `convoy` subcommand.
#[derive(Debug, clap::Args)]
pub struct ConvoyArgs {
  /// Path to save file.
  pub file: path::PathBuf,
}

impl ConvoyArgs {
  /// Executes the convoy command.
  ///
  /// # Errors
  ///
  /// Returns an error if the save file cannot be read or parsed.
  pub fn run(self) -> Result<(), error::Error> {
    let bytes = fs::read(&self.file)?;
    let slot = fe13::SaveSlot::from_bytes(&bytes)?;
    println!("Slot  Item ID  Uses");
    println!("-------------------");
    for it in &slot.convoy {
      println!("{:4}    {:5}   {:4}", it.slot, it.item_id.0, it.uses);
    }
    Ok(())
  }
}
