<script lang="ts">
  import Card from './Card.svelte';
  import NumberField from './NumberField.svelte';
  export let minimum: string;
  export let maximum: string;
  export let disabled = false;
  export let onMinimumChange: (value: string) => void;
  export let onMaximumChange: (value: string) => void;
</script>

<Card padding="0">
  <div class="row">
    <span class="name">Minimum RAM</span>
    <div class="control">
      <NumberField
        value={minimum}
        min={0}
        step={0.1}
        width="80px"
        {disabled}
        onValueChange={onMinimumChange}
      />
      <span class="unit">GB</span>
    </div>
  </div>
  <div class="row bordered">
    <span class="name">Maximum RAM</span>
    <div class="control">
      <NumberField
        value={maximum}
        min={0}
        step={0.1}
        width="80px"
        {disabled}
        onValueChange={onMaximumChange}
      />
      <span class="unit">GB</span>
    </div>
  </div>
</Card>

<style>
  .row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 11px 14px;
  }
  .row.bordered {
    border-top: 1px solid var(--msc-hairline-subtle);
  }
  .name {
    font-size: 13px;
    font-weight: 500;
    color: var(--msc-text-primary);
  }
  .control {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .unit {
    font-size: 11px;
    color: var(--msc-text-tertiary);
  }
</style>
