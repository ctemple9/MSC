import { describe, expect, it } from 'vitest';
import { StartupMilestones } from '../../src/lib/lifecycle/startupProgress';

const line = (text: string, ts = '2026-11-18T12:00:01Z') => ({ text, ts, source: 'stdout' });

describe('normal startup milestone progress', () => {
  it('counts a single launch without initiation shutdown or connection stages', () => {
    const progress = new StartupMilestones('java', 'paper', [], []);
    expect(progress.snapshot()).toMatchObject({ completed: 0, total: 3, value: 0 });
    progress.operation({
      id: 'start',
      type: 'start',
      state: 'running',
      progress: { current: 1, total: 2 },
    });
    expect(progress.snapshot()).toMatchObject({ completed: 1, total: 3, value: 1 / 3 });
    progress.observe([line('Preparing level "world"')]);
    expect(progress.snapshot()).toMatchObject({
      completed: 2,
      label: 'Waiting for the server-ready signal…',
    });
    progress.operation({ id: 'start', type: 'start', state: 'succeeded' });
    expect(progress.snapshot()).toMatchObject({ completed: 3, total: 3, value: 1 });
  });

  it('does not count earlier logs, helper chatter or errors as this run loading', () => {
    const old = line('Preparing level "old"', '2026-11-18T12:00:00Z');
    const progress = new StartupMilestones('java', 'fabric', ['Geyser', 'mods'], [old]);
    progress.observe([
      old,
      { ...line('Geyser started'), origin: 'helper' },
      { ...line('Geyser loading failed'), level: 'error' },
    ]);
    expect(progress.snapshot().completed).toBe(0);
    progress.observe([
      line('Loading Minecraft 1.21 with Fabric Loader'),
      line('Geyser initializing'),
    ]);
    expect(progress.snapshot()).toMatchObject({ completed: 2, total: 6 });
    progress.ready();
    expect(progress.snapshot().value).toBe(1);
  });
});
