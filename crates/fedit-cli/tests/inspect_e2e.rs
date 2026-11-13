//! End-to-end integration tests for `fedit inspect`.

use std::process;

const CHAPTER0: &str = "../../tests/fixtures/20200831-225628/Chapter0";
const GLOBAL: &str = "../../tests/fixtures/20200831-225628/Global";
const FIXTURE_DIR: &str = "../../tests/fixtures/20200831-225628";

#[test]
fn test_inspect_slot_save() {
  let output = process::Command::new(env!("CARGO_BIN_EXE_fedit"))
    .arg("inspect")
    .arg(CHAPTER0)
    .output()
    .expect("fedit inspect execution");
  assert!(output.status.success(), "command exit status");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("Slot Save"), "slot save kind");
  assert!(stdout.contains("Robin"), "avatar name");
  assert!(stdout.contains("Chapter 19"), "chapter title");
}

#[test]
fn test_inspect_global_save() {
  let output = process::Command::new(env!("CARGO_BIN_EXE_fedit"))
    .arg("inspect")
    .arg(GLOBAL)
    .output()
    .expect("fedit inspect execution");
  assert!(output.status.success(), "command exit status");
  let stdout = String::from_utf8_lossy(&output.stdout);
  assert!(stdout.contains("Global"), "global kind");
  assert!(stdout.contains("Valid"), "crc32 valid");
}

#[test]
fn test_inspect_archive_directory_rejected() {
  let status = process::Command::new(env!("CARGO_BIN_EXE_fedit"))
    .arg("inspect")
    .arg(FIXTURE_DIR)
    .status()
    .expect("fedit inspect directory");
  assert!(!status.success(), "directory inspection rejected");
}
