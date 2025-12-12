<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { fly } from "svelte/transition";
  import { invoke, convertFileSrc } from "@tauri-apps/api/tauri";
  import type { FileInfo } from "../types";
  import Button from "./ui/Button.svelte";
  import Badge from "./ui/Badge.svelte";
  import Kbd from "./ui/Kbd.svelte";

  export let file: FileInfo | null = null;
  export let isOpen: boolean = false;

  const dispatch = createEventDispatcher<{
    close: void;
  }>();

  let textContent = "";
  let isLoading = true;
  let error = "";
  let imageError = false;

  // Add timestamp to bust browser cache when re-opening same file
  $: assetUrl = file ? convertFileSrc(file.path) + `?t=${Date.now()}` : "";

  $: if (file && isOpen) {
    loadPreview();
  }

  async function loadPreview() {
    isLoading = true;
    error = "";
    textContent = "";
    imageError = false;

    if (file?.file_type === "text") {
      try {
        textContent = await invoke<string>("read_text_preview", {
          filePath: file.path,
          maxBytes: 10000,
        });
      } catch (e) {
        error = "Could not load preview";
      }
    }

    isLoading = false;
  }

  function handleImageError() {
    imageError = true;
  }

  function close() {
    dispatch("close");
  }

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    if (bytes < 1024 * 1024 * 1024)
      return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
    return `${(bytes / (1024 * 1024 * 1024)).toFixed(2)} GB`;
  }

  function getFileExtension(name: string): string {
    const parts = name.split(".");
    return parts.length > 1 ? parts[parts.length - 1].toUpperCase() : "";
  }
</script>

{#if isOpen && file}
  <div
    class="preview-panel"
    transition:fly={{ x: 400, duration: 250, opacity: 1 }}
  >
    <div class="panel-header">
      <div class="header-info">
        <span class="panel-title">Preview</span>
        {#if file}
          <Badge variant="neutral"
            >{getFileExtension(file.name) || file.file_type}</Badge
          >
        {/if}
      </div>
      <Button variant="ghost" size="sm" on:click={close}>
        Close
        <Kbd>Esc</Kbd>
      </Button>
    </div>

    <div class="panel-content">
      <!-- <div class="file-info">
        <h3 class="file-name">{file.name}</h3>
        <p class="file-path">{file.path}</p>
        <div class="file-meta">
          <Badge variant="neutral">{formatSize(file.size)}</Badge>
          <Badge variant="neutral">{file.file_type}</Badge>
        </div>
      </div> -->

      <div class="preview-area">
        {#if isLoading}
          <div class="loading">Loading...</div>
        {:else if file.file_type === "image"}
          {#if imageError}
            <div class="fallback">
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
              <span>Could not load image</span>
            </div>
          {:else}
            <img
              src={assetUrl}
              alt={file.name}
              class="image-preview"
              on:error={handleImageError}
            />
          {/if}
        {:else if file.file_type === "pdf"}
          {#key file.path}
            <object
              data={assetUrl}
              type="application/pdf"
              class="pdf-preview"
              title={file.name}
            >
              <div class="fallback">
                <svg
                  width="48"
                  height="48"
                  viewBox="0 0 24 24"
                  fill="none"
                  stroke="currentColor"
                  stroke-width="1.5"
                >
                  <path
                    d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"
                  />
                  <polyline points="14 2 14 8 20 8" />
                </svg>
                <span>PDF preview not available</span>
              </div>
            </object>
          {/key}
        {:else if file.file_type === "text"}
          {#if error}
            <div class="fallback">
              <svg
                width="48"
                height="48"
                viewBox="0 0 24 24"
                fill="none"
                stroke="currentColor"
                stroke-width="1.5"
              >
                <path
                  d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"
                />
                <polyline points="14 2 14 8 20 8" />
              </svg>
              <span>{error}</span>
            </div>
          {:else}
            <pre
              class="text-preview">{textContent}{#if textContent.length >= 10000}...{/if}</pre>
          {/if}
        {:else}
          <div class="fallback">
            <svg
              width="48"
              height="48"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              stroke-width="1.5"
            >
              <path
                d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"
              />
              <polyline points="14 2 14 8 20 8" />
            </svg>
            <span>No preview available</span>
          </div>
        {/if}
      </div>
    </div>
  </div>
{/if}

<style>
  .preview-panel {
    position: fixed;
    top: 0;
    right: 0;
    width: 400px;
    height: 100vh;
    background: var(--bg-secondary);
    border-left: 1px solid var(--border-color);
    display: flex;
    flex-direction: column;
    z-index: 100;
  }

  .panel-header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0 16px;
    height: 48px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .header-info {
    display: flex;
    align-items: center;
    gap: 10px;
  }

  .panel-title {
    font-size: 13px;
    font-weight: 600;
    color: var(--text-primary);
  }

  .panel-content {
    flex: 1;
    display: flex;
    flex-direction: column;
    overflow: hidden;
  }

  .file-info {
    padding: 16px;
    border-bottom: 1px solid var(--border-color);
  }

  .file-name {
    font-size: 14px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 4px;
    word-break: break-all;
    font-family: var(--font-mono);
  }

  .file-path {
    font-size: 11px;
    color: var(--text-muted);
    margin-bottom: 12px;
    word-break: break-all;
    font-family: var(--font-mono);
  }

  .file-meta {
    display: flex;
    gap: 8px;
  }

  .preview-area {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 16px;
    background: var(--bg-primary);
    overflow: auto;
  }

  .loading {
    color: var(--text-muted);
    font-size: 13px;
  }

  .image-preview {
    max-width: 100%;
    max-height: 100%;
    object-fit: contain;
    border-radius: var(--radius-sm);
  }

  .pdf-preview {
    width: 100%;
    height: 100%;
    border: none;
    border-radius: var(--radius-sm);
    background: white;
  }

  .text-preview {
    width: 100%;
    height: 100%;
    overflow: auto;
    font-family: var(--font-mono);
    font-size: 12px;
    line-height: 1.6;
    color: var(--text-secondary);
    white-space: pre-wrap;
    word-wrap: break-word;
    text-align: left;
    margin: 0;
    padding: 16px;
    background: var(--bg-secondary);
    border-radius: var(--radius-sm);
  }

  .fallback {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 12px;
    color: var(--text-muted);
    font-size: 13px;
  }

  .fallback svg {
    opacity: 0.5;
  }
</style>
