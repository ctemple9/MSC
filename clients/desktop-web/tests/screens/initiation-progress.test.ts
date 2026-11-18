import { describe, expect, it } from 'vitest';
import {
  completeInitiationRun,
  initiationMilestones,
  settleInitiationPass,
} from '../../src/lib/sections/server-editor/initiationProgress';

describe('initiation progress follows the runs the engine performs', () => {
  it('finishes a helper-free run without counting a second pass', () => {
    const plan = initiationMilestones('java', 'purpur', [], false);
    expect(plan.some((step) => step.id === 'connections' || step.id.startsWith('two:'))).toBe(
      false,
    );
    const ready = settleInitiationPass(plan, 'one', false);
    expect(ready.find((step) => step.id === 'one:ready')?.done).toBe(true);
    expect(ready.find((step) => step.id === 'one:stopped')?.done).toBe(false);
    expect(completeInitiationRun(ready, 'one').every((step) => step.done)).toBe(true);
  });

  it('settles the first pass without prematurely finishing the connection run', () => {
    const plan = initiationMilestones('java', 'fabric', ['Geyser', 'mods'], true);
    const firstStopped = settleInitiationPass(plan, 'one', true);
    expect(
      firstStopped.filter((step) => step.id.startsWith('one:')).every((step) => step.done),
    ).toBe(true);
    expect(
      firstStopped.filter((step) => !step.id.startsWith('one:')).every((step) => !step.done),
    ).toBe(true);
    const choicesMade = firstStopped.map((step) =>
      step.id === 'connections' ? { ...step, done: true } : step,
    );
    expect(completeInitiationRun(choicesMade, 'two').every((step) => step.done)).toBe(true);
  });

  it('honors direct engine completion even if a second pass was initially expected', () => {
    const plan = initiationMilestones('bedrock', '', [], true);
    const completed = completeInitiationRun(plan, 'one');
    expect(completed.every((step) => step.id.startsWith('one:') && step.done)).toBe(true);
    expect(plan.every((step) => !step.done)).toBe(true);
  });
});
