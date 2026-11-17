import { describe, expect, it } from 'vitest';
import {
  canAdvanceConfigure,
  defaultWizardDraft,
  javaSelectionKey,
} from '../../src/lib/sections/fleet/wizard/model';

describe('creation Configure Java requirement', () => {
  it('requires an explicit path for the resolved server setup before continuing', () => {
    const draft = defaultWizardDraft();
    draft.serverName = 'Survival';
    expect(canAdvanceConfigure(draft)).toBe(false);

    draft.javaPath = '   ';
    draft.javaSelectionKey = javaSelectionKey(draft);
    expect(canAdvanceConfigure(draft)).toBe(false);

    draft.javaPath = '/opt/java/bin/java';
    draft.javaRequirementPending = true;
    expect(canAdvanceConfigure(draft)).toBe(false);
    draft.javaRequirementPending = false;
    expect(canAdvanceConfigure(draft)).toBe(true);

    draft.versionId = '1.21.1';
    expect(canAdvanceConfigure(draft)).toBe(false);
    draft.javaSelectionKey = javaSelectionKey(draft);
    expect(canAdvanceConfigure(draft)).toBe(true);
  });

  it('does not require a Java executable for Bedrock creation', () => {
    const draft = defaultWizardDraft();
    draft.serverName = 'Bedrock';
    draft.serverType = 'bedrock';
    expect(canAdvanceConfigure(draft)).toBe(true);
  });
});
