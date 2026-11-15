//! Scrollable roster unit selector component.
//!
//! Provides a list view of character units with face avatars and summaries,
//! managing active selection state.

use awakening::roster;
use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length};

/// Messages emitted by the unit selector component.
#[derive(Debug, Clone, Copy)]
pub enum UnitSelectorMessage {
  /// A unit at the specified roster index was selected.
  Selected(usize),
}

/// Scrollable list selector for character units.
pub struct UnitSelector {
  /// Index of the currently selected unit in the roster.
  pub selected: Option<usize>,
  /// List of character units in the active roster.
  pub units: Vec<roster::Unit>,
}

impl UnitSelector {
  /// Constructs a new unit selector from a roster list.
  #[must_use]
  pub fn new(units: Vec<roster::Unit>) -> Self {
    let selected = if units.is_empty() { None } else { Some(0) };
    Self { selected, units }
  }

  /// Updates the selection state upon receiving a message.
  pub fn update(&mut self, message: UnitSelectorMessage) {
    match message {
      UnitSelectorMessage::Selected(index) => {
        if index < self.units.len() {
          self.selected = Some(index);
        }
      }
    }
  }

  /// Returns a reference to the currently selected unit.
  #[must_use]
  pub fn selected_unit(&self) -> Option<&roster::Unit> {
    self.selected.and_then(|idx| self.units.get(idx))
  }

  /// Renders a face avatar image or falls back to an initials badge.
  fn render_avatar<'a>(
    character: roster::Character,
  ) -> Element<'a, UnitSelectorMessage> {
    if let Some(path) = fedit_core::assets::resolve_portrait(character) {
      container(iced::widget::image(path).width(36).height(36))
        .width(36)
        .height(36)
        .into()
    } else {
      let initials: String = character.name().chars().take(2).collect();
      container(text(initials).size(14))
        .width(36)
        .height(36)
        .center_x(36)
        .center_y(36)
        .into()
    }
  }

  /// Renders the scrollable unit selector view.
  #[must_use]
  pub fn view(&self) -> Element<'_, UnitSelectorMessage> {
    let mut items = column![].spacing(6).width(Length::Fill);

    for (index, unit) in self.units.iter().enumerate() {
      let is_selected = self.selected == Some(index);
      let name = unit.name();

      let avatar = Self::render_avatar(unit.character());

      let info = column![
        text(name).size(15),
        text(format!("Class: {} | HP: {}", unit.class(), unit.stats.hp))
          .size(12),
      ]
      .spacing(2);

      let content = row![avatar, info].spacing(10).align_y(Alignment::Center);

      let item_btn = button(content)
        .width(Length::Fill)
        .padding([8, 12])
        .style(if is_selected {
          button::primary
        } else {
          button::secondary
        })
        .on_press(UnitSelectorMessage::Selected(index));

      items = items.push(item_btn);
    }

    scrollable(items).height(Length::Fill).into()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  fn make_mock_unit(id: u16) -> roster::Unit {
    roster::Unit::new(
      roster::Character::from_raw(id),
      roster::Class::from_raw(1),
      roster::UnitStats::default(),
      0,
    )
  }

  #[test]
  fn test_selector_empty_roster() {
    let selector = UnitSelector::new(Vec::new());
    assert_eq!(selector.selected, None);
    assert_eq!(selector.selected_unit(), None);
  }

  #[test]
  fn test_selector_with_units() {
    let units = vec![make_mock_unit(3), make_mock_unit(4)];
    let selector = UnitSelector::new(units);
    assert_eq!(selector.selected, Some(0));
    assert_eq!(selector.selected_unit().map(roster::Unit::raw_id), Some(3));
  }

  #[test]
  fn test_selector_update_selection() {
    let units = vec![make_mock_unit(3), make_mock_unit(4)];
    let mut selector = UnitSelector::new(units);
    selector.update(UnitSelectorMessage::Selected(1));
    assert_eq!(selector.selected, Some(1));
    assert_eq!(selector.selected_unit().map(roster::Unit::raw_id), Some(4));
  }

  #[test]
  fn test_selector_ignore_out_of_bounds() {
    let units = vec![make_mock_unit(1)];
    let mut selector = UnitSelector::new(units);
    selector.update(UnitSelectorMessage::Selected(99));
    assert_eq!(selector.selected, Some(0));
  }
}
