//! End-to-end integration tests for CLI verbosity flags.

use std::process;

const FIXTURE: &str = "../../tests/fixtures/20200831-225628/Chapter0";

#[test]
fn test_verbose_and_quiet_flags() {
  let bin = env!("CARGO_BIN_EXE_fedit");

  let out_v = process::Command::new(bin)
    .args(["-v", "inspect", FIXTURE])
    .output()
    .expect("fedit -v inspect");
  assert!(out_v.status.success(), "verbose inspect ok");

  let out_q = process::Command::new(bin)
    .args(["-q", "inspect", FIXTURE])
    .output()
    .expect("fedit -q inspect");
  assert!(out_q.status.success(), "quiet inspect ok");
}
