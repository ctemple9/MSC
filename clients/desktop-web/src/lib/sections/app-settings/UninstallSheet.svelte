<script lang="ts">
  import { onMount } from 'svelte';
  import Sheet from '../../components/base/Sheet.svelte';
  import Button from '../../components/base/Button.svelte';
  import ConfirmDialog from '../../components/ConfirmDialog.svelte';
  import { getPlatform } from '../../platform';
  import type { UninstallInventory } from '../../platform/types';
  import { errorMessage } from '../shared/types';

  export let onClose: () => void;
  let inventory: UninstallInventory | undefined;
  let installers: string[] = [];
  let loading = false;
  let busy = false;
  let error = '';
  let confirmation = '';
  let acknowledged = false;
  let keepReport = true;
  let showConfirmation = false;
  let scheduled = false;
  let reportPath = '';
  let selectedPaths: string[] = [];
  $: targets = inventory
    ? [
        ...new Map(
          inventory.entries
            .filter((entry) => entry.state !== 'missing')
            .map((entry) => [entry.path ?? entry.identity, entry]),
        ).values(),
      ]
    : [];
  $: blocked = inventory?.entries.some((entry) => entry.state === 'blocked') ?? true;
  $: ready = !!inventory && !loading && !busy && acknowledged && confirmation === 'UNINSTALL MSC';

  async function refresh(): Promise<void> {
    if (loading || busy) return;
    loading = true;
    inventory = undefined;
    selectedPaths = [];
    confirmation = '';
    acknowledged = false;
    error = '';
    try {
      inventory = await (await getPlatform()).previewLocalUninstall(installers);
      selectedPaths = [
        ...new Set(
          inventory.entries
            .filter((entry) => entry.state !== 'missing' && entry.path)
            .map((entry) => entry.path!),
        ),
      ];
    } catch (caught) {
      error = errorMessage(caught);
    } finally {
      loading = false;
    }
  }
  async function addInstaller(): Promise<void> {
    const path = await (
      await getPlatform()
    ).pickFilePath({
      label: 'Select an MSC installer',
      extensions: ['dmg', 'msi', 'deb', 'rpm'],
    });
    if (!path || installers.includes(path)) return;
    installers = [...installers, path];
    await refresh();
  }
  async function removeInstaller(path: string): Promise<void> {
    installers = installers.filter((candidate) => candidate !== path);
    await refresh();
  }
  async function uninstall(): Promise<void> {
    showConfirmation = false;
    if (!ready || !inventory) return;
    busy = true;
    error = '';
    try {
      const platform = await getPlatform();
      const result = await platform.uninstallLocal({
        confirmation,
        fingerprint: inventory.fingerprint,
        installers,
        keepReport,
        removeAnyway: selectedPaths.filter((path) =>
          inventory!.entries.some((entry) => entry.path === path && entry.state === 'blocked'),
        ),
        keepPaths: targets
          .filter((entry) => entry.path && !selectedPaths.includes(entry.path))
          .map((entry) => entry.path!),
      });
      reportPath = result.reportPath;
      scheduled = true;
      await platform.quitApplication();
    } catch (caught) {
      error = errorMessage(caught);
    } finally {
      busy = false;
    }
  }
  onMount(() => void refresh());
</script>

<Sheet
  title="Uninstall MSC"
  size="lg"
  onClose={busy || scheduled || showConfirmation ? undefined : onClose}
>
  <div class="stack">
    {#if scheduled}
      <h2>Uninstall scheduled</h2>
      <p>
        MSC must close before removal can continue. The worker verifies the local inventory again,
        then stops Minecraft and removes the installation.
      </p>
      <p class="path">Result or failure report: {reportPath}</p>
      <Button
        variant="destructive"
        onclick={() => void getPlatform().then((platform) => platform.quitApplication())}
        >Close MSC</Button
      >
    {:else}
      <h2>Remove MSC from this computer</h2>
      <p>
        Everything listed below is selected for removal by default. Uncheck anything you want to
        keep. Selected folders and everything inside them, including worlds and backups, are
        permanently deleted. Remote computers are unaffected.
      </p>
      {#if loading}<p role="status">Inspecting the local installation…</p>{/if}
      {#if inventory}
        <h3>{inventory.computer}</h3>
        <h3>Removal details</h3>
        <ul class="targets">
          {#each targets as entry}
            <li class:blocked={entry.state === 'blocked'}>
              <span class="path">{entry.identity}</span>
              {#if entry.path && entry.path !== entry.identity}<span class="path detail"
                  >{entry.path}</span
                >{/if}
              <span class="detail">{entry.problem ?? entry.evidence}</span>
              {#if entry.path}
                <label class="acknowledgement"
                  ><input
                    type="checkbox"
                    bind:group={selectedPaths}
                    value={entry.path}
                    disabled={busy || loading}
                  />Remove this item and its contents</label
                >
              {/if}
            </li>
          {/each}
        </ul>
        {#if blocked}<p role="status">
            Some items could not be verified. They are still selected for removal; uncheck any you
            want to keep. The final confirmation applies to the selected paths and their entire
            contents.
          </p>{/if}
        <h3>What stays</h3>
        <ul class="exclusions">
          {#each inventory.exclusions as exclusion}<li>{exclusion}</li>{/each}
        </ul>
        {#each inventory.warnings as warning}<p class="detail">{warning}</p>{/each}
      {/if}
      <div class="actions">
        <Button size="sm" disabled={loading || busy} onclick={() => void addInstaller()}
          >Add installer…</Button
        >
        <Button size="sm" disabled={loading || busy} onclick={() => void refresh()}
          >Refresh preview</Button
        >
      </div>
      {#each installers as installer}
        <div class="installer">
          <span class="path">{installer}</span><Button
            size="sm"
            disabled={loading || busy}
            onclick={() => void removeInstaller(installer)}>Remove from list</Button
          >
        </div>
      {/each}
      <p class="detail">
        Listed installers follow your removal choices above. Unknown or renamed installers are not
        discovered automatically; add them explicitly if you want them removed. OS package caches
        are managed by the operating system.
      </p>
      <label class="acknowledgement"
        ><input type="checkbox" bind:checked={acknowledged} disabled={loading || busy} />I
        understand the listed verified files and any paths I selected will be permanently deleted.</label
      >
      <label class="acknowledgement"
        ><input type="checkbox" bind:checked={keepReport} disabled={busy} />Keep a removal report in
        my home folder.</label
      >
      <div class="confirmation">
        <label for="local-uninstall-confirm">Type <code>UNINSTALL MSC</code> to continue</label>
        <input
          id="local-uninstall-confirm"
          class="confirm-field"
          bind:value={confirmation}
          autocomplete="off"
          spellcheck="false"
          disabled={loading || busy}
        />
      </div>
      <p class="detail">
        MSC will close. An OS authorization prompt may appear while the worker removes services and
        the app. Failed removal keeps a report; scheduled work is not yet completed.
      </p>
      <div class="actions">
        <Button disabled={busy} onclick={onClose}>Cancel</Button>
        <Button variant="destructive" disabled={!ready} onclick={() => (showConfirmation = true)}
          >{busy ? 'Scheduling removal…' : 'Uninstall MSC…'}</Button
        >
      </div>
    {/if}
    {#if error}<p class="error" role="alert">{error}</p>{/if}
  </div>
</Sheet>
<ConfirmDialog
  open={showConfirmation}
  context={`Computer: ${inventory?.computer ?? 'This computer'}`}
  title="Permanently uninstall MSC?"
  message="The selected items will be permanently deleted, including their entire contents. Unchecked items will be kept. MSC will close and the installed app will be removed. Remote engines remain installed."
  confirmLabel="Uninstall and close MSC"
  onConfirm={() => void uninstall()}
  onClose={() => (showConfirmation = false)}
/>

<style>
  .stack {
    display: flex;
    flex-direction: column;
    gap: 16px;
  }
  h2,
  h3,
  p {
    margin: 0;
  }
  h2 {
    font-size: 20px;
    font-weight: 500;
  }
  h3 {
    font-size: 14px;
    font-weight: 500;
  }
  p,
  .acknowledgement {
    font-size: 13px;
    line-height: 1.5;
  }
  .detail,
  .exclusions {
    font-size: 12px;
    line-height: 1.5;
    color: var(--msc-text-secondary);
  }
  .targets {
    list-style: none;
    padding: 0;
    margin: 0;
  }
  .targets li {
    display: flex;
    flex-direction: column;
    gap: 4px;
    padding: 10px 0;
    border-bottom: 1px solid var(--msc-hairline);
  }
  .path {
    font-size: 12px;
    overflow-wrap: anywhere;
  }
  .exclusions {
    margin: 0;
    padding-left: 20px;
  }
  .error,
  .blocked .detail {
    color: var(--msc-status-error);
  }
  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 10px;
  }
  .installer {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
  }
  .acknowledgement {
    display: flex;
    align-items: flex-start;
    gap: 10px;
  }
  .acknowledgement input {
    margin-top: 4px;
  }
  .confirmation {
    display: grid;
    gap: 8px;
    font-size: 13px;
  }
  .confirm-field {
    padding: 9px 12px;
    background: var(--msc-tier-terminal);
    color: var(--msc-text-primary);
    border: 1px solid var(--msc-hairline);
    border-radius: 6px;
    font: inherit;
  }
  .confirm-field:focus-visible {
    outline: 2px solid var(--msc-text-secondary);
    outline-offset: 2px;
  }
</style>
