//! Integration tests for save slot parsing, mutation, and roundtrips.

use fe13;

const CHAPTER0: &[u8] =
  include_bytes!("../../../tests/fixtures/20200831-225628/Chapter0");

#[test]
fn test_chapter0_save_slot_roundtrip() {
  let mut slot = fe13::SaveSlot::from_bytes(CHAPTER0).expect("parse slot");
  assert_eq!(slot.preview().avatar_name(), "Robin");
  assert_eq!(slot.preview().chapter_title(), "Chapter 19");
  assert_eq!(slot.preview().difficulty(), Some(fe13::Difficulty::Hard));
  assert_eq!(slot.units().count(), 20);

  slot.set_gold(999_999);
  assert_eq!(slot.preview().gold(), 999_999);
  assert_eq!(slot.user().gold(), 999_999);

  let repacked = slot.to_bytes().expect("repack slot");
  let reloaded = fe13::SaveSlot::from_bytes(&repacked).expect("reload slot");
  assert_eq!(reloaded.preview().avatar_name(), "Robin");
  assert_eq!(reloaded.preview().chapter_title(), "Chapter 19");
  assert_eq!(
    reloaded.preview().difficulty(),
    Some(fe13::Difficulty::Hard)
  );
  assert_eq!(reloaded.preview().gold(), 999_999);
  assert_eq!(reloaded.user().gold(), 999_999);
  assert_eq!(reloaded.units().count(), 20);
}
