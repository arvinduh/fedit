//! In-game elapsed play time duration and formatting.
//!
//! Encapsulates duration counters in seconds without 24-hour wrap-around.

use std::fmt;
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

impl fmt::Display for PlayTime {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{:02}:{:02}:{:02}",
      self.hours(),
      self.minutes(),
      self.seconds()
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_play_time_formatting() {
    assert_eq!(PlayTime::from_secs(0).to_string(), "00:00:00");
    assert_eq!(PlayTime::from_secs(75).to_string(), "00:01:15");
    assert_eq!(PlayTime::from_secs(3661).to_string(), "01:01:01");
    assert_eq!(PlayTime::from_secs(360_000).to_string(), "100:00:00");
  }
}
