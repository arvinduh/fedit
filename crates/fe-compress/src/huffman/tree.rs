//! Huffman tree data structures and frequency counters.
//!
//! Owns node representations and symbol code types for tree building.

use std::cmp;
use std::collections;

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

/// Builds a Huffman tree arena and root index from payload frequencies.
#[must_use]
pub fn build_tree(payload: &[u8]) -> (Vec<Node>, usize) {
  let mut freq = [0u32; 256];
  for &b in payload {
    freq[usize::from(b)] += 1;
  }
  let (mut heap, mut arena) = (collections::BinaryHeap::new(), Vec::new());
  for sym in 0u8..=255 {
    let count = freq[usize::from(sym)];
    if count > 0 {
      let idx = arena.len();
      arena.push(Node::Leaf { symbol: sym });
      heap.push(cmp::Reverse((count, idx, idx)));
    }
  }
  for sym in [0u8, 1u8] {
    if heap.len() >= 2 {
      break;
    }
    if freq[usize::from(sym)] == 0 {
      let idx = arena.len();
      arena.push(Node::Leaf { symbol: sym });
      heap.push(cmp::Reverse((0, idx, idx)));
    }
  }
  while heap.len() > 1 {
    let (Some(cmp::Reverse((w1, _, n1))), Some(cmp::Reverse((w2, _, n2)))) =
      (heap.pop(), heap.pop())
    else {
      break;
    };
    let idx = arena.len();
    arena.push(Node::Internal {
      left: n1,
      right: n2,
    });
    heap.push(cmp::Reverse((w1 + w2, idx, idx)));
  }
  let root = heap.pop().map_or(0, |cmp::Reverse((_, _, n))| n);
  (arena, root)
}

/// Builds bit code representation for all 256 symbols from tree arena.
#[must_use]
pub fn build_codes(arena: &[Node], root: usize) -> [Code; 256] {
  let mut codes = [Code::default(); 256];
  fill_codes(arena, root, 0, 0, &mut codes);
  codes
}

/// Recursively traverses tree to populate symbol codes.
fn fill_codes(
  arena: &[Node],
  idx: usize,
  bits: u32,
  len: u8,
  codes: &mut [Code; 256],
) {
  match arena.get(idx) {
    Some(Node::Leaf { symbol }) => {
      codes[usize::from(*symbol)] = Code {
        bits,
        len: len.max(1),
      };
    }
    Some(Node::Internal { left, right }) => {
      fill_codes(arena, *left, bits << 1, len + 1, codes);
      fill_codes(arena, *right, (bits << 1) | 1, len + 1, codes);
    }
    None => {}
  }
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

  #[test]
  fn test_build_tree_and_codes() {
    let (arena, root) = build_tree(b"hello huffman");
    let codes = build_codes(&arena, root);
    assert!(codes[usize::from(b'h')].len > 0, "symbol h encoded");
  }
}
