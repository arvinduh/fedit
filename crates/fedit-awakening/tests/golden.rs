//! Holistic golden catalog verification tests.
//!
//! Validates `fedit-awakening` item and class models against canonical 3DS ROM
//! game data fixtures and asserts deep structural accuracy across campaign saves.

use fedit_awakening as awakening;
use serde_json;

use awakening::convoy::ItemType;
use awakening::roster::Character;
use awakening::roster::Class;

const GOLDEN_ITEMS_JSON: &str = include_str!("fixtures/golden/items.json");
const GOLDEN_CLASSES_JSON: &str = include_str!("fixtures/golden/classes.json");

const GLOBAL_RAW: &[u8] = include_bytes!("fixtures/20200831-225628/Global");
const GOLDEN_GLOBAL_JSON: &str = include_str!("fixtures/golden/global.json");

const CHAPTER0_RAW: &[u8] = include_bytes!("fixtures/20200831-225628/Chapter0");
const GOLDEN_CHAPTER0_JSON: &str =
  include_str!("fixtures/golden/chapter0.json");

const CHAPTER1_RAW: &[u8] = include_bytes!("fixtures/20200831-225628/Chapter1");
const GOLDEN_CHAPTER1_JSON: &str =
  include_str!("fixtures/golden/chapter1.json");

const CHAPTER2_RAW: &[u8] = include_bytes!("fixtures/20200831-225628/Chapter2");
const GOLDEN_CHAPTER2_JSON: &str =
  include_str!("fixtures/golden/chapter2.json");

const MAP0_RAW: &[u8] = include_bytes!("fixtures/20200831-225628/Map0");
const GOLDEN_MAP0_JSON: &str = include_str!("fixtures/golden/map0.json");

const MAP1_RAW: &[u8] = include_bytes!("fixtures/20200831-225628/Map1");
const GOLDEN_MAP1_JSON: &str = include_str!("fixtures/golden/map1.json");

#[derive(Debug, serde::Deserialize)]
struct GoldenGlobal {
  renown: u32,
  flags: [bool; 8],
  logbook_avatars_count: usize,
}

#[derive(Debug, serde::Deserialize)]
#[expect(dead_code, reason = "serde json schema completeness")]
struct GoldenSave {
  is_chapter: bool,
  is_west: bool,
  user: GoldenUser,
  unit_groups: GoldenUnitGroups,
  forged_weapons_count: usize,
  forged_weapons: Vec<GoldenForge>,
  convoy_items_count: usize,
  convoy_active_items: Vec<GoldenConvoyItem>,
}

#[derive(Debug, serde::Deserialize)]
#[expect(dead_code, reason = "serde json schema completeness")]
struct GoldenUser {
  playtime_frames: u32,
  money: u32,
  renown: u32,
  difficulty: u8,
  is_lunatic_plus: bool,
  current_chapter: u8,
  count_total_chapters: u8,
  count_last_chapter: u8,
  chapters_beaten: u8,
}

#[derive(Debug, serde::Deserialize)]
struct GoldenUnitGroups {
  blue_units_count: usize,
  red_units_count: usize,
  green_units_count: usize,
  main_units_count: usize,
  dead_units_count: usize,
  other_units_count: usize,
  main_units: Vec<GoldenUnit>,
}

#[derive(Debug, serde::Deserialize)]
#[expect(dead_code, reason = "serde json schema completeness")]
struct GoldenUnit {
  slot: usize,
  unit_id: u16,
  unit_name: String,
  class_id: u8,
  level: u8,
  exp: u8,
  growths: [u8; 8],
  items: [GoldenItemSlot; 5],
  equipped_skills: [u16; 5],
  weapon_exp: [u8; 6],
}

#[derive(Debug, serde::Deserialize)]
struct GoldenItemSlot {
  id: u16,
  uses: u8,
}

#[derive(Debug, serde::Deserialize)]
struct GoldenForge {
  position: usize,
  name: String,
  weapon_id: u16,
  might: u8,
  hit: u8,
  crit: u8,
}

#[derive(Debug, serde::Deserialize)]
struct GoldenConvoyItem {
  slot: usize,
  uses: u8,
}

#[expect(
  clippy::too_many_lines,
  reason = "comprehensive golden save assertion suite"
)]
fn assert_save_matches_golden(save: &awakening::Save, expected: &GoldenSave) {
  assert_eq!(save.gold(), expected.user.money, "money mismatch");
  assert_eq!(save.renown(), expected.user.renown, "renown mismatch");
  assert_eq!(
    save.user().difficulty(),
    expected.user.difficulty,
    "difficulty mismatch"
  );
  assert_eq!(
    save.user().is_lunatic_plus(),
    expected.user.is_lunatic_plus,
    "lunatic+ mismatch"
  );
  assert_eq!(
    save.user().current_chapter(),
    expected.user.current_chapter,
    "chapter mismatch"
  );
  assert_eq!(
    save.user().play_time(),
    expected.user.playtime_frames,
    "playtime frames mismatch"
  );
  assert_eq!(
    save.user().count_total_chapters(),
    expected.user.count_total_chapters,
    "total chapters mismatch"
  );
  assert_eq!(
    save.user().count_last_chapter(),
    expected.user.count_last_chapter,
    "last chapter mismatch"
  );

  assert_eq!(
    save.groups().blue().len(),
    expected.unit_groups.blue_units_count,
    "blue units count mismatch"
  );
  assert_eq!(
    save.groups().red().len(),
    expected.unit_groups.red_units_count,
    "red units count mismatch"
  );
  assert_eq!(
    save.groups().green().len(),
    expected.unit_groups.green_units_count,
    "green units count mismatch"
  );
  assert_eq!(
    save.groups().main().len(),
    expected.unit_groups.main_units_count,
    "main units count mismatch"
  );
  assert_eq!(
    save.groups().dead().len(),
    expected.unit_groups.dead_units_count,
    "dead units count mismatch"
  );
  assert_eq!(
    save.groups().other().len(),
    expected.unit_groups.other_units_count,
    "other units count mismatch"
  );

  for (i, exp_unit) in expected.unit_groups.main_units.iter().enumerate() {
    assert_eq!(i, exp_unit.slot, "unit #{i} slot mismatch");
    let actual_unit = &save.groups().main()[i];
    assert_eq!(
      actual_unit.character().id(),
      exp_unit.unit_id,
      "unit #{i} id mismatch"
    );
    assert_eq!(
      actual_unit.class(),
      Class::from_id(exp_unit.class_id),
      "unit #{i} ({}) class mismatch",
      actual_unit.character()
    );
    assert_eq!(
      actual_unit.level(),
      exp_unit.level,
      "unit #{i} ({}) level mismatch",
      actual_unit.character()
    );
    assert_eq!(
      actual_unit.exp(),
      exp_unit.exp,
      "unit #{i} ({}) exp mismatch",
      actual_unit.character()
    );
    assert_eq!(
      actual_unit.growths(),
      exp_unit.growths,
      "unit #{i} ({}) growths mismatch",
      actual_unit.character()
    );
    assert_eq!(
      *actual_unit.weapon_ranks(),
      exp_unit.weapon_exp,
      "unit #{i} ({}) weapon ranks mismatch",
      actual_unit.character()
    );

    for s in 0..5 {
      let exp_item = &exp_unit.items[s];
      let actual_item = &actual_unit.inventory()[s];
      if exp_item.id == 0 {
        assert!(actual_item.is_none(), "unit #{i} slot #{s} expected empty");
      } else {
        let it = actual_item
          .as_ref()
          .unwrap_or_else(|| panic!("unit #{i} slot #{s} should have item"));
        assert_eq!(
          it.item_type().id(),
          exp_item.id,
          "unit #{i} slot #{s} item ID mismatch"
        );
        assert_eq!(
          it.durability(),
          u16::from(exp_item.uses),
          "unit #{i} slot #{s} item durability mismatch"
        );
      }
    }

    for s in 0..5 {
      let exp_skill = exp_unit.equipped_skills[s];
      let actual_skill = actual_unit.skills()[s];
      assert_eq!(
        actual_skill.id(),
        exp_skill,
        "unit #{i} skill #{s} mismatch"
      );
    }
  }

  assert_eq!(
    save.forges().len(),
    expected.forged_weapons_count,
    "forged weapons count mismatch"
  );
  for (i, exp_forge) in expected.forged_weapons.iter().enumerate() {
    let actual_forge = &save.forges()[i];
    assert_eq!(
      actual_forge.position, exp_forge.position,
      "forge #{i} position mismatch"
    );
    assert_eq!(
      actual_forge.name, exp_forge.name,
      "forge #{i} name mismatch"
    );
    assert_eq!(
      actual_forge.weapon_id, exp_forge.weapon_id,
      "forge #{i} weapon ID mismatch"
    );
    assert_eq!(
      actual_forge.might,
      exp_forge.might.cast_signed(),
      "forge #{i} might mismatch"
    );
    assert_eq!(
      actual_forge.hit,
      exp_forge.hit.cast_signed(),
      "forge #{i} hit mismatch"
    );
    assert_eq!(
      actual_forge.crit,
      exp_forge.crit.cast_signed(),
      "forge #{i} crit mismatch"
    );
  }

  assert_eq!(
    save.convoy.len(),
    expected.convoy_active_items.len(),
    "convoy active items count mismatch"
  );
  for exp_convoy in &expected.convoy_active_items {
    let actual = save
      .convoy
      .find_by_id(u16::try_from(exp_convoy.slot).expect("valid id"))
      .unwrap_or_else(|| {
        panic!("convoy slot {} expected to exist", exp_convoy.slot)
      });
    assert_eq!(
      actual.durability(),
      u16::from(exp_convoy.uses),
      "convoy slot {} durability mismatch",
      exp_convoy.slot
    );
  }
}

#[test]
fn test_golden_global() {
  let global = awakening::global::GlobalSave::from_bytes(GLOBAL_RAW)
    .expect("parse global");
  let expected: GoldenGlobal =
    serde_json::from_str(GOLDEN_GLOBAL_JSON).expect("parse global.json");

  assert_eq!(global.renown(), expected.renown);
  assert_eq!(global.global_flags(), expected.flags);
  assert_eq!(global.logbook_count(), expected.logbook_avatars_count);
}

#[test]
fn test_golden_chapter0() {
  let save =
    awakening::Save::from_bytes(CHAPTER0_RAW).expect("parse Chapter0 save");
  let expected: GoldenSave =
    serde_json::from_str(GOLDEN_CHAPTER0_JSON).expect("parse chapter0.json");

  assert_save_matches_golden(&save, &expected);
}

#[test]
fn test_golden_chapter1() {
  let save =
    awakening::Save::from_bytes(CHAPTER1_RAW).expect("parse Chapter1 save");
  let expected: GoldenSave =
    serde_json::from_str(GOLDEN_CHAPTER1_JSON).expect("parse chapter1.json");

  assert_save_matches_golden(&save, &expected);
}

#[test]
fn test_golden_chapter2() {
  let save =
    awakening::Save::from_bytes(CHAPTER2_RAW).expect("parse Chapter2 save");
  let expected: GoldenSave =
    serde_json::from_str(GOLDEN_CHAPTER2_JSON).expect("parse chapter2.json");

  assert_save_matches_golden(&save, &expected);
}

#[test]
fn test_golden_map0() {
  let save = awakening::Save::from_bytes(MAP0_RAW).expect("parse Map0 save");
  let expected: GoldenSave =
    serde_json::from_str(GOLDEN_MAP0_JSON).expect("parse map0.json");

  assert_save_matches_golden(&save, &expected);
}

#[test]
fn test_golden_map1() {
  let save = awakening::Save::from_bytes(MAP1_RAW).expect("parse Map1 save");
  let expected: GoldenSave =
    serde_json::from_str(GOLDEN_MAP1_JSON).expect("parse map1.json");

  assert_save_matches_golden(&save, &expected);
}

#[test]
fn test_all_items_match_golden_catalog() {
  let items_value: serde_json::Value =
    serde_json::from_str(GOLDEN_ITEMS_JSON).expect("parse items.json");
  let items = items_value.as_array().expect("array of items");

  for it in items {
    let raw_u64 = it["id"].as_u64().expect("item id");
    let id = u16::try_from(raw_u64).expect("valid u16 item id");
    if id == 0 {
      continue;
    }

    let item_type = ItemType::from_id(id);
    match item_type {
      ItemType::Unknown(unknown_id) => {
        panic!("item ID {unknown_id} should be recognized in ItemType");
      }
      known => {
        let expected_name = it["name"].as_str().expect("name");
        let stats = &it["stats"];
        let expected_uses = u8::try_from(stats["uses"].as_u64().expect("uses"))
          .expect("valid u8 uses");
        let expected_might =
          u8::try_from(stats["might"].as_u64().expect("might"))
            .expect("valid u8 might");
        let expected_hit = u8::try_from(stats["hit"].as_u64().expect("hit"))
          .expect("valid u8 hit");
        let expected_crit = u8::try_from(stats["crit"].as_u64().expect("crit"))
          .expect("valid u8 crit");

        assert_eq!(known.id(), id, "mismatched raw_id for {}", known.name());
        assert_eq!(known.name(), expected_name, "mismatched name for ID {id}");
        assert_eq!(
          known.might(),
          expected_might,
          "mismatched might for {}",
          known.name()
        );
        assert_eq!(
          known.hit(),
          expected_hit,
          "mismatched hit for {}",
          known.name()
        );
        assert_eq!(
          known.crit(),
          expected_crit,
          "mismatched crit for {}",
          known.name()
        );

        let expected_durability = if expected_uses > 0 {
          Some(expected_uses)
        } else {
          None
        };
        assert_eq!(
          known.max_durability(),
          expected_durability,
          "mismatched max_durability for {}",
          known.name()
        );
      }
    }
  }
}

#[test]
fn test_classes_match_golden_catalog() {
  let classes_value: serde_json::Value =
    serde_json::from_str(GOLDEN_CLASSES_JSON).expect("parse classes.json");
  let classes = classes_value.as_array().expect("array of classes");

  for c in classes {
    let raw_u64 = c["id"].as_u64().expect("class id");
    let id = u8::try_from(raw_u64).expect("valid u8 class id");
    let cls = Class::from_id(id);
    if let Class::Unknown(unknown_id) = cls {
      panic!("class ID {unknown_id} should be recognized");
    }
  }
}

#[test]
fn test_chapter0_convoy_snapshot() {
  let save =
    awakening::Save::from_bytes(CHAPTER0_RAW).expect("parse Chapter0 save");
  assert_eq!(save.convoy.len(), 55);

  let expected_items: &[(u16, ItemType, u16)] = &[
    (1, ItemType::BronzeSword, 48),
    (2, ItemType::IronSword, 15),
    (3, ItemType::SteelSword, 51),
    (4, ItemType::SilverSword, 30),
    (6, ItemType::Armorslayer, 50),
    (7, ItemType::Wyrmslayer, 16),
    (8, ItemType::KillingEdge, 27),
  ];

  for (expected_id, expected_type, expected_uses) in expected_items {
    let convoy_entry =
      save.convoy.find_by_id(*expected_id).expect("item exists");
    assert_eq!(convoy_entry.item_type(), *expected_type);
    assert_eq!(convoy_entry.durability(), *expected_uses);
  }
}

#[test]
fn test_chapter0_roster_snapshot() {
  let save =
    awakening::Save::from_bytes(CHAPTER0_RAW).expect("parse Chapter0 save");
  assert_eq!(save.roster.len(), 25);

  let expected_roster: &[(Character, Class, u8, u8, u8)] = &[
    (Character::Chrom, Class::GreatLord, 60, 38, 34),
    (Character::RobinF, Class::Grandmaster, 68, 37, 38),
    (Character::Lucina, Class::Grandmaster, 71, 35, 36),
    (Character::Frederick, Class::GreatKnight, 32, 17, 18),
    (Character::SayRi, Class::Swordmaster, 43, 19, 14),
    (Character::Gaius, Class::Thief, 22, 7, 5),
    (Character::Cherche, Class::WyvernLord, 38, 20, 20),
    (Character::Henry, Class::DarkMage, 31, 7, 14),
    (Character::Olivia, Class::Dancer, 18, 3, 3),
    (Character::Cordelia, Class::PegasusKnight, 25, 9, 8),
    (Character::Tharja, Class::DarkMage, 26, 4, 10),
    (Character::Nowi, Class::Manakete, 19, 5, 3),
    (Character::Gregor, Class::Mercenary, 31, 13, 11),
    (Character::Sumia, Class::PegasusKnight, 18, 6, 5),
    (Character::LonQu, Class::Myrmidon, 20, 6, 7),
    (Character::Kellam, Class::Knight, 21, 10, 12),
    (Character::Virion, Class::Archer, 19, 6, 6),
    (Character::Sully, Class::Cavalier, 20, 7, 7),
    (Character::Vaike, Class::Fighter, 24, 9, 5),
    (Character::Stahl, Class::Cavalier, 22, 8, 8),
    (Character::Panne, Class::Taguel, 28, 9, 8),
    (Character::Maribelle, Class::Troubadour, 18, 0, 3),
    (Character::Ricken, Class::Mage, 20, 3, 6),
    (Character::Lissa, Class::Cleric, 17, 2, 3),
    (Character::Miriel, Class::Mage, 18, 0, 3),
  ];

  for (i, (expected_char, expected_class, hp, str_, def)) in
    expected_roster.iter().enumerate()
  {
    let unit = &save.roster[i];
    assert_eq!(
      unit.character(),
      *expected_char,
      "mismatched character at index {i}"
    );
    assert_eq!(
      unit.class(),
      *expected_class,
      "mismatched class for {:?}",
      unit.character()
    );
    assert_eq!(
      unit.hp.total(),
      *hp,
      "mismatched HP for {:?}",
      unit.character()
    );
    assert_eq!(
      unit.strength.total(),
      *str_,
      "mismatched strength for {:?}",
      unit.character()
    );
    assert_eq!(
      unit.defense.total(),
      *def,
      "mismatched defense for {:?}",
      unit.character()
    );
  }
}
