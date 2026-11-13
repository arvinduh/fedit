//! Command-line process host for the `fedit` Fire Emblem save toolchain.
//!
//! Parses CLI arguments, configures logging, and maps application results to
//! process exit codes. Subcommands and domain logic are delegated to child
//! modules and library crates.

mod cli;

use std::process;

fn main() -> process::ExitCode {
  let _ = size_of::<cli::error::Error>();
  process::ExitCode::SUCCESS
}
