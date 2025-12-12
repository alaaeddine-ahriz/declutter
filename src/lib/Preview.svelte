<script lang="ts">
  import { invoke } from "@tauri-apps/api/tauri";
  import { convertFileSrc } from "@tauri-apps/api/tauri";
  import type { FileInfo, DirectoryNode } from "../types";

  export let file: FileInfo;

  let textContent = "";
  let isLoading = true;
  let imageLoading = true;
  let error = "";
  let folderTree: DirectoryNode | null = null;

  $: assetUrl = convertFileSrc(file.path) + `?t=${Date.now()}`;

  $: if (file) {
    loadPreview();
  }

  async function loadPreview() {
    isLoading = true;
    imageLoading = true;
    error = "";
    textContent = "";
    folderTree = null;

    if (file.file_type === "text") {
      try {
        textContent = await invoke<string>("read_text_preview", {
          filePath: file.path,
          maxBytes: 2000,
        });
      } catch (e) {
        error = "Could not load preview";
      }
    } else if (file.file_type === "folder") {
      try {
        folderTree = await invoke<DirectoryNode>("list_directory_tree", {
          folderPath: file.path,
          maxDepth: 3,
        });
      } catch (e) {
        error = "Could not load folder contents";
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

  function formatSize(bytes: number): string {
    if (bytes < 1024) return `${bytes} B`;
    if (bytes < 1024 * 1024) return `${(bytes / 1024).toFixed(1)} KB`;
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  function countItems(node: DirectoryNode): { files: number; folders: number } {
    let files = 0;
    let folders = 0;

    if (node.children) {
      for (const child of node.children) {
        if (child.is_directory) {
          folders++;
          const sub = countItems(child);
          files += sub.files;
          folders += sub.folders;
        } else {
          files++;
        }
      }
    }
    return { files, folders };
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
  {:else if file.file_type === "folder"}
    {#if error}
      <div class="fallback"><span>{error}</span></div>
    {:else if folderTree}
      <div class="folder-tree">
        <div class="folder-summary">
          {#if folderTree.children}
            {@const counts = countItems(folderTree)}
            <span class="summary-text">
              {counts.folders} folder{counts.folders !== 1 ? "s" : ""}, {counts.files}
              file{counts.files !== 1 ? "s" : ""}
            </span>
            <span class="summary-size">{formatSize(folderTree.size)}</span>
          {:else}
            <span class="summary-text">Empty folder</span>
          {/if}
        </div>
        {#if folderTree.children && folderTree.children.length > 0}
          <div class="tree-list">
            {#each folderTree.children.slice(0, 15) as node}
              <div class="tree-item" class:is-folder={node.is_directory}>
                {#if node.is_directory}
                  <svg
                    class="tree-icon"
                    width="12"
                    height="12"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                  >
                    <path
                      d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
                    ></path>
                  </svg>
                {:else}
                  <svg
                    class="tree-icon"
                    width="12"
                    height="12"
                    viewBox="0 0 24 24"
                    fill="none"
                    stroke="currentColor"
                    stroke-width="2"
                  >
                    <path
                      d="M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z"
                    />
                    <polyline points="14 2 14 8 20 8" />
                  </svg>
                {/if}
                <span class="tree-name">{node.name}</span>
              </div>
            {/each}
            {#if folderTree.children.length > 15}
              <div class="tree-more">
                +{folderTree.children.length - 15} more...
              </div>
            {/if}
          </div>
        {/if}
      </div>
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
          <path
            d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
          ></path>
        </svg>
        <span>Empty folder</span>
      </div>
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

  .folder-tree {
    width: 100%;
    height: 100%;
    display: flex;
    flex-direction: column;
    padding: 12px;
    overflow: auto;
    background: var(--bg-primary);
  }

  .folder-summary {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding-bottom: 8px;
    margin-bottom: 8px;
    border-bottom: 1px solid var(--border-subtle);
  }

  .summary-text {
    font-size: 11px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .summary-size {
    font-size: 11px;
    color: var(--text-muted);
    font-family: var(--font-mono);
  }

  .tree-list {
    display: flex;
    flex-direction: column;
    gap: 2px;
  }

  .tree-item {
    display: flex;
    align-items: center;
    gap: 6px;
    padding: 3px 0;
  }

  .tree-icon {
    flex-shrink: 0;
    color: var(--text-muted);
  }

  .tree-item.is-folder .tree-icon {
    color: var(--primary);
  }

  .tree-name {
    font-size: 11px;
    color: var(--text-secondary);
    font-family: var(--font-mono);
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .tree-more {
    font-size: 11px;
    color: var(--text-muted);
    font-style: italic;
    padding: 4px 0;
  }
</style>
