<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { FileInfo } from "../types";

  const dispatch = createEventDispatcher<{
    folderSelected: { path: string; files: FileInfo[] };
  }>();

  let isLoading = false;
  let error = "";

  async function selectFolder() {
    isLoading = true;
    error = "";

    try {
      const path = await invoke<string | null>("select_folder");
      
      if (path) {
        const files = await invoke<FileInfo[]>("list_files", { folderPath: path });
        
        if (files.length === 0) {
          error = "No files found in this folder.";
        } else {
          dispatch("folderSelected", { path, files });
        }
      }
    } catch (e) {
      error = String(e);
    } finally {
      isLoading = false;
    }
  }
</script>

<div class="folder-select">
  <div class="content">
    <h2 class="heading">Declutter</h2>
    <p class="description">Quickly sort files: keep or delete</p>

    <button class="btn btn-primary" on:click={selectFolder} disabled={isLoading}>
      {#if isLoading}
        Loading...
      {:else}
        Open Folder
      {/if}
    </button>

    {#if error}
      <p class="error">{error}</p>
    {/if}

    <div class="instructions">
      <span><kbd>←</kbd> delete</span>
      <span><kbd>→</kbd> keep</span>
    </div>
  </div>
</div>

<style>
  .folder-select {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 2rem;
  }

  .content {
    text-align: center;
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
    margin-bottom: 1.5rem;
  }

  .error {
    color: var(--danger);
    font-size: 0.75rem;
    margin-top: 0.75rem;
  }

  .instructions {
    display: flex;
    justify-content: center;
    gap: 1.5rem;
    margin-top: 1.5rem;
    font-size: 0.75rem;
    color: var(--text-muted);
  }
</style>

