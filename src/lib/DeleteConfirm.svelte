<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { FileInfo, DeleteResult, TriageResult } from "../types";
  import Button from "./ui/Button.svelte";
  import Card from "./ui/Card.svelte";
  import Kbd from "./ui/Kbd.svelte";

  export let filesToDelete: FileInfo[];
  export let keptCount: number;
  export let totalCount: number;

  const dispatch = createEventDispatcher<{
    cancel: void;
    complete: TriageResult;
  }>();

  let isDeleting = false;
  let deleteError: string | null = null;
  let removedCount = 0;
  let showConfirmDialog = false;

  function removeFromList(index: number) {
    filesToDelete = filesToDelete.filter((_, i) => i !== index);
    removedCount++;
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  function getTotalSize(): string {
    const total = filesToDelete.reduce((sum, f) => sum + f.size, 0);
    return formatSize(total);
  }

  function handleMainAction() {
    if (filesToDelete.length === 0) {
      dispatch("complete", {
        kept: keptCount + removedCount,
        deleted: 0,
        total: totalCount,
        savedSize: 0,
      });
      return;
    }
    showConfirmDialog = true;
  }

  async function executeDelete() {
    isDeleting = true;
    deleteError = null;

    try {
      const paths = filesToDelete.map((f) => f.path);
      const result: DeleteResult = await invoke("delete_files", {
        filePaths: paths,
      });

      if (result.failed.length > 0) {
        const failedFiles = result.failed.map(([path, err]) => {
          const name = path.split("/").pop();
          return `${name}: ${err}`;
        });
        deleteError = `Failed to delete ${result.failed.length} file(s):\n${failedFiles.join("\n")}`;
        showConfirmDialog = false; // Go back to review on error
      } else {
        const savedSize = result.success.reduce((total, path) => {
          const file = filesToDelete.find((f) => f.path === path);
          return total + (file ? file.size : 0);
        }, 0);

        dispatch("complete", {
          kept: keptCount + removedCount + result.failed.length,
          deleted: result.success.length,
          total: totalCount,
          savedSize,
        });
      }
    } catch (e) {
      deleteError = `Delete failed: ${e}`;
      isDeleting = false;
      showConfirmDialog = false;
    }
  }

  function handleCancel() {
    if (showConfirmDialog) {
      showConfirmDialog = false;
    } else {
      dispatch("cancel");
    }
  }

  function handleKeydown(event: KeyboardEvent) {
    if (isDeleting) return;

    if (showConfirmDialog) {
      if (event.key === "Enter") {
        event.preventDefault();
        executeDelete();
      } else if (event.key === "Escape" || event.key === "Backspace") {
        event.preventDefault();
        showConfirmDialog = false;
      }
    } else {
      if (event.key === "Enter") {
        event.preventDefault();
        handleMainAction();
      } else if (event.key === "Backspace") {
        event.preventDefault();
        handleCancel();
      }
    }
  }

  onMount(() => {
    window.addEventListener("keydown", handleKeydown);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeydown);
  });
</script>

<div class="delete-confirm">
  <div class="content">
    {#if showConfirmDialog}
      <div class="confirmation-view">
        <h2 class="heading">Are you sure?</h2>
        <p class="description">
          This will move {filesToDelete.length} file{filesToDelete.length !== 1
            ? "s"
            : ""} to the trash.
          <br />
          You can restore them later if needed.
        </p>

        <div class="actions">
          <Button
            variant="outline"
            on:click={() => (showConfirmDialog = false)}
            disabled={isDeleting}
          >
            Cancel
            <Kbd>Esc</Kbd>
          </Button>
          <Button
            variant="danger"
            on:click={executeDelete}
            disabled={isDeleting}
          >
            {#if isDeleting}
              Deleting...
            {:else}
              Yes, Move to Trash
              <Kbd>⏎</Kbd>
            {/if}
          </Button>
        </div>
      </div>
    {:else}
      {#if filesToDelete.length > 0}
        <h2 class="heading">Review deletions</h2>
        <p class="description">
          {filesToDelete.length} file{filesToDelete.length !== 1 ? "s" : ""} · {getTotalSize()}
        </p>
      {/if}

      {#if filesToDelete.length === 0}
        <div class="empty-state">
          <p>No files to delete</p>
        </div>
      {:else}
        <div class="file-list-container">
          <Card padding="none">
            <div class="file-list">
              {#each filesToDelete as file, index}
                <div class="file-item">
                  <div class="file-info">
                    <span class="file-name">{file.name}</span>
                    <span class="file-size">{formatSize(file.size)}</span>
                  </div>
                  <button
                    class="remove-btn"
                    on:click={() => removeFromList(index)}
                    title="Remove from list"
                    disabled={isDeleting}
                  >
                    <svg
                      width="14"
                      height="14"
                      viewBox="0 0 24 24"
                      fill="none"
                      stroke="currentColor"
                      stroke-width="2"
                    >
                      <line x1="18" y1="6" x2="6" y2="18" />
                      <line x1="6" y1="6" x2="18" y2="18" />
                    </svg>
                  </button>
                </div>
              {/each}
            </div>
          </Card>
        </div>
      {/if}

      {#if deleteError}
        <div class="error-message">
          {deleteError}
        </div>
      {/if}

      <div class="actions">
        <Button variant="outline" on:click={handleCancel} disabled={isDeleting}>
          Go Back
          <Kbd>⌫</Kbd>
        </Button>
        <Button
          variant="danger"
          on:click={handleMainAction}
          disabled={isDeleting}
        >
          {#if filesToDelete.length === 0}
            Done
            <Kbd>⏎</Kbd>
          {:else}
            Trash {filesToDelete.length}
            <Kbd>⏎</Kbd>
          {/if}
        </Button>
      </div>
    {/if}
  </div>
</div>

<style>
  .delete-confirm {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 24px;
    max-width: 560px;
    margin: 0 auto;
    width: 100%;
  }

  .content {
    display: flex;
    flex-direction: column;
    flex: 1;
    justify-content: center;
  }

  .heading {
    font-size: 18px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 4px;
    text-align: center;
  }

  .description {
    color: var(--text-muted);
    font-size: 13px;
    margin-bottom: 16px;
    text-align: center;
    font-family: var(--font-mono);
  }

  .empty-state {
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    font-size: 14px;
    height: 150px;
    width: 100%;
  }

  .file-list-container {
    margin-bottom: 16px;
  }

  .file-list {
    max-height: 320px;
    overflow-y: auto;
  }

  .file-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 10px 12px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .file-item:last-child {
    border-bottom: none;
  }

  .file-info {
    display: flex;
    flex-direction: column;
    gap: 2px;
    flex: 1;
    min-width: 0;
  }

  .file-name {
    font-size: 13px;
    color: var(--text-primary);
    font-family: var(--font-mono);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .file-size {
    font-size: 11px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .remove-btn {
    display: flex;
    align-items: center;
    justify-content: center;
    width: 28px;
    height: 28px;
    border-radius: var(--radius-sm);
    color: var(--text-muted);
    transition: all var(--transition-fast);
    flex-shrink: 0;
  }

  .remove-btn:hover:not(:disabled) {
    color: var(--text-primary);
    background: var(--bg-hover);
  }

  .remove-btn:disabled {
    opacity: 0.4;
    cursor: not-allowed;
  }

  .error-message {
    color: var(--accent-text);
    font-size: 13px;
    background: var(--accent-subtle);
    padding: 12px 16px;
    border-radius: var(--radius-md);
    margin-bottom: 16px;
    white-space: pre-line;
    font-family: var(--font-mono);
  }

  .actions {
    display: flex;
    gap: 12px;
    justify-content: center;
  }
</style>
