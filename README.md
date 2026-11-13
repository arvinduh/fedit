# fedit

A modular Rust toolchain for Nintendo 3DS *Fire Emblem: Awakening* save archives.

## Usage

Inspect save slot headers, campaign gold, play time, and CRC-32 integrity:

```bash
cargo run -p fedit-cli -- inspect Chapter0
cargo run -p fedit-cli -- inspect Global
```

Decompress save payload and preview sidecar to disk:

```bash
cargo run -p fedit-cli -- unpack Chapter0 -o payload.bin -p preview.bin --json
```

Repack raw payload into a compressed `PMOC` container:

```bash
cargo run -p fedit-cli -- repack payload.bin -p preview.bin -o repacked.sav
```

Mutate save parameters in-place:

```bash
cargo run -p fedit-cli -- edit Chapter0 --gold 999999 --avatar-name Morgan
```

## Workspace Architecture

- `crates/fe-compress`: Zero-I/O Nintendo 0x28 Huffman codec, PMOC header, CRC-32.
- `crates/fe13`: In-memory save domain models (Preview, EDNI, RESU, TINU, NART).
- `crates/fedit-cli`: CLI binary and process host.
