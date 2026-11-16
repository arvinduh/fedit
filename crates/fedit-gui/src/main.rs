//! Native desktop GUI for the Fire Emblem: Awakening save editor.
//!
//! Provides a high-performance Tauri v2 desktop application coordinating
//! save slot inspection, combat attribute mutations, and skill repertoire management.

#![expect(
  clippy::needless_pass_by_value,
  reason = "tauri command handler signature requires owned state and parameters"
)]

use std::path;
use std::sync;

use awakening::roster;
use fedit_core::SaveSet;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default)]
struct SessionState {
  saveset: Option<SaveSet>,
  active_slot_index: usize,
  loaded_slots: [Option<awakening::SaveSlot>; 3],
}

struct AppState(sync::Mutex<SessionState>);

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ItemDto {
  name: String,
  kind: String,
  uses: u16,
  max_uses: u16,
  is_equipped: bool,
  forge_mt: Option<u8>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SkillDto {
  id: u16,
  name: String,
  description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct UnitDto {
  id: usize,
  slug: String,
  name: String,
  title: String,
  class_name: String,
  level: u8,
  exp: u8,
  cur_hp: u8,
  max_hp: u8,
  boots: u8,
  stats: [u8; 8],
  caps: [u8; 8],
  bases: [u8; 8],
  additions: [i8; 8],
  growths: [u8; 8],
  inventory: Vec<ItemDto>,
  skills: Vec<SkillDto>,
  repertoire: Vec<SkillDto>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct SlotSummaryDto {
  slot_index: usize,
  chapter: String,
  difficulty: String,
  mode: String,
  play_time: String,
  gold: u32,
  unit_count: usize,
}

fn map_unit_to_dto(idx: usize, u: &roster::Unit) -> UnitDto {
  let char_id = u.character();
  let class_id = u.class();
  let slug = char_id.slug().to_string();

  let mut inventory = Vec::new();
  for slot_idx in 0..5 {
    if let Some(item_slot) = u.item(slot_idx) {
      let is_eq = u.equipped_slot() == Some(slot_idx);
      inventory.push(ItemDto {
        name: item_slot.item().name().to_string(),
        kind: format!("{:?}", item_slot.item().kind()),
        uses: item_slot.uses(),
        max_uses: u16::from(item_slot.item().max_durability().unwrap_or(1)),
        is_equipped: is_eq,
        forge_mt: None,
      });
    }
  }

  let mut skills = Vec::new();
  for &sk in &u.skills() {
    if sk != roster::Skill::None {
      skills.push(SkillDto {
        id: sk.raw_id(),
        name: sk.name().to_string(),
        description: String::new(),
      });
    }
  }

  let mut repertoire = Vec::new();
  for &sk in u.repertoire() {
    if sk != roster::Skill::None {
      repertoire.push(SkillDto {
        id: sk.raw_id(),
        name: sk.name().to_string(),
        description: String::new(),
      });
    }
  }

  UnitDto {
    id: idx,
    slug,
    name: char_id.name().to_string(),
    title: char_id.name().to_string(),
    class_name: class_id.name().to_string(),
    level: u.level,
    exp: u.exp,
    cur_hp: u.stats.hp,
    max_hp: u.stats.hp,
    boots: 0,
    stats: [
      u.stats.hp,
      u.stats.strength,
      u.stats.magic,
      u.stats.skill,
      u.stats.speed,
      u.stats.luck,
      u.stats.defense,
      u.stats.resistance,
    ],
    caps: class_id.max_stats(),
    bases: class_id.base_stats(),
    additions: char_id.additions(),
    growths: u.growths,
    inventory,
    skills,
    repertoire,
  }
}

#[tauri::command]
fn open_save_path(
  state: tauri::State<'_, AppState>,
  target_path: String,
) -> Result<Vec<SlotSummaryDto>, String> {
  let mut session = state.0.lock().map_err(|e| e.to_string())?;
  let p = path::PathBuf::from(&target_path);
  let (saveset, _target) = SaveSet::open_path(p).map_err(|e| e.to_string())?;

  let mut summaries = Vec::new();
  for (idx, slot_sum) in saveset.slots().iter().enumerate() {
    if idx < 3
      && let Ok(slot_data) = saveset.load_slot(&slot_sum.filename)
    {
      summaries.push(SlotSummaryDto {
        slot_index: idx,
        chapter: slot_data.preview().chapter_title().clone(),
        difficulty: slot_data
          .preview()
          .difficulty()
          .map_or_else(|| "Unknown".to_string(), |d| format!("{d:?}")),
        mode: slot_data
          .preview()
          .mode()
          .map_or_else(|| "Unknown".to_string(), |m| format!("{m:?}")),
        play_time: slot_data.preview().play_time().to_string(),
        gold: slot_data.gold(),
        unit_count: slot_data.roster.len(),
      });
      session.loaded_slots[idx] = Some(slot_data);
    }
  }

  session.saveset = Some(saveset);
  session.active_slot_index = 0;
  Ok(summaries)
}

#[tauri::command]
fn get_roster(
  state: tauri::State<'_, AppState>,
  slot_index: usize,
) -> Result<Vec<UnitDto>, String> {
  let session = state.0.lock().map_err(|e| e.to_string())?;
  let slot = session
    .loaded_slots
    .get(slot_index)
    .and_then(|s| s.as_ref())
    .ok_or_else(|| "Slot not loaded".to_string())?;

  Ok(
    slot
      .roster
      .iter()
      .enumerate()
      .map(|(idx, u)| map_unit_to_dto(idx, u))
      .collect(),
  )
}

#[tauri::command]
fn mutate_unit_stat(
  state: tauri::State<'_, AppState>,
  slot_index: usize,
  unit_index: usize,
  stat_index: usize,
  value: u8,
) -> Result<UnitDto, String> {
  let mut session = state.0.lock().map_err(|e| e.to_string())?;
  let slot = session
    .loaded_slots
    .get_mut(slot_index)
    .and_then(|s| s.as_mut())
    .ok_or_else(|| "Slot not loaded".to_string())?;

  let unit = slot
    .roster
    .get_mut(unit_index)
    .ok_or_else(|| "Unit out of bounds".to_string())?;

  unit
    .set_growth(stat_index, value)
    .map_err(|e| e.to_string())?;

  Ok(map_unit_to_dto(unit_index, unit))
}

#[tauri::command]
fn learn_unit_skill(
  state: tauri::State<'_, AppState>,
  slot_index: usize,
  unit_index: usize,
  skill_id: u16,
) -> Result<UnitDto, String> {
  let mut session = state.0.lock().map_err(|e| e.to_string())?;
  let slot = session
    .loaded_slots
    .get_mut(slot_index)
    .and_then(|s| s.as_mut())
    .ok_or_else(|| "Slot not loaded".to_string())?;

  let unit = slot
    .roster
    .get_mut(unit_index)
    .ok_or_else(|| "Unit out of bounds".to_string())?;

  let skill = roster::Skill::from_raw(skill_id);
  unit.learn_skill(skill);

  Ok(map_unit_to_dto(unit_index, unit))
}

#[tauri::command]
fn unlearn_unit_skill(
  state: tauri::State<'_, AppState>,
  slot_index: usize,
  unit_index: usize,
  skill_id: u16,
) -> Result<UnitDto, String> {
  let mut session = state.0.lock().map_err(|e| e.to_string())?;
  let slot = session
    .loaded_slots
    .get_mut(slot_index)
    .and_then(|s| s.as_mut())
    .ok_or_else(|| "Slot not loaded".to_string())?;

  let unit = slot
    .roster
    .get_mut(unit_index)
    .ok_or_else(|| "Unit out of bounds".to_string())?;

  let skill = roster::Skill::from_raw(skill_id);
  unit.unlearn_skill(skill);

  Ok(map_unit_to_dto(unit_index, unit))
}

#[tauri::command]
fn equip_unit_skill(
  state: tauri::State<'_, AppState>,
  slot_index: usize,
  unit_index: usize,
  slot_pos: usize,
  skill_id: u16,
) -> Result<UnitDto, String> {
  let mut session = state.0.lock().map_err(|e| e.to_string())?;
  let slot = session
    .loaded_slots
    .get_mut(slot_index)
    .and_then(|s| s.as_mut())
    .ok_or_else(|| "Slot not loaded".to_string())?;

  let unit = slot
    .roster
    .get_mut(unit_index)
    .ok_or_else(|| "Unit out of bounds".to_string())?;

  let skill = roster::Skill::from_raw(skill_id);
  unit.set_skill(slot_pos, skill).map_err(|e| e.to_string())?;

  Ok(map_unit_to_dto(unit_index, unit))
}

#[tauri::command]
fn save_slot(
  state: tauri::State<'_, AppState>,
  slot_index: usize,
) -> Result<String, String> {
  let session = state.0.lock().map_err(|e| e.to_string())?;
  let saveset = session.saveset.as_ref().ok_or("No saveset open")?;
  let summary = saveset
    .slots()
    .get(slot_index)
    .ok_or("Slot index out of bounds")?;
  let slot = session
    .loaded_slots
    .get(slot_index)
    .and_then(|s| s.as_ref())
    .ok_or("Slot data not loaded")?;

  let filename = summary.filename.as_str();
  saveset
    .save_slot(filename, slot, true)
    .map_err(|e| e.to_string())?;
  Ok("Slot saved successfully".to_string())
}

fn main() {
  tauri::Builder::default()
    .manage(AppState(sync::Mutex::new(SessionState::default())))
    .setup(|app| {
      #[cfg(debug_assertions)]
      {
        use tauri::Manager;
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 5173));
        let timeout = std::time::Duration::from_millis(50);
        let is_dev =
          std::net::TcpStream::connect_timeout(&addr, timeout).is_ok();
        let maybe_win = if is_dev {
          app.get_webview_window("main")
        } else {
          None
        };
        if let (Some(win), Ok(url)) =
          (maybe_win, "http://localhost:5173".parse::<tauri::Url>())
        {
          let _ = win.navigate(url);
        }
      }
      Ok(())
    })
    .invoke_handler(tauri::generate_handler![
      open_save_path,
      get_roster,
      mutate_unit_stat,
      learn_unit_skill,
      unlearn_unit_skill,
      equip_unit_skill,
      save_slot,
    ])
    .run(tauri::generate_context!())
    .expect("error while running fedit tauri application");
}
