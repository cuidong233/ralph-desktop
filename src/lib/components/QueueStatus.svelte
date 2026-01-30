<script lang="ts">
  import { queueStatus, projectQueue, runningProjects } from '$lib/stores/queue';
  import { _ } from 'svelte-i18n';

  const statusText = $derived(() => {
    const status = $queueStatus;
    if (status.runningCount === 0 && status.queueLength === 0) {
      return $_('queue.idle');
    }
    const parts: string[] = [];
    if (status.runningCount > 0) {
      parts.push($_('queue.runningCount', { values: { count: status.runningCount } }));
    }
    if (status.queueLength > 0) {
      parts.push($_('queue.queueCount', { values: { count: status.queueLength } }));
    }
    return parts.join(', ');
  });

  const statusColor = $derived(() => {
    const status = $queueStatus;
    if (status.runningCount > 0) return 'text-vscode-success';
    if (status.queueLength > 0) return 'text-vscode-info';
    return 'text-vscode-muted';
  });
</script>

<div class="flex items-center gap-2 px-3 py-2 bg-vscode-sidebar border-t border-vscode">
  <div class="flex items-center gap-1.5">
    <div class="w-2 h-2 rounded-full {$queueStatus.runningCount > 0 ? 'bg-vscode-success animate-pulse' : 'bg-vscode-border'}"></div>
    <span class="text-xs {statusColor()}">{statusText()}</span>
  </div>
  {#if $queueStatus.availableSlots > 0}
    <span class="text-xs text-vscode-muted ml-auto">
      {$_('queue.availableSlots', { values: { count: $queueStatus.availableSlots } })}
    </span>
  {/if}
</div>
