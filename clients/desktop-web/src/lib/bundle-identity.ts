export const bundleIdentity = Object.freeze({
  id: 'msc-shared-client',
  version: '0.1.50',
});

export function bundleLabel(): string {
  return `${bundleIdentity.id} v${bundleIdentity.version}`;
}
