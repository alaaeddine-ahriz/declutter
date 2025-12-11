<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { FileInfo, DeleteResult, TriageResult } from "../types";
  import Button from "./ui/Button.svelte";
  import Card from "./ui/Card.svelte";

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
    {#if filesToDelete.length > 0}
      <h2 class="heading">Review Files to Delete</h2>
      <p class="description">
        {filesToDelete.length} file{filesToDelete.length !== 1 ? "s" : ""} selected
        ({getTotalSize()})
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
                <!-- Reuse Button but make it small/ghost/danger for removal -->
                <Button
                  variant="ghost"
                  size="sm"
                  on:click={() => removeFromList(index)}
                  title="Remove from delete list"
                  disabled={isDeleting}
                  style="color: var(--text-muted); padding: 0.25rem; height: auto;"
                >
                  ✕
                </Button>
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
      <Button variant="secondary" on:click={handleCancel} disabled={isDeleting}>
        Go Back
      </Button>
      <Button variant="danger" on:click={confirmDelete} disabled={isDeleting}>
        {#if isDeleting}
          Deleting...
        {:else if filesToDelete.length === 0}
          Done
        {:else}
          Delete {filesToDelete.length} File{filesToDelete.length !== 1
            ? "s"
            : ""}
        {/if}
      </Button>
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
    justify-content: center; /* Center vertical content */
  }

  .heading {
    font-size: 1.25rem;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 0.25rem;
    text-align: center; /* Center text */
  }

  .description {
    color: var(--text-muted);
    font-size: 0.8125rem;
    margin-bottom: 0.5rem; /* Reduced from 2rem to 0.5rem for tighter layout */
    text-align: center; /* Center text */
  }

  .empty-state {
    /* flex: 1; -- Removed to prevent excessive spreading */
    display: flex;
    align-items: center;
    justify-content: center;
    color: var(--text-muted);
    font-size: 1rem; /* Slightly larger as it is now the main focus when empty */
    height: 150px; /* Fixed modest height instead of filling screen */
    width: 100%;
  }

  .file-list {
    flex: 1;
    overflow-y: auto;
    max-height: 400px;
    text-align: left; /* Keep file list left-aligned for readability */
  }

  /* ... file items ... */

  .actions {
    display: flex;
    gap: 0.75rem;
    margin-top: 1rem; /* Reduced from 1.5rem to 1rem */
    justify-content: center; /* Center buttons */
  }
</style>
