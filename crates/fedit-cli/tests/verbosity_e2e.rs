//! End-to-end integration tests for CLI verbosity flags.

use std::process;

const FIXTURE: &str =
  "../fedit-awakening/tests/fixtures/20200831-225628/Chapter0";

#[test]
fn test_verbose_and_quiet_flags() {
  let bin = env!("CARGO_BIN_EXE_fedit");

  let out_v = process::Command::new(bin)
    .args(["-v", "inspect", FIXTURE])
    .output()
    .expect("fedit -v inspect");
  assert!(out_v.status.success(), "verbose inspect ok");

  let out_vv = process::Command::new(bin)
    .args(["-vv", "inspect", FIXTURE])
    .output()
    .expect("fedit -vv inspect");
  assert!(out_vv.status.success(), "debug inspect ok");

  let out_vvv = process::Command::new(bin)
    .args(["-vvv", "inspect", FIXTURE])
    .output()
    .expect("fedit -vvv inspect");
  assert!(out_vvv.status.success(), "trace inspect ok");

  let out_q = process::Command::new(bin)
    .args(["-q", "inspect", FIXTURE])
    .output()
    .expect("fedit -q inspect");
  assert!(out_q.status.success(), "quiet inspect ok");
}
