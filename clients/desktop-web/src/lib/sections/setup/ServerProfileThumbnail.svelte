<script lang="ts">
  import { onDestroy } from 'svelte';
  import type { Schema, ScreenApi } from '../shared/types';
  import { imageObjectUrl, mutate } from '../shared/types';
  import { pickUploadSource, type UploadSource } from '../shared/upload-source';
  import StagedUploadSheet from '../components/StagedUploadSheet.svelte';
  export let api: ScreenApi | undefined;
  export let server: Schema['ServerDTO'];
  export let active = true;
  export let selected = false;
  export let disabled = false;
  export let canEdit = true;
  export let onOpen: () => void;
  let image: string | undefined;
  let pending: UploadSource | undefined;
  let error = '';
  let disposed = false;
  let generation = 0;
  $: path = `/v1/servers/${encodeURIComponent(server.id)}/thumbnail`;
  $: if (active && api) void load(api, path);
  async function load(client: ScreenApi, target: string) {
    const version = ++generation;
    try {
      const bytes = await client.getBytes?.(target);
      if (disposed || version !== generation) return;
      if (image) URL.revokeObjectURL(image);
      image = bytes ? imageObjectUrl(bytes) : undefined;
    } catch {
      if (!disposed && version === generation) {
        if (image) URL.revokeObjectURL(image);
        image = undefined;
      }
    }
  }
  async function choose() {
    error = '';
    try {
      pending =
        (await pickUploadSource(
          api,
          'world-thumbnail',
          {
            label: `Thumbnail for ${server.name}`,
            extensions: ['png', 'jpg', 'jpeg', 'gif', 'webp'],
          },
          { targetServerId: server.id },
          () => !disposed,
        )) ?? undefined;
    } catch (failure) {
      error = failure instanceof Error ? failure.message : String(failure);
    }
  }
  async function save(upload: Schema['StagedUploadCompleteResultDTO'], scoped: ScreenApi) {
    await mutate(scoped, path, { stagedUploadId: upload.stagedUploadId });
    await load(scoped, path);
  }
  onDestroy(() => {
    disposed = true;
    generation++;
    if (image) URL.revokeObjectURL(image);
  });
</script>

<div class="cover">
  <button class="open" {disabled} onclick={onOpen} aria-label={`Open ${server.name}`}>
    {#if image}<img src={image} alt="" />{:else}
      <svg viewBox="0 0 240 112" fill="none" aria-hidden="true">
        <path d="M0 94h30V78h29V61h31V43h31v18h30V32h30v29h29v17h30v34H0Z" />
        <path d="M0 94h30V78h29V61h31V43h31m0 18h30V32h30m0 29h29v17h30" />
      </svg>
    {/if}
    <span class="edition">{server.serverType === 'bedrock' ? 'BEDROCK' : 'JAVA'}</span>
    {#if selected}<span class="selected">Selected</span>{/if}
  </button>
  {#if canEdit}<button class="change" onclick={() => void choose()}
      >{image ? 'Change thumbnail' : 'Set thumbnail'}</button
    >{/if}
</div>
{#if error}<p class="error" role="alert">{error}</p>{/if}
{#if pending}<StagedUploadSheet
    {api}
    purpose="world-thumbnail"
    source={pending}
    completionStage="installing"
    onComplete={save}
    onClose={() => (pending = undefined)}
  />{/if}

<style>
  .cover {
    position: relative;
    height: 112px;
    background: var(--msc-tier-chrome);
  }
  .open {
    display: block;
    padding: 0;
    border: 0;
    width: 100%;
    height: 100%;
    background: transparent;
    cursor: pointer;
    color: var(--msc-text-secondary);
  }
  img,
  svg {
    width: 100%;
    height: 100%;
    object-fit: cover;
  }
  svg path:first-child {
    fill: var(--msc-hairline);
  }
  svg path:last-child {
    stroke: var(--msc-text-tertiary);
    stroke-width: 1;
  }
  .edition,
  .selected {
    position: absolute;
    top: 12px;
    font-size: 10px;
    background: var(--msc-tier-chrome);
    padding: 3px 5px;
    border-radius: 3px;
  }
  .edition {
    left: 10px;
    letter-spacing: 1px;
  }
  .selected {
    right: 10px;
  }
  .change {
    position: absolute;
    bottom: 8px;
    left: 10px;
    border: 1px solid var(--msc-hairline);
    border-radius: 4px;
    background: var(--msc-tier-chrome);
    color: var(--msc-text-primary);
    padding: 4px 7px;
    font: inherit;
    font-size: 10px;
    cursor: pointer;
  }
  button:focus-visible {
    outline: 2px solid var(--msc-text-secondary);
    outline-offset: -2px;
  }
  .error {
    color: var(--msc-status-error);
    font-size: 11px;
    padding: 8px;
  }
</style>
