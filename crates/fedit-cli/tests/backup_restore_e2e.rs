//! End-to-end integration tests for `fedit backup` and `fedit restore`.

use std::env;
use std::fs;
use std::process;

const FIXTURE: &str =
  "../fedit-awakening/tests/fixtures/20200831-225628/Chapter0";

#[test]
fn test_cli_backup_and_restore() {
  let tmp = env::temp_dir().join("fedit_cli_backup_restore_test");
  let _ = fs::remove_dir_all(&tmp);
  fs::create_dir_all(&tmp).expect("create temp dir");

  let save = tmp.join("Chapter0");
  fs::copy(FIXTURE, &save).expect("copy save");
  let bin = env!("CARGO_BIN_EXE_fedit");

  // 1. Create a backup
  let backup_out = process::Command::new(bin)
    .arg("backup")
    .arg(&save)
    .output()
    .expect("run backup");
  assert!(backup_out.status.success(), "backup success");
  let s_bak = String::from_utf8_lossy(&backup_out.stdout);
  assert!(
    s_bak.contains("Snapshot backup created:"),
    "snapshot confirmed"
  );

  // 2. Corrupt/modify save file
  fs::write(&save, b"corrupted file content").expect("modify save");

  // 3. Restore to original
  let restore_out = process::Command::new(bin)
    .arg("restore")
    .arg(&save)
    .output()
    .expect("run restore");
  assert!(restore_out.status.success(), "restore success");
  let s_rest = String::from_utf8_lossy(&restore_out.stdout);
  assert!(s_rest.contains("Restored"), "restore message");

  // 4. Verify save file restored to original
  let restored_bytes = fs::read(&save).expect("read restored");
  let fixture_bytes = fs::read(FIXTURE).expect("read fixture");
  assert_eq!(restored_bytes, fixture_bytes);

  let _ = fs::remove_dir_all(&tmp);
}
