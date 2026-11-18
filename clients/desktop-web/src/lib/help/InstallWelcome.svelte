<script lang="ts">
  import { onMount } from 'svelte';
  import Button from '../components/base/Button.svelte';
  import OnboardingCard from './OnboardingCard.svelte';

  const persistenceKey = 'msc.install-welcome-dismissed';
  let dismissed = true;

  onMount(() => {
    try {
      dismissed = localStorage.getItem(persistenceKey) === 'true';
    } catch {
      dismissed = false;
    }
  });

  function dismiss(): void {
    dismissed = true;
    try {
      localStorage.setItem(persistenceKey, 'true');
    } catch {
      // Keep Okay usable even when client preferences cannot be saved.
    }
  }

  function focusOkay(node: HTMLElement): { destroy: () => void } {
    const previousFocus = document.activeElement;
    const button = node.querySelector('button');
    button?.focus();
    function handleKey(event: KeyboardEvent): void {
      if (event.key === 'Tab') {
        event.preventDefault();
        button?.focus();
      } else if (event.key === 'Escape') {
        event.preventDefault();
        event.stopPropagation();
        dismiss();
      }
    }
    node.addEventListener('keydown', handleKey);
    return {
      destroy: () => {
        node.removeEventListener('keydown', handleKey);
        if (previousFocus instanceof HTMLElement && previousFocus.isConnected)
          previousFocus.focus();
      },
    };
  }
</script>

{#if !dismissed}
  <div
    class="welcome-overlay"
    role="dialog"
    aria-modal="true"
    aria-label="Welcome to MSC!"
    use:focusOkay
  >
    <OnboardingCard
      title="Welcome to MSC!"
      body="Please install the engine or connect to a host to get started."
    >
      <Button variant="primary" onclick={dismiss}>Okay</Button>
    </OnboardingCard>
  </div>
{/if}

<style>
  .welcome-overlay {
    position: fixed;
    inset: 0;
    z-index: 500;
    background: rgba(0, 0, 0, 0.72);
  }
</style>
