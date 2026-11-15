<script lang="ts">
  // Retain the shared component's import path; status is text-only across MSC.
  export let tone: 'ok' | 'warn' | 'error' | 'bedrock' = 'ok';
  export let label: string;
</script>

<span class="status-dot no-dot"><span class="label {tone}">{label}</span></span>

<style>
  .status-dot {
    display: inline-flex;
    align-items: center;
  }
  .label {
    font-size: 12px;
    font-weight: 500;
  }
  .label.ok {
    color: var(--msc-status-ok);
  }
  .label.warn {
    color: var(--msc-status-warn);
  }
  .label.error {
    color: var(--msc-status-error);
  }
  .label.bedrock {
    color: var(--msc-status-bedrock);
  }
</style>
