/** Preserve preferences when moving to the current product namespace.
 * Keep the old keys until an explicit reset so an older installation can still
 * read them; never overwrite a value already saved by this version.
 */
export function migrateProductPreferences(): void {
  try {
    if (typeof localStorage === 'undefined') return;
    const keys = Array.from({ length: localStorage.length }, (_, index) => localStorage.key(index));
    for (const key of keys) {
      if (!key?.startsWith('msc2.')) continue;
      const nextKey = `msc.${key.slice('msc2.'.length)}`;
      if (localStorage.getItem(nextKey) !== null) continue;
      const value = localStorage.getItem(key);
      if (value !== null) localStorage.setItem(nextKey, value);
    }
  } catch {
    // Restricted/full browser storage must not prevent opening the application.
  }
}

migrateProductPreferences();
