<script lang="ts">
  import { onDestroy } from 'svelte';
  import Menu from '../../components/base/Menu.svelte';
  import { getPlatform, type EngineServiceAction, type EngineServiceStatus } from '../../platform';
  import { createServiceStatusMonitor } from './service-status';
  export let hostId: string;
  export let local: boolean;
  export let active: boolean;
  export let position: { x: number; y: number } | undefined;
  export let onClose: () => void;
  export let onBusy: (value: boolean) => void = () => {};
  export let onRefresh: () => void | Promise<void>;
  export let onStatus: (status: string) => void;
  export let onError: (message: string) => void;
  export let onEdit: () => void;
  export let onRemove: () => void;
  let service: EngineServiceStatus | undefined;
  let busy = false;
  $: onBusy(busy);
  export async function repair(): Promise<void> {
    await manage('repair');
  }
  export async function install(): Promise<void> {
    await manage('install');
  }
  let indicator = true;
  let indicatorAvailable = false;
  const monitor = createServiceStatusMonitor(
    async () => (await getPlatform()).engineServiceStatus(),
    (value) => {
      service = value;
      onStatus(value.state);
    },
    (error) => {
      onStatus('unavailable');
      onError(String(error));
    },
  );
  $: monitor.configure(local && active && !busy ? hostId : undefined);
  $: if (position && local) void readIndicator();
  async function readIndicator() {
    try {
      const platform = await getPlatform();
      indicatorAvailable = !!platform.setEngineIndicatorEnabled;
      indicator = (await platform.engineIndicatorEnabled?.()) ?? true;
    } catch (error) {
      onError(String(error));
    }
  }
  async function manage(action: EngineServiceAction) {
    if (!local || busy) return;
    busy = true;
    try {
      service = await (await getPlatform()).manageEngineService(action);
      onStatus(service.state);
      await onRefresh();
    } catch (error) {
      onError(String(error));
    } finally {
      busy = false;
    }
  }
  async function toggleIndicator() {
    try {
      await (await getPlatform()).setEngineIndicatorEnabled?.(!indicator);
      indicator = !indicator;
    } catch (error) {
      onError(String(error));
    }
  }
  $: items = local
    ? [
        {
          label:
            service?.state === 'not-installed'
              ? 'Install engine'
              : service?.state === 'running'
                ? 'Stop engine'
                : 'Start engine',
          disabled: busy || !service || service.state === 'unavailable',
          onSelect: () =>
            void manage(
              service?.state === 'not-installed'
                ? 'install'
                : service?.state === 'running'
                  ? 'stop'
                  : 'start',
            ),
        },
        {
          label: 'Repair engine',
          disabled: busy || !service,
          onSelect: () => void manage('repair'),
        },
        { label: 'Refresh status', disabled: busy, onSelect: () => void onRefresh() },
        ...(indicatorAvailable
          ? [
              {
                label: indicator ? 'Hide indicator' : 'Show indicator',
                disabled: busy,
                onSelect: () => void toggleIndicator(),
              },
            ]
          : []),
      ]
    : [
        { label: 'Refresh status', onSelect: () => void onRefresh() },
        { label: 'Edit connection', onSelect: onEdit },
        { label: 'Remove connection', onSelect: onRemove },
      ];
  onDestroy(() => monitor.dispose());
</script>

{#if position}<Menu x={position.x} y={position.y} {items} {onClose} />{/if}
