//! Individual item inventory slots with uses and forge data.
//!
//! Encapsulates an item entry in personal inventory or convoy storage, tracking
//! durability, forged modifications, and broken weapon states.

use crate::error;

use super::catalog::Item;
use super::forge::Forge;

/// An individual item slot within a unit's inventory or convoy storage.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ItemSlot {
  /// Base item enumeration.
  item: Item,
  /// Current remaining durability uses or stack count.
  uses: u16,
  /// Optional forge modifications.
  forge: Option<Forge>,
}

impl ItemSlot {
  /// Creates a new un-forged item slot with default canonical durability.
  #[must_use]
  pub fn new(item: Item) -> Self {
    let uses = u16::from(item.max_durability().unwrap_or(1));
    Self {
      item,
      uses,
      forge: None,
    }
  }

  /// Creates an item slot with a specified durability uses count.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::DurabilityOverflow`] if `uses > 4095`.
  pub fn with_uses(item: Item, uses: u16) -> Result<Self, error::Error> {
    if uses > 0x0fff {
      return Err(error::Error::DurabilityOverflow { uses, max: 0x0fff });
    }
    Ok(Self {
      item,
      uses,
      forge: None,
    })
  }

  /// Returns the base item variant.
  #[must_use]
  pub const fn item(&self) -> Item {
    self.item
  }

  /// Returns the remaining uses count.
  #[must_use]
  pub const fn uses(&self) -> u16 {
    self.uses
  }

  /// Returns a reference to the forge modifications, if any.
  #[must_use]
  pub const fn forge(&self) -> Option<&Forge> {
    self.forge.as_ref()
  }

  /// Returns a mutable reference to the forge modifications, if any.
  pub fn forge_mut(&mut self) -> Option<&mut Forge> {
    self.forge.as_mut()
  }

  /// Sets the base item variant, resetting uses to canonical max if applicable.
  pub fn set_item(&mut self, item: Item) {
    self.item = item;
    if let Some(max) = item.max_durability()
      && self.uses > u16::from(max)
    {
      self.uses = u16::from(max);
    }
  }

  /// Sets the remaining uses count with bounds validation.
  ///
  /// # Errors
  ///
  /// Returns [`error::Error::DurabilityOverflow`] if `uses > 4095`.
  pub fn set_uses(&mut self, uses: u16) -> Result<(), error::Error> {
    if uses > 0x0fff {
      return Err(error::Error::DurabilityOverflow { uses, max: 0x0fff });
    }
    self.uses = uses;
    Ok(())
  }

  /// Sets the forge modifications on this item slot.
  pub fn set_forge(&mut self, forge: Option<Forge>) {
    self.forge = forge;
  }

  /// Repairs the item back to its canonical maximum durability.
  pub fn repair(&mut self) {
    if let Some(max) = self.item.max_durability() {
      self.uses = u16::from(max);
    }
  }

  /// Returns true if the weapon is broken (0 uses and not unbreakable).
  #[must_use]
  pub const fn is_broken(&self) -> bool {
    self.uses == 0 && !self.item.is_unbreakable()
  }
}
