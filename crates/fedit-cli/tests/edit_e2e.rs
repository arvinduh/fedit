//! End-to-end integration tests for `fedit edit`.

use std::env;
use std::fs;
use std::process;

const FIXTURE: &str = "../../tests/fixtures/20200831-225628/Chapter0";

#[test]
fn test_edit_save_slot() {
  let tmp = env::temp_dir().join("fedit_edit_test");
  let _ = fs::remove_dir_all(&tmp);
  fs::create_dir_all(&tmp).expect("create temp dir");

  let save = tmp.join("Chapter0");
  fs::copy(FIXTURE, &save).expect("copy save");
  let bin = env!("CARGO_BIN_EXE_fedit");

  let e = process::Command::new(bin)
    .args(["edit"])
    .arg(&save)
    .args([
      "--gold",
      "999999",
      "--avatar-name",
      "Morgan",
      "--play-time",
      "150:00:00",
      "--backup",
    ])
    .status()
    .expect("edit");
  assert!(e.success(), "edit ok");
  assert!(tmp.join("Chapter0.bak").is_file(), "backup created");

  let out = process::Command::new(bin)
    .args(["inspect"])
    .arg(&save)
    .output()
    .expect("inspect");
  assert!(out.status.success(), "inspect ok");
  let s = String::from_utf8_lossy(&out.stdout);
  assert!(s.contains("Morgan"), "avatar mutated");
  assert!(s.contains("999999"), "gold mutated");
  assert!(s.contains("150:00:00"), "play time mutated");
  let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_edit_archive_directory_rejected() {
  let tmp = env::temp_dir().join("fedit_edit_dir_test");
  let _ = fs::remove_dir_all(&tmp);
  fs::create_dir_all(&tmp).expect("create temp dir");

  let bin = env!("CARGO_BIN_EXE_fedit");
  let edit_status = process::Command::new(bin)
    .args(["edit"])
    .arg(&tmp)
    .args(["--gold", "888888"])
    .status()
    .expect("edit directory");
  assert!(!edit_status.success(), "directory editing rejected");

  let _ = fs::remove_dir_all(&tmp);
}
