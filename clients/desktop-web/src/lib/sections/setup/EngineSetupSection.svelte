<script context="module" lang="ts">
  import { writable } from 'svelte/store';
  // Host switching rebuilds this view before the connection attempt finishes.
  const connectingHost = writable('');
</script>

<script lang="ts">
  import { onDestroy, tick } from 'svelte';
  import HowMscWorks from './HowMscWorks.svelte';
  import WelcomeMscSheet from '../../help/WelcomeMscSheet.svelte';
  import ServerProfileThumbnail from './ServerProfileThumbnail.svelte';
  import { can } from '../shared/types';
  import EngineProfileActions from './EngineProfileActions.svelte';
  import ConfirmDialog from '../../components/ConfirmDialog.svelte';
  import Button from '../../components/base/Button.svelte';
  import Sheet from '../../components/base/Sheet.svelte';
  import Menu from '../../components/base/Menu.svelte';
  import RemoteConnectionWizard from './connection/RemoteConnectionWizard.svelte';
  import {
    LOCAL_HOST_ID,
    hostAddressSummary,
    type HostId,
    type HostRecord,
    type RemoteHostConnectionInput,
  } from '../../hosts/types';
  import { getPlatform, type EngineReadiness } from '../../platform';
  import type { Schema, ScreenProps } from '../shared/types';
  import type { RemoteDesktopPairingResult } from '../../auth/desktop';
  import { formatConnectionFailure } from '../../hosts/connection-errors';
  export let permissions: readonly string[] | undefined = undefined;
  export let api: ScreenProps['api'] = undefined;
  export let readiness: EngineReadiness = 'starting';
  export let onEngineRetry: (() => void | Promise<void>) | undefined = undefined;
  export let hostId = '';
  export let active = true;
  export let servers: readonly Schema['ServerDTO'][] = [];
  export let serverId = '';
  export let serverRunning = false;
  export let onOpenServer: ((id: string) => Promise<void>) | undefined = undefined;
  export let onFleet: (() => void) | undefined = undefined;
  export let hosts: readonly HostRecord[] = [];
  export let activeHostId: HostId = '';
  export let isDesktopShell = false;
  export let isLocalHost = true;
  export let onConnectHost:
    ((input: RemoteHostConnectionInput) => Promise<RemoteDesktopPairingResult | void>) | undefined =
    undefined;
  export let onRemoveHost: (() => Promise<void>) | undefined = undefined;
  export let onSwitchHost: ((hostId: HostId) => void | Promise<void>) | undefined = undefined;
  let choosingComputer = false;
  let actionPosition: { x: number; y: number } | undefined;
  let disconnectedMenu: { x: number; y: number; host: HostRecord } | undefined;
  let serviceState = 'checking';
  let serviceBusy = false;
  let profileActions: EngineProfileActions | undefined;
  $: localNotInstalled =
    isLocalHost && (readiness === 'missing' || serviceState === 'not-installed');
  $: localServiceStopped = isLocalHost && serviceState === 'stopped';
  $: localRecovery =
    !localNotInstalled &&
    !localServiceStopped &&
    isLocalHost &&
    isDesktopShell &&
    (readiness === 'unavailable' || readiness === 'incompatible' || serviceState === 'unavailable');
  let editingComputer: HostRecord | undefined;
  let removingComputer = false;
  $: connectedView =
    readiness === 'ready' && !localNotInstalled && !localServiceStopped && !choosingComputer;
  let howItWorksOpen = false;
  $: showInstallGuide = localNotInstalled && focusedHostId === LOCAL_HOST_ID;
  $: visibleProfiles = showInstallGuide
    ? computerProfiles.filter((host) => host.id === LOCAL_HOST_ID)
    : connectedView
      ? computerProfiles.filter((host) => host.id === activeHostId)
      : computerProfiles;
  $: selectedServer = servers.find((server) => server.id === serverId);
  let profileRail: HTMLDivElement | undefined;
  let focusedHostId = '';
  let lastHostId = '';
  $: if (activeHostId !== lastHostId) {
    lastHostId = activeHostId;
    void focusComputer(activeHostId);
  }
  $: focusedComputer = computerProfiles.find((computer) => computer.id === focusedHostId);
  $: focusedIndex = computerProfiles.findIndex((computer) => computer.id === focusedHostId);
  async function focusComputer(id: string): Promise<void> {
    actionPosition = undefined;
    disconnectedMenu = undefined;
    focusedHostId = id;
    await tick();
    const profile = [
      ...(profileRail?.querySelectorAll<HTMLElement>('[data-profile-id]') ?? []),
    ].find((node) => node.dataset.profileId === id);
    if (profile && profileRail)
      profileRail.scrollTo({
        left:
          profile.offsetLeft -
          profileRail.offsetLeft -
          (profileRail.clientWidth - profile.offsetWidth) / 2,
        behavior: window.matchMedia('(prefers-reduced-motion: reduce)').matches
          ? 'instant'
          : 'smooth',
      });
  }
  function slideComputer(direction: number): void {
    const next = computerProfiles[Math.max(0, focusedIndex) + direction];
    if (next) void focusComputer(next.id);
  }
  $: connecting = $connectingHost;
  let openingServer = '';
  let error = '';
  let connectionOpen = false;
  let players: number | undefined;
  let hostMemory: Schema['HostMemoryDTO'] | undefined;
  $: availableBytes =
    hostMemory?.usedBytes != null && hostMemory.totalBytes && hostMemory.totalBytes > 0
      ? Math.max(0, hostMemory.totalBytes - hostMemory.usedBytes)
      : undefined;
  $: ramLabel =
    availableBytes !== undefined && hostMemory?.totalBytes
      ? `${(availableBytes / 1073741824).toFixed(2)} / ${Number((hostMemory.totalBytes / 1073741824).toFixed(2))} GB`
      : 'Unavailable';
  $: ramPercent =
    availableBytes !== undefined && hostMemory?.totalBytes
      ? Math.max(0, Math.min(100, (availableBytes / hostMemory.totalBytes) * 100))
      : undefined;
  $: computerAddress =
    servers.find((server) => server.hostAddress)?.hostAddress ??
    (!isLocalHost ? hosts.find((host) => host.id === activeHostId)?.ssh.hostname : undefined) ??
    'Unavailable';
  let revealedAddressHost = '';
  $: addressVisible = revealedAddressHost === activeHostId;
  let generation = 0;
  let statsKey = '';
  let timer: ReturnType<typeof setInterval> | undefined;
  $: computerProfiles = [
    ...hosts.filter((host) => host.id === LOCAL_HOST_ID),
    ...hosts.filter((host) => host.id !== LOCAL_HOST_ID),
  ];

  $: orderedServers = [
    ...servers.filter((server) => server.id === serverId),
    ...servers.filter((server) => server.id !== serverId),
  ];
  $: configurePlayers(api, active && readiness === 'ready' ? `${hostId}:${serverId}` : '');

  function configurePlayers(client: ScreenProps['api'], key: string): void {
    if (statsKey === key) return;
    statsKey = key;
    const version = ++generation;
    clearInterval(timer);
    players = undefined;
    hostMemory = undefined;
    if (!client || !key) return;
    const refresh = async () => {
      try {
        const [online, memory] = await Promise.allSettled([
          client.get<Schema['PlayersResponseDTO']>('/v1/players'),
          client.get<Schema['HostMemoryDTO']>('/v1/host/memory'),
        ]);
        if (version === generation) {
          players = online.status === 'fulfilled' ? online.value.count : undefined;
          hostMemory = memory.status === 'fulfilled' ? memory.value : undefined;
        }
      } catch {
        if (version === generation) players = undefined;
      }
    };
    void refresh();
    timer = setInterval(() => void refresh(), 5000);
  }
  onDestroy(() => {
    generation++;
    clearInterval(timer);
  });

  function connectionLabel(
    host: HostRecord,
    currentReadiness: EngineReadiness,
    nativeState: string,
    notInstalled: boolean,
    selectedHost: string,
  ): string {
    if (host.id === selectedHost) {
      if (host.id === LOCAL_HOST_ID && notInstalled) return 'Not installed';
      if (host.id === LOCAL_HOST_ID && nativeState === 'stopped' && !connecting)
        return 'Service stopped';
      return {
        ready: 'Connected',
        starting: 'Connecting…',
        stopped: 'Service stopped',
        missing: 'Not installed',
        incompatible: 'Update needed',
        unavailable: isLocalHost ? 'Needs attention' : 'Disconnected',
      }[currentReadiness];
    }
    return 'Disconnected';
  }
  async function chooseComputer(host: HostRecord): Promise<void> {
    if (connecting) return;
    error = '';
    connectingHost.set(host.id);
    choosingComputer = false;
    try {
      if (host.id === LOCAL_HOST_ID && isDesktopShell) {
        const platform = await getPlatform();
        let service = await platform.engineServiceStatus();
        if (service.state === 'stopped') service = await platform.manageEngineService('start');
        if (service.state !== 'running') {
          // Installation and recovery stay in the existing service controls.
          serviceState = service.state;
          actionPosition =
            service.state === 'not-installed' ? undefined : { x: window.innerWidth / 2, y: 300 };
          if (host.id !== activeHostId) await onSwitchHost?.(host.id);
          error = service.detail || 'The local Background Service could not be started.';
          return;
        }
      }
      if (host.id !== activeHostId) await onSwitchHost?.(host.id);
      else if (readiness !== 'ready') await onEngineRetry?.();
    } catch (failure) {
      error = formatConnectionFailure(failure, 'network');
    } finally {
      if ($connectingHost === host.id) connectingHost.set('');
    }
  }
  async function chooseServer(id: string): Promise<void> {
    if (openingServer) return;
    openingServer = id;
    error = '';
    try {
      await onOpenServer?.(id);
    } catch (failure) {
      error = formatConnectionFailure(failure, 'network');
    } finally {
      openingServer = '';
    }
  }
  async function connect(
    input: RemoteHostConnectionInput,
  ): Promise<RemoteDesktopPairingResult | void> {
    choosingComputer = false;
    const result = await onConnectHost?.(input);
    if (!result || result.state === 'paired') connectionOpen = false;
    return result;
  }
</script>

<div class="computer-library">
  <header class="library-heading">
    <div>
      <p class="msc-type-overline">MSC BACKGROUND SERVICE</p>
      <h1>Your computers</h1>
      <p class="intro">Choose a computer to run/manage MSC.</p>
    </div>
    <div class="heading-actions">
      <Button onclick={() => (howItWorksOpen = true)}>How does this work?</Button>
      {#if isDesktopShell}<Button
          onclick={() => {
            editingComputer = undefined;
            connectionOpen = true;
          }}>Connect a computer</Button
        >{/if}
    </div>
  </header>

  {#if !connectedView}<div class="profile-navigation">
      <p class="msc-type-overline">
        {focusedComputer?.id === LOCAL_HOST_ID
          ? 'This computer'
          : (focusedComputer?.displayName ?? 'Computers')}
      </p>
      <div class="carousel-actions">
        <Button
          size="sm"
          disabled={focusedIndex <= 0}
          onclick={() => slideComputer(-1)}
          variant="ghost-icon"
          label="Previous computer"
          title="Previous computer">←</Button
        >
        <span>{computerProfiles.length ? focusedIndex + 1 : 0} / {computerProfiles.length}</span>
        <Button
          size="sm"
          disabled={focusedIndex >= computerProfiles.length - 1}
          onclick={() => slideComputer(1)}
          variant="ghost-icon"
          label="Next computer"
          title="Next computer">→</Button
        >
      </div>
    </div>
  {/if}
  <div
    class:connected-view={connectedView || showInstallGuide}
    class="profile-rail"
    bind:this={profileRail}
    role="region"
    aria-label="Computer profiles"
  >
    {#each visibleProfiles as computer (computer.id)}
      {@const local = computer.id === LOCAL_HOST_ID}
      {@const connected =
        computer.id === activeHostId &&
        readiness === 'ready' &&
        !(local && (localNotInstalled || localServiceStopped))}
      <section
        class="computer-choice"
        class:focused={computer.id === focusedHostId}
        data-profile-id={computer.id}
        aria-label={local ? 'This computer' : computer.displayName}
      >
        <button
          class="computer-profile"
          disabled={!!connecting}
          onclick={(event) => {
            void focusComputer(computer.id);
            if (local && computer.id === activeHostId && localNotInstalled) return;
            if ((local && computer.id === activeHostId) || connected)
              actionPosition = { x: event.clientX, y: event.clientY };
            else disconnectedMenu = { x: event.clientX, y: event.clientY, host: computer };
          }}
          aria-label={connected
            ? `Open ${computer.displayName} controls`
            : `Choose ${computer.displayName}`}
        >
          <svg class="computer-art" viewBox="0 0 176 112" fill="none" aria-hidden="true">
            <rect x="29" y="12" width="118" height="75" rx="5" />
            <path d="M30 72h116M74 87v12m28-12v12m-41 0h54" />
            <path class="screen-detail" d="M43 29h30m-30 9h48m-48 9h23" />
            <path class="screen-detail" d="m112 41 7 7 14-16" />
          </svg>
          <span class="profile-caption">{local ? 'LOCAL COMPUTER' : 'REMOTE COMPUTER'}</span>
          <h2>{local ? 'This computer' : computer.displayName}</h2>
          <span class="connection-label" class:connected
            >{connecting === computer.id
              ? 'Connecting…'
              : connectionLabel(
                  computer,
                  readiness,
                  serviceState,
                  localNotInstalled,
                  activeHostId,
                )}</span
          >
        </button>

        <p class="address">{local ? 'On this device' : hostAddressSummary(computer)}</p>
        <div class="profile-action">
          {#if connected}<Button
              size="sm"
              onclick={() => {
                choosingComputer = connectedView;
                if (!choosingComputer) void focusComputer(computer.id);
              }}>{connectedView ? 'Switch computer' : 'View servers'}</Button
            >
          {:else if local && computer.id === activeHostId && localNotInstalled}
            <Button size="sm" disabled={serviceBusy} onclick={() => void profileActions?.install()}
              >{serviceBusy ? 'Installing…' : 'Install engine'}</Button
            >
          {:else if local && computer.id === activeHostId && localRecovery}
            <Button size="sm" disabled={serviceBusy} onclick={() => void profileActions?.repair()}
              >{serviceBusy ? 'Repairing…' : 'Repair engine'}</Button
            >
          {:else}<Button
              size="sm"
              disabled={!!connecting}
              onclick={() => {
                void focusComputer(computer.id);
                void chooseComputer(computer);
              }}
              >{local && computer.id === activeHostId && localNotInstalled
                ? 'Set up engine'
                : 'Connect'}</Button
            >{/if}
        </div>
        {#if local && computer.id === activeHostId && localNotInstalled}
          <div class="profile-recovery" role="status">
            <p>Install the MSC Background Service to run and manage servers on this computer.</p>
          </div>
        {:else if local && computer.id === activeHostId && localRecovery}
          <div class="profile-recovery" role="status">
            <p>
              {readiness === 'incompatible'
                ? 'This service needs a compatible version.'
                : 'MSC cannot reach the local Background Service.'} Repair the engine to restore it. Your
              servers and worlds are kept.
            </p>
          </div>
        {/if}
      </section>
    {:else}<p class="intro">Your computers will appear here once MSC finishes loading.</p>{/each}
    {#if connectedView}
      <div class="computer-facts">
        <div class="selected-server-summary">
          <p class="fact-label">Selected server</p>
          <h2>{selectedServer?.name ?? 'No server selected'}</h2>
          {#if selectedServer}<p class="server-state" class:running={serverRunning}>
              {serverRunning ? 'Running' : 'Stopped'}
            </p>{/if}
        </div>
        <div class="memory-summary">
          <div class="fact-line">
            <span class="fact-label">RAM available</span><span>{ramLabel}</span>
          </div>
          {#if ramPercent !== undefined}
            <div
              class="memory-track"
              role="meter"
              aria-label="Computer RAM available"
              aria-valuemin="0"
              aria-valuemax="100"
              aria-valuenow={ramPercent}
              aria-valuetext={ramLabel}
            >
              <span style:width={`${ramPercent}%`}></span>
            </div>
          {/if}
        </div>
        <dl class="connection-facts">
          <div>
            <dt>Background Service</dt>
            <dd class="service-state">
              {isLocalHost ? (serviceState === 'running' ? 'Running' : serviceState) : 'Connected'}
            </dd>
          </div>
          <div class="address-row">
            <dt>Address</dt>
            <dd>
              <button
                class="address-toggle"
                onclick={() => (revealedAddressHost = addressVisible ? '' : activeHostId)}
                aria-label={addressVisible ? 'Hide computer address' : 'Show computer address'}
                aria-pressed={addressVisible}
                >{addressVisible ? computerAddress : 'Click to reveal'}</button
              >
            </dd>
          </div>
          <div>
            <dt>Servers on this computer</dt>
            <dd>{servers.length}</dd>
          </div>
        </dl>
      </div>
    {/if}
    {#if showInstallGuide}<div class="install-guide"><HowMscWorks /></div>{/if}
  </div>
  {#if connectedView && focusedComputer && focusedHostId === activeHostId}
    <section class="server-branch" aria-label={`${focusedComputer.displayName} servers`}>
      <div class="shelf-heading">
        <div>
          <p class="msc-type-overline">
            {focusedHostId === LOCAL_HOST_ID ? 'This computer' : focusedComputer.displayName} / Minecraft
            servers
          </p>
          <h2>{servers.length} {servers.length === 1 ? 'server' : 'servers'}</h2>
        </div>
      </div>
      <div class="server-shelf">
        {#each orderedServers as server (server.id)}
          {@const selected = server.id === serverId}
          <div class="server-card" class:selected>
            <ServerProfileThumbnail
              {api}
              {server}
              {active}
              {selected}
              disabled={!onOpenServer || !!openingServer}
              canEdit={can(permissions, 'worlds')}
              onOpen={() => void chooseServer(server.id)}
            />
            <button
              class="server-caption"
              disabled={!onOpenServer || !!openingServer}
              onclick={() => void chooseServer(server.id)}
            >
              <div class="server-title">
                <h3>{server.name}</h3>
                <span aria-hidden="true">→</span>
              </div>
              <p>
                {server.serverType === 'bedrock'
                  ? 'Bedrock Dedicated Server'
                  : server.javaFlavor || 'Java Edition'}
              </p>
              <div class="server-meta">
                <span class:running={selected && serverRunning}
                  >{selected ? (serverRunning ? 'Running' : 'Stopped') : 'Not selected'}</span
                >
                {#if selected && serverRunning}<span
                    >{players === undefined ? 'Players unavailable' : `${players} online`}</span
                  >{:else if server.firstStartRequired}<span>Setup needed</span>{/if}
              </div>
              {#if server.gamePort}<p class="port">Port {server.gamePort}</p>{/if}
            </button>
          </div>
        {:else}
          <div class="empty-library">
            <h3>No servers here yet</h3>
            <p>Create a server or bring an existing one to this computer.</p>
            {#if onFleet}<Button size="sm" onclick={onFleet}>Manage servers</Button>{/if}
          </div>
        {/each}
      </div>
      {#if servers.length && onFleet}<button class="manage-link" onclick={onFleet}
          >Manage servers →</button
        >{/if}
    </section>
  {/if}
  {#if error}<p class="error" role="alert">{error}</p>{/if}
  <p class="library-footnote">
    One connection at a time. Your servers keep running when you switch computers or close MSC.
  </p>
</div>

{#if connectionOpen}
  <Sheet title="Connect a computer" size="lg" onClose={() => (connectionOpen = false)}>
    <RemoteConnectionWizard
      initialHost={editingComputer}
      hosts={hosts.filter((host) => host.id !== LOCAL_HOST_ID)}
      onConnect={connect}
    />
  </Sheet>
{/if}

{#if disconnectedMenu}
  <Menu
    x={disconnectedMenu.x}
    y={disconnectedMenu.y}
    items={[
      {
        label: 'Connect',
        onSelect: () => {
          const host = disconnectedMenu?.host;
          disconnectedMenu = undefined;
          if (host) void chooseComputer(host);
        },
      },
    ]}
    onClose={() => (disconnectedMenu = undefined)}
  />
{/if}
<EngineProfileActions
  bind:this={profileActions}
  onBusy={(value) => (serviceBusy = value)}
  {hostId}
  local={isLocalHost && isDesktopShell}
  {active}
  position={focusedHostId === activeHostId && !localNotInstalled ? actionPosition : undefined}
  onClose={() => (actionPosition = undefined)}
  onRefresh={async () => {
    await onEngineRetry?.();
  }}
  onStatus={(value) => (serviceState = value)}
  onError={(value) => (error = value)}
  onEdit={() => {
    editingComputer = hosts.find((host) => host.id === activeHostId);
    connectionOpen = true;
  }}
  onRemove={() => (removingComputer = true)}
/>
<ConfirmDialog
  open={removingComputer}
  title="Remove this connection?"
  message="Only the saved connection and credential are removed. Its servers and files are kept."
  confirmLabel="Remove connection"
  onClose={() => (removingComputer = false)}
  onConfirm={async () => {
    try {
      await onRemoveHost?.();
      removingComputer = false;
    } catch (failure) {
      error = String(failure);
    }
  }}
/>

{#if howItWorksOpen}<WelcomeMscSheet onNext={() => (howItWorksOpen = false)} />{/if}

<style>
  .install-guide {
    flex: 0 1 420px;
    min-width: 0;
  }
  .heading-actions {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 12px;
  }

  .profile-recovery {
    margin-top: 14px;
  }
  .profile-recovery p {
    font-size: 12px;
    line-height: 1.5;
    color: var(--msc-text-secondary);
    margin-bottom: 12px;
  }

  .profile-rail.connected-view {
    justify-content: center;
    align-items: center;
    gap: 48px;
  }
  .profile-rail.connected-view .computer-choice {
    margin-left: 0;
    margin-right: 0;
  }
  .computer-facts {
    flex: 0 1 420px;
    min-width: 0;
    display: grid;
    gap: 26px;
    font-size: 13px;
  }
  .fact-label,
  .connection-facts dt {
    color: var(--msc-text-secondary);
  }
  .selected-server-summary h2 {
    font-size: 24px;
    font-weight: 500;
    line-height: 1.25;
    margin: 7px 0;
    overflow-wrap: anywhere;
  }
  .server-state {
    color: var(--msc-text-secondary);
  }
  .server-state.running,
  .service-state {
    color: var(--msc-status-ok);
  }
  .fact-line,
  .connection-facts > div {
    display: flex;
    justify-content: space-between;
    gap: 16px;
  }
  .fact-line {
    font-variant-numeric: tabular-nums;
  }
  .memory-track {
    height: 5px;
    margin-top: 10px;
    background: var(--msc-hairline);
    border-radius: 2px;
    overflow: hidden;
  }
  .memory-track span {
    display: block;
    height: 100%;
    background: var(--msc-text-secondary);
  }
  .connection-facts {
    display: contents;
  }
  .connection-facts > div {
    display: grid;
    grid-template-columns: max-content minmax(0, 1fr);
    align-items: baseline;
    gap: 16px;
    font-size: 13px;
  }
  .connection-facts dd {
    margin: 0;
    text-align: right;
    min-width: 0;
  }
  .address-toggle {
    display: inline;
    background: transparent;
    border: 0;
    padding: 0;
    color: var(--msc-text-primary);
    font: inherit;
    overflow-wrap: anywhere;
    cursor: pointer;
    text-align: right;
  }
  .address-toggle:hover {
    text-decoration: underline;
  }
  .address-toggle:focus-visible {
    outline: 2px solid var(--msc-text-secondary);
    outline-offset: 3px;
  }
  @media (max-width: 700px) {
    .profile-rail.connected-view {
      flex-wrap: wrap;
    }
  }

  .computer-library {
    max-width: 1200px;
    margin: 0 auto;
    padding: 12px 8px 32px;
  }
  .library-heading {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 24px;
    margin-bottom: 42px;
  }
  h1,
  h2,
  h3,
  p {
    margin: 0;
  }
  h1 {
    font-size: 32px;
    font-weight: 600;
    letter-spacing: -0.8px;
    margin: 8px 0;
  }
  h2 {
    font-size: 24px;
    font-weight: 500;
    letter-spacing: -0.5px;
    margin: 7px 0;
    overflow-wrap: anywhere;
  }
  h3 {
    font-size: 15px;
    font-weight: 500;
    overflow-wrap: anywhere;
  }
  .intro,
  .address,
  .library-footnote {
    font-size: 13px;
    line-height: 1.5;
    color: var(--msc-text-secondary);
  }
  .profile-navigation {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    margin-bottom: 16px;
  }
  .carousel-actions {
    display: flex;
    align-items: center;
    gap: 12px;
  }
  .carousel-actions > span {
    color: var(--msc-text-tertiary);
    font-size: 12px;
  }
  .profile-rail {
    display: flex;
    gap: 28px;
    overflow-x: auto;
    scroll-snap-type: x mandatory;
    padding: 4px 4px 24px;
    margin-bottom: 24px;
    scrollbar-width: thin;
  }
  .computer-choice {
    flex: 0 0 216px;
    scroll-snap-align: center;
    text-align: center;
    min-width: 0;
  }
  .computer-choice:first-child {
    margin-left: auto;
  }
  .computer-choice:last-child {
    margin-right: auto;
  }
  .computer-profile {
    width: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    padding: 20px 14px 18px;
    min-height: 225px;
    border: 1px solid var(--msc-hairline);
    border-radius: 8px;
    background: var(--msc-tier-content);
    color: var(--msc-text-primary);
    cursor: pointer;
  }
  .computer-choice.focused .computer-profile {
    border-color: var(--msc-text-secondary);
  }
  .computer-art {
    width: 160px;
    height: 112px;
    stroke: var(--msc-text-secondary);
    stroke-width: 2;
    margin-bottom: 14px;
  }
  .screen-detail {
    stroke: var(--msc-text-tertiary);
  }
  .profile-caption {
    font-size: 10px;
    letter-spacing: 1px;
    color: var(--msc-text-tertiary);
  }
  .computer-profile h2 {
    font-size: 19px;
    margin: 8px 0 10px;
  }
  .connection-label {
    font-size: 12px;
    color: var(--msc-text-secondary);
    margin-top: auto;
  }
  .connection-label.connected {
    color: var(--msc-status-ok);
  }
  .address {
    margin: 12px 0;
    overflow-wrap: anywhere;
    font-size: 12px;
  }
  .profile-action {
    min-height: 32px;
  }
  .server-branch {
    border-top: 1px solid var(--msc-hairline);
    padding-top: 26px;
    margin-bottom: 24px;
  }
  .shelf-heading {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 16px;
    margin-bottom: 20px;
  }
  .shelf-heading h2 {
    font-size: 21px;
  }
  .server-shelf {
    display: flex;
    flex-wrap: wrap;
    justify-content: center;
    gap: 16px;
  }
  .server-card {
    flex: 0 1 215px;
    max-width: 100%;
    padding: 0;
    border: 1px solid transparent;
    border-radius: 7px;
    background: var(--msc-tier-content);
    color: var(--msc-text-primary);
    text-align: left;
    cursor: pointer;
    overflow: hidden;
    min-width: 0;
  }
  .server-caption {
    display: block;
    width: 100%;
    border: 0;
    background: transparent;
    color: inherit;
    text-align: left;
    font: inherit;
    cursor: pointer;
    padding: 15px;
  }
  .server-title {
    display: flex;
    justify-content: space-between;
    gap: 12px;
    align-items: baseline;
  }
  .server-title > span {
    color: var(--msc-text-tertiary);
  }
  .server-caption p {
    color: var(--msc-text-secondary);
    font-size: 12px;
    margin-top: 6px;
  }
  .server-meta {
    display: flex;
    justify-content: space-between;
    flex-wrap: wrap;
    gap: 6px;
    margin-top: 17px;
    font-size: 12px;
    color: var(--msc-text-secondary);
  }
  .server-meta .running {
    color: var(--msc-status-ok);
  }
  .server-caption .port {
    color: var(--msc-text-tertiary);
    font-size: 11px;
    margin-top: 8px;
  }
  .computer-profile:hover,
  .server-card:hover {
    border-color: var(--msc-text-tertiary);
  }
  .server-card.selected {
    border-color: var(--msc-status-ok);
    box-shadow: inset 0 0 0 1px var(--msc-status-ok);
  }
  button:focus-visible {
    outline: 2px solid var(--msc-text-secondary);
    outline-offset: 3px;
  }
  button:disabled {
    cursor: default;
    opacity: 0.6;
  }
  .manage-link {
    background: transparent;
    border: 0;
    color: var(--msc-text-secondary);
    padding: 14px 0 0;
    cursor: pointer;
    font: inherit;
    font-size: 12px;
  }
  .empty-library {
    padding: 22px 0 16px;
  }
  .empty-library p {
    font-size: 13px;
    color: var(--msc-text-secondary);
    line-height: 1.5;
    margin: 8px 0 12px;
  }
  .library-footnote {
    font-size: 12px;
  }

  .error {
    color: var(--msc-status-error);
    font-size: 13px;
    margin-bottom: 20px;
  }
  @media (max-width: 700px) {
    .library-heading {
      align-items: flex-start;
      flex-direction: column;
      gap: 16px;
      margin-bottom: 28px;
    }
    .server-card {
      flex-basis: 190px;
    }
  }
</style>
