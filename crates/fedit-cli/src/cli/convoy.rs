//! Convoy storage inspection command.

use std::fs;
use std::io;
use std::path;

use awakening;
use clap;
use serde_json;

use crate::cli::error;

/// Arguments for the `convoy` subcommand.
#[derive(Debug, clap::Args)]
pub struct ConvoyArgs {
  /// Path to save slot file.
  pub file: path::PathBuf,
  /// Target convoy item identifier to mutate.
  #[arg(long, visible_alias = "slot", alias = "item-id")]
  pub item: Option<usize>,
  /// New uses count for the target convoy item.
  #[arg(long)]
  pub uses: Option<u16>,
  /// Create a backup before mutating.
  #[arg(long)]
  pub backup: bool,
  /// Output convoy storage as JSON.
  #[arg(long)]
  pub json: bool,
}

impl ConvoyArgs {
  /// Executes the convoy command.
  ///
  /// # Errors
  ///
  /// Returns an error if the save file cannot be read or parsed.
  pub fn run(self) -> Result<(), error::Error> {
    if self.file.is_dir() {
      return Err(
        io::Error::new(
          io::ErrorKind::InvalidInput,
          "directories not supported; pass a single save slot file",
        )
        .into(),
      );
    }
    if let (Some(target), Some(uses)) = (self.item, self.uses) {
      if self.backup {
        let bak = self.file.with_extension("bak");
        fs::copy(&self.file, &bak)?;
        println!("Backup created at {}", bak.display());
      }
      let mut s = awakening::Save::from_bytes(&fs::read(&self.file)?)?;
      s.convoy.set_durability(target, uses).map_err(|e| {
        io::Error::new(io::ErrorKind::InvalidInput, e.to_string())
      })?;
      crate::atomic::write_atomic(&self.file, &s.to_bytes()?)?;
      println!("Updated convoy item {target} uses to {uses}");
      return Ok(());
    }
    let slot = awakening::Save::from_bytes(&fs::read(&self.file)?)?;
    if self.json {
      let rows: Vec<serde_json::Value> = slot
        .convoy
        .iter()
        .map(|it| {
          serde_json::json!({
            "item_id": it.id(),
            "name": it.item_type().name(),
            "uses": it.durability(),
          })
        })
        .collect();
      println!("{}", serde_json::to_string_pretty(&rows)?);
      return Ok(());
    }
    println!("Item ID  Uses  Name");
    println!("-----------------------------------");
    for it in &slot.convoy {
      println!(
        "{:7}   {:4}  {}",
        it.id(),
        it.durability(),
        it.item_type().name()
      );
    }
    Ok(())
  }
}
