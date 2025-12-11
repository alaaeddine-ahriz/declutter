<script lang="ts">
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import type { FileInfo } from "../types";

  export let file: FileInfo;

  let textContent = "";
  let isLoading = true;
  let error = "";

  $: assetUrl = convertFileSrc(file.path);

  $: if (file) {
    loadPreview();
  }

  async function loadPreview() {
    isLoading = true;
    error = "";
    textContent = "";

    if (file.file_type === "text") {
      try {
        textContent = await invoke<string>("read_text_preview", {
          filePath: file.path,
          maxBytes: 2000,
        });
      } catch (e) {
        error = "Could not load text preview";
      }
    }

    isLoading = false;
  }
</script>

<div class="preview">
  {#if isLoading}
    <div class="loading">Loading...</div>
  {:else if file.file_type === "image"}
    <img src={assetUrl} alt={file.name} class="image-preview" />
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
    /* padding: 1rem; -- Removed padding to let image fill the box */
    overflow: hidden; /* Changed from auto to hidden for cleaner look with images */
  }

  /* ... */

  .image-preview {
    width: 100%;
    height: 100%;
    object-fit: contain;
  }

  .fallback {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 0.5rem;
    color: var(--text-muted);
    font-size: 0.75rem;
  }

  .text-preview {
    width: 100%;
    height: 100%;
    overflow: auto;
    font-family: var(--font-mono);
    font-size: 0.6875rem;
    line-height: 1.5;
    color: var(--text-secondary);
    white-space: pre-wrap;
    word-wrap: break-word;
    text-align: left;
    margin: 0;
  }
</style>
