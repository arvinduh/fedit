//! Integration tests for save archive discovery and persistence.

use std::env;
use std::fs;

use fedit_core;

const FIXTURE_DIR: &str = "../../tests/fixtures/20200831-225628";

#[test]
fn test_archive_discovery_mutation_and_backup() {
  let tmp = env::temp_dir().join("fedit_archive_test");
  let _ = fs::remove_dir_all(&tmp);
  fs::create_dir_all(&tmp).expect("create temp dir");

  for name in ["Global", "Chapter0", "Chapter1", "Chapter2", "Map0", "Map1"] {
    let src = format!("{FIXTURE_DIR}/{name}");
    fs::copy(&src, tmp.join(name)).expect("copy fixture file");
  }

  let mut archive = fedit_core::SaveArchive::open(&tmp).expect("open archive");
  assert!(archive.global.is_some(), "global present");
  let c0 = archive.chapters[0].as_mut().expect("chapter 0");
  assert_eq!(c0.gold(), 697_504, "original gold");

  c0.set_gold(999_999);
  let backup = archive
    .save_archive(true)
    .expect("save archive")
    .expect("backup dir");
  assert!(backup.join("Chapter0").is_file(), "backup file created");

  let reloaded = fedit_core::SaveArchive::open(&tmp).expect("reopen archive");
  let reloaded_c0 = reloaded.chapters[0].as_ref().expect("reloaded c0");
  assert_eq!(reloaded_c0.gold(), 999_999, "mutated gold persisted");

  let orig = fedit_core::SaveArchive::open(&backup).expect("open backup");
  let orig_c0 = orig.chapters[0].as_ref().expect("backup c0");
  assert_eq!(orig_c0.gold(), 697_504, "original gold in backup");

  let _ = fs::remove_dir_all(&tmp);
  let _ = fs::remove_dir_all(&backup);
}
