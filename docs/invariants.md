# Fire Emblem: Awakening Domain Invariants

This document specifies the core invariant rules enforced across `fedit` domain
models, save file serialization, unit mutation, and game state transitions.

## 1. Character & Biological Gender Invariants

- **Fixed Gender:** 63 out of 65 playable characters possess immutable
  biological gender (e.g. Chrom is strictly male; Lucina is strictly female).
- **Variable Gender:** Exactly two characters possess mutable gender: Robin
  (avatar) and Morgan (child).
- **Inverse Avatar Child Invariant:** Morgan's biological gender is strictly the
  opposite of Robin's biological gender.
- **Save File Identifiers (PID):**
  - Robin: `0` (Male), `1` (Female).
  - Morgan: `39` (Male), `40` (Female).
  - Calling `Character::raw_id()` on Robin or Morgan returns `None` to prevent
    ambiguous lookups; caller must query `Unit::raw_id()` or pass Robin's avatar
    gender via `Character::raw_id_for_avatar_gender()`.

## 2. Story Anchors & Permadeath Safeguards

- **Story Anchors:** Chrom, Robin, and Lucina are designated story anchor units
  required for cutscene progression and game-over trigger checks.
- **Permadeath Protection:** Story anchor units can never be assigned
  `UnitStatus::Dead`. Attempting to kill an anchor unit returns
  `Error::AnchorUnitProtected`.
- **Casual Mode Invariant:** Under Casual game mode, downed units transition to
  `UnitStatus::Active` at the conclusion of battle; permadeath is disabled.
- **Force Deployment:** Chrom and Robin are mandatory deployments in main story
  chapters.

## 3. Class Eligibility & Equipment Constraints

- **Gender-Exclusive Classes:**
  - Male-only: Barbarian, Berserker, Fighter, Warrior, Dread Fighter.
  - Female-only: Pegasus Knight, Falcon Knight, Dark Flier, Troubadour,
    Valkyrie, Dancer, Bride.
- **Gender-Paired Auto-Resolution:** Mutating a unit's biological gender
  automatically resolves reciprocal class pairs (e.g. Priest <-> Cleric,
  War Monk <-> War Cleric). If a class is exclusive and has no reciprocal
  equivalent, the reclassing is rejected.
- **Active Weapon Auto-Unequip:** When a unit's class changes, any currently
  equipped weapon that is incompatible with the new class's weapon proficiencies
  is automatically unequipped to prevent battle animation engine crashes.
- **Single Equipped Weapon Invariant:** At most one weapon slot can have the
  equipped flag (`0x1000`) set at any time.

## 4. Paralogues & Child Unit Lifecycle

- **Chapter 13 Gate:** Child paralogues (Paralogues 5 through 16) cannot be
  unlocked before Chapter 13 is cleared. Attempting to set them to `Unlocked`
  before Chapter 13 returns `Error::ChildParalogueGatedByChapter13`.
- **Parental S-Rank Prerequisite:** A child paralogue cannot be unlocked unless
  the child's canonical primary parent (mother, or Chrom for Lucina) has an
  active S-Rank marriage in the support table.
- **Roster Presence Invariant:** A completed child paralogue cannot be reverted
  to `Locked` or `Unlocked` if the corresponding child unit currently exists
  within the player's active roster (`Error::ChildInRosterParalogueConflict`).
  The child unit must be removed from the roster prior to resetting stage
  progression.

## 5. Support Affinity & Marriage Bounds

- **Monogamy Invariant:** A character may hold an S-Rank romantic support with
  at most one spouse simultaneously.
- **Support Pair Eligibility:** Only canonically compatible character pairs
  can establish support ranks. Incompatible pairs return
  `Error::IncompatibleSupportPair`.

## 6. Numerical Boundaries & Growth Caps

- **Hit Points:** Maximum HP is bounded between 10 and 99. Dead units have 0 HP.
- **Combat Stats:** Base combat attributes (Strength, Magic, Skill, Speed, Luck,
  Defense, Resistance) are bounded between 0 and 80.
- **Stat Growths:** Level-up stat growth bonuses are bounded between 0 and 255.
- **Weapon Ranks:** Weapon proficiency exp is bounded between 0 and 255 (Rank A
  corresponds to 90 exp).
- **Item Durability:** Uses byte is masked to 12 bits (`0..=4095`). Values
  exceeding 4095 return `Error::DurabilityOverflow`.
