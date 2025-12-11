<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import type { FileInfo } from "../types";
  import Button from "./ui/Button.svelte";

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
</script>

<div class="folder-select-container">
  <!-- <header>
    <div class="logo">Declutter</div>
  </header> -->

  <main class="hero">
    <div class="graphic">
      <!-- Placeholder for the abstract 3D shape -->
      <div class="orb"></div>
    </div>

    <h1>Reclaim your digital space</h1>
    <p class="subtitle">
      Select a folder to automatically sort and organize your files.
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
          <svg
            xmlns="http://www.w3.org/2000/svg"
            width="20"
            height="20"
            viewBox="0 0 24 24"
            fill="currentColor"
            stroke="currentColor"
            stroke-width="0"
            style="margin-right: 0.5rem;"
            ><path
              d="M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2v11z"
            ></path></svg
          >
          Open Folder
        {/if}
      </Button>

      <!-- <p class="drop-hint">or drop a folder here</p> -->
    </div>

    {#if error}
      <p class="error">{error}</p>
    {/if}
  </main>

  <!-- Hidden version info or tiny footer if needed -->
</div>

<style>
  .folder-select-container {
    display: flex;
    flex-direction: column;
    flex: 1; /* Changed from height: 100% to flex: 1 to fill parent .content correctly */
    width: 100%;
    background-color: #f8fafc; /* Light slate/gray bg */
    color: #1e293b;
    position: relative;
  }

  header {
    padding: 1.5rem 2rem;
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
  }

  .logo {
    font-weight: 700;
    font-size: 1.25rem;
    color: #0f172a;
    display: flex;
    align-items: center;
    gap: 0.5rem;
  }

  /* Simple colored bar for logo decoration if desired, or plain text as per image */
  .logo::before {
    content: "";
    display: inline-block;
    width: 4px;
    height: 16px;
    background-color: #0ea5e9; /* Cyan 500 */
    border-radius: 2px;
  }

  .hero {
    flex: 1;
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    text-align: center;
    padding: 2rem;
    max-width: 600px;
    margin: 0 auto;
  }

  .graphic {
    margin-bottom: 2.5rem;
  }

  .orb {
    width: 160px;
    height: 160px;
    border-radius: 50%;
    /* Gradient mimicking the teal/green swirl */
    background: radial-gradient(circle at 30% 30%, #5eead4, #0f766e);
    box-shadow:
      0 20px 25px -5px rgba(0, 0, 0, 0.1),
      0 10px 10px -5px rgba(0, 0, 0, 0.04);
    position: relative;
    overflow: hidden;
  }

  /* Add a pseudo-element to create a bit of that "swirl" effect abstractly */
  .orb::after {
    content: "";
    position: absolute;
    top: 20%;
    left: -10%;
    width: 120%;
    height: 120%;
    background: radial-gradient(
      circle at 70% 70%,
      rgba(255, 255, 255, 0.4),
      transparent 60%
    );
    border-radius: 50%;
    mix-blend-mode: overlay;
  }

  h1 {
    font-size: 2.5rem;
    font-weight: 800;
    color: #0f172a; /* Slate 900 */
    margin-bottom: 1rem;
    line-height: 1.2;
    letter-spacing: -0.025em;
  }

  .subtitle {
    font-size: 1.125rem;
    color: #64748b; /* Slate 500 */
    margin-bottom: 3rem;
    line-height: 1.6;
    max-width: 400px;
  }

  .actions {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 1rem;
    width: 100%;
  }

  /* Removed .btn-primary, .btn-primary:hover, .btn-primary:active, .btn-primary:disabled as they are replaced by Button component */

  .folder-icon {
    /* SVG styling handled inline generally, but can override here */
    fill: currentColor;
  }

  .drop-hint {
    font-size: 0.875rem;
    color: #94a3b8; /* Slate 400 */
    margin-top: 0.5rem;
  }

  .error {
    color: #ef4444;
    margin-top: 1rem;
    font-size: 0.875rem;
    background: #fef2f2;
    padding: 0.5rem 1rem;
    border-radius: 0.375rem;
  }
</style>
