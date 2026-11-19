<script lang="ts">
  import { onDestroy, onMount } from 'svelte';
  import type { Schema, ScreenApi } from '../sections/shared/types';
  import Button from '../components/base/Button.svelte';
  import OnboardingCard from './OnboardingCard.svelte';
  import AppTour from './AppTour.svelte';
  import {
    appTourStep,
    readAppTour,
    saveAppTour,
    tourCreationClosed,
    type AppTourCheckpoint,
  } from './appTour';
  import SetupIntro from './SetupIntro.svelte';
  import TourOverlay from './TourOverlay.svelte';
  import {
    applicableTourSteps,
    firstLaunchStage,
    nextTourStep,
    tourServerContext,
    type FirstLaunchState,
  } from './onboarding';
  import type { OnboardingGuide } from './types';

  export let api: ScreenApi;
  export let engineReady = false;
  export let hostId = '';
  export let server: Schema['ServerDTO'] | undefined = undefined;
  export let initiationVisible = false;
  export let onPrepareTourStep: (id: string) => Promise<boolean> = async () => true;
  let continuation: AppTourCheckpoint | null = null;
  let continuationLoading = false;
  let continuationError = '';
  let explorationRequested = false;

  function checkpoint(value: AppTourCheckpoint): void {
    continuation = value;
    saveAppTour(value);
  }
  async function continueAfterCreation(): Promise<void> {
    if (continuationLoading) return;
    continuationLoading = true;
    continuationError = '';
    const creationHostId = hostId;
    const creationApi = api.snapshot?.() ?? api;
    try {
      if (!onboarding?.steps.some((step) => step.id === 'created-done')) {
        onboarding = await creationApi.get<OnboardingGuide>('/v1/guides/onboarding');
      }
      if (!onboarding?.steps.some((step) => step.id === 'created-done')) {
        continuationError = 'Restart or repair the engine to load the expanded tour, then retry.';
        return;
      }
      const status = await creationApi.get<Schema['RemoteAPIStatus']>('/v1/status');
      if (!status.activeServerId) throw new Error('No created server selected');
      checkpoint({
        hostId: creationHostId,
        serverId: status.activeServerId,
        step: explorationRequested ? 'console' : 'created-done',
      });
    } catch {
      continuationError =
        'The tour could not find the created server. You can retry without creating another server.';
    } finally {
      continuationLoading = false;
    }
  }

  export let onResetRecovery: () => void = () => {};
  let resetRecovery = false;

  let onboarding: OnboardingGuide | null = null;
  let ready = false;
  let tourIndex = 0;
  let state: FirstLaunchState = {
    setupComplete: false,
    tourComplete: false,
  };

  $: stage = firstLaunchStage(state);
  $: onboardingOpen =
    engineReady &&
    ready &&
    !resetRecovery &&
    onboarding !== null &&
    stage !== 'complete' &&
    (!continuation || $appTourStep !== null);
  $: tourSteps = onboarding
    ? applicableTourSteps(
        onboarding.steps.filter(
          (step) =>
            step.order <=
            (onboarding!.steps.find((entry) => entry.id === 'create')?.order ?? Infinity),
        ),
        $tourServerContext,
      )
    : [];

  function setPageScrollLocked(locked: boolean): void {
    if (typeof document === 'undefined' || !document.body) return;
    document.documentElement.classList.toggle('msc-onboarding-open', locked);
    document.body.classList.toggle('msc-onboarding-open', locked);
  }

  $: setPageScrollLocked(onboardingOpen);

  function readState(setupComplete: boolean): FirstLaunchState {
    if (!onboarding) return state;
    const tourComplete = localStorage.getItem(onboarding.reopen.persistenceKey) === 'true';
    return {
      setupComplete,
      tourComplete,
    };
  }
  function writeState(next: FirstLaunchState): void {
    state = next;
    if (!onboarding) return;
    localStorage.setItem(onboarding.reopen.persistenceKey, String(next.tourComplete));
  }
  function completeTour(): void {
    writeState({ ...state, tourComplete: true });
    window.dispatchEvent(new CustomEvent('msc:tour-complete'));
  }
  function completeSetup(): void {
    writeState({ ...state, setupComplete: true });
    window.dispatchEvent(new CustomEvent('msc:setup-complete'));
  }
  function restart(): void {
    continuation = null;
    explorationRequested = false;
    continuationError = '';
    saveAppTour(null);
    tourCreationClosed.set(false);
    tourIndex = 0;
    writeState({ ...state, tourComplete: false });
  }
  function advance(userActionCompleted = false): void {
    if (!onboarding) return;
    if (tourSteps.length === 0) {
      completeTour();
      return;
    }
    const next = nextTourStep(tourSteps, tourIndex, userActionCompleted);
    if (next === tourIndex && tourIndex === tourSteps.length - 1) {
      completeTour();
    } else {
      tourIndex = next;
    }
  }

  onMount(() => {
    if (!engineReady) return;
    const load = async () => {
      try {
        const [loadedOnboarding, hostSetup, reset] = await Promise.all([
          api.get<OnboardingGuide>('/v1/guides/onboarding'),
          api.get<{ complete: boolean }>('/v1/config/host-setup'),
          api.get<{ state: string }>('/v1/host/reset').catch(() => null),
        ]);
        onboarding = loadedOnboarding;
        state = readState(hostSetup.complete);
        continuation = readAppTour();
        if (continuation?.step === 'created-done')
          checkpoint({ ...continuation, step: 'initiate' });
        resetRecovery = reset?.state === 'blocked' || reset?.state === 'pending';
        if (resetRecovery) onResetRecovery();
      } catch {
        // A pre-P11.24 engine does not serve guide data yet. The regular client
        // remains usable rather than inventing local onboarding prose.
      } finally {
        ready = true;
      }
    };
    void load();
    const onRestart = () => restart();
    const onExplore = () => {
      if (!server) return;
      explorationRequested = true;
      continuationError = '';
      // An explicit Explore action starts a fresh app tour; saved checkpoints
      // remain for reopening an unfinished session, not for this button.
      checkpoint({ hostId, serverId: server.id, step: 'console' });
      writeState({ setupComplete: true, tourComplete: false });
      if (!onboarding?.steps.some((step) => step.id === 'created-done')) {
        continuationError = 'Restart or repair the engine to load the expanded tour, then retry.';
      }
    };
    window.addEventListener('msc:explore-app', onExplore);
    const refreshRecovery = () => {
      void api
        .get<{ state: string }>('/v1/host/reset')
        .then((reset) => {
          resetRecovery = reset.state === 'blocked' || reset.state === 'pending';
        })
        .catch(() => {});
    };
    window.addEventListener('msc:restart-tour', onRestart);
    window.addEventListener('msc:reset-recovery-closed', refreshRecovery);
    return () => {
      window.removeEventListener('msc:restart-tour', onRestart);
      window.removeEventListener('msc:explore-app', onExplore);
      window.removeEventListener('msc:reset-recovery-closed', refreshRecovery);
    };
  });

  onDestroy(() => setPageScrollLocked(false));
</script>

{#if engineReady && ready && onboarding && !resetRecovery}
  {#if stage === 'setup'}
    <div class="gate-backdrop" role="presentation">
      <section
        class="gate"
        aria-live="polite"
        aria-labelledby="first-launch-title"
        data-onboarding-stage={stage}
      >
        <SetupIntro {api} onComplete={completeSetup} />
      </section>
    </div>
  {:else if stage === 'tour' && continuationError}
    <div
      class="gate-backdrop"
      style="z-index:500"
      role="dialog"
      aria-modal="true"
      aria-label="Tour paused"
    >
      <OnboardingCard title="Tour paused" body={continuationError}>
        <Button variant="secondary" onclick={completeTour}>Finish for now</Button>
        <Button variant="primary" onclick={() => void continueAfterCreation()}>Retry</Button>
      </OnboardingCard>
    </div>
  {:else if stage === 'tour' && continuation}
    <AppTour
      guide={onboarding}
      checkpoint={continuation}
      {hostId}
      {server}
      {initiationVisible}
      onPrepare={onPrepareTourStep}
      onCheckpoint={checkpoint}
      onFinish={completeTour}
    />
  {:else if stage === 'tour' && !continuationLoading}
    <TourOverlay
      steps={tourSteps}
      stepIndex={tourIndex}
      skipLabel={onboarding.skip.label}
      onAdvance={advance}
      onSkip={completeTour}
      onComplete={() => void continueAfterCreation()}
    />
  {/if}
{/if}

<style>
  .gate-backdrop {
    position: fixed;
    z-index: 40;
    inset: 0;
    display: grid;
    place-items: center;
    padding: 1rem;
    background: rgba(7, 12, 16, 0.76);
  }
  .gate {
    width: min(100%, 38rem);
    max-height: calc(100vh - 2rem);
    padding: 1.5rem;
    overflow-y: auto;
    border: 1px solid var(--msc-border);
    border-radius: var(--msc-radius-lg);
    background: var(--msc-surface-raised);
    box-shadow: var(--msc-shadow);
    scrollbar-width: none;
  }
  .gate::-webkit-scrollbar {
    display: none;
    width: 0;
  }
  :global(html.msc-onboarding-open),
  :global(body.msc-onboarding-open) {
    overflow: hidden;
  }
</style>
