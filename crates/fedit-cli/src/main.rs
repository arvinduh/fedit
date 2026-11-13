//! Command-line process host for the `fedit` Fire Emblem save toolchain.
//!
//! Parses CLI arguments, configures logging, and maps application results to
//! process exit codes. Subcommands and domain logic are delegated to child
//! modules and library crates.

mod atomic;
mod cli;

use std::process;

use clap::Parser;
use env_logger;

fn main() -> process::ExitCode {
  env_logger::init();
  match cli::Args::parse().run() {
    Ok(()) => process::ExitCode::SUCCESS,
    Err(err) => {
      eprintln!("[ERROR] {err}");
      process::ExitCode::FAILURE
    }
  }
}
