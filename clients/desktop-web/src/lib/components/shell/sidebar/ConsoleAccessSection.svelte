<script lang="ts">
  // Connection quick controls only expose services enabled for this server.
  // Deeper setup remains in the server editor.
  import { onDestroy, onMount } from 'svelte';
  import Button from '../../base/Button.svelte';
  import type { Schema, ScreenApi } from '../../../sections/shared/types';
  import { call, errorMessage, mutate } from '../../../sections/shared/types';
  import { serverEditorPaths } from '../../../sections/server-editor/model';

  export let api: ScreenApi | undefined = undefined;
  export let serverType: string | undefined = undefined;
  export let activeServerId: string | undefined = undefined;
  export let running = false;
  export let canControl = true;
  export let showXboxBroadcast = false;

  let status: Schema['BroadcastStatusDTO'] | undefined;
  let jarStatus: Schema['BroadcastJarStatusDTO'] | undefined;
  let playit: Schema['PlayitStatusResponseDTO'] | undefined;
  let broadcastBusy = false;
  let playitBusy = false;
  let notice = '';
  let loadedForServerId: string | undefined;
  let loadedRunning: boolean | undefined;
  let loadVersion = 0;
  let refreshTimer: ReturnType<typeof setInterval> | undefined;

  $: isBedrock = serverType === 'bedrock';
  $: broadcastRunning = isBedrock
    ? (status?.bedrockBroadcastRunning ?? false)
    : (status?.xboxBroadcastRunning ?? false);
  // Bedrock's Xbox Broadcast ships as a background process alongside BDS --
  // no separate "helper installed" concept the way Java's downloadable JAR
  // has, so it always shows controls; Java hides them behind the same
  // jar-installed gate the oracle's own XboxBroadcastSidebarRow uses.
  $: showBroadcastControls = isBedrock || (jarStatus?.installed ?? false);

  $: if (activeServerId !== loadedForServerId || running !== loadedRunning) {
    if (activeServerId !== loadedForServerId) {
      status = undefined;
      jarStatus = undefined;
      playit = undefined;
      notice = '';
    }
    loadedForServerId = activeServerId;
    loadedRunning = running;
    void load();
  }

  async function load(): Promise<void> {
    const version = ++loadVersion;
    const nextValues = await Promise.all([
      call(api, status, serverEditorPaths.broadcastStatus),
      call(api, jarStatus, serverEditorPaths.broadcastJarStatus),
      call(api, playit, serverEditorPaths.playit),
    ]);
    if (version !== loadVersion) return;
    [status, jarStatus, playit] = nextValues;
  }

  onMount(() => {
    // Helper startup is asynchronous and is owned by the engine, so the first
    // read after the Minecraft process starts can still be stale. This keeps
    // the sidebar's Start/Stop labels reconciled without a manual refresh.
    refreshTimer = setInterval(() => void load(), 1000);
  });

  onDestroy(() => {
    if (refreshTimer) clearInterval(refreshTimer);
    loadVersion += 1;
  });

  async function toggleBroadcast(): Promise<void> {
    if (broadcastBusy) return;
    broadcastBusy = true;
    try {
      const path = broadcastRunning
        ? serverEditorPaths.broadcastStop
        : serverEditorPaths.broadcastStart;
      await mutate<Schema['BroadcastSimpleResultDTO']>(api, path);
      await load();
    } catch (error) {
      notice = errorMessage(error);
    } finally {
      broadcastBusy = false;
    }
  }

  async function togglePlayit(): Promise<void> {
    if (playitBusy || !playit) return;
    playitBusy = true;
    try {
      const path = playit.isRunning ? serverEditorPaths.playitStop : serverEditorPaths.playitStart;
      await mutate(api, path);
      await load();
    } catch (error) {
      notice = errorMessage(error);
    } finally {
      playitBusy = false;
    }
  }
</script>

{#if playit?.playitEnabled || showXboxBroadcast || notice}
  <div class="console-access">
    {#if playit?.playitEnabled}
      <div class="service">
        <div class="service-header">
          <p class="msc-type-overline">playit.gg</p>
          <Button
            variant="secondary"
            size="sm"
            disabled={playitBusy || !playit?.playitEnabled || !canControl}
            onclick={togglePlayit}
          >
            {playit?.isRunning ? 'Stop' : 'Start'}
          </Button>
        </div>
      </div>
    {/if}

    {#if showXboxBroadcast}
      <div class="service">
        <div class="service-header">
          <p class="msc-type-overline">Xbox Broadcast</p>
          {#if showBroadcastControls}
            <Button
              variant="secondary"
              size="sm"
              disabled={broadcastBusy || !canControl}
              onclick={toggleBroadcast}
            >
              {broadcastRunning ? 'Stop' : 'Start'}
            </Button>
          {/if}
        </div>
      </div>
    {/if}

    {#if notice}<p class="notice" role="status">{notice}</p>{/if}
  </div>
{/if}

<style>
  .console-access {
    margin-bottom: 12px;
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .service {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }
  .service :global(.msc-type-overline) {
    line-height: 1;
    margin: 0;
  }
  .service-header {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 8px;
  }
  .notice {
    margin: 0;
    font-size: 10px;
    color: var(--msc-text-tertiary);
  }
</style>
