<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import type { AppState, FileInfo, Action, TriageResult } from "./types";
  import FolderSelect from "./lib/FolderSelect.svelte";
  import FileViewer from "./lib/FileViewer.svelte";
  import DeleteConfirm from "./lib/DeleteConfirm.svelte";
  import Summary from "./lib/Summary.svelte";
  import Settings from "./lib/Settings.svelte";
  import PreviewPanel from "./lib/PreviewPanel.svelte";
  import Button from "./lib/ui/Button.svelte";
  import Kbd from "./lib/ui/Kbd.svelte";

  let appState: AppState = "idle";
  let folderPath: string = "";
  let files: FileInfo[] = [];
  let currentIndex: number = 0;
  let actionHistory: Action[] = [];
  let result: TriageResult = { kept: 0, deleted: 0, total: 0 };
  let filesToDelete: FileInfo[] = [];

  let keptCount: number = 0;

  // Track previous state to return to from settings
  let previousState: AppState = "idle";

  // Preview panel state
  let previewOpen: boolean = false;
  let previewFile: FileInfo | null = null;

  function handleFolderSelected(
    event: CustomEvent<{ path: string; files: FileInfo[] }>,
  ) {
    folderPath = event.detail.path;
    files = event.detail.files;
    currentIndex = 0;
    actionHistory = [];
    result = { kept: 0, deleted: 0, total: files.length };
    filesToDelete = [];
    keptCount = 0;

    if (files.length > 0) {
      appState = "triaging";
    }
  }

  function handleTriageComplete(
    event: CustomEvent<{ filesToDelete: FileInfo[]; keptCount: number }>,
  ) {
    filesToDelete = event.detail.filesToDelete;
    keptCount = event.detail.keptCount;
    closePreview();
    appState = "confirming";
  }

  function handleDeleteCancel() {
    appState = "triaging";
  }

  function handleDeleteComplete(event: CustomEvent<TriageResult>) {
    result = event.detail;
    appState = "complete";
  }

  function handleStartOver() {
    appState = "idle";
    folderPath = "";
    files = [];
    currentIndex = 0;
    actionHistory = [];
    result = { kept: 0, deleted: 0, total: 0, savedSize: 0 };
    filesToDelete = [];
    keptCount = 0;
    closePreview();
  }

  function handleOpenPreview(event: CustomEvent<{ file: FileInfo }>) {
    previewFile = event.detail.file;
    previewOpen = true;
  }

  function closePreview() {
    previewOpen = false;
    previewOpen = false;
    previewFile = null;
  }

  function openSettings() {
    previousState = appState;
    appState = "settings";
  }

  function closeSettings() {
    appState = previousState;
  }

  function truncatePath(path: string, maxLength: number = 50): string {
    if (path.length <= maxLength) return path;
    const parts = path.split("/");
    if (parts.length <= 3) return path;
    return `.../${parts.slice(-3).join("/")}`;
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Escape") {
      event.preventDefault();
      if (previewOpen) {
        closePreview();
      } else if (appState !== "idle" && appState !== "settings") {
        handleStartOver();
      } else if (appState === "settings") {
        closeSettings();
      }
    } else if (
      event.key === "s" &&
      appState === "idle" &&
      !event.ctrlKey &&
      !event.metaKey &&
      !event.altKey
    ) {
      event.preventDefault();
      openSettings();
    }
  }

  // Update preview file when current file changes
  $: if (previewOpen && files[currentIndex]) {
    previewFile = files[currentIndex];
  }

  onMount(() => {
    window.addEventListener("keydown", handleKeydown);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeydown);
  });
</script>

<main class="app" class:preview-open={previewOpen}>
  {#if appState !== "idle"}
    <header class="header">
      <div class="back-btn">
        <Button variant="ghost" size="sm" on:click={handleStartOver}>
          <svg
            width="16"
            height="16"
            viewBox="0 0 24 24"
            fill="none"
            stroke="currentColor"
            stroke-width="2"
            stroke-linecap="round"
            stroke-linejoin="round"
          >
            <path d="m15 18-6-6 6-6" />
          </svg>
          Back
          <Kbd>Esc</Kbd>
        </Button>
      </div>
      {#if appState === "triaging"}
        <span class="path">{truncatePath(folderPath)}</span>
      {/if}
    </header>
  {/if}

  <div class="content">
    {#if appState === "idle"}
      <div class="idle-container">
        <FolderSelect on:folderSelected={handleFolderSelected} />
        <div class="settings-link">
          <Button variant="ghost" size="sm" on:click={openSettings}
            >Settings <Kbd>S</Kbd></Button
          >
        </div>
      </div>
    {:else if appState === "settings"}
      <Settings on:back={closeSettings} />
    {:else if appState === "triaging"}
      <FileViewer
        {files}
        bind:currentIndex
        bind:actionHistory
        {previewOpen}
        on:complete={handleTriageComplete}
        on:openPreview={handleOpenPreview}
      />
    {:else if appState === "confirming"}
      <DeleteConfirm
        bind:filesToDelete
        {keptCount}
        totalCount={files.length}
        on:cancel={handleDeleteCancel}
        on:complete={handleDeleteComplete}
      />
    {:else if appState === "complete"}
      <Summary {result} on:startOver={handleStartOver} />
    {/if}
  </div>

  <PreviewPanel
    file={previewFile}
    isOpen={previewOpen}
    on:close={closePreview}
  />
</main>

<style>
  .app {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-primary);
    color: var(--text-primary);
    transition: margin-right var(--transition-base);
  }

  .app.preview-open {
    margin-right: 400px;
  }

  .header {
    padding: 0 16px;
    display: flex;
    align-items: center;
    justify-content: center;
    height: 48px;
    position: relative;
    width: 100%;
    border-bottom: 1px solid var(--border-subtle);
  }

  .back-btn {
    position: absolute;
    left: 12px;
    top: 50%;
    transform: translateY(-50%);
  }

  .path {
    font-size: 12px;
    color: var(--text-muted);
    font-family: var(--font-mono);
    max-width: 400px;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .idle-container {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 16px;
    width: 100%;
  }

  .settings-link {
    position: absolute;
    bottom: 24px;
    right: 24px;
    opacity: 0.6;
    transition: opacity 0.2s;
  }

  .settings-link:hover {
    opacity: 1;
  }
</style>
