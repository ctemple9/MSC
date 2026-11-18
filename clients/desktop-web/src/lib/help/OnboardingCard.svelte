<script lang="ts">
  export let title: string;
  export let body: string;
</script>

<div class="bookend-card">
  <h2>{title}</h2>
  <p>{body}</p>
  <div class="actions"><slot /></div>
</div>

<style>
  .bookend-card {
    position: fixed;
    top: 50%;
    left: 50%;
    transform: translate(-50%, -50%);
    width: min(420px, calc(100vw - 48px));
    max-height: calc(100dvh - 48px);
    overflow-y: auto;
    box-sizing: border-box;
    padding: 28px;
    border-radius: var(--msc-radius-3);
    background: var(--msc-tier-chrome);
    border: 1px solid var(--msc-hairline);
    text-align: center;
    pointer-events: auto;
  }
  h2 {
    margin: 0 0 8px;
    font-size: 19px;
    font-weight: 600;
    color: var(--msc-text-primary);
  }
  p {
    margin: 0;
    font-size: 13px;
    line-height: 1.5;
    color: var(--msc-text-secondary);
  }
  .actions {
    display: flex;
    justify-content: center;
    gap: 8px;
    margin-top: 14px;
  }
</style>
