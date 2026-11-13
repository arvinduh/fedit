//! End-to-end integration tests for `fedit unit`.

use std::fs;
use std::process;

const FIXTURE: &str = "../../tests/fixtures/20200831-225628/Chapter0";

#[test]
fn test_unit_cli_inspect_and_mutate() {
  let bin = env!("CARGO_BIN_EXE_fedit");
  let out = process::Command::new(bin)
    .args(["unit", FIXTURE, "--index", "0"])
    .output()
    .expect("fedit unit inspect");
  assert!(out.status.success(), "inspect status ok");
  assert!(String::from_utf8_lossy(&out.stdout).contains("Idx: 0"));

  let tmp = std::env::temp_dir().join("fedit_unit_test");
  let _ = fs::remove_dir_all(&tmp);
  fs::create_dir_all(&tmp).expect("create tempdir");
  let file = tmp.join("Chapter0");
  fs::copy(FIXTURE, &file).expect("copy fixture");

  let m_out = process::Command::new(bin)
    .args([
      "unit",
      file.to_str().unwrap(),
      "--index",
      "0",
      "--hp",
      "80",
      "--strength",
      "40",
    ])
    .output()
    .expect("fedit unit mutate");
  assert!(m_out.status.success(), "mutate status ok");
  assert!(file.with_extension("bak").exists(), "backup exists");

  let verify_out = process::Command::new(bin)
    .args(["unit", file.to_str().unwrap(), "--index", "0"])
    .output()
    .expect("fedit unit inspect after");
  assert!(String::from_utf8_lossy(&verify_out.stdout).contains("80"));
  let _ = fs::remove_dir_all(&tmp);
}
