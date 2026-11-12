//! Integration tests for PMOC container unpack and repack roundtrip.

use fe_compress;

const CHAPTER0: &[u8] =
  include_bytes!("../../../tests/fixtures/20200831-225628/Chapter0");
const GLOBAL: &[u8] =
  include_bytes!("../../../tests/fixtures/20200831-225628/Global");

#[test]
fn test_roundtrip_chapter0() {
  let c1 = fe_compress::unpack(CHAPTER0).expect("unpack chapter0");
  assert_eq!(c1.kind, fe_compress::ContainerKind::Slot, "kind is slot");
  assert_eq!(c1.preview.len(), 192, "preview length");
  assert_eq!(c1.payload.len(), 12_298, "payload length");
  let repacked =
    fe_compress::repack(&c1.preview, &c1.payload).expect("repack chapter0");
  let c2 = fe_compress::unpack(&repacked).expect("unpack repacked chapter0");
  assert_eq!(c1, c2, "roundtrip container matches");
}

#[test]
fn test_roundtrip_global() {
  let c1 = fe_compress::unpack(GLOBAL).expect("unpack global");
  assert_eq!(
    c1.kind,
    fe_compress::ContainerKind::Global,
    "kind is global"
  );
  assert!(c1.preview.is_empty(), "global preview is empty");
  assert_eq!(c1.payload.len(), 1_425, "payload length");
  let repacked =
    fe_compress::repack(&c1.preview, &c1.payload).expect("repack global");
  let c2 = fe_compress::unpack(&repacked).expect("unpack repacked global");
  assert_eq!(c1, c2, "roundtrip container matches");
}
