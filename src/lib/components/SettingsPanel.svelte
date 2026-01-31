<script lang="ts">
  import { config, availableClis, updateConfig } from '$lib/stores/settings';
  import { setTheme } from '$lib/stores/theme';
  import * as api from '$lib/services/tauri';
  import type { GlobalConfig, CliType, Theme } from '$lib/types';
  import { _, locale } from 'svelte-i18n';
  import { setLocaleFromConfig } from '$lib/i18n';

  interface Props {
    onClose: () => void;
  }

  let { onClose }: Props = $props();

  let localConfig = $state<GlobalConfig>({ ...$config });
  let saving = $state(false);

  async function handleSave() {
    saving = true;
    try {
      await api.saveConfig(localConfig);
      updateConfig(localConfig);
      setTheme(localConfig.theme);
      setLocaleFromConfig(localConfig.language);
      onClose();
    } catch (error) {
      console.error('Failed to save config:', error);
    } finally {
      saving = false;
    }
  }

  async function handleResetPermissions() {
    if (confirm($_('settings.resetConfirm'))) {
      localConfig.permissionsConfirmed = false;
      localConfig.permissionsConfirmedAt = undefined;
    }
  }

  const fieldClass = 'w-full h-10 px-3 rounded-lg border border-vscode bg-vscode-input text-vscode focus-vscode';

  const languageOptions = $derived([
    { value: 'system', label: $_('settings.languageSystem') },
    { value: 'en', label: 'English' },
    { value: 'zh-CN', label: '简体中文' },
    { value: 'zh-TW', label: '繁體中文' },
    { value: 'es', label: 'Español' },
    { value: 'hi', label: 'हिन्दी' },
    { value: 'ar', label: 'العربية' },
    { value: 'pt', label: 'Português' },
    { value: 'ru', label: 'Русский' },
    { value: 'ja', label: '日本語' },
    { value: 'de', label: 'Deutsch' },
    { value: 'fr', label: 'Français' },
    { value: 'bn', label: 'বাংলা' }
  ]);

  const languageMap = $derived.by(() => {
    const map = new Map<string, string>();
    for (const option of languageOptions) {
      map.set(option.value, option.label);
    }
    return map;
  });

  function normalizeLocale(input: string) {
    const value = input.trim();
    if (value.toLowerCase().startsWith('zh')) {
      if (value.toLowerCase().includes('tw') || value.toLowerCase().includes('hk')) {
        return 'zh-TW';
      }
      return 'zh-CN';
    }
    return value.split('-')[0];
  }

  const systemLanguageLabel = $derived.by(() => {
    if (typeof navigator === 'undefined') {
      return languageMap.get('en') || 'English';
    }
    const raw = navigator.language || 'en';
    const normalized = normalizeLocale(raw);
    return languageMap.get(raw) || languageMap.get(normalized) || raw;
  });
</script>

<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
  <div class="bg-vscode-panel border border-vscode rounded-lg shadow-xl max-w-lg w-full m-4 max-h-[90vh] overflow-hidden flex flex-col">
    <!-- Header -->
    <div class="p-4 border-b border-vscode flex items-center justify-between">
      <h2 class="text-lg font-semibold text-vscode">{$_('settings.title')}</h2>
      <button
        class="text-vscode-dim hover:text-vscode"
        onclick={onClose}
      >
        ✕
      </button>
    </div>

    <!-- Content -->
    <div class="flex-1 overflow-y-auto p-4 space-y-6">
      <!-- CLI Settings -->
      <section class="space-y-3 pb-4 border-b border-vscode">
        <h3 class="text-sm font-medium text-vscode mb-3">{$_('settings.cliSection')}</h3>
        <div class="space-y-3">
          <div>
            <label class="block text-sm text-vscode-muted mb-1">{$_('settings.defaultCli')}</label>
            <select
              class={fieldClass}
              bind:value={localConfig.defaultCli}
            >
              {#each $availableClis.filter(c => c.available) as cli}
                <option value={cli.cliType}>{cli.name}</option>
              {/each}
            </select>
          </div>
        </div>
      </section>

      <!-- Loop Settings -->
      <section class="space-y-3 pb-4 border-b border-vscode">
        <h3 class="text-sm font-medium text-vscode mb-3">{$_('settings.loopSection')}</h3>
        <div class="space-y-3">
          <div>
            <label class="block text-sm text-vscode-muted mb-1">{$_('settings.defaultMaxIterations')}</label>
            <input
              type="number"
              class={fieldClass}
              bind:value={localConfig.defaultMaxIterations}
              min="1"
              max="500"
            />
          </div>
          <div>
            <label class="block text-sm text-vscode-muted mb-1">{$_('settings.maxConcurrent')}</label>
            <input
              type="number"
              class={fieldClass}
              bind:value={localConfig.maxConcurrentProjects}
              min="1"
              max="10"
            />
          </div>
          <div>
            <label class="block text-sm text-vscode-muted mb-1">{$_('settings.iterationTimeout')}</label>
            <input
              type="number"
              class={fieldClass}
              value={localConfig.iterationTimeoutMs / 60000}
              oninput={(e) => {
                const minutes = Number.parseInt(e.currentTarget.value, 10);
                localConfig.iterationTimeoutMs = Number.isFinite(minutes) && minutes > 0 ? minutes * 60000 : 0;
              }}
              min="0"
            />
          </div>
          <div>
            <label class="block text-sm text-vscode-muted mb-1">{$_('settings.idleTimeout')}</label>
            <input
              type="number"
              class={fieldClass}
              value={localConfig.idleTimeoutMs / 60000}
              oninput={(e) => {
                const minutes = Number.parseInt(e.currentTarget.value, 10);
                localConfig.idleTimeoutMs = Number.isFinite(minutes) && minutes > 0 ? minutes * 60000 : 0;
              }}
              min="0"
            />
          </div>
        </div>
      </section>

      <!-- Appearance -->
      <section class="space-y-3 pb-4 border-b border-vscode">
        <h3 class="text-sm font-medium text-vscode mb-3">{$_('settings.appearance')}</h3>
        <div class="space-y-3">
          <div>
            <label class="block text-sm text-vscode-muted mb-1">{$_('settings.theme')}</label>
            <select
              class={fieldClass}
              bind:value={localConfig.theme}
            >
              <option value="system">{$_('settings.themeSystem')}</option>
              <option value="light">{$_('settings.themeLight')}</option>
              <option value="dark">{$_('settings.themeDark')}</option>
            </select>
          </div>
          <div>
            <label class="block text-sm text-vscode-muted mb-1">{$_('settings.language')}</label>
            <select
              class={fieldClass}
              bind:value={localConfig.language}
            >
              {#each languageOptions as option}
                <option value={option.value}>{option.label}</option>
              {/each}
            </select>
            {#if localConfig.language === 'system'}
              <div class="text-xs text-vscode-muted mt-1">
                {$_('settings.languageSystemHint', { values: { language: systemLanguageLabel } })}
              </div>
            {/if}
          </div>
        </div>
      </section>

      <!-- Storage -->
      <section class="space-y-3 pb-4 border-b border-vscode">
        <h3 class="text-sm font-medium text-vscode mb-3">{$_('settings.storage')}</h3>
        <div class="space-y-3">
          <div>
            <label class="block text-sm text-vscode-muted mb-1">{$_('settings.logRetention')}</label>
            <input
              type="number"
              class={fieldClass}
              bind:value={localConfig.logRetentionDays}
              min="1"
              max="90"
            />
          </div>
        </div>
      </section>

      <!-- Security -->
      <section class="space-y-3 pb-4 border-b border-vscode">
        <h3 class="text-sm font-medium text-vscode mb-3">{$_('settings.security')}</h3>
        <div class="space-y-3">
          <div class="flex items-center justify-between">
            <div>
              <div class="text-sm text-vscode-muted">{$_('settings.permissionsStatus')}</div>
              <div class="text-xs text-vscode-muted">
                {#if localConfig.permissionsConfirmed}
                  {$_('settings.permissionsConfirmed')}
                  {#if localConfig.permissionsConfirmedAt}
                    ({new Date(localConfig.permissionsConfirmedAt).toLocaleDateString($locale || undefined)})
                  {/if}
                {:else}
                  {$_('settings.permissionsNotConfirmed')}
                {/if}
              </div>
            </div>
            <button
              class="px-3 py-1.5 text-sm bg-vscode-input border border-vscode text-vscode-error rounded hover:bg-vscode-hover"
              onclick={handleResetPermissions}
            >
              {$_('settings.reset')}
            </button>
          </div>
        </div>
      </section>

      <!-- CLI Info -->
      <section class="space-y-3">
        <h3 class="text-sm font-medium text-vscode mb-3">{$_('settings.installedClis')}</h3>
        <div class="space-y-2">
          {#each $availableClis as cli}
            <div class="flex items-center justify-between px-3 py-2 bg-vscode-input border border-vscode rounded-lg">
              <div class="flex items-center gap-2">
                <span class={cli.available ? 'text-vscode-success' : 'text-vscode-muted'}>
                  {cli.available ? '✓' : '✕'}
                </span>
                <span class="text-sm text-vscode">{cli.name}</span>
              </div>
              <span class="text-xs text-vscode-muted">{cli.version || $_('cliMissing.notInstalled')}</span>
            </div>
          {/each}
        </div>
      </section>
    </div>

    <!-- Footer -->
    <div class="p-4 border-t border-vscode flex justify-end gap-3">
      <button
        class="px-4 py-2 text-vscode-dim hover:text-vscode"
        onclick={onClose}
      >
        {$_('settings.cancel')}
      </button>
      <button
        class="px-4 py-2 bg-vscode-accent bg-vscode-accent-hover text-white rounded-lg disabled:opacity-50"
        onclick={handleSave}
        disabled={saving}
      >
        {saving ? $_('settings.saving') : $_('settings.save')}
      </button>
    </div>
  </div>
</div>
