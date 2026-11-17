<script lang="ts">
  import { onDestroy } from 'svelte';
  import Card from '../../../components/base/Card.svelte';
  import Field from '../../../components/base/Field.svelte';
  import Button from '../../../components/base/Button.svelte';
  import Sheet from '../../../components/base/Sheet.svelte';
  import JavaInstallSheet from '../../server-editor/JavaInstallSheet.svelte';
  import { serverEditorPaths } from '../../server-editor/model';
  import { getPlatform } from '../../../platform';
  import { tourJavaSelectionPending } from '../../../help/onboarding';
  import { onboardingAnchor } from '../../../help/tourAnchors';
  import type { Schema, ScreenApi } from '../../shared/types';
  import { errorMessage } from '../../shared/types';
  import {
    javaSelectionKey,
    requiredJavaMajorForVersion,
    versionsForCreatePath,
    type WizardDraft,
  } from './model';

  export let api: ScreenApi | undefined;
  export let draft: WizardDraft;

  let pathDraft = draft.javaPath ?? '';
  let loading = false;
  let busy = false;
  let error = '';
  let version = '';
  let requiredMajor = 25;
  let resolvedVersionId: string | undefined;
  let requestKey = '';
  let lastApi: ScreenApi | undefined;
  let generation = 0;
  let disposed = false;
  let showDetected = false;
  let showInstall = false;
  let runtimes: Schema['JavaRuntimeDTO'][] = [];

  $: tourJavaSelectionPending.set(showDetected || showInstall);
  $: contextKey = javaSelectionKey(draft);
  $: if (contextKey !== requestKey || api !== lastApi) {
    const hostChanged = lastApi !== undefined && api !== lastApi;
    requestKey = contextKey;
    lastApi = api;
    void loadRequirement(contextKey, api, hostChanged);
  }
  $: if (pathDraft !== (draft.javaPath ?? '')) selectPath(pathDraft);

  async function loadRequirement(
    key: string,
    client: ScreenApi | undefined,
    hostChanged = false,
  ): Promise<void> {
    const request = ++generation;
    loading = true;
    error = '';
    version = '';
    resolvedVersionId = undefined;
    if (draft.javaSelectionKey !== key || hostChanged) {
      pathDraft = '';
      draft = { ...draft, javaPath: undefined, javaSelectionKey: undefined };
    }
    draft.javaRequirementPending = true;
    try {
      if (!client) throw new Error('Connect to an engine to check the required Java version.');
      const response = await client.get<Schema['VersionsResponseDTO']>(
        versionsForCreatePath('java', draft.javaFlavor),
      );
      if (disposed || request !== generation || client.isCurrent?.() === false) return;
      const entry = draft.versionId
        ? response.versions.find((entry) => entry.id === draft.versionId)
        : (response.versions.find((entry) => entry.isLatest) ?? response.versions[0]);
      if (!entry?.mcVersion)
        throw new Error(
          'The Minecraft version could not be resolved. Choose a version and try again.',
        );
      version = entry.mcVersion;
      requiredMajor = requiredJavaMajorForVersion(version);
      resolvedVersionId = entry.id;
      if (!draft.versionId) {
        draft = { ...draft, versionId: entry.id };
        requestKey = javaSelectionKey(draft);
        if (pathDraft.trim()) draft.javaSelectionKey = requestKey;
      }
      draft.javaRequirementPending = false;
      if (pathDraft.trim()) selectPath(pathDraft);
    } catch (cause) {
      if (!disposed && request === generation) error = errorMessage(cause);
    } finally {
      if (!disposed && request === generation) loading = false;
    }
  }

  function selectPath(value: string): void {
    pathDraft = value;
    const next = { ...draft, javaPath: value };
    // Pin the release whose requirement was shown when a runtime is specified.
    if (value.trim() && resolvedVersionId && !next.versionId) next.versionId = resolvedVersionId;
    next.javaSelectionKey = value.trim() ? javaSelectionKey(next) : undefined;
    draft = next;
  }

  async function detect(): Promise<void> {
    if (!api || busy) return;
    busy = true;
    error = '';
    const client = api;
    const context = contextKey;
    try {
      const response = await client.get<Schema['JavaRuntimesResponseDTO']>(
        serverEditorPaths.javaRuntimes,
      );
      if (disposed || client !== api || context !== contextKey || client.isCurrent?.() === false)
        return;
      runtimes = response.runtimes ?? [];
      showDetected = true;
    } catch (cause) {
      if (!disposed) error = errorMessage(cause);
    } finally {
      if (!disposed) busy = false;
    }
  }

  async function browse(): Promise<void> {
    if (busy) return;
    busy = true;
    error = '';
    try {
      const chosen = await (await getPlatform()).pickFilePath({ label: 'Choose Java executable' });
      if (!disposed && chosen) selectPath(chosen);
    } catch (cause) {
      if (!disposed) error = errorMessage(cause);
    } finally {
      if (!disposed) busy = false;
    }
  }

  function installed(event: { major: number; runtimePath?: string }): void {
    if (!event.runtimePath)
      throw new Error(
        'Java installed, but no executable path was returned. Use Detect to select it.',
      );
    if (event.major < requiredMajor)
      throw new Error(`This server needs Java ${requiredMajor} or newer.`);
    selectPath(event.runtimePath);
  }

  onDestroy(() => {
    disposed = true;
    tourJavaSelectionPending.set(false);
  });
</script>

<section class="java-setup" use:onboardingAnchor={'ob_server_java'}>
  <p class="msc-type-overline">Java Executable</p>
  <Card padding="0">
    <div class="path-row">
      <span class="name">Executable Path</span>
      <div class="path-control">
        <Field
          bind:value={pathDraft}
          placeholder="Choose or enter a Java executable"
          disabled={busy || loading}
        />
      </div>
    </div>
  </Card>
  <div class="actions">
    <Button
      variant="secondary"
      size="sm"
      disabled={!api || busy || loading || !version}
      onclick={detect}>{busy ? 'Working…' : 'Detect'}</Button
    >
    <Button variant="secondary" size="sm" disabled={busy || loading || !version} onclick={browse}
      >Browse…</Button
    >
    <Button
      variant="secondary"
      size="sm"
      disabled={!api || busy || loading || !version}
      onclick={() => (showInstall = true)}>Install Java…</Button
    >
    {#if error && !loading}<Button
        variant="secondary"
        size="sm"
        onclick={() => void loadRequirement(contextKey, api)}>Retry version check</Button
      >{/if}
  </div>
  <p class="hint">
    {loading
      ? 'Checking the required Java version…'
      : version
        ? `Minecraft ${version} requires Java ${requiredMajor} or newer. Specify the executable for this server to continue.`
        : 'The required Java version must be checked before continuing.'}
  </p>
  {#if error}<p class="hint warn" role="alert">{error}</p>{/if}
</section>

{#if showDetected}
  <Sheet title="Detected Java Runtimes" size="sm" onClose={() => (showDetected = false)}>
    <p class="hint">Minecraft {version} requires Java {requiredMajor} or newer.</p>
    {#if runtimes.length === 0}<p class="hint">
        No Java runtimes found. Browse for an executable or install Java.
      </p>{/if}
    <div class="runtime-list">
      {#each runtimes as runtime (runtime.executablePath)}
        <button
          type="button"
          class="runtime-row"
          class:selected={pathDraft === runtime.executablePath}
          disabled={(runtime.majorVersion ?? 0) < requiredMajor}
          onclick={() => {
            selectPath(runtime.executablePath);
            showDetected = false;
          }}
        >
          <span class="runtime-info">
            <span class="runtime-heading">
              {#if runtime.majorVersion}<span class="runtime-version"
                  >Java {runtime.majorVersion}</span
                >{/if}
              <span class="runtime-name">{runtime.name}</span>
            </span>
            <span class="runtime-path" title={runtime.executablePath}>{runtime.executablePath}</span
            >
            {#if (runtime.majorVersion ?? 0) < requiredMajor}
              <span class="hint">Needs Java {requiredMajor} or newer</span>
            {/if}
          </span>
          <span class="tag"
            >{(runtime.majorVersion ?? 0) < requiredMajor ? 'Unavailable' : 'Use'}</span
          >
        </button>
      {/each}
    </div>
    <div class="footer">
      <Button variant="secondary" onclick={() => (showDetected = false)}>Close</Button>
    </div>
  </Sheet>
{/if}
{#if showInstall}
  <JavaInstallSheet
    {api}
    initialMajor={requiredMajor}
    selectionScope="this server"
    onClose={() => (showInstall = false)}
    onInstalled={installed}
  />
{/if}

<style>
  .java-setup {
    display: flex;
    flex-direction: column;
    gap: 8px;
  }
  .path-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    padding: 11px 14px;
  }
  .name {
    font-size: 13px;
    font-weight: 500;
    color: var(--msc-text-primary);
  }
  .path-control {
    width: min(100%, 360px);
    min-width: 0;
  }
  .actions {
    display: flex;
    flex-wrap: wrap;
    gap: 8px;
  }
  .hint {
    margin: 0;
    font-size: 11.5px;
    color: var(--msc-text-tertiary);
    line-height: 1.5;
  }
  .warn {
    color: var(--msc-status-warn);
  }
  .runtime-list {
    display: flex;
    flex-direction: column;
    gap: 6px;
    margin-top: 12px;
    max-height: 280px;
    overflow-y: auto;
  }
  .runtime-row {
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: 12px;
    width: 100%;
    padding: 11px 14px;
    background: var(--msc-tier-chrome);
    border: 1px solid transparent;
    border-radius: 8px;
    color: var(--msc-text-primary);
    font: inherit;
    text-align: left;
    cursor: pointer;
  }
  .runtime-row:hover,
  .runtime-row.selected {
    border-color: rgba(255, 255, 255, 0.28);
  }
  .runtime-row:disabled {
    opacity: 0.55;
    cursor: not-allowed;
  }
  .runtime-info {
    display: flex;
    flex-direction: column;
    gap: 3px;
    min-width: 0;
  }
  .runtime-heading {
    display: flex;
    align-items: center;
    gap: 8px;
  }
  .runtime-version {
    font-size: 13px;
    font-weight: 500;
  }
  .runtime-name,
  .runtime-path {
    overflow: hidden;
    color: var(--msc-text-tertiary);
    text-overflow: ellipsis;
    white-space: nowrap;
  }
  .runtime-name {
    font-size: 11px;
  }
  .runtime-path {
    font-family: var(--msc-font-mono, monospace);
    font-size: 10.5px;
  }
  .tag {
    flex-shrink: 0;
    color: var(--msc-text-tertiary);
    font-size: 10px;
  }
  .footer {
    display: flex;
    justify-content: flex-end;
    gap: 8px;
    margin-top: 14px;
  }
  @media (max-width: 620px) {
    .path-row {
      flex-direction: column;
      align-items: stretch;
    }
    .path-control {
      width: 100%;
    }
  }
</style>
