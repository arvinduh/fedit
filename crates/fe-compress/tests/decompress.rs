//! Integration tests for decompression on real fixtures.

use fe_compress::huffman;
use fe_compress::pmoc;

const CHAPTER0: &[u8] =
  include_bytes!("../../../tests/fixtures/20200831-225628/Chapter0");
const GLOBAL: &[u8] =
  include_bytes!("../../../tests/fixtures/20200831-225628/Global");

#[test]
fn test_decompress_chapter0() {
  let preview = &CHAPTER0[..0xC0];
  let header = pmoc::Header::parse(&CHAPTER0[0xC0..0xD0]).expect("pmoc header");
  assert_eq!(header.decompressed_len, 12_298, "chapter0 payload length");
  let decompressed =
    huffman::decompress(&CHAPTER0[0xD0..]).expect("decompress");
  assert_eq!(decompressed.len(), 12_298, "decompressed len");
  assert_eq!(&decompressed[..4], b"EDNI", "starts with EDNI magic");
  let crc = pmoc::compute_crc32(preview, &decompressed);
  assert_eq!(crc, 0xd370_51d6, "chapter0 crc");
}

#[test]
fn test_decompress_global() {
  let header = pmoc::Header::parse(&GLOBAL[..0x10]).expect("pmoc header");
  assert_eq!(header.decompressed_len, 1_425, "global payload length");
  let decompressed = huffman::decompress(&GLOBAL[0x10..]).expect("decompress");
  assert_eq!(decompressed.len(), 1_425, "decompressed len");
  let crc = pmoc::compute_crc32(&[], &decompressed);
  assert_eq!(crc, 0xa588_bf86, "global crc");
}
