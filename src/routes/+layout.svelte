<script lang="ts">
  import '../app.css';
  import { onMount } from 'svelte';
  import { _ } from 'svelte-i18n';
  import '$lib/i18n';
  import { projects, currentProjectId, currentProject, updateProjects, updateCurrentProject, updateProjectStatus, selectProject } from '$lib/stores/projects';
  import { config, availableClis, updateConfig, setAvailableClis } from '$lib/stores/settings';
  import { loopState, setStatus, setIteration, addLog, setError } from '$lib/stores/loop';
  import { gitRepoCheckRequest, clearGitRepoCheck, requestGitRepoCheck } from '$lib/stores/gitRepoCheck';
  import { dequeueProject, isInQueue, markRunning } from '$lib/stores/queue';
  import { notifySuccess, notifyError, notifyWarning } from '$lib/stores/notifications';
  import { initTheme } from '$lib/stores/theme';
  import * as api from '$lib/services/tauri';
  import { CODEX_GIT_REPO_CHECK_REQUIRED, startLoopWithGuard } from '$lib/services/loopStart';
  import { setLocaleFromConfig } from '$lib/i18n';
  import type { LoopEvent } from '$lib/types';
  import type { RecoveryInfo } from '$lib/services/tauri';
  import RecoveryDialog from '$lib/components/RecoveryDialog.svelte';
  import NotificationToast from '$lib/components/NotificationToast.svelte';
  import GitRepoCheckDialog from '$lib/components/GitRepoCheckDialog.svelte';

  let { children } = $props();
  let initialized = $state(false);
  let showPermissionDialog = $state(false);
  let showRecoveryDialog = $state(false);
  let interruptedTasks = $state<RecoveryInfo[]>([]);
  let gitRepoBusy = $state(false);

  // Ensure i18n has an initial locale before first render.
  setLocaleFromConfig('system');

  onMount(async () => {
    try {
      // Load config
      const loadedConfig = await api.getConfig();
      updateConfig(loadedConfig);
      setLocaleFromConfig(loadedConfig.language);
      initTheme(loadedConfig.theme);

      // Check if permissions need confirmation
      if (!loadedConfig.permissionsConfirmed) {
        showPermissionDialog = true;
      }

      // Detect CLIs
      const clis = await api.detectInstalledClis();
      setAvailableClis(clis);

      // Load projects
      const projectList = await api.listProjects();
      updateProjects(projectList);

      // Check for interrupted tasks
      const interrupted = await api.checkInterruptedTasks();
      if (interrupted.length > 0) {
        interruptedTasks = interrupted;
        showRecoveryDialog = true;
      }

      // Listen to loop events
      await api.listenToLoopEvents(handleLoopEvent);

      // Clean up old logs on startup
      await api.cleanupLogs();

      initialized = true;
    } catch (error) {
      console.error('Initialization error:', error);
      initialized = true; // Still show UI even on error
    }
  });

  function handleLoopEvent(event: LoopEvent) {
    const statusMap: Record<string, string> = {
      iterationStart: 'running',
      pausing: 'pausing',
      paused: 'paused',
      resumed: 'running',
      completed: 'done',
      maxIterationsReached: 'failed',
      stopped: 'cancelled'
    };

    if (event.type === 'output' && event.content) {
      addLog({
        iteration: event.iteration || 0,
        content: event.content,
        isStderr: event.isStderr || false,
        timestamp: new Date()
      });
    }

    if (event.type === 'error' && event.error) {
      if (event.error.includes(CODEX_GIT_REPO_CHECK_REQUIRED)) {
        requestGitRepoCheck(event.projectId, 'runtime');
        return;
      }
      setError(event.error);
      notifyError($_('notifications.executionErrorTitle'), event.error);
      updateProjectStatus(event.projectId, 'failed');
    }

    if (event.type === 'completed') {
      notifySuccess($_('notifications.taskCompletedTitle'), $_('notifications.taskCompletedMessage'));
    }

    if (event.type === 'maxIterationsReached') {
      notifyWarning(
        $_('notifications.maxIterationsTitle'),
        $_('notifications.maxIterationsMessage', { values: { iteration: event.iteration } })
      );
    }

    if (event.iteration !== undefined) {
      setIteration(event.iteration);
    }

    const newStatus = statusMap[event.type];
    if (newStatus) {
      setStatus(newStatus as any);
      updateProjectStatus(event.projectId, newStatus as any);
    }
  }

  async function handleConfirmPermissions() {
    await api.confirmPermissions();
    const loadedConfig = await api.getConfig();
    updateConfig(loadedConfig);
    initTheme(loadedConfig.theme);
    showPermissionDialog = false;
  }

  function handleRecoverTask(projectId: string) {
    selectProject(projectId);
    interruptedTasks = interruptedTasks.filter(t => t.projectId !== projectId);
    if (interruptedTasks.length === 0) {
      showRecoveryDialog = false;
    }
  }

  function handleCancelTask(projectId: string) {
    interruptedTasks = interruptedTasks.filter(t => t.projectId !== projectId);
    if (interruptedTasks.length === 0) {
      showRecoveryDialog = false;
    }
  }

  function handleDismissRecovery() {
    showRecoveryDialog = false;
  }

  async function startLoopFromDialog(projectId: string) {
    const started = await startLoopWithGuard(projectId);
    if (started && isInQueue(projectId)) {
      dequeueProject(projectId);
      markRunning(projectId);
    }
  }

  async function handleInitGitRepo(projectId: string) {
    gitRepoBusy = true;
    try {
      await api.initProjectGitRepo(projectId);
      clearGitRepoCheck();
      await startLoopFromDialog(projectId);
    } catch (error) {
      console.error('Failed to init git repo:', error);
      notifyError($_('notifications.gitInitFailed'), String(error));
    } finally {
      gitRepoBusy = false;
    }
  }

  async function handleSkipGitRepoCheck(projectId: string) {
    gitRepoBusy = true;
    try {
      const updated = await api.setProjectSkipGitRepoCheck(projectId, true);
      if ($currentProject && $currentProject.id === updated.id) {
        updateCurrentProject(updated);
      }
      clearGitRepoCheck();
      await startLoopFromDialog(projectId);
    } catch (error) {
      console.error('Failed to skip git repo check:', error);
      notifyError($_('notifications.skipGitFailed'), String(error));
    } finally {
      gitRepoBusy = false;
    }
  }

  function handleCancelGitRepoCheck() {
    clearGitRepoCheck();
  }
</script>

{#if showPermissionDialog}
  <div class="fixed inset-0 bg-black/50 flex items-center justify-center z-50">
    <div class="bg-vscode-panel border border-vscode rounded-lg shadow-xl max-w-lg p-6 m-4">
      <h2 class="text-xl font-bold text-vscode-warning mb-4">
        ⚠️ {$_('permissions.title')}
      </h2>
      <div class="text-vscode space-y-3 mb-6">
        <p>{$_('permissions.intro')}</p>
        <ul class="list-disc list-inside space-y-1 ml-2">
          <li>{$_('permissions.bullet1')}</li>
          <li>{$_('permissions.bullet2')}</li>
          <li>{$_('permissions.bullet3')}</li>
        </ul>
        <p class="font-medium">{$_('permissions.recommendationTitle')}</p>
        <ul class="list-disc list-inside space-y-1 ml-2">
          <li>{$_('permissions.recommendation1')}</li>
          <li>{$_('permissions.recommendation2')}</li>
          <li>{$_('permissions.recommendation3')}</li>
        </ul>
      </div>
      <div class="flex justify-end gap-3">
        <button
          class="px-4 py-2 rounded-lg bg-vscode-panel border border-vscode text-vscode-dim hover:bg-vscode-hover"
          onclick={() => window.close()}
        >
          {$_('permissions.cancel')}
        </button>
        <button
          class="px-4 py-2 rounded-lg bg-vscode-accent bg-vscode-accent-hover text-white"
          onclick={handleConfirmPermissions}
        >
          {$_('permissions.confirm')}
        </button>
      </div>
    </div>
  </div>
{/if}

{#if showRecoveryDialog && interruptedTasks.length > 0}
  <RecoveryDialog
    tasks={interruptedTasks}
    onRecover={handleRecoverTask}
    onCancel={handleCancelTask}
    onDismiss={handleDismissRecovery}
  />
{/if}

{#if $gitRepoCheckRequest}
  {@const pending = $gitRepoCheckRequest}
  {@const meta = $projects.find(p => p.id === pending.projectId)}
  <GitRepoCheckDialog
    projectName={meta?.name || $currentProject?.name || $_('app.unknownProject')}
    projectPath={meta?.path || $currentProject?.path || ''}
    reason={pending.reason}
    busy={gitRepoBusy}
    onInit={() => handleInitGitRepo(pending.projectId)}
    onSkip={() => handleSkipGitRepoCheck(pending.projectId)}
    onCancel={handleCancelGitRepoCheck}
  />
{/if}

{#if initialized}
  {@render children()}
{:else}
  <div class="flex items-center justify-center h-screen bg-vscode-editor">
    <div class="text-center">
      <div class="animate-spin rounded-full h-12 w-12 border-b-2 border-vscode-accent mx-auto mb-4"></div>
      <p class="text-vscode-muted">{$_('common.loading')}</p>
    </div>
  </div>
{/if}

<NotificationToast />
