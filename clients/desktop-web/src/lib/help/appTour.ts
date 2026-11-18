import { writable } from 'svelte/store';

export const appTourStep = writable<string | null>(null);
export const tourCreationClosed = writable(false);
export type AppTourCheckpoint = { hostId: string; serverId: string; step: string };
export const APP_TOUR_KEY = 'msc.app-tour-checkpoint';
export function readAppTour(): AppTourCheckpoint | null {
  try {
    const value = JSON.parse(localStorage.getItem(APP_TOUR_KEY) ?? 'null');
    return value &&
      typeof value.hostId === 'string' &&
      typeof value.serverId === 'string' &&
      typeof value.step === 'string'
      ? value
      : null;
  } catch {
    return null;
  }
}
export function saveAppTour(value: AppTourCheckpoint | null): void {
  try {
    if (value) localStorage.setItem(APP_TOUR_KEY, JSON.stringify(value));
    else localStorage.removeItem(APP_TOUR_KEY);
  } catch {
    /* The current walkthrough remains usable without saved preferences. */
  }
}

export function appTourMatches(
  checkpoint: AppTourCheckpoint,
  hostId: string,
  serverId: string | undefined,
): boolean {
  return checkpoint.hostId === hostId && checkpoint.serverId === serverId;
}

export function nextAppTourStep(current: string, orderedIds: readonly string[]): string | null {
  if (current === 'initiate') return 'waiting-initiation';
  if (current === 'waiting-initiation') return null;
  const index = orderedIds.indexOf(current);
  return index < 0 ? null : (orderedIds[index + 1] ?? null);
}
