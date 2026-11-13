//! Command-line process host for the `fedit` Fire Emblem save toolchain.
//!
//! Parses CLI arguments, configures logging, and maps application results to
//! process exit codes. Subcommands and domain logic are delegated to child
//! modules and library crates.

mod atomic;
mod cli;
pub mod error;

use std::process;

use clap::Parser;
use env_logger;
use log;

fn main() -> process::ExitCode {
  let args = cli::Cli::parse();
  init_logger(args.verbose, args.quiet);
  match args.command.run() {
    Ok(()) => process::ExitCode::SUCCESS,
    Err(err) => {
      eprintln!("[ERROR] {err}");
      process::ExitCode::FAILURE
    }
  }
}

/// Configures global log filter level based on verbosity flags.
fn init_logger(verbose: u8, quiet: bool) {
  let filter = if quiet {
    log::LevelFilter::Error
  } else {
    match verbose {
      0 => log::LevelFilter::Warn,
      1 => log::LevelFilter::Info,
      2 => log::LevelFilter::Debug,
      _ => log::LevelFilter::Trace,
    }
  };
  env_logger::Builder::from_default_env()
    .filter_level(filter)
    .init();
}
