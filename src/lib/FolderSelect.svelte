<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { FileInfo } from "../types";
  import Button from "./ui/Button.svelte";
  import Kbd from "./ui/Kbd.svelte";

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
        const files = await invoke<FileInfo[]>("list_files", {
          folderPath: path,
        });

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

  function handleKeydown(event: KeyboardEvent) {
    if ((event.metaKey || event.ctrlKey) && event.key === "o") {
      event.preventDefault();
      if (!isLoading) selectFolder();
    }
  }

  onMount(() => {
    window.addEventListener("keydown", handleKeydown);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeydown);
  });
</script>

<div class="folder-select">
  <div class="content">
    <h1 class="title">declutter</h1>
    <p class="description">
      Swipe through files. Keep or delete. Clean up fast.
    </p>

    <div class="actions">
      <Button
        variant="primary"
        size="lg"
        on:click={selectFolder}
        disabled={isLoading}
      >
        {#if isLoading}
          Loading...
        {:else}
          Open Folder
          <Kbd>⌘O</Kbd>
        {/if}
      </Button>
    </div>

    {#if error}
      <p class="error">{error}</p>
    {/if}
  </div>
</div>

<style>
  .folder-select {
    display: flex;
    flex-direction: column;
    flex: 1;
    width: 100%;
    background-color: var(--bg-primary);
  }

  .content {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 32px;
    max-width: 400px;
    margin: 0 auto;
  }

  .title {
    font-size: 28px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 8px;
    letter-spacing: -0.02em;
  }

  .description {
    font-size: 14px;
    color: var(--text-muted);
    margin-bottom: 32px;
    line-height: 1.6;
  }

  .actions {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 20px;
    width: 100%;
  }

  .error {
    color: var(--accent-text);
    margin-top: 16px;
    font-size: 13px;
    background: var(--accent-subtle);
    padding: 8px 16px;
    border-radius: var(--radius-md);
  }
</style>
