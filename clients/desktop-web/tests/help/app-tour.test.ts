import { describe, expect, it } from 'vitest';
import guide from '../../../../content/guides/onboarding.json';
import { appTourMatches, nextAppTourStep } from '../../src/lib/help/appTour';

const ids = guide.steps.map((step) => step.id);
describe('app walkthrough boundaries', () => {
  it('waits for guided initiation instead of advancing on a timer or another Next', () => {
    expect(nextAppTourStep('created-done', ids)).toBe('initiate');
    expect(nextAppTourStep('initiate', ids)).toBe('waiting-initiation');
    expect(nextAppTourStep('waiting-initiation', ids)).toBeNull();
  });
  it('offers optional exploration after connection addresses and before console', () => {
    expect(nextAppTourStep('start-controls', ids)).toBe('server-controls');
    expect(nextAppTourStep('server-controls', ids)).toBe('connection');
    expect(nextAppTourStep('connection', ids)).toBe('explore-choice');
    expect(nextAppTourStep('explore-choice', ids)).toBe('console');
  });
  it('pauses a saved tour when the host or selected server differs', () => {
    const saved = { hostId: 'local', serverId: 'first', step: 'connection' };
    expect(appTourMatches(saved, 'local', 'first')).toBe(true);
    expect(appTourMatches(saved, 'remote', 'first')).toBe(false);
    expect(appTourMatches(saved, 'local', 'another')).toBe(false);
    expect(appTourMatches(saved, 'local', undefined)).toBe(false);
  });
});
