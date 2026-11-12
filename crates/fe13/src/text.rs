//! UTF-16LE fixed-width string reading and writing helpers.
//!
//! Owns null-terminated UTF-16LE conversion for names and titles.

use std::char;

/// Reads a null-terminated UTF-16LE byte slice into a [`String`].
#[must_use]
pub fn read_utf16le(bytes: &[u8]) -> String {
  let iter = bytes
    .as_chunks::<2>()
    .0
    .iter()
    .map(|&c| u16::from_le_bytes(c))
    .take_while(|&u| u != 0);
  char::decode_utf16(iter)
    .map(|r| r.unwrap_or(char::REPLACEMENT_CHARACTER))
    .collect()
}

/// Encodes a UTF-8 string into a zero-padded UTF-16LE buffer.
pub fn write_utf16le(buf: &mut [u8], text: &str) {
  buf.fill(0);
  let mut offset = 0;
  for code_point in text.encode_utf16() {
    if offset + 2 > buf.len() {
      break;
    }
    let bytes = code_point.to_le_bytes();
    buf[offset..offset + 2].copy_from_slice(&bytes);
    offset += 2;
  }
}
