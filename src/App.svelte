<script lang="ts">
  import type { AppState, FileInfo, Action, TriageResult } from "./types";
  import FolderSelect from "./lib/FolderSelect.svelte";
  import FileViewer from "./lib/FileViewer.svelte";
  import DeleteConfirm from "./lib/DeleteConfirm.svelte";
  import Summary from "./lib/Summary.svelte";

  let appState: AppState = "idle";
  let folderPath: string = "";
  let files: FileInfo[] = [];
  let currentIndex: number = 0;
  let actionHistory: Action[] = [];
  let result: TriageResult = { kept: 0, deleted: 0, total: 0 };
  let filesToDelete: FileInfo[] = [];
  let keptCount: number = 0;

  function handleFolderSelected(event: CustomEvent<{ path: string; files: FileInfo[] }>) {
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

  function handleTriageComplete(event: CustomEvent<{ filesToDelete: FileInfo[]; keptCount: number }>) {
    filesToDelete = event.detail.filesToDelete;
    keptCount = event.detail.keptCount;
    appState = "confirming";
  }

  function handleDeleteCancel() {
    // Go back to triaging - user can undo and continue
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
    result = { kept: 0, deleted: 0, total: 0 };
    filesToDelete = [];
    keptCount = 0;
  }
</script>

<main class="app">
  <header class="header">
    <h1 class="title">Declutter</h1>
    {#if appState === "triaging"}
      <p class="subtitle">{folderPath}</p>
    {/if}
  </header>

  <div class="content">
    {#if appState === "idle"}
      <FolderSelect on:folderSelected={handleFolderSelected} />
    {:else if appState === "triaging"}
      <FileViewer
        {files}
        bind:currentIndex
        bind:actionHistory
        on:complete={handleTriageComplete}
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
</main>

<style>
  .app {
    min-height: 100vh;
    display: flex;
    flex-direction: column;
    background-color: var(--bg-primary);
  }

  .header {
    padding: 1.5rem 2rem;
    border-bottom: 1px solid var(--border-color);
    background-color: var(--bg-secondary);
  }

  .title {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
  }

  .subtitle {
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-top: 0.25rem;
    font-family: var(--font-mono);
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
  }
</style>


