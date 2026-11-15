<script lang="ts">
  // The one card depth MSC uses. Flat, opaque, borderless — separated from
  // its surroundings by tier contrast, not a border. docs/msc/renderings/status-card.html
  export let padding = '15px 16px';
  export let as: 'div' | 'section' = 'div';
</script>

<svelte:element this={as} class="card" style="padding: {padding};">
  <slot />
</svelte:element>

<style>
  .card {
    background: var(--msc-tier-content);
    border-radius: 12px;
    box-shadow: var(--msc-shadow-card);
  }
</style>
