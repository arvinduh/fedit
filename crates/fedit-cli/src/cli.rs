//! Command-line interface definitions and routing for `fedit`.
//!
//! Subcommands and arguments for inspecting, unpacking, repacking, and
//! editing Nintendo 3DS Fire Emblem: Awakening save files.

pub mod edit;
pub mod error;
pub mod inspect;
pub mod repack;
pub mod unpack;

use clap;
use clap::builder::styling;

/// Cargo-style terminal help styling.
pub const STYLES: styling::Styles = styling::Styles::styled()
  .header(styling::AnsiColor::Green.on_default().bold())
  .usage(styling::AnsiColor::Green.on_default().bold())
  .literal(styling::AnsiColor::Cyan.on_default().bold())
  .placeholder(styling::AnsiColor::Cyan.on_default());

/// Top-level command-line argument router for `fedit`.
#[derive(Debug, clap::Parser)]
#[command(
  name = "fedit",
  version,
  about = "Modular Nintendo 3DS Fire Emblem save toolchain",
  styles = STYLES
)]
#[expect(dead_code, reason = "wired in main process host commit")]
pub enum Args {
  /// Inspect save container headers, statistics, and checksums.
  Inspect(inspect::InspectArgs),
  /// Decompress save containers to disk.
  Unpack(unpack::UnpackArgs),
  /// Repack uncompressed save payloads into compressed containers.
  Repack(repack::RepackArgs),
  /// Mutate save slot parameters in-place.
  Edit(edit::EditArgs),
}

impl Args {
  /// Dispatches the chosen subcommand.
  ///
  /// # Errors
  ///
  /// Returns an error if subcommand execution fails.
  #[expect(dead_code, reason = "wired in main process host commit")]
  pub fn run(self) -> Result<(), error::Error> {
    match self {
      Self::Inspect(args) => args.run(),
      Self::Unpack(args) => args.run(),
      Self::Repack(args) => args.run(),
      Self::Edit(args) => args.run(),
    }
  }
}
