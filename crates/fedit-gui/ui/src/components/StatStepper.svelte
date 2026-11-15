<script lang="ts">
  interface Props {
    label: string;
    value: number;
    cap: number;
    base: number;
    addition: number;
    growth: number;
    onChange: (newValue: number) => void;
  }

  let {
    label,
    value,
    cap,
    base,
    addition,
    growth,
    onChange,
  }: Props = $props();

  let isBumping = $state(false);
  const isCapped = $derived(value >= cap);
  const progressPercent = $derived(
    Math.min(100, Math.max(0, (value / cap) * 100))
  );

  function triggerBump() {
    isBumping = true;
    setTimeout(() => {
      isBumping = false;
    }, 200);
  }

  function handleDecrement() {
    if (value > 0) {
      triggerBump();
      onChange(value - 1);
    }
  }

  function handleIncrement() {
    if (value < cap) {
      triggerBump();
      onChange(value + 1);
    }
  }

  function handleInput(e: Event) {
    const input = e.currentTarget as HTMLInputElement;
    const parsed = parseInt(input.value, 10);
    if (!isNaN(parsed)) {
      onChange(Math.max(0, Math.min(cap, parsed)));
    }
  }
</script>

<div class="stat-stepper" class:capped={isCapped}>
  <div class="header-row">
    <span class="stat-label">{label}</span>
    {#if isCapped}
      <span class="max-badge">★ MAX</span>
    {/if}
  </div>

  <div class="controls-row">
    <button
      type="button"
      class="stepper-btn"
      onclick={handleDecrement}
      disabled={value <= 0}
      aria-label="Decrease {label}"
    >
      -
    </button>

    <div class="input-wrapper" class:bump={isBumping}>
      <input
        type="text"
        class="stat-input"
        value={value}
        onchange={handleInput}
        aria-label="{label} value"
      />
    </div>

    <button
      type="button"
      class="stepper-btn"
      onclick={handleIncrement}
      disabled={value >= cap}
      aria-label="Increase {label}"
    >
      +
    </button>
  </div>

  <!-- Cap Progress Bar -->
  <div class="progress-track">
    <div
      class="progress-fill"
      style:width={`${progressPercent}%`}
      class:capped-fill={isCapped}
    ></div>
  </div>

  <div class="breakdown-row">
    <span class="breakdown-text">
      Base {base} + Add {addition} + Grw {growth}
    </span>
    <span class="cap-text">/ {cap}</span>
  </div>
</div>

<style>
  .stat-stepper {
    background: var(--color-surface-card);
    border: 1px solid var(--color-border);
    border-radius: var(--radius-md);
    padding: 10px 12px;
    display: flex;
    flex-direction: column;
    gap: 6px;
    transition:
      border-color 0.2s var(--ease-out),
      box-shadow 0.2s var(--ease-out);
  }

  .stat-stepper:hover {
    border-color: var(--color-border-hover);
  }

  .stat-stepper.capped {
    border-color: rgba(245, 158, 11, 0.35);
    background: linear-gradient(
      180deg,
      rgba(245, 158, 11, 0.05) 0%,
      var(--color-surface-card) 100%
    );
  }

  .header-row {
    display: flex;
    justify-content: space-between;
    align-items: center;
  }

  .stat-label {
    font-size: 11px;
    font-weight: 700;
    letter-spacing: 0.06em;
    color: var(--color-text-dim);
    text-transform: uppercase;
  }

  .max-badge {
    font-size: 10px;
    font-weight: 700;
    color: var(--color-falchion-gold);
    background: var(--color-falchion-glow);
    padding: 1px 6px;
    border-radius: var(--radius-xs);
    letter-spacing: 0.04em;
  }

  .controls-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }

  .stepper-btn {
    width: 28px;
    height: 28px;
    border-radius: var(--radius-sm);
    background: var(--color-surface-elevated);
    border: 1px solid var(--color-border);
    color: var(--color-text-secondary);
    font-size: 15px;
    font-weight: 600;
    transition:
      background 0.15s,
      color 0.15s,
      transform 0.1s var(--ease-spring);
  }

  .stepper-btn:hover:not(:disabled) {
    background: var(--color-border-hover);
    color: var(--color-text-primary);
  }

  .stepper-btn:active:not(:disabled) {
    transform: scale(0.92);
  }

  .stepper-btn:disabled {
    opacity: 0.3;
    cursor: not-allowed;
  }

  .input-wrapper {
    flex: 1;
    display: flex;
    justify-content: center;
    transition: transform 0.2s var(--ease-spring);
  }

  .input-wrapper.bump {
    transform: scale(1.14);
  }

  .stat-input {
    width: 100%;
    text-align: center;
    font-size: 18px;
    font-weight: 700;
    font-variant-numeric: tabular-nums;
    color: var(--color-text-primary);
  }

  .stat-stepper.capped .stat-input {
    color: var(--color-falchion-gold);
  }

  .progress-track {
    width: 100%;
    height: 3px;
    background: var(--color-surface-elevated);
    border-radius: var(--radius-full);
    overflow: hidden;
  }

  .progress-fill {
    height: 100%;
    background: var(--color-ylisse-primary);
    border-radius: var(--radius-full);
    transition: width 0.3s var(--ease-spring);
  }

  .progress-fill.capped-fill {
    background: var(--color-falchion-gold);
    box-shadow: 0 0 8px rgba(245, 158, 11, 0.4);
  }

  .breakdown-row {
    display: flex;
    justify-content: space-between;
    font-size: 10px;
    color: var(--color-text-muted);
    font-variant-numeric: tabular-nums;
  }

  .cap-text {
    font-weight: 500;
  }
</style>
