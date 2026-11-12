//! Huffman tree data structures and frequency counters.
//!
//! Owns node representations and symbol code types for tree building.

/// Bit code representation for an 8-bit symbol.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Code {
  /// Code bits packed MSB-first.
  pub bits: u32,
  /// Bit length of the code.
  pub len: u8,
}

/// A binary tree node in the flat tree arena.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Node {
  /// Leaf node carrying an 8-bit symbol.
  Leaf {
    /// Symbol value.
    symbol: u8,
  },
  /// Internal node pointing to left and right arena indices.
  Internal {
    /// Arena index of left child (bit 0).
    left: usize,
    /// Arena index of right child (bit 1).
    right: usize,
  },
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_node_and_code() {
    let leaf = Node::Leaf { symbol: 0x42 };
    let internal = Node::Internal { left: 0, right: 1 };
    assert_ne!(leaf, internal, "leaf and internal are distinct");
    let code = Code {
      bits: 0b101,
      len: 3,
    };
    assert_eq!(code.len, 3, "code len");
  }
}
