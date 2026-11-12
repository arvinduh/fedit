//! Nintendo 0x28 8-bit Huffman codecs.
//!
//! Owns stream decoding; callers own container headers.

pub mod tree;

use crate::error;

/// Decompresses a Nintendo 0x28 8-bit Huffman stream.
///
/// # Errors
///
/// Fails on bad header, corrupt tree, or length mismatch.
pub fn decompress(s: &[u8]) -> Result<Vec<u8>, error::Error> {
  let byte = s.first().copied().unwrap_or(0);
  if byte != 0x28 || s.len() < 5 {
    return Err(error::Error::InvalidHuffmanHeader { byte });
  }
  let u_len = u32::from_le_bytes([s[1], s[2], s[3], 0]) as usize;
  let bit_off = (usize::from(s[4]) * 2 + 9) & !3;
  let (mut out, mut node) = (Vec::with_capacity(u_len), 5);
  let words = s.get(bit_off..).ok_or(error::Error::CorruptHuffmanTree)?;
  for &c in words.as_chunks::<4>().0 {
    let word = u32::from_le_bytes(c);
    for shift in (0..32).rev() {
      let bit = ((word >> shift) & 1) as usize;
      let nb = *s.get(node).ok_or(error::Error::CorruptHuffmanTree)?;
      let next = (node & !1) + usize::from(nb & 0x3F) * 2 + 2 + bit;
      if next >= bit_off {
        return Err(error::Error::CorruptHuffmanTree);
      }
      if (nb & (0x80 >> bit)) != 0 {
        out.push(s[next]);
        node = 5;
        if out.len() == u_len {
          return Ok(out);
        }
      } else {
        node = next;
      }
    }
  }
  if out.len() == u_len {
    Ok(out)
  } else {
    Err(error::Error::LengthMismatch {
      expected: u_len,
      actual: out.len(),
    })
  }
}

/// Compresses a raw byte payload into a Nintendo 0x28 8-bit Huffman stream.
///
/// # Errors
///
/// Fails if payload length overflows 24 bits or tree building fails.
pub fn compress(payload: &[u8]) -> Result<Vec<u8>, error::Error> {
  let (arena, root) = tree::build_tree(payload);
  let tree_table = tree::build_tree_table(&arena, root)?;
  let codes = tree::build_codes(&arena, root);
  let u_len = payload.len();
  let len_bytes = u32::try_from(u_len)
    .map_err(|_| error::Error::CorruptHuffmanTree)?
    .to_le_bytes();
  let mut out = Vec::with_capacity(4 + tree_table.len() + payload.len());
  out.push(0x28);
  out.extend_from_slice(&len_bytes[..3]);
  out.extend_from_slice(&tree_table);
  let (mut acc, mut bit_count) = (0u32, 0u8);
  for &b in payload {
    let code = codes[usize::from(b)];
    for i in (0..code.len).rev() {
      acc = (acc << 1) | ((code.bits >> i) & 1);
      bit_count += 1;
      if bit_count == 32 {
        out.extend_from_slice(&acc.to_le_bytes());
        acc = 0;
        bit_count = 0;
      }
    }
  }
  if bit_count > 0 {
    acc <<= 32 - bit_count;
    out.extend_from_slice(&acc.to_le_bytes());
  }
  Ok(out)
}
