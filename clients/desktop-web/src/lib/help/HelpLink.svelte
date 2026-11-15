<script lang="ts">
  export let helpId: string;
  export let hostId = 'local-agent';
  export let serverId = 'survival';
</script>

<a
  class="help-link"
  href={`/hosts/${encodeURIComponent(hostId)}/servers/${encodeURIComponent(serverId)}/handbook?topic=${encodeURIComponent(helpId)}`}
>
  Learn more
</a>

<style>
  .help-link {
    color: var(--msc-text-secondary);
    font-size: 12px;
    text-decoration: underline;
    text-underline-offset: 2px;
  }
  .help-link:hover {
    color: var(--msc-text-primary);
  }
  .help-link:focus-visible {
    outline: 2px solid var(--msc-hairline-field-focus);
    outline-offset: 2px;
  }
</style>
