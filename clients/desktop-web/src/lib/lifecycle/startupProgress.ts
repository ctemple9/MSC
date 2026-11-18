import {
  initiationMilestones,
  observedStartupMilestones,
} from '../sections/server-editor/initiationProgress';
import type { Schema } from '../sections/shared/types';
import type { StartupProgress } from './serverLifecycle';
import { consoleLineKey, type ConsoleLine } from '../sections/console/model';

/** Normal Start has one runtime launch; it never includes setup choices or shutdown. */
export class StartupMilestones {
  private steps;
  private readonly seen: Set<string>;
  private readonly since: number;

  constructor(
    serverType: string,
    flavor: string,
    private readonly extensions: readonly string[],
    baseline: readonly ConsoleLine[],
  ) {
    this.steps = initiationMilestones(serverType, flavor, extensions, false)
      .filter((step) => !step.id.endsWith(':stopped'))
      .map((step) => ({
        ...step,
        id: step.id.slice(4),
        label: step.label.replace(/^Pass 1: /, ''),
      }));
    this.seen = new Set(baseline.map(consoleLineKey));
    const timestamps = baseline.map((line) => timestamp(line.ts)).filter(Number.isFinite);
    this.since = timestamps.length ? Math.max(...timestamps) : -Infinity;
  }

  observe(lines: readonly ConsoleLine[]): void {
    const ids = new Set<string>();
    for (const line of lines) {
      const key = consoleLineKey(line);
      const time = timestamp(line.ts);
      if (
        this.seen.has(key) ||
        !Number.isFinite(time) ||
        time <= this.since ||
        line.level === 'error' ||
        (line.origin && line.origin !== 'server')
      )
        continue;
      this.seen.add(key);
      for (const id of observedStartupMilestones(line.text, this.extensions)) ids.add(id);
    }
    this.mark(ids);
  }

  operation(operation: Schema['OperationDTO']): void {
    if (operation.progress && operation.progress.current > 0) this.mark(new Set(['launch']));
    if (operation.state === 'succeeded') this.ready();
  }

  ready(): void {
    this.steps = this.steps.map((step) => ({ ...step, done: true }));
  }

  private mark(ids: ReadonlySet<string>): void {
    this.steps = this.steps.map((step) => (ids.has(step.id) ? { ...step, done: true } : step));
  }

  snapshot(): StartupProgress {
    const completed = this.steps.filter((step) => step.done).length;
    const next = this.steps.find((step) => !step.done);
    const labels: Record<string, string> = {
      launch: this.steps[0]?.label.replace(/^Launch /, 'Starting ') + '…',
      loader: 'Initializing the mod loader…',
      world: 'Preparing the world…',
      ready: 'Waiting for the server-ready signal…',
    };
    return {
      label: next
        ? (labels[next.id] ?? next.label.replace(/^Begin loading /, 'Loading ') + '…')
        : 'Server ready',
      value: completed / this.steps.length,
      completed,
      total: this.steps.length,
    };
  }
}

function timestamp(value: string): number {
  const numeric = Number(value);
  return Number.isFinite(numeric) ? numeric : Date.parse(value);
}

export function installedStartupExtensions(
  flavor: string,
  components: Schema['ComponentsStatusDTO'] | null,
  addons: Schema['AddonsResponseDTO'] | null,
): string[] {
  const enabled = addons?.addons.filter((item) => item.isEnabled) ?? [];
  const installed =
    components?.components.filter(
      (item) => item.installedVersion !== undefined || item.installedLabel !== undefined,
    ) ?? [];
  const names = ['Geyser', 'Floodgate'].filter(
    (helper) =>
      installed.some((item) => item.name.toLowerCase().includes(helper.toLowerCase())) ||
      enabled.some((item) =>
        `${item.displayName} ${item.jarStem}`.toLowerCase().includes(helper.toLowerCase()),
      ),
  );
  if (enabled.some((item) => !/geyser|floodgate/i.test(`${item.displayName} ${item.jarStem}`)))
    names.push(
      ['fabric', 'forge', 'neoforge', 'quilt'].includes(flavor.toLowerCase()) ? 'mods' : 'plugins',
    );
  return names;
}
