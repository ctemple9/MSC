import { describe, expect, it, vi } from 'vitest';
import {
  prepareInstalledEngine,
  type EnginePreparationPlatform,
  type EngineServiceStatus,
} from '../../src/lib/platform';

const status = (state: EngineServiceStatus['state']): EngineServiceStatus => ({
  available: state !== 'unavailable',
  platform: 'macos',
  serviceName: 'com.ctemple.msc2.agent',
  state,
  detail: state,
});

describe('local engine installation boundary', () => {
  it('keeps an explicitly stopped engine stopped until the user starts it', async () => {
    const platform: EnginePreparationPlatform = {
      kind: 'tauri',
      engineHealthCheck: vi.fn(async () => true),
      engineServiceStatus: vi.fn(async () => status('stopped')),
      manageEngineService: vi.fn(async () => status('running')),
    };
    const healthCheck = vi.fn().mockResolvedValue(true);

    await expect(
      prepareInstalledEngine(platform, healthCheck, { attempts: 2, delayMs: 0 }),
    ).resolves.toMatchObject({ state: 'stopped' });
    expect(platform.manageEngineService).not.toHaveBeenCalled();
    expect(healthCheck).not.toHaveBeenCalled();
  });

  it('does not install a missing service during automatic launch', async () => {
    const platform: EnginePreparationPlatform = {
      kind: 'tauri',
      engineHealthCheck: vi.fn(async () => true),
      engineServiceStatus: vi.fn(async () => status('not-installed')),
      manageEngineService: vi.fn(async () => status('running')),
    };
    const healthCheck = vi.fn().mockResolvedValue(true);

    await expect(prepareInstalledEngine(platform, healthCheck)).resolves.toMatchObject({
      state: 'not-installed',
    });
    expect(platform.manageEngineService).not.toHaveBeenCalled();
    expect(healthCheck).not.toHaveBeenCalled();
  });
});
