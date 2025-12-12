<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from "svelte";
  import { settings, checkKeyCombo, formatKeyCombo } from "./stores/settings";
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
  let deletedSize = 0;

  // Explore mode state
  let markedForDelete: Set<string> = new Set();

  // Initialize marked files from existing action history if needed (mostly for switching modes mid-session if allowed)
  // For now we assume fresh start or compatible state logic

  $: isExploreMode = $settings.mode === "explore";

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
    deletedSize += currentFile.size;
    advance();
  }

  function undo() {
    if (actionHistory.length === 0) return;

    const lastAction = actionHistory[actionHistory.length - 1];
    actionHistory = actionHistory.slice(0, -1);

    if (lastAction.type === "delete") {
      deletedCount--;
      deletedSize -= files[lastAction.fileIndex].size;
    } else {
      keptCount--;
    }

    currentIndex = lastAction.fileIndex;
  }

  function advance() {
    if (currentIndex < files.length - 1) {
      currentIndex++;
    } else {
      stopEarly();
    }
  }

  function stopEarly() {
    let filesToDeleteList: FileInfo[] = [];

    if ($settings.mode === "explore") {
      filesToDeleteList = files.filter((f) => markedForDelete.has(f.path));
      keptCount = files.length - filesToDeleteList.length;
    } else {
      filesToDeleteList = actionHistory
        .filter((action) => action.type === "delete")
        .map((action) => files[action.fileIndex]);
    }

    dispatch("complete", {
      filesToDelete: filesToDeleteList,
      keptCount,
    });
  }

  // Explore Mode Functions
  function exploreNext() {
    if (currentIndex < files.length - 1) {
      currentIndex++;
    }
  }

  function explorePrevious() {
    if (currentIndex > 0) {
      currentIndex--;
    }
  }

  function toggleMarkDelete() {
    if (!currentFile) return;

    const newMarked = new Set(markedForDelete);
    if (newMarked.has(currentFile.path)) {
      newMarked.delete(currentFile.path);
      deletedCount--;
      deletedSize -= currentFile.size;
    } else {
      newMarked.add(currentFile.path);
      deletedCount++;
      deletedSize += currentFile.size;
    }
    markedForDelete = newMarked;

    // Auto-advance is optional, maybe better to stay put in explore mode?
    // User request said "goes right and left", implying manual nav.
    // Let's stay put.
  }

  function handleKeydown(event: KeyboardEvent) {
    if ($settings.mode === "explore") {
      if (checkKeyCombo(event, $settings.keybindings.exploreNext)) {
        event.preventDefault();
        exploreNext();
      } else if (checkKeyCombo(event, $settings.keybindings.explorePrevious)) {
        event.preventDefault();
        explorePrevious();
      } else if (checkKeyCombo(event, $settings.keybindings.exploreDelete)) {
        event.preventDefault();
        toggleMarkDelete();
      } else if (checkKeyCombo(event, $settings.keybindings.preview)) {
        event.preventDefault();
        openPreview();
      }
      return;
    }

    if (checkKeyCombo(event, $settings.keybindings.keep)) {
      event.preventDefault();
      keepFile();
    } else if (checkKeyCombo(event, $settings.keybindings.delete)) {
      event.preventDefault();
      deleteFile();
    } else if (checkKeyCombo(event, $settings.keybindings.undo)) {
      event.preventDefault();
      undo();
    } else if (checkKeyCombo(event, $settings.keybindings.preview)) {
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
          {#if deletedSize > 0}
            <span class="stat-sep">·</span>
            <span class="stat">{formatSize(deletedSize)} saved</span>
          {/if}
        </div>
      </div>

      <button
        class="card-button"
        on:click={openPreview}
        title="Click to open preview"
      >
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
            {#if previewOpen && currentFile.file_type === "image"}
              <div class="preview-icon">
                <svg
                  width="48"
                  height="48"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="1.5"
                >
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
                <Kbd>{formatKeyCombo($settings.keybindings.preview)}</Kbd>
                <span>to preview</span>
              </div>
            {/if}

            {#if isExploreMode && currentFile && markedForDelete.has(currentFile.path)}
              <div class="delete-overlay">
                <div class="delete-badge">
                  <svg
                    width="24"
                    height="24"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                  >
                    <path d="M3 6h18"></path>
                    <path d="M19 6v14c0 1-1 2-2 2H7c-1 0-2-1-2-2V6"></path>
                    <path d="M8 6V4c0-1 1-2 2-2h4c1 0 2 1 2 2v2"></path>
                  </svg>
                  <span>Marked for Deletion</span>
                </div>
              </div>
            {/if}
          </div>
        </Card>
      </button>

      <div class="actions">
        {#if $settings.mode === "explore"}
          <Button
            variant="ghost"
            on:click={explorePrevious}
            disabled={currentIndex === 0}
          >
            <Kbd>{formatKeyCombo($settings.keybindings.explorePrevious)}</Kbd> Previous
          </Button>

          <Button
            variant={markedForDelete.has(currentFile?.path || "")
              ? "secondary"
              : "danger"}
            on:click={toggleMarkDelete}
          >
            {#if markedForDelete.has(currentFile?.path || "")}
              Unmark
            {:else}
              Mark Delete
            {/if}
            <Kbd>{formatKeyCombo($settings.keybindings.exploreDelete)}</Kbd>
          </Button>

          <Button
            variant="primary"
            on:click={exploreNext}
            disabled={currentIndex === files.length - 1}
          >
            Next <Kbd>{formatKeyCombo($settings.keybindings.exploreNext)}</Kbd>
          </Button>
        {:else}
          <Button variant="danger" on:click={deleteFile}>
            <Kbd>{formatKeyCombo($settings.keybindings.delete)}</Kbd>
            Delete
          </Button>

          {#if actionHistory.length > 0}
            {#if actionHistory.length > 0}
              <Button variant="outline" on:click={undo}>
                Undo
                <Kbd>{formatKeyCombo($settings.keybindings.undo)}</Kbd>
              </Button>
            {/if}
          {/if}

          <Button variant="primary" on:click={keepFile}>
            Keep
            <Kbd>{formatKeyCombo($settings.keybindings.keep)}</Kbd>
          </Button>
        {/if}
      </div>

      {#if deletedCount > 0}
        <div class="footer-actions">
          <Button variant="ghost" size="sm" on:click={stopEarly}>
            Stop & Review ({deletedCount})
          </Button>
        </div>
      {/if}
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

  .footer-actions {
    display: flex;
    justify-content: center;
    margin-top: 16px;
  }

  .delete-overlay {
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0, 0, 0, 0.4);
    display: flex;
    align-items: center;
    justify-content: center;
    pointer-events: none;
    backdrop-filter: blur(2px);
  }

  .delete-badge {
    background: var(--danger);
    color: white;
    padding: 12px 24px;
    border-radius: var(--radius-full);
    display: flex;
    align-items: center;
    gap: 8px;
    font-weight: 500;
    box-shadow: 0 4px 12px rgba(0, 0, 0, 0.2);
  }
</style>
