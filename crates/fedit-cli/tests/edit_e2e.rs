//! End-to-end integration tests for `fedit edit`.

use std::env;
use std::fs;
use std::process;

const FIXTURE: &str = "../../tests/fixtures/20200831-225628/Chapter0";
const FIXTURE_DIR: &str = "../../tests/fixtures/20200831-225628";

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
    .args(["--gold", "999999", "--avatar-name", "Morgan"])
    .status()
    .expect("edit");
  assert!(e.success(), "edit ok");

  let out = process::Command::new(bin)
    .args(["inspect"])
    .arg(&save)
    .output()
    .expect("inspect");
  assert!(out.status.success(), "inspect ok");
  let s = String::from_utf8_lossy(&out.stdout);
  assert!(s.contains("Morgan"), "avatar mutated");
  assert!(s.contains("999999"), "gold mutated");
  let _ = fs::remove_dir_all(&tmp);
}

#[test]
fn test_edit_archive_directory_with_backup() {
  let tmp = env::temp_dir().join("fedit_edit_dir_test");
  let _ = fs::remove_dir_all(&tmp);
  fs::create_dir_all(&tmp).expect("create temp dir");

  for name in ["Global", "Chapter0", "Chapter1", "Chapter2", "Map0", "Map1"] {
    fs::copy(format!("{FIXTURE_DIR}/{name}"), tmp.join(name))
      .expect("copy fixture");
  }

  let bin = env!("CARGO_BIN_EXE_fedit");
  let edit_status = process::Command::new(bin)
    .args(["edit"])
    .arg(&tmp)
    .args(["--chapter", "0", "--gold", "888888", "--avatar-name", "Morgan"])
    .status()
    .expect("edit directory");
  assert!(edit_status.success(), "edit success");

  let out = process::Command::new(bin)
    .args(["inspect"])
    .arg(tmp.join("Chapter0"))
    .output()
    .expect("inspect mutated slot");
  assert!(out.status.success(), "inspect success");
  let s = String::from_utf8_lossy(&out.stdout);
  assert!(s.contains("Morgan"), "mutated avatar persisted");
  assert!(s.contains("888888"), "mutated gold persisted");

  let backups_dir = tmp.join("backups");
  assert!(backups_dir.is_dir(), "backups directory was created");
  let has_backup = fs::read_dir(&backups_dir)
    .expect("read backups dir")
    .filter_map(Result::ok)
    .any(|e| e.path().join("Chapter0").is_file());
  assert!(has_backup, "backup chapter 0 file was created");

  let _ = fs::remove_dir_all(&tmp);
}
