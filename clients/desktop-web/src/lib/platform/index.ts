import { isTauri } from '@tauri-apps/api/core';
import type { FetchLike } from '../api/client';
import { desktopCredentialAdapter, type TransportCredentialAdapter } from '../api/auth';
import {
  DesktopSessionAuth,
  loadTauriDesktopCredentialBridge,
  loadTauriNativeFileUploader,
  loadTauriNativeFileDownloader,
} from '../auth/desktop';
import { loadTauriPlatform } from './tauri';
import type {
  EngineReadiness,
  EngineServiceAction,
  EngineServiceStatus,
  PlatformAdapter,
  NativeFileUploader,
  NativeFileDownloader,
} from './types';

export const LOCAL_ENGINE_ORIGIN = 'http://127.0.0.1:48001';

export interface EngineTransport {
  readonly baseUrl: string;
  readonly hostId: string;
  readonly fetchImpl?: FetchLike;
  readonly nativeFileUploader?: NativeFileUploader;
  readonly nativeFileDownloader?: NativeFileDownloader;
  readonly credentialAdapter: TransportCredentialAdapter;
}

export interface EnginePreparationPlatform {
  readonly kind: PlatformAdapter['kind'];
  engineHealthCheck(): Promise<boolean>;
  engineServiceStatus(): Promise<EngineServiceStatus>;
  engineIndicatorEnabled?(): Promise<boolean>;
  setEngineIndicatorEnabled?(enabled: boolean): Promise<void>;
  manageEngineService(action: EngineServiceAction): Promise<EngineServiceStatus>;
}

export interface EngineHealthCheckOptions {
  readonly attempts?: number;
  readonly delayMs?: number;
}

export class EngineHealthTimeoutError extends Error {
  constructor() {
    super('The local engine service is running but its health endpoint did not respond.');
    this.name = 'EngineHealthTimeoutError';
  }
}

export type {
  EngineReadiness,
  EngineServiceAction,
  EngineServiceStatus,
  DesktopNotification,
  FilePickerRequest,
  MenuEntry,
  PickedFile,
  PlatformAdapter,
  UpdateCheckResult,
  UpdateInstallResult,
  TauriPlatformDependencies,
} from './types';
export { createTauriPlatform } from './tauri';

/** Reports an installed service without changing an explicit stopped state. */
export async function prepareInstalledEngine(
  platform: EnginePreparationPlatform,
  healthCheck: () => Promise<boolean> = localEngineHealthCheck,
  options: EngineHealthCheckOptions = {},
): Promise<EngineServiceStatus> {
  const status = await platform.engineServiceStatus();
  if (status.state !== 'running') return status;

  const attempts = options.attempts ?? 20;
  const delayMs = options.delayMs ?? 250;
  for (let attempt = 0; attempt < attempts; attempt += 1) {
    if (await healthCheck()) return status;
    if (attempt + 1 < attempts) await delay(delayMs);
  }
  throw new EngineHealthTimeoutError();
}

/** Reports the locally installed service without changing its stopped state. */
export async function prepareLocalEngine(): Promise<EngineServiceStatus> {
  const platform = await getPlatform();
  return prepareInstalledEngine(platform, () => platform.engineHealthCheck());
}

/** Selects authentication at the shell boundary, before ApiClient is built. */
export async function createEngineTransport(hostId: string): Promise<EngineTransport> {
  const configuredBaseUrl = import.meta.env.VITE_MSC_API_BASE_URL;
  const auth = new DesktopSessionAuth(await loadTauriDesktopCredentialBridge());
  const authenticatedHostId =
    hostId === 'local-agent' ? (await auth.bootstrapLocal()).agentHostId : hostId;
  return {
    baseUrl: configuredBaseUrl ?? LOCAL_ENGINE_ORIGIN,
    hostId: authenticatedHostId,
    fetchImpl: auth.fetchForHost(authenticatedHostId),
    nativeFileUploader: await loadTauriNativeFileUploader(),
    nativeFileDownloader: await loadTauriNativeFileDownloader(),
    credentialAdapter: desktopCredentialAdapter(),
  };
}

let platform: Promise<PlatformAdapter> | undefined;

/** Loads the native adapter; the shared frontend is shipped inside Tauri only. */
export function getPlatform(): Promise<PlatformAdapter> {
  if (!isTauri()) {
    throw new Error('The MSC desktop client must run inside its Tauri shell.');
  }
  platform ??= loadTauriPlatform();
  return platform;
}

export async function openExternal(url: string): Promise<void> {
  await (await getPlatform()).openExternal(url);
}

async function localEngineHealthCheck(): Promise<boolean> {
  try {
    const response = await fetch(`${LOCAL_ENGINE_ORIGIN}/v1/healthz`, {
      credentials: 'omit',
    });
    return response.ok;
  } catch {
    return false;
  }
}

async function delay(milliseconds: number): Promise<void> {
  await new Promise((resolve) => setTimeout(resolve, milliseconds));
}
