//! End-to-end integration tests for `fedit roster`.

use std::process;

const FIXTURE: &str = "../../tests/fixtures/20200831-225628/Chapter0";

#[test]
fn test_roster_table_and_json() {
  let bin = env!("CARGO_BIN_EXE_fedit");
  let table_out = process::Command::new(bin)
    .args(["roster", FIXTURE])
    .output()
    .expect("fedit roster table");
  assert!(table_out.status.success(), "table status ok");
  let s = String::from_utf8_lossy(&table_out.stdout);
  assert!(s.contains("Idx  Unit Class"), "table header");

  let json_out = process::Command::new(bin)
    .args(["roster", FIXTURE, "--json"])
    .output()
    .expect("fedit roster json");
  assert!(json_out.status.success(), "json status ok");
  let js = String::from_utf8_lossy(&json_out.stdout);
  assert!(js.contains("\"unit_id\": 3"), "contains unit id 3");
  assert!(js.contains("\"hp\": 75"), "contains hp 75");
}
