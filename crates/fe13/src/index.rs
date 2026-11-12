//! EDNI master block index parser.
//!
//! Resolves block offsets for subsystem data within the save payload.

/// Preview offset subtracted from table entries.
pub const PREVIEW_OFFSET: usize = 0xC0;

/// Block offsets within a decompressed save payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct EdniIndex {
  /// RESU user campaign state offset.
  pub user_offset: usize,
  /// PAMG game state offset.
  pub pamg_offset: usize,
  /// TINU unit roster offset.
  pub unit_offset: usize,
  /// IFER block offset.
  pub ifer_offset: usize,
  /// NART convoy storage offset.
  pub convoy_offset: usize,
  /// 62UD support table offset.
  pub d62_offset: usize,
  /// TSVE event state offset.
  pub tsve_offset: usize,
}
