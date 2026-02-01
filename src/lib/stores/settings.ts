import { writable } from 'svelte/store';
import type { GlobalConfig, CliInfo } from '../types';

const defaultConfig: GlobalConfig = {
  version: '1.0.0',
  defaultCli: 'claude',
  defaultMaxIterations: 50,
  maxConcurrentProjects: 3,
  iterationTimeoutMs: 0,
  idleTimeoutMs: 0,
  theme: 'system',
  language: 'system',
  logRetentionDays: 7,
  permissionsConfirmed: false
};

export const config = writable<GlobalConfig>(defaultConfig);
export const availableClis = writable<CliInfo[]>([]);

export function updateConfig(newConfig: GlobalConfig) {
  config.set(newConfig);
}

export function setAvailableClis(clis: CliInfo[]) {
  availableClis.set(clis);
  // Also verify if current defaultCli is actually available
  config.subscribe(c => {
    if (clis.length > 0 && !clis.find(cli => cli.name.toLowerCase().includes(c.defaultCli))) {
      // If configured CLI is not available, fallback to the first available one
      // This handles the case where user might have "codex" configured but only "claude" installed
      // However, we should be careful not to overwrite user preference if they are just temporarily missing a tool
    }
  })();
}
