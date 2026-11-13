//! End-to-end integration tests for `fedit convoy`.

use std::fs;
use std::process;

const FIXTURE: &str = "../../tests/fixtures/20200831-225628/Chapter0";

#[test]
fn test_convoy_cli() {
  let bin = env!("CARGO_BIN_EXE_fedit");
  let out = process::Command::new(bin)
    .args(["convoy", FIXTURE])
    .output()
    .expect("fedit convoy table");
  assert!(out.status.success(), "table status ok");
  assert!(String::from_utf8_lossy(&out.stdout).contains("Slot  Item ID  Uses"));

  let j_out = process::Command::new(bin)
    .args(["convoy", FIXTURE, "--json"])
    .output()
    .expect("fedit convoy json");
  assert!(j_out.status.success(), "json status ok");
  assert!(String::from_utf8_lossy(&j_out.stdout).contains("\"slot\": 0"));

  let tmp = std::env::temp_dir().join("fedit_convoy_test");
  let _ = fs::remove_dir_all(&tmp);
  fs::create_dir_all(&tmp).expect("create tempdir");
  let file = tmp.join("Chapter0");
  fs::copy(FIXTURE, &file).expect("copy fixture");
  let m_out = process::Command::new(bin)
    .args([
      "convoy",
      file.to_str().unwrap(),
      "--slot",
      "0",
      "--uses",
      "25",
      "--backup",
    ])
    .output()
    .expect("fedit convoy mutate");
  assert!(m_out.status.success(), "mutate status ok");
  assert!(file.with_extension("bak").exists(), "backup exists");

  let dir_out = process::Command::new(bin)
    .args(["convoy", tmp.to_str().unwrap()])
    .status()
    .expect("convoy dir");
  assert!(!dir_out.success(), "directory rejected");

  let _ = fs::remove_dir_all(&tmp);
}
