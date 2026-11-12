//! Command-line process host for the `fedit` Fire Emblem save toolchain.
//!
//! Parses CLI arguments, configures logging, and maps application results to
//! process exit codes. Subcommands and domain logic are delegated to child
//! modules and library crates.

use std::process;

fn main() -> process::ExitCode {
  process::ExitCode::SUCCESS
}
