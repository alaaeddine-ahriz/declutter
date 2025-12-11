<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { FileInfo, Action } from "../types";
  import Preview from "./Preview.svelte";
  import Button from "./ui/Button.svelte";
  import Card from "./ui/Card.svelte";
  import Badge from "./ui/Badge.svelte";

  export let files: FileInfo[];
  export let currentIndex: number;
  export let actionHistory: Action[];

  const dispatch = createEventDispatcher<{
    complete: { filesToDelete: FileInfo[]; keptCount: number };
  }>();

  let keptCount = 0;
  let deletedCount = 0;

  async function revealInExplorer() {
    if (!currentFile) return;
    try {
      await invoke("reveal_in_explorer", { path: currentFile.path });
    } catch (e) {
      console.error("Failed to reveal in explorer:", e);
    }
  }

  $: currentFile = files[currentIndex] || null;
  $: progress = `${currentIndex + 1} of ${files.length}`;
  $: progressPercent = (currentIndex / files.length) * 100;

  function keepFile() {
    if (!currentFile) return;

    actionHistory = [
      ...actionHistory,
      {
        type: "keep",
        fileIndex: currentIndex,
        originalPath: currentFile.path,
      },
    ];

    keptCount++;
    advance();
  }

  function deleteFile() {
    if (!currentFile) return;

    actionHistory = [
      ...actionHistory,
      {
        type: "delete",
        fileIndex: currentIndex,
        originalPath: currentFile.path,
      },
    ];

    deletedCount++;
    advance();
  }

  function undo() {
    if (actionHistory.length === 0) return;

    const lastAction = actionHistory[actionHistory.length - 1];
    actionHistory = actionHistory.slice(0, -1);

    if (lastAction.type === "delete") {
      deletedCount--;
    } else {
      keptCount--;
    }

    // Go back to the previous file
    currentIndex = lastAction.fileIndex;
  }

  function advance() {
    if (currentIndex < files.length - 1) {
      currentIndex++;
    } else {
      // Collect all files marked for deletion
      const filesToDelete = actionHistory
        .filter((action) => action.type === "delete")
        .map((action) => files[action.fileIndex]);

      dispatch("complete", {
        filesToDelete,
        keptCount,
      });
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "ArrowRight") {
      event.preventDefault();
      keepFile();
    } else if (event.key === "ArrowLeft") {
      event.preventDefault();
      deleteFile();
    } else if (event.key === "Backspace") {
      event.preventDefault();
      undo();
    }
  }

  onMount(() => {
    window.addEventListener("keydown", handleKeydown);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeydown);
  });

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }
</script>

<div class="file-viewer">
  <div class="main-content">
    {#if currentFile}
      <div class="header">
        <span class="progress-text">{progress}</span>
        <div class="stats-inline">
          <Badge variant="success">{keptCount} kept</Badge>
          <Badge variant="danger">{deletedCount} deleted</Badge>
        </div>
      </div>

      <Card padding="none">
        <div class="file-info-header">
          <span class="file-name">{currentFile.name}</span>
          <div class="file-meta">
            <Button
              variant="ghost"
              size="sm"
              on:click={revealInExplorer}
              title="Show in Finder"
            >
              <svg
                width="14"
                height="14"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="2"
              >
                <path
                  d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6"
                />
                <polyline points="15 3 21 3 21 9" />
                <line x1="10" y1="14" x2="21" y2="3" />
              </svg>
            </Button>
            <Badge variant="neutral">{formatSize(currentFile.size)}</Badge>
          </div>
        </div>

        <div class="preview-container">
          <Preview file={currentFile} />
        </div>
      </Card>

      <div class="actions">
        <Button variant="danger" on:click={deleteFile}>
          <kbd style="margin-right: 0.5rem">←</kbd> Delete
        </Button>

        {#if actionHistory.length > 0}
          <Button variant="ghost" on:click={undo}>Undo</Button>
        {/if}

        <Button variant="success" on:click={keepFile}>
          Keep <kbd style="margin-left: 0.5rem">→</kbd>
        </Button>
      </div>
    {/if}
  </div>
</div>

<style>
  .file-viewer {
    flex: 1;
    display: flex;
    flex-direction: column;
  }

  .main-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    justify-content: center;
    padding: 1rem 1.5rem;
    max-width: 600px;
    margin: 0 auto;
    width: 100%;
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
  }

  .progress-text {
    font-size: 0.75rem;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .stats-inline {
    display: flex;
    gap: 0.5rem;
  }

  .file-info-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--border-color);
    background: var(--bg-primary);
  }

  .file-name {
    font-size: 0.875rem;
    font-weight: 500;
    color: var(--text-primary);
    word-break: break-all;
    flex: 1;
    margin-right: 1rem;
  }

  .file-meta {
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  .preview-container {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    min-height: 180px;
    max-height: 280px;
    overflow: hidden;
  }

  .actions {
    display: flex;
    align-items: center;
    justify-content: space-between; /* Spread them out or keep centered? minimalistic usually groups them. Let's keep centered but with more gap */
    justify-content: center;
    margin-top: 1.5rem;
    gap: 1rem;
  }
</style>
