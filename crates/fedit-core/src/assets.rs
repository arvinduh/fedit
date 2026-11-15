//! Character portrait, icon, and sprite asset resolution and caching.
//!
//! Maps domain characters, skills, items, and classes to disk image assets
//! using canonical filename slugs.

use std::fs;
use std::path;

use awakening::convoy;
use awakening::roster;

use crate::error;
use crate::paths;

/// Converts a human-readable name into a lowercase alphanumeric `snake_case` slug.
#[must_use]
pub fn slugify(name: &str) -> String {
  let mut s = String::with_capacity(name.len());
  for c in name.chars() {
    match c {
      'a'..='z' | '0'..='9' => s.push(c),
      'A'..='Z' => s.push(c.to_ascii_lowercase()),
      '+' => {
        if !s.ends_with('_') && !s.is_empty() {
          s.push('_');
        }
        s.push_str("plus_");
      }
      ' ' | '-' | '\'' | '/' | '.' | '(' | ')'
        if !s.ends_with('_') && !s.is_empty() =>
      {
        s.push('_');
      }
      _ => {}
    }
  }
  while s.ends_with('_') {
    s.pop();
  }
  s
}

/// Converts a skill into a filesystem slug.
#[must_use]
pub fn skill_slug(skill: roster::Skill) -> String {
  slugify(skill.name())
}

/// Converts an item kind category into a static filesystem slug.
#[must_use]
pub const fn item_kind_slug(kind: convoy::ItemKind) -> &'static str {
  match kind {
    convoy::ItemKind::Sword => "sword",
    convoy::ItemKind::Lance => "lance",
    convoy::ItemKind::Axe => "axe",
    convoy::ItemKind::Bow => "bow",
    convoy::ItemKind::Tome => "tome",
    convoy::ItemKind::Staff => "staff",
    convoy::ItemKind::Stone => "stone",
    convoy::ItemKind::Consumable => "consumable",
    convoy::ItemKind::Seal => "seal",
    convoy::ItemKind::StatBooster => "stat_booster",
    convoy::ItemKind::Valuable => "valuable",
    convoy::ItemKind::Unknown => "unknown",
  }
}

/// Converts a concrete item into a filesystem slug.
#[must_use]
pub fn item_slug(item: convoy::Item) -> String {
  slugify(item.name())
}

/// Converts a class into a filesystem slug.
#[must_use]
pub fn class_slug(class: roster::Class) -> String {
  slugify(class.name())
}

/// Resolves the file path for a character's portrait image.
///
/// Looks first in the local portable `./assets/portraits/` directory,
/// then falls back to the user application data directory.
#[must_use]
pub fn resolve_portrait(character: roster::Character) -> Option<path::PathBuf> {
  let slug = character.slug();
  let filename = format!("{slug}.png");

  let portable = paths::portable_portraits_dir().join(&filename);
  if portable.exists() {
    return Some(portable);
  }

  let user = paths::portraits_dir().join(&filename);
  if user.exists() {
    return Some(user);
  }

  None
}

/// Resolves the file path for a skill icon image.
///
/// Looks first in `./assets/skills/`, then falls back to user data directory.
#[must_use]
pub fn resolve_skill_icon(skill: roster::Skill) -> Option<path::PathBuf> {
  if skill.is_none() {
    return None;
  }
  let slug = skill_slug(skill);
  let filename = format!("{slug}.png");

  let portable = paths::portable_skills_dir().join(&filename);
  if portable.exists() {
    return Some(portable);
  }

  let user = paths::skills_dir().join(&filename);
  if user.exists() {
    return Some(user);
  }

  None
}

/// Resolves the file path for an item or weapon icon image.
///
/// Looks first for the specific item slug in `./assets/items/{slug}.png`,
/// then falls back to category icon `./assets/items/{category}.png`.
#[must_use]
pub fn resolve_item_icon(item: convoy::Item) -> Option<path::PathBuf> {
  let slug = item_slug(item);
  let filename = format!("{slug}.png");

  let portable = paths::portable_items_dir().join(&filename);
  if portable.exists() {
    return Some(portable);
  }

  let user = paths::items_dir().join(&filename);
  if user.exists() {
    return Some(user);
  }

  // Fallback to category icon (e.g. sword.png)
  let cat_slug = item_kind_slug(item.kind());
  let cat_filename = format!("{cat_slug}.png");

  let portable_cat = paths::portable_items_dir().join(&cat_filename);
  if portable_cat.exists() {
    return Some(portable_cat);
  }

  let user_cat = paths::items_dir().join(&cat_filename);
  if user_cat.exists() {
    return Some(user_cat);
  }

  None
}

/// Resolves the file path for an animated map sprite sheet or image.
///
/// Looks first in `./assets/sprites/`, then falls back to user data directory.
#[must_use]
pub fn resolve_map_sprite(
  class: roster::Class,
  gender: roster::Gender,
) -> Option<path::PathBuf> {
  let slug = class_slug(class);
  let gender_suffix = match gender {
    roster::Gender::Male => "m",
    roster::Gender::Female => "f",
  };
  let filename = format!("{slug}_{gender_suffix}.png");

  let portable = paths::portable_sprites_dir().join(&filename);
  if portable.exists() {
    return Some(portable);
  }

  let user = paths::sprites_dir().join(&filename);
  if user.exists() {
    return Some(user);
  }

  // Fallback without gender suffix
  let fallback_name = format!("{slug}.png");
  let portable_fallback = paths::portable_sprites_dir().join(&fallback_name);
  if portable_fallback.exists() {
    return Some(portable_fallback);
  }

  let user_fallback = paths::sprites_dir().join(&fallback_name);
  if user_fallback.exists() {
    return Some(user_fallback);
  }

  None
}

/// Adjusts pixel RGB values matching a key color to the given target RGB.
///
/// Useful for dynamically recoloring hair or clothing in raw RGBA byte slices.
pub fn tint_pixels(
  pixels: &mut [u8],
  key_color: [u8; 3],
  target_rgb: [u8; 3],
  tolerance: u8,
) {
  let (chunks, _) = pixels.as_chunks_mut::<4>();
  for chunk in chunks {
    let r = chunk[0];
    let g = chunk[1];
    let b = chunk[2];
    let a = chunk[3];
    if a > 0 {
      let dr = r.abs_diff(key_color[0]);
      let dg = g.abs_diff(key_color[1]);
      let db = b.abs_diff(key_color[2]);
      if dr <= tolerance && dg <= tolerance && db <= tolerance {
        chunk[0] = target_rgb[0];
        chunk[1] = target_rgb[1];
        chunk[2] = target_rgb[2];
      }
    }
  }
}

/// Computes a two-character uppercase initials badge for a character name.
///
/// For multi-word names (e.g., "Robin (F)"), takes the first alphanumeric
/// letter of the first two words. For single-word names (e.g., "Cordelia"),
/// takes the first two alphanumeric characters.
#[must_use]
pub fn initials(name: &str) -> String {
  let cleaned_words: Vec<String> = name
    .split_whitespace()
    .filter_map(|w| {
      let filtered: String =
        w.chars().filter(|c| c.is_alphanumeric()).collect();
      if filtered.is_empty() {
        None
      } else {
        Some(filtered)
      }
    })
    .collect();

  if cleaned_words.len() >= 2 {
    let first = cleaned_words[0].chars().next().unwrap_or('?');
    let second = cleaned_words[1].chars().next().unwrap_or('?');
    format!(
      "{}{}",
      first.to_ascii_uppercase(),
      second.to_ascii_uppercase()
    )
  } else if let Some(first_word) = cleaned_words.first() {
    first_word
      .chars()
      .take(2)
      .collect::<String>()
      .to_uppercase()
  } else {
    String::new()
  }
}

/// Ensures all necessary asset directories exist on disk.
///
/// # Errors
///
/// Returns [`error::Error::Io`] if creating any asset directory fails.
pub fn ensure_asset_dirs() -> Result<path::PathBuf, error::Error> {
  let dir = paths::portraits_dir();
  fs::create_dir_all(&dir).map_err(|source| error::Error::Io {
    path: dir.clone(),
    source,
  })?;

  let skills = paths::skills_dir();
  fs::create_dir_all(&skills).map_err(|source| error::Error::Io {
    path: skills.clone(),
    source,
  })?;

  let items = paths::items_dir();
  fs::create_dir_all(&items).map_err(|source| error::Error::Io {
    path: items.clone(),
    source,
  })?;

  let sprites = paths::sprites_dir();
  fs::create_dir_all(&sprites).map_err(|source| error::Error::Io {
    path: sprites.clone(),
    source,
  })?;

  Ok(dir)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_slugify() {
    assert_eq!(slugify("Aether"), "aether");
    assert_eq!(slugify("HP +5"), "hp_plus_5");
    assert_eq!(slugify("Dual Strike+"), "dual_strike_plus");
    assert_eq!(slugify("Lon'qu"), "lon_qu");
    assert_eq!(slugify("Robin (F)"), "robin_f");
    assert_eq!(slugify("Bow Knight"), "bow_knight");
  }

  #[test]
  fn test_tint_pixels() {
    let mut pixels = [100, 150, 200, 255, 0, 0, 0, 255];
    tint_pixels(&mut pixels, [100, 150, 200], [255, 0, 0], 5);
    assert_eq!(&pixels[0..4], &[255, 0, 0, 255]);
    assert_eq!(&pixels[4..8], &[0, 0, 0, 255]);
  }

  #[test]
  fn test_resolve_portrait_non_existent() {
    let path = resolve_portrait(roster::Character::Chrom);
    assert!(path.is_none() || path.unwrap().ends_with("chrom.png"));
  }

  #[test]
  fn test_resolve_skill_none() {
    assert!(resolve_skill_icon(roster::Skill::None).is_none());
  }

  #[test]
  fn test_initials_formatting() {
    assert_eq!(initials("Cordelia"), "CO");
    assert_eq!(initials("Chrom"), "CH");
    assert_eq!(initials("Robin M"), "RM");
    assert_eq!(initials("Robin (F)"), "RF");
    assert_eq!(initials("Lucina"), "LU");
    assert_eq!(initials("A"), "A");
    assert_eq!(initials(""), "");
  }
}
