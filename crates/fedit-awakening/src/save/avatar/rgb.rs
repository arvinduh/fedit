//! RGB color representation for avatar cosmetic customization.
//!
//! Provides a zero-cost 24-bit color value object for Robin's hair palette,
//! supporting conversions to/from raw 3-byte arrays and hexadecimal strings.

use thiserror;

/// Errors that can occur when parsing RGB colors.
#[derive(Clone, Copy, Debug, Eq, PartialEq, thiserror::Error)]
pub enum Error {
  /// Hex color string was invalid.
  #[error("invalid hex color string")]
  InvalidHexColor,
  /// Hex string length was invalid (expected 6 or 7 with '#').
  #[error("invalid hex string length: {len} (expected 6 or 7)")]
  InvalidHexLength {
    /// Provided length.
    len: usize,
  },
}

/// 24-bit RGB color representation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Rgb {
  /// Red channel (0..=255).
  pub r: u8,
  /// Green channel (0..=255).
  pub g: u8,
  /// Blue channel (0..=255).
  pub b: u8,
}

impl Rgb {
  /// Constructs a new [`Rgb`] value from individual channel bytes.
  #[must_use]
  pub const fn new(r: u8, g: u8, b: u8) -> Self {
    Self { r, g, b }
  }

  /// Converts the color into a raw 3-byte array `[R, G, B]`.
  #[must_use]
  pub const fn to_raw(self) -> [u8; 3] {
    [self.r, self.g, self.b]
  }

  /// Constructs an [`Rgb`] value from a raw 3-byte array `[R, G, B]`.
  #[must_use]
  pub(crate) const fn from_raw(raw: [u8; 3]) -> Self {
    Self {
      r: raw[0],
      g: raw[1],
      b: raw[2],
    }
  }

  /// Returns the lowercase hexadecimal representation `"#rrggbb"`.
  #[must_use]
  pub fn to_hex(self) -> String {
    format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
  }

  /// Parses an [`Rgb`] value from a hex string (e.g. `"#RRGGBB"` or `"RRGGBB"`).
  ///
  /// # Errors
  ///
  /// Returns [`Error::InvalidHexLength`] if the string is not 6 characters (or 7 with '#'),
  /// or [`Error::InvalidHexColor`] if hex decoding fails.
  pub fn from_hex(hex: &str) -> Result<Self, Error> {
    let s = hex.strip_prefix('#').unwrap_or(hex);
    if s.len() != 6 {
      return Err(Error::InvalidHexLength { len: hex.len() });
    }
    let r =
      u8::from_str_radix(&s[0..2], 16).map_err(|_| Error::InvalidHexColor)?;
    let g =
      u8::from_str_radix(&s[2..4], 16).map_err(|_| Error::InvalidHexColor)?;
    let b =
      u8::from_str_radix(&s[4..6], 16).map_err(|_| Error::InvalidHexColor)?;
    Ok(Self { r, g, b })
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_rgb_raw_roundtrip() {
    let rgb = Rgb::new(0x12, 0x34, 0x56);
    assert_eq!(rgb.to_raw(), [0x12, 0x34, 0x56]);
    assert_eq!(Rgb::from_raw([0x12, 0x34, 0x56]), rgb);
  }

  #[test]
  fn test_rgb_hex_roundtrip() {
    let rgb = Rgb::new(255, 128, 0);
    assert_eq!(rgb.to_hex(), "#ff8000");

    let parsed_with_hash = Rgb::from_hex("#ff8000").expect("parse with hash");
    assert_eq!(parsed_with_hash, rgb);

    let parsed_without_hash =
      Rgb::from_hex("FF8000").expect("parse without hash");
    assert_eq!(parsed_without_hash, rgb);
  }

  #[test]
  fn test_rgb_invalid_hex() {
    assert_eq!(
      Rgb::from_hex("#12345"),
      Err(Error::InvalidHexLength { len: 6 })
    );
    assert_eq!(
      Rgb::from_hex("#1234567"),
      Err(Error::InvalidHexLength { len: 8 })
    );
    assert_eq!(Rgb::from_hex("nothex"), Err(Error::InvalidHexColor));
  }
}
