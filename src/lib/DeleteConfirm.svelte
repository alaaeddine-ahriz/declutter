<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { FileInfo, DeleteResult, TriageResult } from "../types";

  export let filesToDelete: FileInfo[];
  export let keptCount: number;
  export let totalCount: number;

  const dispatch = createEventDispatcher<{
    cancel: void;
    complete: TriageResult;
  }>();

  let isDeleting = false;
  let deleteError: string | null = null;

  function removeFromList(index: number) {
    filesToDelete = filesToDelete.filter((_, i) => i !== index);
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

  async function confirmDelete() {
    if (filesToDelete.length === 0) {
      dispatch("complete", {
        kept: keptCount,
        deleted: 0,
        total: totalCount,
      });
      return;
    }

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
      }

      dispatch("complete", {
        kept: keptCount + (filesToDelete.length - result.success.length),
        deleted: result.success.length,
        total: totalCount,
      });
    } catch (e) {
      deleteError = `Delete failed: ${e}`;
      isDeleting = false;
    }
  }

  function handleCancel() {
    dispatch("cancel");
  }
</script>

<div class="delete-confirm">
  <div class="content">
    <h2 class="heading">Review Files to Delete</h2>
    <p class="description">
      {filesToDelete.length} file{filesToDelete.length !== 1 ? "s" : ""} selected
      ({getTotalSize()})
    </p>

    {#if filesToDelete.length === 0}
      <div class="empty-state">
        <p>No files to delete</p>
      </div>
    {:else}
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
              title="Remove from delete list"
              disabled={isDeleting}
            >
              ✕
            </button>
          </div>
        {/each}
      </div>
    {/if}

    {#if deleteError}
      <div class="error-message">
        {deleteError}
      </div>
    {/if}

    <div class="actions">
      <button class="btn btn-secondary" on:click={handleCancel} disabled={isDeleting}>
        Go Back
      </button>
      <button
        class="btn btn-danger"
        on:click={confirmDelete}
        disabled={isDeleting}
      >
        {#if isDeleting}
          Deleting...
        {:else if filesToDelete.length === 0}
          Done
        {:else}
          Delete {filesToDelete.length} File{filesToDelete.length !== 1 ? "s" : ""}
        {/if}
      </button>
    </div>
  </div>
</div>

<style>
  .delete-confirm {
    flex: 1;
    display: flex;
    flex-direction: column;
    padding: 1.5rem;
    max-width: 600px;
    margin: 0 auto;
    width: 100%;
  }

  .content {
    display: flex;
    flex-direction: column;
    flex: 1;
  }

  .heading {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 0.25rem;
  }

  .description {
    color: var(--text-muted);
    font-size: 0.8125rem;
    margin-bottom: 1rem;
  }

  .empty-state {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    font-size: 0.875rem;
  }

  .file-list {
    flex: 1;
    overflow-y: auto;
    border: 1px solid var(--border-color);
    border-radius: 8px;
    background: var(--bg-secondary);
    max-height: 400px;
  }

  .file-item {
    display: flex;
    align-items: center;
    justify-content: space-between;
    padding: 0.75rem 1rem;
    border-bottom: 1px solid var(--border-color);
  }

  .file-item:last-child {
    border-bottom: none;
  }

  .file-info {
    display: flex;
    flex-direction: column;
    gap: 0.125rem;
    flex: 1;
    min-width: 0;
  }

  .file-name {
    font-size: 0.875rem;
    color: var(--text-primary);
    word-break: break-all;
  }

  .file-size {
    font-size: 0.75rem;
    font-family: var(--font-mono);
    color: var(--text-muted);
  }

  .remove-btn {
    width: 28px;
    height: 28px;
    border-radius: 4px;
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    font-size: 0.75rem;
    flex-shrink: 0;
    margin-left: 0.75rem;
  }

  .remove-btn:hover:not(:disabled) {
    background: var(--bg-primary);
    color: var(--danger);
  }

  .remove-btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .error-message {
    margin-top: 1rem;
    padding: 0.75rem;
    background: rgba(239, 68, 68, 0.1);
    border: 1px solid var(--danger);
    border-radius: 6px;
    color: var(--danger);
    font-size: 0.8125rem;
    white-space: pre-wrap;
  }

  .actions {
    display: flex;
    gap: 0.75rem;
    margin-top: 1.5rem;
    justify-content: flex-end;
  }

  .btn {
    padding: 0.625rem 1.25rem;
    font-size: 0.875rem;
    font-weight: 500;
    border-radius: 6px;
    transition: all 0.15s ease;
  }

  .btn:disabled {
    opacity: 0.5;
    cursor: not-allowed;
  }

  .btn-secondary {
    border: 1px solid var(--border-color);
    background-color: var(--bg-secondary);
    color: var(--text-primary);
  }

  .btn-secondary:hover:not(:disabled) {
    background-color: var(--bg-primary);
  }

  .btn-danger {
    background-color: var(--danger);
    color: white;
    border: none;
  }

  .btn-danger:hover:not(:disabled) {
    opacity: 0.9;
  }
</style>


