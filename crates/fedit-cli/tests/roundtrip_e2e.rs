//! End-to-end integration tests for `fedit unpack` and `repack`.

use std::env;
use std::fs;
use std::process;

const FIXTURE: &str = "../../tests/fixtures/20200831-225628/Chapter0";

#[test]
fn test_unpack_repack_roundtrip() {
  let tmp = env::temp_dir().join("fedit_roundtrip");
  let _ = fs::remove_dir_all(&tmp);
  fs::create_dir_all(&tmp).expect("create temp dir");
  let (p, prev, rep) = (tmp.join("p"), tmp.join("pr"), tmp.join("r"));
  let bin = env!("CARGO_BIN_EXE_fedit");

  let u = process::Command::new(bin)
    .args(["unpack", FIXTURE, "-o"])
    .arg(&p)
    .args(["-p"])
    .arg(&prev)
    .status()
    .expect("unpack");
  assert!(u.success(), "unpack ok");

  let r = process::Command::new(bin)
    .arg("repack")
    .arg(&p)
    .args(["-p"])
    .arg(&prev)
    .args(["-o"])
    .arg(&rep)
    .status()
    .expect("repack");
  assert!(r.success(), "repack ok");

  let out = process::Command::new(bin)
    .args(["inspect"])
    .arg(&rep)
    .output()
    .expect("inspect");
  assert!(out.status.success(), "inspect ok");
  let s = String::from_utf8_lossy(&out.stdout);
  assert!(s.contains("Robin") && s.contains("Valid"), "repacked data");
  let _ = fs::remove_dir_all(&tmp);
}
