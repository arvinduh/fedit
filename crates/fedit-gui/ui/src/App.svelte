<script lang="ts">
  import MenuBar from "./components/MenuBar.svelte";
  import RosterView from "./views/RosterView.svelte";
  import { SAMPLE_SLOT } from "./lib/sample_data";
  import type { UnitDto } from "./lib/types";

  let activeSlot = $state(0);
  let activeTab = $state(1); // 1 = Roster
  let isDirty = $state(false);

  let currentSlot = $state(SAMPLE_SLOT);

  function handleUnitChange(updated: UnitDto) {
    isDirty = true;
    currentSlot = {
      ...currentSlot,
      units: currentSlot.units.map((u) => (u.id === updated.id ? updated : u)),
    };
  }

  function handleSave() {
    isDirty = false;
  }
</script>

<div class="app-shell">
  <MenuBar
    {activeSlot}
    {activeTab}
    {isDirty}
    onSelectSlot={(s) => {
      activeSlot = s;
    }}
    onSelectTab={(t) => {
      activeTab = t;
    }}
    onSave={handleSave}
  />

  <main class="app-body">
    {#if activeTab === 1}
      <RosterView
        units={currentSlot.units}
        onUnitChange={handleUnitChange}
      />
    {:else}
      <div class="placeholder-view">
        <p class="placeholder-text">Tab {activeTab} view</p>
      </div>
    {/if}
  </main>
</div>

<style>
  .app-shell {
    width: 100vw;
    height: 100vh;
    display: flex;
    flex-direction: column;
    overflow: hidden;
    background: var(--color-bg-base);
  }

  .app-body {
    flex: 1;
    overflow: hidden;
    display: flex;
  }

  .placeholder-view {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
  }

  .placeholder-text {
    font-size: 14px;
    color: var(--color-text-dim);
  }
</style>
