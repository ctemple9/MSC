<script lang="ts">
  // Neutral selected fill on chrome track — NOT accent-tinted.
  // docs/msc/renderings/primitives.html
  export let options: readonly { value: string; label: string }[] = [];
  export let value = '';
  export let onchange: ((value: string) => void) | undefined = undefined;

  function select(next: string) {
    value = next;
    onchange?.(next);
  }
</script>

<div class="track" role="tablist">
  {#each options as option (option.value)}
    <button
      type="button"
      role="tab"
      class="segment"
      class:selected={option.value === value}
      aria-selected={option.value === value}
      onclick={() => select(option.value)}
    >
      {option.label}
    </button>
  {/each}
</div>

<style>
  .track {
    display: inline-flex;
    background: var(--msc-tier-chrome);
    border: 1px solid var(--msc-hairline-subtle);
    border-radius: 9px;
    padding: 3px;
    gap: 2px;
  }

  .segment {
    font-family: inherit;
    font-size: 12px;
    color: var(--msc-text-secondary);
    background: transparent;
    border: none;
    border-radius: 6px;
    padding: 5px 14px;
    cursor: pointer;
    transition: background 120ms ease;
  }

  .segment.selected {
    color: var(--msc-text-primary);
    font-weight: 500;
    background: var(--msc-neutral-elevated);
  }
</style>
