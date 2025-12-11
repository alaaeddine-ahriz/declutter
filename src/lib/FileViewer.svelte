<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { FileInfo, Action } from "../types";
  import Preview from "./Preview.svelte";
  import Button from "./ui/Button.svelte";
  import Card from "./ui/Card.svelte";
  import Badge from "./ui/Badge.svelte";
  import Kbd from "./ui/Kbd.svelte";

  export let files: FileInfo[];
  export let currentIndex: number;
  export let actionHistory: Action[];
  export let previewOpen: boolean = false;

  const dispatch = createEventDispatcher<{
    complete: { filesToDelete: FileInfo[]; keptCount: number };
    openPreview: { file: FileInfo };
  }>();

  let keptCount = 0;
  let deletedCount = 0;

  async function revealInExplorer(event: CustomEvent<MouseEvent>) {
    event.detail.stopPropagation();
    if (!currentFile) return;
    try {
      await invoke("reveal_in_explorer", { path: currentFile.path });
    } catch (e) {
      console.error("Failed to reveal in explorer:", e);
    }
  }

  $: currentFile = files[currentIndex] || null;
  $: progress = `${currentIndex + 1}/${files.length}`;

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

    currentIndex = lastAction.fileIndex;
  }

  function advance() {
    if (currentIndex < files.length - 1) {
      currentIndex++;
    } else {
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
    } else if ((event.metaKey || event.ctrlKey) && event.key === "z") {
      event.preventDefault();
      undo();
    } else if (event.key === " ") {
      event.preventDefault();
      openPreview();
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

  function openPreview() {
    if (!currentFile) return;
    dispatch("openPreview", { file: currentFile });
  }
</script>

<div class="file-viewer">
  <div class="main-content">
    {#if currentFile}
      <div class="header">
        <span class="progress">{progress}</span>
        <div class="stats">
          <span class="stat">{keptCount} kept</span>
          <span class="stat-sep">·</span>
          <span class="stat">{deletedCount} deleted</span>
        </div>
      </div>

      <button class="card-button" on:click={openPreview} title="Click to open preview">
        <Card padding="none">
          <div class="file-header">
            <span class="file-name">{currentFile.name}</span>
            <div class="file-meta">
              <Button
                variant="ghost"
                size="sm"
                on:click={revealInExplorer}
                title="Reveal in Finder"
              >
                <svg
                  width="14"
                  height="14"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="2"
                >
                  <path d="M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" />
                  <polyline points="15 3 21 3 21 9" />
                  <line x1="10" y1="14" x2="21" y2="3" />
                </svg>
              </Button>
              <Badge variant="neutral">{formatSize(currentFile.size)}</Badge>
            </div>
          </div>

          <div class="preview-container">
            {#if previewOpen && currentFile.file_type === "image"}
              <div class="preview-icon">
                <svg width="48" height="48" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="1.5">
                  <rect x="3" y="3" width="18" height="18" rx="2" ry="2" />
                  <circle cx="8.5" cy="8.5" r="1.5" />
                  <polyline points="21 15 16 10 5 21" />
                </svg>
                <span>Viewing in preview</span>
              </div>
            {:else}
              <Preview file={currentFile} />
            {/if}
            {#if !previewOpen}
              <div class="preview-hint">
                <Kbd>Space</Kbd>
                <span>to preview</span>
              </div>
            {/if}
          </div>
        </Card>
      </button>

      <div class="actions">
        <Button variant="danger" on:click={deleteFile}>
          <Kbd>←</Kbd>
          Delete
        </Button>

        {#if actionHistory.length > 0}
          <Button variant="outline" on:click={undo}>
            Undo
            <Kbd>⌘Z</Kbd>
          </Button>
        {/if}

        <Button variant="primary" on:click={keepFile}>
          Keep
          <Kbd>→</Kbd>
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
    padding: 16px 24px;
    max-width: 560px;
    margin: 0 auto;
    width: 100%;
  }

  .card-button {
    width: 100%;
    text-align: left;
    cursor: pointer;
    border-radius: var(--radius-lg);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 12px;
  }

  .progress {
    font-size: 12px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .stats {
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 12px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .stat-sep {
    color: var(--border-color);
  }

  .file-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 10px 12px;
    border-bottom: 1px solid var(--border-color);
    background: var(--bg-tertiary);
  }

  .file-name {
    font-size: 13px;
    font-weight: 500;
    color: var(--text-primary);
    word-break: break-all;
    flex: 1;
    margin-right: 12px;
    font-family: var(--font-mono);
  }

  .file-meta {
    display: flex;
    align-items: center;
    gap: 8px;
  }

  .preview-container {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 240px;
    overflow: hidden;
    background: var(--bg-primary);
    position: relative;
  }

  .preview-icon {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    color: var(--text-muted);
    font-size: 12px;
  }

  .preview-icon svg {
    opacity: 0.5;
  }

  .preview-hint {
    position: absolute;
    bottom: 8px;
    right: 8px;
    display: flex;
    align-items: center;
    gap: 6px;
    font-size: 11px;
    color: var(--text-muted);
    opacity: 0;
    transition: opacity var(--transition-fast);
  }

  .card-button:hover .preview-hint {
    opacity: 1;
  }

  .actions {
    display: flex;
    align-items: center;
    justify-content: center;
    margin-top: 16px;
    gap: 12px;
  }
</style>
