# fedit

A modular Rust toolchain for Nintendo 3DS _Fire Emblem: Awakening_ save
archives.

## Usage

### Global Options

- `-v, --verbose`: Increase logging verbosity (`-v` = info, `-vv` = debug,
  `-vvv` = trace).
- `-q, --quiet`: Silence non-error output.

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

### Launch Desktop Graphical Interface

Launch the Tauri desktop UI to visually inspect and edit campaign saves:

```bash
# Launch empty interface (use Open Folder / Open File in toolbar)
cargo run -p fedit-gui
```

### Visual Assets & Custom Sprites

The desktop application resolves visual assets from either a portable local
`./assets/` folder (adjacent to the executable or repository root) or the system
data directory (`%LOCALAPPDATA%\fedit\assets` on Windows,
`~/.local/share/fedit/assets` on Linux,
`~/Library/Application Support/fedit/assets` on macOS):

- `assets/portraits/{slug}.png`: Character portrait avatars (e.g. `chrom.png`).
- `assets/skills/{slug}.png`: Skill badges (e.g. `aether.png`, `hp_plus_5.png`).
- `assets/items/{slug}.png`: Weapon and item icons (e.g. `falchion.png`,
  `sword.png`).
- `assets/sprites/{slug}_{m|f}.png`: Class map sprites (e.g. `lord_m.png`),
  recolored dynamically with the character's or Avatar's custom hair RGB.

If assets are not present on disk, the interface gracefully falls back to
clean vector initials badges and category glyphs.

## Workspace Architecture

- `crates/fedit-compress`: Zero-I/O Nintendo 0x28 Huffman codec, PMOC container,
  CRC-32.
- `crates/fedit-core`: Shared types, asset resolution, and save directory
  abstractions.
- `crates/fedit-awakening`: FE13 domain models (`SaveSlot`, `GlobalSave`,
  `Preview`, `EDNI`, `Unit`, `Convoy`, `Paralogue`, `AvatarCustomization`).
- `crates/fedit-cli`: Subcommand CLI host and atomic file persistence.
- `crates/fedit-gui`: Desktop graphical interface built on Tauri v2 and Svelte 5
  with Material 3 Expressive motion and typography.
- `docs/invariants.md`: System-wide invariants, data constraints, and domain
  rules.
