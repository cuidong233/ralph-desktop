import { writable, derived } from 'svelte/store';
import { setTheme as setAppTheme } from '@tauri-apps/api/app';
import type { Theme } from '../types';

// Current theme setting
export const themeSetting = writable<Theme>('system');

// Actual applied theme (resolved from system preference if needed)
export const appliedTheme = derived(themeSetting, ($setting) => {
  if ($setting === 'system') {
    if (typeof window !== 'undefined') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    return 'light';
  }
  return $setting;
});

function resolveTheme(theme: Theme): Exclude<Theme, 'system'> {
  if (theme === 'system') {
    if (typeof window !== 'undefined') {
      return window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light';
    }
    return 'light';
  }
  return theme;
}

async function applyWindowTheme(theme: Theme) {
  if (typeof window === 'undefined') return;
  try {
    const windowTheme = theme === 'system' ? null : theme;
    await setAppTheme(windowTheme);
  } catch (error) {
    // Ignore when running outside of Tauri or if API is unavailable.
    console.warn('Failed to apply window theme:', error);
  }
}

// Initialize theme from config
export function initTheme(theme: Theme) {
  themeSetting.set(theme);
  applyTheme(theme);
}

// Set and apply theme
export function setTheme(theme: Theme) {
  themeSetting.set(theme);
  applyTheme(theme);
}

// Apply theme to document
function applyTheme(theme: Theme) {
  if (typeof document === 'undefined') return;

  const root = document.documentElement;
  const resolved = resolveTheme(theme);

  root.dataset.theme = resolved;
  root.classList.toggle('dark', resolved === 'dark');
  void applyWindowTheme(theme);
}

// Listen for system theme changes
if (typeof window !== 'undefined') {
  window.matchMedia('(prefers-color-scheme: dark)').addEventListener('change', (e) => {
    let currentSetting: Theme = 'system';
    themeSetting.subscribe(v => currentSetting = v)();

    if (currentSetting === 'system') {
      const resolved = e.matches ? 'dark' : 'light';
      document.documentElement.dataset.theme = resolved;
      document.documentElement.classList.toggle('dark', e.matches);
    }
  });
}
