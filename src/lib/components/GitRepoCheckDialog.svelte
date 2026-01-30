<script lang="ts">
  import type { GitRepoCheckReason } from '$lib/stores/gitRepoCheck';
  import { _ } from 'svelte-i18n';

  interface Props {
    projectName: string;
    projectPath: string;
    reason: GitRepoCheckReason;
    busy: boolean;
    onInit: () => void;
    onSkip: () => void;
    onCancel: () => void;
  }

  let { projectName, projectPath, reason, busy, onInit, onSkip, onCancel }: Props = $props();
</script>

<div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
  <div class="bg-vscode-panel border border-vscode rounded-lg shadow-xl max-w-lg w-full m-4">
    <div class="p-4 border-b border-vscode flex items-start gap-3">
      <div class="text-vscode-warning text-xl">⚠️</div>
      <div>
        <h2 class="text-base font-semibold text-vscode">
          {$_('gitRepo.title')}
        </h2>
        <p class="text-sm text-vscode-dim mt-1">
          {#if reason === 'runtime'}
            {$_('gitRepo.runtimeReason')}
          {:else}
            {$_('gitRepo.preflightReason')}
          {/if}
        </p>
      </div>
    </div>

    <div class="p-4 space-y-3">
      <div class="text-sm text-vscode">
        <div class="font-medium">{projectName}</div>
        <div class="text-vscode-muted break-all">{projectPath}</div>
      </div>
      <div class="text-xs text-vscode-muted">
        {$_('gitRepo.projectNote')}
      </div>
    </div>

    <div class="p-4 border-t border-vscode flex justify-end gap-2">
      <button
        class="px-3 py-1.5 text-sm text-vscode-dim hover:text-vscode"
        onclick={onCancel}
        disabled={busy}
      >
        {$_('gitRepo.cancel')}
      </button>
      <button
        class="px-3 py-1.5 text-sm bg-vscode-input border border-vscode rounded text-vscode hover:bg-vscode-hover disabled:opacity-50"
        onclick={onSkip}
        disabled={busy}
      >
        {$_('gitRepo.skip')}
      </button>
      <button
        class="px-3 py-1.5 text-sm bg-vscode-accent text-white rounded hover:bg-vscode-accent-hover disabled:opacity-50"
        onclick={onInit}
        disabled={busy}
      >
        {$_('gitRepo.init')}
      </button>
    </div>
  </div>
</div>
