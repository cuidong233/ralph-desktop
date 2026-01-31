<script lang="ts">
  import type { ProjectMeta, ProjectStatus } from '$lib/types';
  import { _ } from 'svelte-i18n';

  interface Props {
    projects: ProjectMeta[];
    selectedId: string | null;
    onSelect: (id: string) => void;
    onDelete: (id: string) => void;
  }

  let { projects, selectedId, onSelect, onDelete }: Props = $props();

  // Simple colored dot for status
  const statusColors: Record<ProjectStatus, string> = {
    brainstorming: '#a855f7',  // purple
    ready: '#6e6e6e',          // gray
    queued: '#3794ff',         // blue
    running: '#4ec9b0',        // green
    pausing: '#cca700',        // yellow
    paused: '#cca700',         // yellow
    done: '#4ec9b0',           // green
    partial: '#3794ff',        // blue
    failed: '#f14c4c',         // red
    cancelled: '#6e6e6e'       // gray
  };

  const animatedStatuses: ProjectStatus[] = ['running', 'pausing'];

  function getStatusColor(status: ProjectStatus) {
    return statusColors[status] || statusColors.ready;
  }

  function shouldAnimate(status: ProjectStatus) {
    return animatedStatuses.includes(status);
  }
</script>

<div class="">
  {#if projects.length === 0}
    <div class="px-4 py-6 text-center text-vscode-muted text-sm">
      {$_('app.noProjects')}
    </div>
  {:else}
    {#each projects as project (project.id)}
      <div
        class="w-full px-4 py-2 text-left hover:bg-vscode-hover transition-colors group cursor-pointer flex items-center gap-3
          {selectedId === project.id ? 'bg-vscode-active' : ''}"
        onclick={() => onSelect(project.id)}
        onkeydown={(e) => e.key === 'Enter' && onSelect(project.id)}
        role="button"
        tabindex="0"
      >
        <!-- Simple status dot -->
        <div
          class="w-2 h-2 rounded-full flex-shrink-0 {shouldAnimate(project.status) ? 'animate-pulse' : ''}"
          style="background-color: {getStatusColor(project.status)}"
        ></div>
        <div class="flex-1 min-w-0">
          <div class="text-sm text-vscode truncate">
            {project.name}
          </div>
          <div class="text-xs text-vscode-muted truncate">
            {project.path}
          </div>
        </div>
        <button
          class="opacity-0 group-hover:opacity-100 p-1 hover:bg-[#f14c4c20] rounded text-[#f14c4c]"
          onclick={(e) => { e.stopPropagation(); onDelete(project.id); }}
          title={$_('common.delete')}
        >
          <svg class="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24" stroke-width="2">
            <path stroke-linecap="round" stroke-linejoin="round" d="M6 18L18 6M6 6l12 12"/>
          </svg>
        </button>
      </div>
    {/each}
  {/if}
</div>
