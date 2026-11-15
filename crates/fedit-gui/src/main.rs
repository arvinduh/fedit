//! Native desktop GUI for the Fire Emblem: Awakening save editor.
//!
//! Provides the primary desktop application window with split layout,
//! embedding the unit selector sidebar and detailed editor views.

use std::env;
use std::fs;
use std::path::PathBuf;

use awakening;
use iced::widget::{column, container, row, text};
use iced::{Element, Length};

mod components;

use components::UnitSelector;
use components::UnitSelectorMessage;

/// Top-level application state.
struct App {
  /// Unit selector sidebar component.
  selector: UnitSelector,
  /// Loaded save file path.
  _file_path: Option<PathBuf>,
}

/// Messages handled by the top-level application.
#[derive(Debug, Clone, Copy)]
enum Message {
  /// Message routed to the unit selector component.
  Selector(UnitSelectorMessage),
}

impl App {
  /// Initializes the application state, loading save data if available.
  fn new() -> Self {
    let path = env::args().nth(1).map_or_else(
      || PathBuf::from("tests/fixtures/20200831-225628/Chapter0"),
      PathBuf::from,
    );

    let units = fs::read(&path)
      .ok()
      .and_then(|bytes| awakening::SaveSlot::from_bytes(&bytes).ok())
      .map(|slot| slot.roster.to_vec())
      .unwrap_or_default();

    Self {
      selector: UnitSelector::new(units),
      _file_path: Some(path),
    }
  }

  /// Updates application state upon receiving a message.
  fn update(&mut self, message: Message) {
    match message {
      Message::Selector(msg) => self.selector.update(msg),
    }
  }

  /// Renders the main application view.
  fn view(&self) -> Element<'_, Message> {
    let sidebar = container(self.selector.view().map(Message::Selector))
      .width(280)
      .height(Length::Fill)
      .padding(10);

    let details = self.view_details();

    row![sidebar, details].into()
  }

  /// Renders details for the currently selected unit.
  fn view_details(&self) -> Element<'_, Message> {
    if let Some(unit) = self.selector.selected_unit() {
      let name = unit.name();
      let s = &unit.stats;
      let header: Element<'_, Message> = if let Some(path) =
        fedit_core::assets::resolve_portrait(unit.character())
      {
        row![
          iced::widget::image(path).width(64).height(64),
          column![
            text(format!("Unit: {name}")).size(24),
            text(format!("Class: {}", unit.class())).size(16),
          ]
          .spacing(4)
        ]
        .spacing(16)
        .align_y(iced::Alignment::Center)
        .into()
      } else {
        column![
          text(format!("Unit: {name}")).size(24),
          text(format!("Class: {}", unit.class())).size(16),
        ]
        .spacing(4)
        .into()
      };

      container(
        column![
          header,
          column![
            text(format!("HP:         {}", s.hp)),
            text(format!("Strength:   {}", s.strength)),
            text(format!("Magic:      {}", s.magic)),
            text(format!("Skill:      {}", s.skill)),
            text(format!("Speed:      {}", s.speed)),
            text(format!("Luck:       {}", s.luck)),
            text(format!("Defense:    {}", s.defense)),
            text(format!("Resistance: {}", s.resistance)),
          ]
          .spacing(4)
        ]
        .spacing(12)
        .padding(20),
      )
      .width(Length::Fill)
      .height(Length::Fill)
      .into()
    } else {
      container(text("No unit selected").size(18))
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .into()
    }
  }
}

/// Entry point initializing and launching the desktop GUI.
fn main() -> iced::Result {
  iced::application(App::new, App::update, App::view)
    .title("Fire Emblem: Awakening Save Editor")
    .run()
}
