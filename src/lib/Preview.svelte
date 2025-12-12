<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import type { FileInfo } from "../types";

  export let file: FileInfo;

  let textContent = "";
  let isLoading = true;
  let imageLoading = true;
  let error = "";

  $: assetUrl = convertFileSrc(file.path) + `?t=${Date.now()}`;

  $: if (file) {
    loadPreview();
  }

  async function loadPreview() {
    isLoading = true;
    imageLoading = true; // Reset image loading state
    error = "";
    textContent = "";

    if (file.file_type === "text") {
      try {
        textContent = await invoke<string>("read_text_preview", {
          filePath: file.path,
          maxBytes: 2000,
        });
      } catch (e) {
        error = "Could not load preview";
      }
    }

    isLoading = false;
  }

  function handleImageLoad() {
    imageLoading = false;
  }

  function handleImageError() {
    imageLoading = false;
    error = "Could not load image";
  }
</script>

<div class="preview">
  {#if isLoading}
    <div class="loading">Loading...</div>
  {:else if file.file_type === "image"}
    <div class="image-container">
      {#if imageLoading}
        <div class="loading">Loading image...</div>
      {/if}
      <img
        src={assetUrl}
        alt={file.name}
        class="image-preview"
        class:hidden={imageLoading}
        on:load={handleImageLoad}
        on:error={handleImageError}
      />
    </div>
    {#if error}
      <div class="fallback">
        <svg
          width="32"
          height="32"
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
    {/if}
  {:else if file.file_type === "pdf"}
    <div class="fallback">
      <svg
        width="32"
        height="32"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="1.5"
      >
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
        <polyline points="14 2 14 8 20 8" />
      </svg>
      <span>PDF</span>
    </div>
  {:else if file.file_type === "text"}
    {#if error}
      <div class="fallback"><span>{error}</span></div>
    {:else}
      <pre
        class="text-preview">{textContent}{#if textContent.length >= 2000}...{/if}</pre>
    {/if}
  {:else}
    <div class="fallback">
      <svg
        width="32"
        height="32"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="1.5"
      >
        <path d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" />
        <polyline points="14 2 14 8 20 8" />
      </svg>
      <span>No preview</span>
    </div>
  {/if}
</div>

<style>
  .preview {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
  }

  .loading {
    color: var(--text-muted);
    font-size: 13px;
  }

  .image-container {
    width: 100%;
    height: 100%;
    display: flex;
    align-items: center;
    justify-content: center;
    position: relative;
  }

  .image-preview {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .hidden {
    display: none;
  }

  .fallback {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    color: var(--text-muted);
    font-size: 12px;
  }

  .text-preview {
    width: 100%;
    height: 100%;
    overflow: auto;
    font-family: var(--font-mono);
    font-size: 11px;
    line-height: 1.6;
    color: var(--text-secondary);
    white-space: pre-wrap;
    word-wrap: break-word;
    text-align: left;
    margin: 0;
    padding: 12px;
    background: var(--bg-primary);
  }
</style>
