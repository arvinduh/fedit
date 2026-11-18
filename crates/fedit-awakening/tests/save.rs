//! Integration tests for save parsing, mutation, and roundtrips.

use fedit_awakening as awakening;

const CHAPTER0: &[u8] = include_bytes!("fixtures/20200831-225628/Chapter0");
const MAP0: &[u8] = include_bytes!("fixtures/20200831-225628/Map0");

#[test]
fn test_chapter0_save_roundtrip() {
  let mut slot = awakening::Save::from_bytes(CHAPTER0).expect("parse slot");
  assert_eq!(slot.preview().avatar_name(), "Robin");
  assert_eq!(slot.preview().chapter_title(), "Chapter 19: The Conqueror");
  assert_eq!(
    slot.preview().difficulty(),
    Some(awakening::Difficulty::Hard)
  );
  assert_eq!(slot.roster.len(), 25);
  let sayri = slot.find_unit_by_name("say'ri").expect("say'ri exists");
  assert_eq!(sayri.hp.total(), 43);
  assert_eq!(sayri.strength.total(), 19);
  assert_eq!(sayri.magic.total(), 8);
  assert_eq!(sayri.skill.total(), 26);
  assert_eq!(sayri.speed.total(), 29);
  assert_eq!(sayri.luck.total(), 22);
  assert_eq!(sayri.defense.total(), 14);
  assert_eq!(sayri.resistance.total(), 12);
  assert_eq!(slot.convoy.len(), 55);
  assert_eq!(slot.gold(), 11_575);

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

  slot.roster[0].hp.set_total(99).expect("set hp");
  slot.roster[0].strength.set_total(44).expect("set strength");
  let first_item_id = usize::from(slot.convoy[0].id());
  slot
    .convoy
    .set_durability(first_item_id, 42)
    .expect("set durability");
  assert_eq!(slot.gold(), 999_999);
  assert_eq!(slot.user().gold(), 999_999);

  let repacked = slot.to_bytes().expect("repack slot");
  let reloaded = awakening::Save::from_bytes(&repacked).expect("reload slot");
  assert_eq!(reloaded.preview().avatar_name(), "Robin");
  assert_eq!(
    reloaded.preview().chapter_title(),
    "Chapter 19: The Conqueror"
  );
  assert_eq!(
    reloaded.preview().difficulty(),
    Some(awakening::Difficulty::Hard)
  );
  assert_eq!(reloaded.gold(), 999_999);
  assert_eq!(reloaded.user().gold(), 999_999);
  assert_eq!(reloaded.roster.len(), 25);
  assert_eq!(reloaded.roster[0].hp.total(), 99);
  assert_eq!(reloaded.roster[0].strength.total(), 44);
  assert_eq!(reloaded.convoy.len(), 55);
  assert_eq!(reloaded.convoy[0].durability(), 42);
}

#[test]
fn test_map0_save_roundtrip() {
  let slot = awakening::Save::from_bytes(MAP0).expect("parse map0 slot");
  assert_eq!(slot.roster.len(), 9);
  assert_eq!(slot.deployed_units().len(), 2);
  assert_eq!(slot.enemy_units().len(), 4);
  assert_eq!(slot.convoy.len(), 5);
  let repacked = slot.to_bytes().expect("repack map0");
  let reloaded = awakening::Save::from_bytes(&repacked).expect("reload map0");
  assert_eq!(reloaded.roster.len(), 9);
  assert_eq!(reloaded.deployed_units().len(), 2);
  assert_eq!(reloaded.enemy_units().len(), 4);
  assert_eq!(reloaded.convoy.len(), 5);
}

#[test]
fn test_casual_mode_and_story_anchor_safeguards() {
  let mut slot = awakening::Save::from_bytes(CHAPTER0).expect("parse slot");
  assert_eq!(slot.preview().mode(), Some(awakening::GameMode::Classic));

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

  slot.set_mode(awakening::GameMode::Casual);
  assert_eq!(slot.preview().mode(), Some(awakening::GameMode::Casual));

  let virion = &slot.roster[virion_idx];
  assert_eq!(virion.status(), awakening::roster::UnitStatus::Active);
  assert!(virion.hp.total() > 0);
  assert!(virion.strength.total() > 0);

  let res =
    slot.set_unit_status(virion_idx, awakening::roster::UnitStatus::Dead);
  assert!(res.is_err());
}

#[test]
fn test_availability_abstraction_and_force_deploy_protection() {
  let mut slot = awakening::Save::from_bytes(CHAPTER0).expect("parse slot");
  let chrom_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Chrom)
    .expect("chrom index");
  assert!(slot.set_unit_active(chrom_idx, false).is_err());

  let fred_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Frederick)
    .expect("fred index");
  assert!(slot.set_unit_active(fred_idx, false).is_ok());
  assert_eq!(
    slot.roster[fred_idx].availability(),
    awakening::roster::Availability::Inactive
  );
  assert_eq!(
    slot.roster[fred_idx].status(),
    awakening::roster::UnitStatus::Dead
  );

  assert!(slot.set_unit_active(fred_idx, true).is_ok());
  assert_eq!(
    slot.roster[fred_idx].availability(),
    awakening::roster::Availability::Active
  );
  assert_eq!(
    slot.roster[fred_idx].status(),
    awakening::roster::UnitStatus::Active
  );
  assert!(slot.roster[fred_idx].hp.total() > 0);
  assert!(slot.roster[fred_idx].strength.total() > 0);
}

#[test]
fn test_chapter_rollback_dead_recruitment_conflict() {
  let mut slot = awakening::Save::from_bytes(CHAPTER0).expect("parse slot");
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

  assert!(slot.set_unit_active(cordelia_idx, false).is_ok());
  assert!(slot.set_unit_active(olivia_idx, false).is_ok());

  assert!(slot.set_chapter(6).is_err());
  assert!(slot.set_chapter(10).is_err());
  assert!(slot.set_chapter(27).is_err());
  assert!(slot.set_chapter(15).is_ok());

  for i in 0..slot.roster.len() {
    if slot.roster[i].availability()
      == awakening::roster::Availability::Inactive
      && slot.roster[i].character().recruitment_chapter() >= Some(6)
    {
      assert!(slot.set_unit_active(i, true).is_ok());
    }
  }

  assert!(slot.set_chapter(6).is_ok());
}

#[test]
fn test_avatar_customization_roundtrip_and_safeguards() {
  let mut slot = awakening::Save::from_bytes(CHAPTER0).expect("parse slot");
  let mut custom = slot
    .avatar_customization()
    .expect("read avatar customization");
  assert_eq!(custom.name, "Robin");
  assert_eq!(custom.gender, awakening::roster::Gender::Female);

  let mut neutral_custom = custom.clone();
  neutral_custom.flaw = neutral_custom.asset;
  assert!(slot.set_avatar_customization(&neutral_custom).is_ok());

  let robin_idx = slot
    .roster
    .iter()
    .position(|u| u.character().is_robin())
    .expect("robin idx");
  slot.roster[robin_idx]
    .set_class(awakening::roster::Class::PegasusKnight)
    .expect("reclass robin female to pegasus knight");

  let mut male_custom = custom.clone();
  male_custom.gender = awakening::roster::Gender::Male;
  let err = slot.set_avatar_customization(&male_custom).unwrap_err();
  match err {
    awakening::save::Error::Unit(
      awakening::roster::UnitError::ClassGenderMismatch { .. },
    ) => {}
    other => panic!("expected ClassGenderMismatch, got {other:?}"),
  }

  slot.roster[robin_idx]
    .set_class(awakening::roster::Class::Grandmaster)
    .expect("reclass robin to grandmaster");

  custom.name = "Strategist".to_string();
  custom.gender = awakening::roster::Gender::Male;
  custom.hair_color = awakening::avatar::Rgb::new(12, 34, 56);
  slot
    .set_avatar_customization(&custom)
    .expect("set avatar customization");

  assert_eq!(slot.preview().avatar_name(), "Strategist");
  let robin = slot.find_unit_by_name("robin").expect("robin in roster");
  assert_eq!(robin.gender(), awakening::roster::Gender::Male);
}

#[test]
fn test_paralogue_lifecycle_and_child_safeguards() {
  let mut slot = awakening::Save::from_bytes(CHAPTER0).expect("parse slot");
  assert_eq!(
    slot.paralogue_state(1).expect("paralogue 1"),
    awakening::ParalogueState::Unlocked
  );

  let lucina_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Lucina);
  assert!(lucina_idx.is_some());

  slot.set_chapter(10).expect("set chapter 10");
  let err = slot
    .set_paralogue_state(5, awakening::ParalogueState::Unlocked)
    .unwrap_err();
  match err {
    awakening::save::Error::Paralogue(
      awakening::social::ParalogueError::ChildParalogueGatedByChapter13 {
        paralogue: 5,
      },
    ) => {}
    other => panic!("expected ChildParalogueGatedByChapter13, got {other:?}"),
  }

  slot.set_chapter(19).expect("set chapter 19");

  assert_eq!(
    slot.child_parents(awakening::roster::Character::Owain),
    (Some(awakening::roster::Character::Lissa), None)
  );
  let err = slot
    .set_paralogue_state(5, awakening::ParalogueState::Unlocked)
    .unwrap_err();
  match err {
    awakening::save::Error::Paralogue(
      awakening::social::ParalogueError::ParentsNotMarried { .. },
    ) => {}
    other => panic!("expected ParentsNotMarried, got {other:?}"),
  }

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

  slot
    .set_paralogue_state(5, awakening::ParalogueState::Unlocked)
    .expect("unlock paralogue 5");
  assert_eq!(
    slot.paralogue_state(5).expect("p5 state"),
    awakening::ParalogueState::Unlocked
  );

  slot
    .set_paralogue_state(5, awakening::ParalogueState::Completed)
    .expect("complete paralogue 5");
  assert_eq!(
    slot.paralogue_state(5).expect("p5 state"),
    awakening::ParalogueState::Completed
  );

  slot
    .roster
    .add_unit(
      awakening::roster::Character::Owain,
      awakening::roster::Class::Myrmidon,
    )
    .expect("add owain");
  let err = slot
    .set_paralogue_state(5, awakening::ParalogueState::Locked)
    .unwrap_err();
  match err {
    awakening::save::Error::Paralogue(
      awakening::social::ParalogueError::ChildInRosterParalogueConflict {
        ..
      },
    ) => {}
    other => panic!("expected ChildInRosterParalogueConflict, got {other:?}"),
  }
}

#[test]
fn test_growths_and_weapon_ranks() {
  let mut slot = awakening::Save::from_bytes(CHAPTER0).expect("parse slot");
  let chrom_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Chrom)
    .expect("chrom index");

  slot.roster[chrom_idx]
    .set_growth(0, 50)
    .expect("set hp growth");
  slot.roster[chrom_idx]
    .set_weapon_rank(0, 75)
    .expect("set sword rank");
  assert_eq!(slot.roster[chrom_idx].growths()[0], 50);
  assert_eq!(slot.roster[chrom_idx].weapon_ranks()[0], 75);

  slot.roster[chrom_idx].max_growths();
  slot.roster[chrom_idx].max_weapon_ranks();
  for stat in slot.roster[chrom_idx].stats() {
    assert_eq!(stat.total(), 99);
  }
  assert_eq!(*slot.roster[chrom_idx].weapon_ranks(), [90; 6]);

  let repacked = slot.to_bytes().expect("repack slot");
  let reloaded = awakening::Save::from_bytes(&repacked).expect("reload slot");
  let reloaded_chrom = reloaded
    .find_unit_by_name("chrom")
    .expect("find reloaded chrom");
  for stat in reloaded_chrom.stats() {
    assert_eq!(stat.total(), 99);
  }
  assert_eq!(*reloaded_chrom.weapon_ranks(), [90; 6]);
}

#[test]
fn test_support_table_persistence_roundtrip() {
  let mut slot = awakening::Save::from_bytes(CHAPTER0).expect("parse slot");
  let chrom = awakening::roster::Character::Chrom;
  let sumia = awakening::roster::Character::Sumia;

  slot
    .supports
    .set_rank(chrom, sumia, awakening::SupportRank::S)
    .expect("set S support");
  assert_eq!(
    slot.supports.rank_between(chrom, sumia),
    awakening::SupportRank::S
  );

  let repacked = slot.to_bytes().expect("repack slot");
  let reloaded = awakening::Save::from_bytes(&repacked).expect("reload slot");
  assert_eq!(
    reloaded.supports.rank_between(chrom, sumia),
    awakening::SupportRank::S
  );
  assert_eq!(reloaded.supports.spouse_of(chrom), Some(sumia));
}

#[test]
fn test_save_combat_stats_99_roundtrip() {
  let mut slot = awakening::Save::from_bytes(CHAPTER0).expect("parse slot");
  let chrom_idx = slot
    .roster
    .iter()
    .position(|u| u.character() == awakening::roster::Character::Chrom)
    .expect("chrom idx");

  slot.roster[chrom_idx].hp.set_total(99).expect("set hp 99");
  slot.roster[chrom_idx]
    .strength
    .set_total(99)
    .expect("set strength 99");
  slot.roster[chrom_idx]
    .defense
    .set_total(99)
    .expect("set defense 99");

  assert_eq!(slot.roster[chrom_idx].hp.total(), 99);
  assert_eq!(slot.roster[chrom_idx].strength.total(), 99);
  assert_eq!(slot.roster[chrom_idx].defense.total(), 99);

  let repacked = slot.to_bytes().expect("repack slot");
  let reloaded = awakening::Save::from_bytes(&repacked).expect("reload slot");
  let reloaded_chrom = reloaded
    .find_unit_by_name("chrom")
    .expect("find reloaded chrom");

  assert_eq!(reloaded_chrom.hp.total(), 99);
  assert_eq!(reloaded_chrom.strength.total(), 99);
  assert_eq!(reloaded_chrom.defense.total(), 99);
}

#[test]
fn test_save_forge_creation_and_map_group_mutations() {
  let bytes = std::fs::read("tests/fixtures/20200831-225628/Chapter0")
    .expect("read chapter0");
  let mut slot = awakening::Save::from_bytes(&bytes).expect("parse slot");

  // Test forge creation & convoy storage
  let forge_pos = slot
    .forge_and_store(
      awakening::convoy::ItemType::IronSword,
      "SuperBlade",
      5,
      10,
      5,
      Some(40),
    )
    .expect("forge and store");
  assert_eq!(forge_pos, 0);
  assert_eq!(slot.forges().len(), 1);
  assert_eq!(slot.forges()[0].name, "SuperBlade");

  // Test equipping forged weapon to unit
  slot
    .give_forged_weapon_to_unit(0, 0, forge_pos, Some(40))
    .expect("give to unit");
  assert!(slot.roster[0].inventory()[0].is_some());

  // Test all_player_units iteration
  let unit_count = slot.all_player_units().count();
  assert_eq!(unit_count, slot.roster.len() + slot.deployed_units().len());

  // Test enemy group mutation on map save
  let map_bytes =
    std::fs::read("tests/fixtures/20200831-225628/Map0").expect("read map0");
  let mut map_slot =
    awakening::Save::from_bytes(&map_bytes).expect("parse map0");
  let initial_red = map_slot.enemy_units().len();
  map_slot
    .bookmark_spawn_enemy(
      awakening::roster::Character::Gangrel,
      awakening::roster::Class::Barbarian,
      [10, 15],
    )
    .expect("spawn gangrel on map0");
  assert_eq!(map_slot.enemy_units().len(), initial_red + 1);

  // Test repacking & reloading
  let repacked = map_slot.to_bytes().expect("repack map slot");
  let reloaded =
    awakening::Save::from_bytes(&repacked).expect("reload map slot");
  assert_eq!(reloaded.enemy_units().len(), initial_red + 1);
}

#[test]
fn test_bookmark_guards() {
  let ch_bytes = std::fs::read("tests/fixtures/20200831-225628/Chapter0")
    .expect("read chapter0");
  let mut ch_slot =
    awakening::Save::from_bytes(&ch_bytes).expect("parse chapter0");
  assert!(!ch_slot.is_bookmark());

  assert!(matches!(
    ch_slot.bookmark_spawn_enemy(
      awakening::roster::Character::Gangrel,
      awakening::roster::Class::Barbarian,
      [5, 5],
    ),
    Err(awakening::save::Error::NotABookmarkSave)
  ));
  assert!(matches!(
    ch_slot.bookmark_deploy_unit(0, [5, 5]),
    Err(awakening::save::Error::NotABookmarkSave)
  ));
  assert!(matches!(
    ch_slot.bookmark_undeploy_unit(0),
    Err(awakening::save::Error::NotABookmarkSave)
  ));
  assert!(matches!(
    ch_slot.bookmark_kill_unit(0),
    Err(awakening::save::Error::NotABookmarkSave)
  ));

  let map_bytes =
    std::fs::read("tests/fixtures/20200831-225628/Map0").expect("read map0");
  let map_slot = awakening::Save::from_bytes(&map_bytes).expect("parse map0");
  assert!(map_slot.is_bookmark());
}

#[test]
fn test_bookmark_boss_and_recruitable_safeguards() {
  let map_bytes =
    std::fs::read("tests/fixtures/20200831-225628/Map0").expect("read map0");
  let mut map_slot =
    awakening::Save::from_bytes(&map_bytes).expect("parse map0");

  let gangrel_idx = map_slot
    .bookmark_spawn_enemy(
      awakening::roster::Character::Gangrel,
      awakening::roster::Class::Barbarian,
      [10, 15],
    )
    .expect("spawn gangrel");

  let err = map_slot.bookmark_remove_enemy(gangrel_idx).unwrap_err();
  match err {
    awakening::save::Error::Group(
      awakening::roster::GroupError::CannotRemoveChapterBoss { character },
    ) => assert_eq!(character, awakening::roster::Character::Gangrel),
    other => panic!("expected CannotRemoveChapterBoss, got {other:?}"),
  }

  let tharja_idx = map_slot
    .bookmark_spawn_enemy(
      awakening::roster::Character::Tharja,
      awakening::roster::Class::DarkMage,
      [11, 15],
    )
    .expect("spawn tharja");

  let err = map_slot.bookmark_remove_enemy(tharja_idx).unwrap_err();
  match err {
    awakening::save::Error::Group(
      awakening::roster::GroupError::CannotRemoveRecruitmentTarget {
        character,
      },
    ) => assert_eq!(character, awakening::roster::Character::Tharja),
    other => panic!("expected CannotRemoveRecruitmentTarget, got {other:?}"),
  }
}

#[test]
fn test_unused_forge_pruning_and_lifecycle() {
  let bytes = std::fs::read("tests/fixtures/20200831-225628/Chapter0")
    .expect("read chapter0");
  let mut slot = awakening::Save::from_bytes(&bytes).expect("parse slot");

  let pos = slot
    .forge_and_store(
      awakening::convoy::ItemType::IronSword,
      "TempBlade",
      2,
      5,
      0,
      Some(30),
    )
    .expect("create forge");

  assert!(slot.is_forge_in_use(pos));

  let err = slot.remove_forged_weapon(pos).unwrap_err();
  assert!(
    matches!(err, awakening::save::Error::ForgeInUse { position } if position == pos)
  );

  assert_eq!(slot.prune_unused_forges(), 0);

  let forged_id = slot.convoy.forged_item_id(pos).expect("valid forge id");
  slot
    .convoy
    .set_durability(usize::from(forged_id), 0)
    .expect("clear durability");

  assert!(!slot.is_forge_in_use(pos));

  assert_eq!(slot.prune_unused_forges(), 1);
  assert!(slot.forges().get(pos).is_none());
}

#[test]
fn test_forge_total_stat_bounds() {
  let bytes = std::fs::read("tests/fixtures/20200831-225628/Chapter0")
    .expect("read chapter0");
  let mut slot = awakening::Save::from_bytes(&bytes).expect("parse slot");

  assert!(
    slot
      .create_forged_weapon(
        awakening::convoy::ItemType::IronSword,
        "ValidSword",
        5,
        15,
        10,
      )
      .is_ok()
  );

  assert!(
    slot
      .create_forged_weapon(
        awakening::convoy::ItemType::IronSword,
        "NegativeMight",
        -10,
        0,
        0,
      )
      .is_err()
  );

  assert!(
    slot
      .create_forged_weapon(
        awakening::convoy::ItemType::IronSword,
        "OverflowMight",
        100,
        0,
        0,
      )
      .is_err()
  );

  assert!(
    slot
      .create_forged_weapon(
        awakening::convoy::ItemType::IronSword,
        "OverflowHit",
        0,
        60,
        0,
      )
      .is_err()
  );
}
