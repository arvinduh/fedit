# fedit

A modular Rust toolchain for Nintendo 3DS _Fire Emblem: Awakening_ save
archives.

## Usage

### Inspect Save Metadata

Inspect campaign gold, play time, avatar details, and CRC-32 integrity:

```bash
cargo run -p fedit-cli -- inspect Chapter0
cargo run -p fedit-cli -- inspect Global
```

### Inspect Character Roster

View all character slots in tabular form or as JSON:

```bash
cargo run -p fedit-cli -- roster Chapter0
cargo run -p fedit-cli -- roster Chapter0 --json
```

### Inspect & Mutate Character Stats

Target characters by case-insensitive name or roster index:

```bash
# View unit stats
cargo run -p fedit-cli -- unit Chapter0 --name frederick
cargo run -p fedit-cli -- unit Chapter0 --index 0

# Mutate stats with bounds validation and upfront backup
cargo run -p fedit-cli -- unit Chapter0 --name frederick --hp 80 --strength 40 --backup
```

### Inspect & Modify Convoy Storage

View or update convoy inventory durability:

```bash
# Inspect convoy
cargo run -p fedit-cli -- convoy Chapter0
cargo run -p fedit-cli -- convoy Chapter0 --json

# Update item uses
cargo run -p fedit-cli -- convoy Chapter0 --slot 0 --uses 25 --backup
```

### Edit Save Slot Parameters

Modify gold, avatar name, difficulty, or play time:

```bash
cargo run -p fedit-cli -- edit Chapter0 \
  --gold 999999 \
  --avatar-name Morgan \
  --difficulty lunatic \
  --play-time 150:00:00 \
  --backup
```

### Decompress & Repack Containers

Extract raw payload and preview sidecars, or repack modified payloads:

```bash
# Unpack PMOC container to raw payload
cargo run -p fedit-cli -- unpack Chapter0 -o payload.bin -p preview.bin --json

# Repack payload into PMOC container
cargo run -p fedit-cli -- repack payload.bin -p preview.bin -o repacked.sav
```

## Workspace Architecture

- `crates/fe-compress`: Zero-I/O Nintendo 0x28 Huffman codec, PMOC container,
  CRC-32.
- `crates/fe13`: In-memory domain models (`SaveSlot`, `GlobalSave`, `Preview`,
  `EDNI`, `Unit`, `Convoy`).
- `crates/fedit-cli`: Subcommand CLI host and atomic file persistence.
