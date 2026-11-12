//! Nintendo 0x28 8-bit Huffman codecs.
//!
//! Owns stream decoding; callers own container headers.

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
