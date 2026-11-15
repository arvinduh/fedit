<script lang="ts">
  import type { UnitDto } from "../lib/types";
  import { popIn } from "../lib/motion";
  import StatStepper from "../components/StatStepper.svelte";
  import CollapsibleCard from "../components/CollapsibleCard.svelte";

  interface Props {
    units: UnitDto[];
    onUnitChange: (updated: UnitDto) => void;
  }

  let { units, onUnitChange }: Props = $props();

  let selectedIndex = $state(0);
  let searchQuery = $state("");

  const selectedUnit = $derived(units[selectedIndex] ?? units[0]);

  const filteredUnits = $derived(
    units.filter(
      (u) =>
        u.name.toLowerCase().includes(searchQuery.toLowerCase()) ||
        u.className.toLowerCase().includes(searchQuery.toLowerCase())
    )
  );

  const STAT_LABELS = ["HP", "STR", "MAG", "SKL", "SPD", "LCK", "DEF", "RES"];

  function handleStatChange(statIndex: number, newVal: number) {
    if (!selectedUnit) return;
    const newStats = [...selectedUnit.stats] as [
      number,
      number,
      number,
      number,
      number,
      number,
      number,
      number,
    ];
    newStats[statIndex] = newVal;
    const updated = {
      ...selectedUnit,
      stats: newStats,
      curHp: statIndex === 0 ? newVal : selectedUnit.curHp,
    };
    onUnitChange(updated);
  }
</script>

<div class="roster-view">
  <!-- Left Column: Master Roster List -->
  <aside class="roster-sidebar">
    <div class="search-box">
      <input
        type="text"
        placeholder="Search roster..."
        bind:value={searchQuery}
        class="search-input"
      />
    </div>

    <div class="units-list">
      {#each filteredUnits as unit (unit.id)}
        {@const isSelected = selectedUnit?.id === unit.id}
        <button
          type="button"
          class="unit-item"
          class:selected={isSelected}
          onclick={() => {
            const idx = units.findIndex((u) => u.id === unit.id);
            if (idx !== -1) selectedIndex = idx;
          }}
        >
          <div class="avatar-wrapper">
            <img
              src={`./portraits/${unit.slug}.png`}
              alt={unit.name}
              class="unit-avatar"
              onerror={(e) => {
                (e.currentTarget as HTMLElement).style.display = "none";
              }}
            />
          </div>

          <div class="unit-info">
            <div class="unit-name-row">
              <span class="unit-name">{unit.name}</span>
              <span class="unit-lv">Lv.{unit.level}</span>
            </div>
            <div class="unit-sub-row">
              <span class="unit-class">{unit.className}</span>
              <span class="unit-hp-stat">{unit.curHp} HP</span>
            </div>
          </div>
        </button>
      {/each}
    </div>
  </aside>

  <!-- Right Column: Tactical Detail Dossier -->
  <main class="dossier-main">
    {#if selectedUnit}
      {#key selectedUnit.id}
        <div class="dossier-content" in:popIn>
          <!-- Hero Identity Card -->
          <section class="hero-card">
            <div class="hero-left">
              <img
                src={`./portraits/${selectedUnit.slug}.png`}
                alt={selectedUnit.name}
                class="hero-portrait"
              />
              <div class="hero-titles">
                <span class="hero-title-tag">{selectedUnit.title}</span>
                <h2 class="hero-name">{selectedUnit.name}</h2>
                <div class="hero-badges">
                  <span class="class-badge">{selectedUnit.className}</span>
                  <span class="level-badge">Level {selectedUnit.level}</span>
                  <span class="exp-badge">EXP {selectedUnit.exp}/100</span>
                  {#if selectedUnit.boots > 0}
                    <span class="boots-badge">+{selectedUnit.boots} Mov</span>
                  {/if}
                </div>
              </div>
            </div>
          </section>

          <!-- Collapsible Section 1: Combat Attributes -->
          <CollapsibleCard
            title="Combat Attributes"
            badge={`HP ${selectedUnit.stats[0]} / STR ${selectedUnit.stats[1]} / SPD ${selectedUnit.stats[4]}`}
          >
            <div class="stats-grid">
              {#each STAT_LABELS as label, i}
                <StatStepper
                  {label}
                  value={selectedUnit.stats[i]}
                  cap={selectedUnit.caps[i]}
                  base={selectedUnit.bases[i]}
                  addition={selectedUnit.additions[i]}
                  growth={selectedUnit.growths[i]}
                  onChange={(v) => handleStatChange(i, v)}
                />
              {/each}
            </div>
          </CollapsibleCard>

          <!-- Collapsible Section 2: Inventory & Equipment -->
          <CollapsibleCard
            title="Inventory & Equipment"
            badge={`${selectedUnit.inventory.length}/5 Items`}
          >
            <div class="inventory-grid">
              {#each selectedUnit.inventory as item, i}
                <div class="item-card" class:equipped={item.isEquipped}>
                  <div class="item-left">
                    {#if item.isEquipped}
                      <span class="equipped-tag">E</span>
                    {/if}
                    <div>
                      <div class="item-name">{item.name}</div>
                      <div class="item-kind">{item.kind}</div>
                    </div>
                  </div>

                  <div class="item-right">
                    <span class="uses-count">
                      {item.uses >= 999 ? "∞" : `${item.uses}/${item.maxUses}`}
                    </span>
                    {#if item.forgeMt}
                      <span class="forge-pill">+{item.forgeMt} Mt</span>
                    {/if}
                  </div>
                </div>
              {/each}
            </div>
          </CollapsibleCard>

          <!-- Collapsible Section 3: Equipped Skills -->
          <CollapsibleCard
            title="Equipped Skills"
            badge={`${selectedUnit.skills.length}/5 Skills`}
          >
            <div class="skills-grid">
              {#each selectedUnit.skills as skill}
                <div class="skill-chip">
                  <span class="skill-glyph">{skill.glyph}</span>
                  <div class="skill-text">
                    <div class="skill-name">{skill.name}</div>
                    <div class="skill-desc">{skill.description}</div>
                  </div>
                </div>
              {/each}
            </div>
          </CollapsibleCard>
        </div>
      {/key}
    {/if}
  </main>
</div>

<style>
  .roster-view {
    display: flex;
    flex: 1;
    overflow: hidden;
    height: calc(100vh - 38px);
  }

  /* Left Column */
  .roster-sidebar {
    width: 280px;
    background: var(--color-surface);
    border-right: 1px solid var(--color-border);
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .search-box {
    padding: 10px 12px;
    border-bottom: 1px solid var(--color-border);
  }

  .search-input {
    width: 100%;
    padding: 6px 12px;
    background: var(--color-surface-input);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-sm);
    color: var(--color-text-primary);
    font-size: 12px;
    transition: border-color 0.15s;
  }

  .search-input:focus {
    border-color: var(--color-ylisse-primary);
  }

  .units-list {
    flex: 1;
    overflow-y: auto;
    padding: 6px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .unit-item {
    width: 100%;
    padding: 8px 10px;
    border-radius: var(--radius-md);
    display: flex;
    align-items: center;
    gap: 10px;
    text-align: left;
    transition:
      background 0.15s,
      border-color 0.15s;
    border: 1px solid transparent;
  }

  .unit-item:hover {
    background: var(--color-surface-elevated);
  }

  .unit-item.selected {
    background: var(--color-surface-card);
    border-color: rgba(99, 102, 241, 0.3);
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  }

  .avatar-wrapper {
    width: 36px;
    height: 36px;
    border-radius: var(--radius-sm);
    overflow: hidden;
    background: var(--color-surface-elevated);
    border: 1px solid var(--color-border);
    flex-shrink: 0;
  }

  .unit-avatar {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .unit-info {
    flex: 1;
    overflow: hidden;
  }

  .unit-name-row {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
  }

  .unit-name {
    font-size: 13px;
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .unit-lv {
    font-size: 11px;
    color: var(--color-text-dim);
    font-variant-numeric: tabular-nums;
  }

  .unit-sub-row {
    display: flex;
    justify-content: space-between;
    font-size: 11px;
    color: var(--color-text-secondary);
  }

  .unit-hp-stat {
    color: var(--color-hp-green);
    font-weight: 500;
  }

  /* Right Column */
  .dossier-main {
    flex: 1;
    overflow-y: auto;
    padding: 20px 24px;
    background: var(--color-bg-base);
  }

  .dossier-content {
    display: flex;
    flex-direction: column;
    gap: 18px;
    max-width: 960px;
    margin: 0 auto;
  }

  /* Hero Identity Card */
  .hero-card {
    background: var(--glass-surface-card);
    backdrop-filter: var(--glass-blur);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    padding: 18px 24px;
    display: flex;
    align-items: center;
    justify-content: space-between;
  }

  .hero-left {
    display: flex;
    align-items: center;
    gap: 20px;
  }

  .hero-portrait {
    width: 72px;
    height: 72px;
    border-radius: var(--radius-md);
    object-fit: cover;
    border: 2px solid var(--color-border-hover);
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
    background: var(--color-surface-elevated);
  }

  .hero-title-tag {
    font-size: 11px;
    font-weight: 600;
    letter-spacing: 0.08em;
    color: var(--color-text-dim);
    text-transform: uppercase;
  }

  .hero-name {
    font-size: 24px;
    font-weight: 700;
    color: var(--color-text-primary);
    margin: 2px 0 6px;
    letter-spacing: -0.01em;
  }

  .hero-badges {
    display: flex;
    align-items: center;
    gap: 8px;
    flex-wrap: wrap;
  }

  .class-badge {
    background: var(--color-ylisse-primary);
    color: #ffffff;
    font-size: 11px;
    font-weight: 600;
    padding: 2px 8px;
    border-radius: var(--radius-sm);
  }

  .level-badge,
  .exp-badge,
  .boots-badge {
    background: var(--color-surface-elevated);
    border: 1px solid var(--color-border);
    color: var(--color-text-secondary);
    font-size: 11px;
    font-weight: 500;
    padding: 2px 8px;
    border-radius: var(--radius-sm);
  }

  .boots-badge {
    color: var(--color-tactician-teal);
    border-color: rgba(20, 184, 166, 0.3);
  }

  /* Stats Grid */
  .stats-grid {
    display: grid;
    grid-template-columns: repeat(4, 1fr);
    gap: 12px;
  }

  @media (max-width: 900px) {
    .stats-grid {
      grid-template-columns: repeat(2, 1fr);
    }
  }

  /* Inventory Grid */
  .inventory-grid {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }

  .item-card {
    background: var(--color-surface-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: 10px 14px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    transition: border-color 0.15s;
  }

  .item-card.equipped {
    border-color: rgba(245, 158, 11, 0.4);
    background: linear-gradient(
      90deg,
      rgba(245, 158, 11, 0.05) 0%,
      var(--color-surface-card) 40%
    );
  }

  .item-left {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .equipped-tag {
    font-size: 11px;
    font-weight: 700;
    color: var(--color-falchion-gold);
    background: var(--color-falchion-glow);
    padding: 2px 6px;
    border-radius: var(--radius-xs);
  }

  .item-name {
    font-size: 13px;
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .item-kind {
    font-size: 11px;
    color: var(--color-text-dim);
  }

  .item-right {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .uses-count {
    font-size: 12px;
    font-weight: 600;
    color: var(--color-text-secondary);
    font-variant-numeric: tabular-nums;
  }

  .forge-pill {
    font-size: 10px;
    font-weight: 700;
    color: var(--color-tactician-teal);
    background: rgba(20, 184, 166, 0.1);
    padding: 2px 6px;
    border-radius: var(--radius-xs);
  }

  /* Skills Grid */
  .skills-grid {
    display: grid;
    grid-template-columns: repeat(2, 1fr);
    gap: 10px;
  }

  .skill-chip {
    background: var(--color-surface-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: 10px 12px;
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .skill-glyph {
    font-size: 20px;
  }

  .skill-name {
    font-size: 12px;
    font-weight: 600;
    color: var(--color-text-primary);
  }

  .skill-desc {
    font-size: 11px;
    color: var(--color-text-secondary);
    margin-top: 2px;
  }
</style>
