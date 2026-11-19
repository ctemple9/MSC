<script lang="ts">
  import { onMount } from 'svelte';
  import WelcomeMscSheet from './WelcomeMscSheet.svelte';

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
      // Keep Next usable even when client preferences cannot be saved.
    }
  }
</script>

{#if !dismissed}<WelcomeMscSheet onNext={dismiss} />{/if}
