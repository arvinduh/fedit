//! In-game elapsed play time duration and formatting.
//!
//! Encapsulates duration counters in seconds without 24-hour wrap-around.

use std::time;

/// In-game elapsed play time duration.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct PlayTime {
  /// Backing duration.
  duration: time::Duration,
}

impl PlayTime {
  /// Creates a new `PlayTime` from total seconds elapsed.
  #[must_use]
  pub const fn from_secs(secs: u64) -> Self {
    Self {
      duration: time::Duration::from_secs(secs),
    }
  }

  /// Returns total elapsed seconds.
  #[must_use]
  pub const fn as_secs(&self) -> u64 {
    self.duration.as_secs()
  }

  /// Returns total hours elapsed.
  #[must_use]
  pub const fn hours(&self) -> u64 {
    self.duration.as_secs() / 3600
  }

  /// Returns minutes component (0..=59).
  #[must_use]
  pub const fn minutes(&self) -> u8 {
    ((self.duration.as_secs() % 3600) / 60) as u8
  }

  /// Returns seconds component (0..=59).
  #[must_use]
  pub const fn seconds(&self) -> u8 {
    (self.duration.as_secs() % 60) as u8
  }
}
