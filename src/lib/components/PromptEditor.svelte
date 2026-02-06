<script lang="ts">
  import type { ProjectState } from '$lib/types';
  import { _ } from 'svelte-i18n';

  interface Props {
    project: ProjectState;
    onSave?: (prompt: string) => void | Promise<void>;
    onCancel?: () => void;
  }

  let { project, onSave, onCancel }: Props = $props();

  let editedPrompt = $state('');
  let isEditing = $state(false);
  let saving = $state(false);
  let saveError = $state<string | null>(null);

  $effect(() => {
    if (!isEditing) {
      editedPrompt = project.task?.prompt || '';
    }
  });

  function handleEdit() {
    saveError = null;
    isEditing = true;
  }

  async function handleSave() {
    saving = true;
    saveError = null;
    try {
      await onSave?.(editedPrompt);
      isEditing = false;
    } catch (error) {
      saveError = error instanceof Error ? error.message : String(error);
    } finally {
      saving = false;
    }
  }

  function handleCancel() {
    editedPrompt = project.task?.prompt || '';
    isEditing = false;
    saveError = null;
    onCancel?.();
  }

  function copyToClipboard() {
    navigator.clipboard.writeText(editedPrompt);
  }
</script>

<div class="bg-vscode-panel rounded-lg border border-vscode">
  <!-- Header -->
  <div class="flex items-center justify-between p-3 border-b border-vscode">
    <h3 class="font-medium text-vscode flex items-center gap-2">
      <span>📝</span>
      <span>{$_('prompt.title')}</span>
    </h3>
    <div class="flex gap-2">
      {#if !isEditing}
        <button
          class="px-3 py-1 text-sm bg-vscode-input hover:bg-vscode-hover rounded text-vscode-dim border border-vscode"
          onclick={copyToClipboard}
        >
          {$_('prompt.copy')}
        </button>
        <button
          class="px-3 py-1 text-sm bg-vscode-accent bg-vscode-accent-hover rounded text-white"
          onclick={handleEdit}
        >
          {$_('prompt.edit')}
        </button>
      {:else}
        <button
          class="px-3 py-1 text-sm bg-vscode-input hover:bg-vscode-hover rounded text-vscode-dim border border-vscode"
          onclick={handleCancel}
        >
          {$_('prompt.cancel')}
        </button>
        <button
          class="px-3 py-1 text-sm bg-vscode-accent bg-vscode-accent-hover rounded text-white"
          onclick={handleSave}
          disabled={saving}
        >
          {$_('prompt.save')}
        </button>
      {/if}
    </div>
  </div>

  <!-- Content -->
  <div class="p-3">
    {#if isEditing}
      <textarea
        class="w-full h-64 p-3 font-mono text-sm bg-vscode-editor border border-vscode rounded-lg resize-none focus-vscode text-vscode"
        bind:value={editedPrompt}
        placeholder={$_('prompt.placeholder')}
      ></textarea>
      {#if saveError}
        <div class="mt-2 text-xs text-vscode-error">
          {$_('task.errorPrefix')} {saveError}
        </div>
      {/if}
    {:else}
      <div class="max-h-64 overflow-y-auto">
        <pre class="whitespace-pre-wrap font-mono text-sm text-vscode bg-vscode-editor p-3 rounded-lg border border-vscode">{editedPrompt || $_('prompt.empty')}</pre>
      </div>
    {/if}
  </div>

  <!-- Stats -->
  <div class="px-3 pb-3">
    <div class="text-xs text-vscode-muted flex gap-4">
      <span>{$_('prompt.chars', { values: { count: editedPrompt.length } })}</span>
      <span>{$_('prompt.lines', { values: { count: editedPrompt.split('\n').length } })}</span>
    </div>
  </div>
</div>
