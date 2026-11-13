//! Integration tests for save slot parsing, mutation, and roundtrips.

use fe13;

const CHAPTER0: &[u8] =
  include_bytes!("../../../tests/fixtures/20200831-225628/Chapter0");
const MAP0: &[u8] =
  include_bytes!("../../../tests/fixtures/20200831-225628/Map0");

#[test]
fn test_chapter0_save_slot_roundtrip() {
  let mut slot = fe13::SaveSlot::from_bytes(CHAPTER0).expect("parse slot");
  assert_eq!(slot.preview().avatar_name(), "Robin");
  assert_eq!(slot.preview().chapter_title(), "Chapter 19: The Conqueror");
  assert_eq!(slot.preview().difficulty(), Some(fe13::Difficulty::Hard));
  assert_eq!(slot.units().count(), 20);
  assert_eq!(slot.roster.len(), 20);
  assert_eq!(slot.convoy.len(), 29);
  assert_eq!(slot.gold(), 697_504);

  slot.set_gold(999_999).expect("set gold");
  assert!(slot.set_gold(1_000_000).is_err());
  let frederick = slot.find_unit_by_name("frederick").expect("frederick exists");
  assert_eq!(frederick.unit_id.0, 3);
  slot.roster[0].stats.hp = 99;
  slot.roster[0].stats.strength = 50;
  slot.convoy[0].uses = 42;
  assert_eq!(slot.gold(), 999_999);
  assert_eq!(slot.user().gold(), 999_999);

  let repacked = slot.to_bytes().expect("repack slot");
  let reloaded = fe13::SaveSlot::from_bytes(&repacked).expect("reload slot");
  assert_eq!(reloaded.preview().avatar_name(), "Robin");
  assert_eq!(
    reloaded.preview().chapter_title(),
    "Chapter 19: The Conqueror"
  );
  assert_eq!(
    reloaded.preview().difficulty(),
    Some(fe13::Difficulty::Hard)
  );
  assert_eq!(reloaded.gold(), 999_999);
  assert_eq!(reloaded.user().gold(), 999_999);
  assert_eq!(reloaded.units().count(), 20);
  assert_eq!(reloaded.roster.len(), 20);
  assert_eq!(reloaded.roster[0].stats.hp, 99);
  assert_eq!(reloaded.roster[0].stats.strength, 50);
  assert_eq!(reloaded.convoy.len(), 29);
  assert_eq!(reloaded.convoy[0].uses, 42);
}

#[test]
fn test_map0_save_slot_roundtrip() {
  let slot = fe13::SaveSlot::from_bytes(MAP0).expect("parse map0 slot");
  assert_eq!(slot.units().count(), 6);
  assert_eq!(slot.roster.len(), 6);
  assert_eq!(slot.convoy.len(), 1);
  let repacked = slot.to_bytes().expect("repack map0");
  let reloaded = fe13::SaveSlot::from_bytes(&repacked).expect("reload map0");
  assert_eq!(reloaded.roster.len(), 6);
  assert_eq!(reloaded.convoy.len(), 1);
}
