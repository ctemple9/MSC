<script lang="ts">
  // Category = neutral (Java/Paper). Status = colored tint (Active, Missing).
  // docs/msc/renderings/primitives.html
  export let variant: 'category' | 'status' = 'category';
  export let tone: 'ok' | 'warn' | 'error' | 'bedrock' = 'ok';
</script>

{#if variant === 'category'}
  <span class="badge category"><slot /></span>
{:else}
  <span class="badge status {tone}"><slot /></span>
{/if}

<style>
  .badge {
    display: inline-block;
    font-size: 11px;
    font-weight: 500;
    padding: 3px 9px;
    border-radius: 6px;
  }

  .category {
    color: rgba(255, 255, 255, 0.8);
    background: var(--msc-neutral-elevated);
  }

  .status.ok {
    color: var(--msc-status-ok);
    background: var(--msc-status-ok-tint);
  }
  .status.warn {
    color: var(--msc-status-warn);
    background: var(--msc-status-warn-tint);
  }
  .status.error {
    color: var(--msc-status-error);
    background: var(--msc-status-error-tint);
  }
  .status.bedrock {
    color: var(--msc-status-bedrock);
    background: var(--msc-status-bedrock-tint);
  }
</style>
