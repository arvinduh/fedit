<script lang="ts">
  interface Props {
    activeSlot: number;
    activeTab: number;
    isDirty: boolean;
    onSelectSlot: (slot: number) => void;
    onSelectTab: (tab: number) => void;
    onSave: () => void;
  }

  let {
    activeSlot,
    activeTab,
    isDirty,
    onSelectSlot,
    onSelectTab,
    onSave,
  }: Props = $props();

  let fileMenuOpen = $state(false);
  let viewMenuOpen = $state(false);

  function closeMenus() {
    fileMenuOpen = false;
    viewMenuOpen = false;
  }
</script>

<svelte:window onclick={closeMenus} />

<header class="menu-bar">
  <!-- Left Menus -->
  <div class="menu-group">
    <div class="dropdown-anchor">
      <button
        type="button"
        class="menu-btn"
        class:active={fileMenuOpen}
        onclick={(e) => {
          e.stopPropagation();
          viewMenuOpen = false;
          fileMenuOpen = !fileMenuOpen;
        }}
      >
        File
      </button>

      {#if fileMenuOpen}
        <div
          class="dropdown-menu"
          role="menu"
          tabindex="-1"
          onkeydown={(e) => e.stopPropagation()}
          onclick={(e) => e.stopPropagation()}
        >
          <button type="button" class="dropdown-item" onclick={closeMenus}>
            <span>Open Save File...</span>
            <kbd class="shortcut">Ctrl+O</kbd>
          </button>
          <button type="button" class="dropdown-item" onclick={closeMenus}>
            <span>Open Folder...</span>
          </button>
          <div class="dropdown-divider"></div>
          <button
            type="button"
            class="dropdown-item"
            onclick={() => {
              closeMenus();
              onSave();
            }}
          >
            <span>Save Slot</span>
            <kbd class="shortcut">Ctrl+S</kbd>
          </button>
        </div>
      {/if}
    </div>

    <div class="dropdown-anchor">
      <button
        type="button"
        class="menu-btn"
        class:active={viewMenuOpen}
        onclick={(e) => {
          e.stopPropagation();
          fileMenuOpen = false;
          viewMenuOpen = !viewMenuOpen;
        }}
      >
        View
      </button>

      {#if viewMenuOpen}
        <div
          class="dropdown-menu"
          role="menu"
          tabindex="-1"
          onkeydown={(e) => e.stopPropagation()}
          onclick={(e) => e.stopPropagation()}
        >
          <button
            type="button"
            class="dropdown-item"
            onclick={() => {
              onSelectTab(0);
              closeMenus();
            }}
          >
            Overview
          </button>
          <button
            type="button"
            class="dropdown-item"
            onclick={() => {
              onSelectTab(1);
              closeMenus();
            }}
          >
            Unit Roster
          </button>
          <button
            type="button"
            class="dropdown-item"
            onclick={() => {
              onSelectTab(2);
              closeMenus();
            }}
          >
            Convoy
          </button>
          <button
            type="button"
            class="dropdown-item"
            onclick={() => {
              onSelectTab(3);
              closeMenus();
            }}
          >
            Supports
          </button>
        </div>
      {/if}
    </div>
  </div>

  <!-- Center: Slot Pills -->
  <div class="slots-group">
    {#each [0, 1, 2] as slot}
      <button
        type="button"
        class="slot-pill"
        class:selected={activeSlot === slot}
        onclick={() => onSelectSlot(slot)}
      >
        <span class="slot-dot"></span>
        <span class="slot-name">Slot {slot + 1}</span>
      </button>
    {/each}
  </div>

  <!-- Right: Save button -->
  <div class="actions-group">
    {#if isDirty}
      <span class="dirty-badge">● Unsaved Changes</span>
    {/if}
    <button type="button" class="save-btn" onclick={onSave}>
      Save Slot
    </button>
  </div>
</header>

<style>
  .menu-bar {
    height: 38px;
    background: var(--color-surface);
    border-bottom: 1px solid var(--color-border);
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0 12px;
    position: relative;
    z-index: 100;
  }

  .menu-group {
    display: flex;
    align-items: center;
    gap: 4px;
  }

  .dropdown-anchor {
    position: relative;
  }

  .menu-btn {
    padding: 4px 8px;
    border-radius: var(--radius-xs);
    font-size: 12px;
    font-weight: 500;
    color: var(--color-text-secondary);
    transition:
      background 0.15s,
      color 0.15s;
  }

  .menu-btn:hover,
  .menu-btn.active {
    background: var(--color-surface-elevated);
    color: var(--color-text-primary);
  }

  .dropdown-menu {
    position: absolute;
    top: calc(100% + 4px);
    left: 0;
    min-width: 180px;
    background: var(--color-surface-elevated);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    box-shadow: var(--glass-shadow);
    padding: 4px;
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .dropdown-item {
    width: 100%;
    padding: 6px 10px;
    border-radius: var(--radius-xs);
    font-size: 12px;
    color: var(--color-text-primary);
    display: flex;
    align-items: center;
    justify-content: space-between;
    transition: background 0.15s;
  }

  .dropdown-item:hover {
    background: var(--color-border-hover);
  }

  .shortcut {
    font-size: 10px;
    color: var(--color-text-dim);
    font-family: inherit;
  }

  .dropdown-divider {
    height: 1px;
    background: var(--color-border);
    margin: 3px 0;
  }

  .slots-group {
    display: flex;
    align-items: center;
    gap: 6px;
    background: var(--color-surface-input);
    padding: 2px 4px;
    border-radius: var(--radius-full);
    border: 1px solid var(--color-border-subtle);
  }

  .slot-pill {
    padding: 3px 10px;
    border-radius: var(--radius-full);
    font-size: 11px;
    font-weight: 500;
    color: var(--color-text-dim);
    display: flex;
    align-items: center;
    gap: 6px;
    transition:
      background 0.2s var(--ease-spring),
      color 0.2s;
  }

  .slot-pill:hover {
    color: var(--color-text-primary);
  }

  .slot-pill.selected {
    background: var(--color-surface-elevated);
    color: var(--color-text-primary);
    box-shadow: 0 1px 4px rgba(0, 0, 0, 0.4);
  }

  .slot-dot {
    width: 5px;
    height: 5px;
    border-radius: var(--radius-full);
    background: currentColor;
    opacity: 0.6;
  }

  .slot-pill.selected .slot-dot {
    background: var(--color-ylisse-primary);
    opacity: 1;
    box-shadow: 0 0 6px var(--color-ylisse-primary);
  }

  .actions-group {
    display: flex;
    align-items: center;
    gap: 12px;
  }

  .dirty-badge {
    font-size: 11px;
    color: var(--color-falchion-gold);
    font-weight: 500;
  }

  .save-btn {
    padding: 4px 12px;
    border-radius: var(--radius-sm);
    background: var(--color-ylisse-primary);
    color: #ffffff;
    font-size: 12px;
    font-weight: 600;
    transition:
      background 0.15s,
      transform 0.1s var(--ease-spring);
  }

  .save-btn:hover {
    background: #4f46e5;
  }

  .save-btn:active {
    transform: scale(0.96);
  }
</style>
