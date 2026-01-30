<script lang="ts">
  import { _ } from 'svelte-i18n';
  interface Props {
    show: boolean;
    onClose: () => void;
  }

  let { show, onClose }: Props = $props();

  const shortcuts = $derived([
    { keys: ['⌘', 'N'], description: $_('shortcuts.newProject') },
    { keys: ['⌘', ','], description: $_('shortcuts.openSettings') },
    { keys: ['⌘', 'Enter'], description: $_('shortcuts.startResume') },
    { keys: ['⌘', 'P'], description: $_('shortcuts.pause') },
    { keys: ['⌘', '.'], description: $_('shortcuts.stop') },
    { keys: ['Esc'], description: $_('shortcuts.closeDialog') },
    { keys: ['↑', '↓'], description: $_('shortcuts.switchProject') },
    { keys: ['⌘', '?'], description: $_('shortcuts.showHelp') },
  ]);
</script>

{#if show}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50" onclick={onClose}>
    <div
      class="bg-vscode-panel border border-vscode rounded-lg shadow-xl max-w-md w-full m-4"
      onclick={(e) => e.stopPropagation()}
    >
      <div class="p-4 border-b border-vscode flex items-center justify-between">
        <h2 class="text-lg font-semibold text-vscode">{$_('shortcuts.title')}</h2>
        <button
          class="p-1 hover:bg-vscode-hover rounded text-vscode-dim"
          onclick={onClose}
        >
          ✕
        </button>
      </div>
      <div class="p-4">
        <div class="space-y-3">
          {#each shortcuts as shortcut}
            <div class="flex items-center justify-between">
              <span class="text-vscode">{shortcut.description}</span>
              <div class="flex gap-1">
                {#each shortcut.keys as key}
                  <kbd class="px-2 py-1 text-xs font-mono bg-vscode-input border border-vscode rounded text-vscode">
                    {key}
                  </kbd>
                {/each}
              </div>
            </div>
          {/each}
        </div>
      </div>
      <div class="p-4 border-t border-vscode">
        <p class="text-xs text-vscode-muted text-center">
          {$_('shortcuts.closeHint')}
        </p>
      </div>
    </div>
  </div>
{/if}
