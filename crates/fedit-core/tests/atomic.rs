//! Integration tests for atomic file persistence.

use std::env;
use std::fs;

use fedit_core;

#[test]
fn test_atomic_write_and_replace() {
  let tmp = env::temp_dir().join("fedit_atomic_test");
  let _ = fs::remove_dir_all(&tmp);
  fs::create_dir_all(&tmp).expect("create temp dir");

  let target = tmp.join("test_save.bin");
  fedit_core::write_atomic(&target, b"initial payload")
    .expect("first atomic write");
  assert_eq!(fs::read(&target).expect("read initial"), b"initial payload");

  fedit_core::write_atomic(&target, b"updated payload")
    .expect("second atomic write");
  assert_eq!(fs::read(&target).expect("read updated"), b"updated payload");

  let _ = fs::remove_dir_all(&tmp);
}
