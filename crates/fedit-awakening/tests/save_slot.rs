//! Integration tests for save slot parsing, mutation, and roundtrips.

use fedit_awakening as awakening;

const CHAPTER0: &[u8] =
  include_bytes!("../../../tests/fixtures/20200831-225628/Chapter0");
const MAP0: &[u8] =
  include_bytes!("../../../tests/fixtures/20200831-225628/Map0");

#[test]
fn test_chapter0_save_slot_roundtrip() {
  let mut slot = awakening::SaveSlot::from_bytes(CHAPTER0).expect("parse slot");
  assert_eq!(slot.preview().avatar_name(), "Robin");
  assert_eq!(slot.preview().chapter_title(), "Chapter 19: The Conqueror");
  assert_eq!(
    slot.preview().difficulty(),
    Some(awakening::types::Difficulty::Hard)
  );
  assert_eq!(slot.roster.len(), 25);
  assert_eq!(slot.convoy.len(), 29);
  assert_eq!(slot.gold(), 697_504);

  slot.set_gold(999_999).expect("set gold");
  assert!(slot.set_gold(1_000_000).is_err());
  let frederick = slot
    .find_unit_by_name("frederick")
    .expect("frederick exists");

  assert_eq!(
    frederick.character(),
    awakening::roster::Character::Frederick
  );
  assert_eq!(frederick.class(), awakening::roster::Class::GreatKnight);
  assert!(frederick.level() >= 1);
  assert_eq!(frederick.status(), awakening::roster::UnitStatus::Active);

  let chrom = slot.find_unit_by_name("chrom").expect("chrom exists");
  assert_eq!(chrom.character(), awakening::roster::Character::Chrom);
  assert_eq!(chrom.class(), awakening::roster::Class::GreatLord);
  assert_eq!(chrom.status(), awakening::roster::UnitStatus::Active);

  let cordelia = slot.find_unit_by_name("cordelia").expect("cordelia exists");
  assert_eq!(cordelia.character(), awakening::roster::Character::Cordelia);
  assert_eq!(cordelia.class(), awakening::roster::Class::PegasusKnight);
  assert_eq!(cordelia.status(), awakening::roster::UnitStatus::Active);

  let olivia = slot.find_unit_by_name("olivia").expect("olivia exists");
  assert_eq!(olivia.character(), awakening::roster::Character::Olivia);
  assert_eq!(olivia.class(), awakening::roster::Class::Dancer);
  assert_eq!(olivia.status(), awakening::roster::UnitStatus::Active);

  slot.roster[0].stats.hp = 99;
  slot.roster[0].stats.strength = 50;
  slot.convoy[0].uses = 42;
  assert_eq!(slot.gold(), 999_999);
  assert_eq!(slot.user().gold(), 999_999);

  let repacked = slot.to_bytes().expect("repack slot");
  let reloaded =
    awakening::SaveSlot::from_bytes(&repacked).expect("reload slot");
  assert_eq!(reloaded.preview().avatar_name(), "Robin");
  assert_eq!(
    reloaded.preview().chapter_title(),
    "Chapter 19: The Conqueror"
  );
  assert_eq!(
    reloaded.preview().difficulty(),
    Some(awakening::types::Difficulty::Hard)
  );
  assert_eq!(reloaded.gold(), 999_999);
  assert_eq!(reloaded.user().gold(), 999_999);
  assert_eq!(reloaded.roster.len(), 25);
  assert_eq!(reloaded.roster[0].stats.hp, 99);
  assert_eq!(reloaded.roster[0].stats.strength, 50);
  assert_eq!(reloaded.convoy.len(), 29);
  assert_eq!(reloaded.convoy[0].uses, 42);
}

#[test]
fn test_map0_save_slot_roundtrip() {
  let slot = awakening::SaveSlot::from_bytes(MAP0).expect("parse map0 slot");
  assert_eq!(slot.roster.len(), 11);
  assert_eq!(slot.convoy.len(), 1);
  let repacked = slot.to_bytes().expect("repack map0");
  let reloaded =
    awakening::SaveSlot::from_bytes(&repacked).expect("reload map0");
  assert_eq!(reloaded.roster.len(), 11);
  assert_eq!(reloaded.convoy.len(), 1);
}

#[test]
fn test_casual_mode_and_story_anchor_safeguards() {
  let mut slot = awakening::SaveSlot::from_bytes(CHAPTER0).expect("parse slot");
  // Chapter0 is Classic mode
  assert_eq!(
    slot.preview().mode(),
    Some(awakening::types::GameMode::Classic)
  );

  // Chrom is story anchor, cannot be set to Dead
  let chrom_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Chrom)
    .expect("chrom index");
  assert!(
    slot
      .set_unit_status(chrom_idx, awakening::roster::UnitStatus::Dead)
      .is_err()
  );

  // Mark Virion as Dead in Classic mode
  let virion_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Virion)
    .expect("virion index");
  slot
    .set_unit_status(virion_idx, awakening::roster::UnitStatus::Dead)
    .unwrap();
  assert_eq!(
    slot.roster[virion_idx].status(),
    awakening::roster::UnitStatus::Dead
  );

  // Switch to Casual mode
  slot.set_mode(awakening::types::GameMode::Casual);
  assert_eq!(
    slot.preview().mode(),
    Some(awakening::types::GameMode::Casual)
  );

  // Switching to Casual mode must have revived Virion
  let virion = &slot.roster[virion_idx];
  assert_eq!(virion.status(), awakening::roster::UnitStatus::Active);
  assert!(virion.stats.hp > 0);
  assert!(virion.stats.strength > 0);

  // In Casual mode, marking any unit Dead must be forbidden
  let res =
    slot.set_unit_status(virion_idx, awakening::roster::UnitStatus::Dead);
  assert!(res.is_err());
}

#[test]
fn test_availability_abstraction_and_force_deploy_protection() {
  let mut slot = awakening::SaveSlot::from_bytes(CHAPTER0).expect("parse slot");
  let chrom_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Chrom)
    .expect("chrom index");
  // Chrom cannot be made inactive
  assert!(slot.set_unit_active(chrom_idx, false).is_err());

  // Frederick can be made inactive (Classic mode -> Dead)
  let fred_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Frederick)
    .expect("fred index");
  assert!(slot.set_unit_active(fred_idx, false).is_ok());
  assert_eq!(
    slot.roster[fred_idx].availability(),
    awakening::roster::unit::Availability::Inactive
  );
  assert_eq!(
    slot.roster[fred_idx].status(),
    awakening::roster::UnitStatus::Dead
  );

  // Reactivating Frederick revives him
  assert!(slot.set_unit_active(fred_idx, true).is_ok());
  assert_eq!(
    slot.roster[fred_idx].availability(),
    awakening::roster::unit::Availability::Active
  );
  assert_eq!(
    slot.roster[fred_idx].status(),
    awakening::roster::UnitStatus::Active
  );
  assert!(slot.roster[fred_idx].stats.hp > 0);
  assert!(slot.roster[fred_idx].stats.strength > 0);
}

#[test]
fn test_chapter_rollback_dead_recruitment_conflict() {
  let mut slot = awakening::SaveSlot::from_bytes(CHAPTER0).expect("parse slot");
  let cordelia_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Cordelia)
    .expect("cordelia index");
  let olivia_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Olivia)
    .expect("olivia index");

  // Deactivate Cordelia (recruited Ch 7) and Olivia (recruited Ch 11)
  assert!(slot.set_unit_active(cordelia_idx, false).is_ok());
  assert!(slot.set_unit_active(olivia_idx, false).is_ok());

  // Reverting to Chapter 6 should be hard-blocked due to Cordelia
  assert!(slot.set_chapter(6).is_err());

  // Reverting to Chapter 10 should be hard-blocked due to Olivia (recruited Ch 11)
  assert!(slot.set_chapter(10).is_err());

  // Target out of bounds
  assert!(slot.set_chapter(27).is_err());

  // Setting to Chapter 15 should succeed (both were recruited before 15)
  assert!(slot.set_chapter(15).is_ok());

  // Revive all dead units recruited >= 6
  for i in 0..slot.roster.len() {
    if slot.roster[i].availability()
      == awakening::roster::unit::Availability::Inactive
      && slot.roster[i].character().recruitment_chapter() >= Some(6)
    {
      assert!(slot.set_unit_active(i, true).is_ok());
    }
  }

  // Now reverting to Chapter 6 must succeed without conflicts!
  assert!(slot.set_chapter(6).is_ok());
}

#[test]
fn test_avatar_customization_roundtrip_and_safeguards() {
  let mut slot = awakening::SaveSlot::from_bytes(CHAPTER0).expect("parse slot");
  let mut custom = slot
    .avatar_customization()
    .expect("read avatar customization");
  assert_eq!(custom.name, "Robin");
  assert_eq!(
    custom.gender,
    awakening::roster::unit::character::Gender::Female
  );

  // Asset and flaw cannot be identical
  let mut invalid_custom = custom.clone();
  invalid_custom.flaw = invalid_custom.asset;
  let err = slot.set_avatar_customization(&invalid_custom).unwrap_err();
  assert!(matches!(err, awakening::error::Error::StatFocusConflict));

  // Reclass Robin (Female) to Pegasus Knight (Female-only class)
  let robin_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Robin)
    .expect("robin idx");
  slot.roster[robin_idx]
    .set_class(awakening::roster::Class::PegasusKnight)
    .expect("reclass robin female to pegasus knight");

  // Attempting to set avatar customization to Male must fail because
  // Robin is currently in a Female-only class (Pegasus Knight)
  let mut male_custom = custom.clone();
  male_custom.gender = awakening::roster::unit::character::Gender::Male;
  let err = slot.set_avatar_customization(&male_custom).unwrap_err();
  assert!(matches!(
    err,
    awakening::error::Error::GenderIncompatibleClass { .. }
  ));

  // Switch Robin to Grandmaster (Universal class)
  slot.roster[robin_idx]
    .set_class(awakening::roster::Class::Grandmaster)
    .expect("reclass robin to grandmaster");

  // Now changing gender to Male succeeds
  custom.name = "Strategist".to_string();
  custom.gender = awakening::roster::unit::character::Gender::Male;
  custom.hair_color = [12, 34, 56];
  slot
    .set_avatar_customization(&custom)
    .expect("set avatar customization");

  assert_eq!(slot.preview().avatar_name(), "Strategist");
  let robin = slot.find_unit_by_name("robin").expect("robin in roster");
  assert_eq!(
    robin.gender(),
    awakening::roster::unit::character::Gender::Male
  );
}

#[test]
fn test_paralogue_lifecycle_and_child_safeguards() {
  let mut slot = awakening::SaveSlot::from_bytes(CHAPTER0).expect("parse slot");
  // Paralogues 1..=4 are accessible
  assert_eq!(
    slot.paralogue_state(1).expect("paralogue 1"),
    awakening::slot::ParalogueState::Unlocked
  );

  // Chapter 19 save has Lucina in roster
  let lucina_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Lucina);
  assert!(lucina_idx.is_some());

  // Test child paralogue 5 (Owain, mother Lissa)
  // Before chapter 13, child paralogues cannot be unlocked
  slot.set_chapter(10).expect("set chapter 10");
  let err = slot
    .set_paralogue_state(5, awakening::slot::ParalogueState::Unlocked)
    .unwrap_err();
  assert!(matches!(
    err,
    awakening::error::Error::ChildParalogueGatedByChapter13 { paralogue: 5 }
  ));

  // Advance back to Chapter 19
  slot.set_chapter(19).expect("set chapter 19");

  // Attempting to unlock paralogue 5 while Lissa is unmarried must fail
  assert_eq!(
    slot.child_parents(awakening::roster::Character::Owain),
    (Some(awakening::roster::Character::Lissa), None)
  );
  let err = slot
    .set_paralogue_state(5, awakening::slot::ParalogueState::Unlocked)
    .unwrap_err();
  assert!(matches!(
    err,
    awakening::error::Error::ParentsNotMarried { .. }
  ));

  // Marry Lissa to Frederick via set_child_father
  slot
    .set_child_father(
      awakening::roster::Character::Owain,
      awakening::roster::Character::Frederick,
    )
    .expect("marry parents");
  assert_eq!(
    slot.child_parents(awakening::roster::Character::Owain),
    (
      Some(awakening::roster::Character::Lissa),
      Some(awakening::roster::Character::Frederick)
    )
  );

  // Unlocking paralogue 5 now succeeds
  slot
    .set_paralogue_state(5, awakening::slot::ParalogueState::Unlocked)
    .expect("unlock paralogue 5");
  assert_eq!(
    slot.paralogue_state(5).expect("p5 state"),
    awakening::slot::ParalogueState::Unlocked
  );

  // Complete paralogue 5
  slot
    .set_paralogue_state(5, awakening::slot::ParalogueState::Completed)
    .expect("complete paralogue 5");
  assert_eq!(
    slot.paralogue_state(5).expect("p5 state"),
    awakening::slot::ParalogueState::Completed
  );

  // If Owain is recruited into the roster, rolling back the completed
  // paralogue 5 to Locked or Unlocked is hard-blocked
  let owain = awakening::roster::Unit::new(
    awakening::roster::Character::Owain,
    awakening::roster::Class::Myrmidon,
    awakening::roster::UnitStats::default(),
    0,
  );
  slot.roster.push(owain);
  let err = slot
    .set_paralogue_state(5, awakening::slot::ParalogueState::Locked)
    .unwrap_err();
  assert!(matches!(
    err,
    awakening::error::Error::ChildInRosterParalogueConflict { .. }
  ));
}

#[test]
fn test_growths_and_weapon_ranks() {
  let mut slot = awakening::SaveSlot::from_bytes(CHAPTER0).expect("parse slot");
  let chrom_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Chrom)
    .expect("chrom index");

  slot.roster[chrom_idx]
    .set_growth(0, 150)
    .expect("set hp growth");
  slot.roster[chrom_idx]
    .set_weapon_rank(0, 75)
    .expect("set sword rank");
  assert_eq!(slot.roster[chrom_idx].growths[0], 150);
  assert_eq!(slot.roster[chrom_idx].weapon_ranks[0], 75);

  slot.roster[chrom_idx].max_growths();
  slot.roster[chrom_idx].max_weapon_ranks();
  assert_eq!(slot.roster[chrom_idx].growths, [255; 8]);
  assert_eq!(slot.roster[chrom_idx].weapon_ranks, [90; 6]);

  let repacked = slot.to_bytes().expect("repack slot");
  let reloaded =
    awakening::SaveSlot::from_bytes(&repacked).expect("reload slot");
  let reloaded_chrom = reloaded
    .find_unit_by_name("chrom")
    .expect("find reloaded chrom");
  assert_eq!(reloaded_chrom.growths, [255; 8]);
  assert_eq!(reloaded_chrom.weapon_ranks, [90; 6]);
}
