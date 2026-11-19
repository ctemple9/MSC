<script lang="ts">
  import Sheet from '../components/base/Sheet.svelte';
  import Button from '../components/base/Button.svelte';
  import HowMscWorks from '../sections/setup/HowMscWorks.svelte';
  export let onNext: () => void;

  function focusSheet(node: HTMLElement) {
    const previous = document.activeElement;
    const dialog = node.closest<HTMLElement>('[role="dialog"]') ?? node;
    let alive = true;
    queueMicrotask(() => {
      if (alive) node.querySelector<HTMLButtonElement>('.next button')?.focus();
    });
    function trapTab(event: KeyboardEvent) {
      if (event.key !== 'Tab') return;
      const targets = [
        ...dialog.querySelectorAll<HTMLElement>(
          'button:not(:disabled), summary, a[href], [tabindex="0"]',
        ),
      ].filter((target) => target.getClientRects().length > 0);
      const first = targets[0];
      const last = targets[targets.length - 1];
      if (event.shiftKey && document.activeElement === first) {
        event.preventDefault();
        last?.focus();
      } else if (!event.shiftKey && document.activeElement === last) {
        event.preventDefault();
        first?.focus();
      }
    }
    dialog.addEventListener('keydown', trapTab);
    return {
      destroy() {
        alive = false;
        dialog.removeEventListener('keydown', trapTab);
        if (previous instanceof HTMLElement && previous.isConnected) previous.focus();
      },
    };
  }
</script>

<Sheet title="Welcome to MSC!" onClose={onNext}>
  <div use:focusSheet>
    <p class="welcome-intro">Install the engine or connect to a computer to get started.</p>
    <HowMscWorks />
    <div class="next"><Button variant="primary" onclick={onNext}>Next</Button></div>
  </div>
</Sheet>

<style>
  .welcome-intro {
    margin: 0 0 26px;
    color: var(--msc-text-secondary);
    font-size: 14px;
    line-height: 1.5;
  }
  .next {
    display: flex;
    justify-content: flex-end;
    margin-top: 28px;
  }
</style>
