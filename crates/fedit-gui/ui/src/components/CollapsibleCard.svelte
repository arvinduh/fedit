<script lang="ts">
  import type { Snippet } from "svelte";

  interface Props {
    title: string;
    badge?: string;
    children: Snippet;
  }

  let {
    title,
    badge = "",
    children,
  }: Props = $props();

  let expanded = $state(true);

  function toggle() {
    expanded = !expanded;
  }
</script>

<div class="collapsible-card" class:expanded>
  <button
    type="button"
    class="card-header"
    onclick={toggle}
    aria-expanded={expanded}
  >
    <div class="header-left">
      <span class="chevron" class:rotated={expanded}>▸</span>
      <h3 class="card-title">{title}</h3>
    </div>
    {#if badge}
      <span class="card-badge">{badge}</span>
    {/if}
  </button>

  {#if expanded}
    <div class="card-divider"></div>
    <div class="card-content">
      {@render children()}
    </div>
  {/if}
</div>

<style>
  .collapsible-card {
    background: var(--glass-surface);
    backdrop-filter: var(--glass-blur);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-lg);
    overflow: hidden;
    transition:
      border-color 0.2s var(--ease-out),
      box-shadow 0.2s var(--ease-out);
  }

  .collapsible-card:hover {
    border-color: var(--color-border-hover);
  }

  .card-header {
    width: 100%;
    padding: 12px 16px;
    display: flex;
    align-items: center;
    justify-content: space-between;
    text-align: left;
    transition: background 0.15s;
  }

  .card-header:hover {
    background: rgba(255, 255, 255, 0.02);
  }

  .header-left {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .chevron {
    font-size: 13px;
    color: var(--color-text-dim);
    transition: transform 0.25s var(--ease-spring);
    display: inline-block;
  }

  .chevron.rotated {
    transform: rotate(90deg);
  }

  .card-title {
    font-size: 13px;
    font-weight: 600;
    letter-spacing: 0.02em;
    color: var(--color-text-primary);
  }

  .card-badge {
    font-size: 11px;
    font-weight: 500;
    color: var(--color-text-dim);
    background: var(--color-surface-elevated);
    border: 1px solid var(--color-border);
    padding: 2px 8px;
    border-radius: var(--radius-full);
  }

  .card-divider {
    height: 1px;
    background: var(--color-border-subtle);
  }

  .card-content {
    padding: 16px;
  }
</style>
