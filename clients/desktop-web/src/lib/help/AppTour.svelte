<script lang="ts">
  import { onDestroy, onMount, tick } from 'svelte';
  import { get } from 'svelte/store';
  import Button from '../components/base/Button.svelte';
  import OnboardingCard from './OnboardingCard.svelte';
  import TourOverlay from './TourOverlay.svelte';
  import {
    appTourStep,
    tourCreationClosed,
    appTourMatches,
    nextAppTourStep,
    type AppTourCheckpoint,
  } from './appTour';
  import { anchorFrames } from './tourAnchors';
  import type { OnboardingGuide, OnboardingStep } from './types';
  import type { Schema } from '../sections/shared/types';

  export let guide: OnboardingGuide;
  export let checkpoint: AppTourCheckpoint;
  export let hostId: string;
  export let server: Schema['ServerDTO'] | undefined;
  export let initiationVisible = false;
  export let onPrepare: (id: string) => Promise<boolean>;
  export let onCheckpoint: (value: AppTourCheckpoint) => void;
  export let onFinish: () => void;

  let preparing = false;
  let preparationError = '';
  let prepared = '';
  let disposed = false;
  let preparationVersion = 0;
  let direction = 1;
  $: sameServer = appTourMatches(checkpoint, hostId, server?.id);
  $: steps = guide.steps.slice(guide.steps.findIndex((step) => step.id === 'created-done'));
  $: index = steps.findIndex((step) => step.id === checkpoint.step);
  $: sourceStep = steps[index];
  $: step = sourceStep
    ? {
        ...adapt(sourceStep, server, guide.variants ?? {}),
        anchor:
          sourceStep.id === 'world-map' && $anchorFrames.ob_tour_map_view
            ? 'ob_tour_map_view'
            : sourceStep.anchor,
      }
    : undefined;
  $: paused = checkpoint.step === 'waiting-initiation' || !sameServer || initiationVisible;
  $: if (paused) {
    prepared = '';
    appTourStep.set(null);
  }
  $: if (!paused && step && prepared !== step.id && !preparing && !preparationError)
    void prepare(step.id);
  $: if (
    sameServer &&
    checkpoint.step === 'waiting-initiation' &&
    !initiationVisible &&
    server?.firstStartRequired === false
  )
    move('start-controls');
  $: if (sameServer && checkpoint.step === 'created-done' && $tourCreationClosed) move('initiate');

  function move(id: string): void {
    preparationError = '';
    prepared = '';
    onCheckpoint({ ...checkpoint, step: id });
  }
  async function prepare(id: string): Promise<void> {
    preparing = true;
    const version = ++preparationVersion;
    appTourStep.set(id);
    try {
      const available = await onPrepare(id);
      await tick();
      if (disposed || version !== preparationVersion || checkpoint.step !== id || !sameServer)
        return;
      if (!available || (id === 'world-map' && !get(anchorFrames).ob_tour_map)) {
        if (direction < 0) back();
        else advance();
        return;
      }
      prepared = id;
    } catch {
      if (!disposed && version === preparationVersion)
        preparationError = 'This screen could not be opened. Retry, or finish the tour for now.';
    } finally {
      if (version === preparationVersion) preparing = false;
    }
  }
  function advance(): void {
    if (checkpoint.step === 'initiate') {
      move('waiting-initiation');
      return;
    }
    if (checkpoint.step === 'done') {
      finish();
      return;
    }
    direction = 1;
    const next = nextAppTourStep(
      checkpoint.step,
      steps.map((step) => step.id),
    );
    if (next) move(next);
    else finish();
  }
  function back(): void {
    direction = -1;
    const previous = steps[index - 1];
    if (previous && previous.id !== 'initiate' && previous.id !== 'created-done') move(previous.id);
  }
  function finish(): void {
    // Keep the optional entry available without requiring server creation again.
    if (index < steps.findIndex((step) => step.id === 'console') || checkpoint.step === 'done')
      onCheckpoint({ ...checkpoint, step: 'console' });
    appTourStep.set(null);
    onFinish();
  }
  function adapt(
    source: OnboardingStep,
    selected: Schema['ServerDTO'] | undefined,
    variants: Record<string, string>,
  ): OnboardingStep {
    let body = source.body;
    const variant = (key: string) => variants[key] ?? '';
    const bedrock = selected?.serverType === 'bedrock';
    const modded = ['fabric', 'forge', 'neoforge', 'quilt'].includes(selected?.javaFlavor ?? '');
    if (source.id === 'initiate' && !selected?.playitEnabled && !selected?.xboxBroadcastEnabled)
      body = variant('initiate-simple') || body;
    if (source.id === 'connection') {
      if (selected?.playitEnabled) body += ` ${variant('connection-playit')}`;
      if (selected?.xboxBroadcastEnabled) body += ` ${variant('connection-broadcast')}`;
    }
    if (source.id === 'world-packs-detail' && bedrock) body = variant('packs-bedrock') || body;
    if (source.id === 'performance')
      body += ` ${variant(bedrock ? 'performance-bedrock' : 'performance-java')}`;
    if (source.id === 'components') {
      const key = bedrock
        ? 'components-bedrock'
        : modded
          ? 'components-modded'
          : selected?.javaFlavor === 'vanilla'
            ? 'components-vanilla'
            : '';
      if (key) body = variant(key) || body;
      if (!bedrock && selected?.bedrockPort) body += ` ${variant('components-helpers')}`;
    }
    if (source.id === 'edit-server' && !bedrock) body += ` ${variant('edit-java')}`;
    return { ...source, body };
  }
  onMount(() => {
    const finished = (event: Event) => {
      const detail = (event as CustomEvent<{ hostId: string; serverId: string }>).detail;
      if (
        detail.hostId === checkpoint.hostId &&
        detail.serverId === checkpoint.serverId &&
        checkpoint.step === 'waiting-initiation'
      )
        move('start-controls');
    };
    const discarded = (event: Event) => {
      const detail = (event as CustomEvent<{ hostId: string; serverId: string }>).detail;
      if (detail.hostId === checkpoint.hostId && detail.serverId === checkpoint.serverId)
        move('initiate');
    };
    window.addEventListener('msc:tour-initiation-finished', finished);
    window.addEventListener('msc:tour-initiation-discarded', discarded);
    return () => {
      window.removeEventListener('msc:tour-initiation-finished', finished);
      window.removeEventListener('msc:tour-initiation-discarded', discarded);
    };
  });
  onDestroy(() => {
    disposed = true;
    preparationVersion++;
    appTourStep.set(null);
  });
</script>

{#if !paused && preparationError}
  <div class="tour-error" role="dialog" aria-modal="true" aria-label="Tour paused">
    <OnboardingCard title="Tour paused" body={preparationError}>
      <Button variant="secondary" onclick={finish}>Finish for now</Button>
      <Button
        variant="primary"
        onclick={() => {
          preparationError = '';
        }}>Retry</Button
      >
    </OnboardingCard>
  </div>
{:else if !paused && !preparing && step && prepared === step.id}
  <TourOverlay
    steps={[step]}
    stepIndex={0}
    appWalkthrough
    onBack={index > 2 ? back : undefined}
    skipLabel={step.id === 'explore-choice' ? 'Finish for now' : 'Finish tour'}
    onAdvance={advance}
    onSkip={finish}
    onComplete={finish}
  />
{/if}

<style>
  .tour-error {
    position: fixed;
    inset: 0;
    z-index: 500;
    background: rgba(0, 0, 0, 0.72);
  }
</style>
