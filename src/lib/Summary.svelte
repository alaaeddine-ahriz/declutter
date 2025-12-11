<script lang="ts">
  import { createEventDispatcher, onMount, onDestroy } from "svelte";
  import type { TriageResult } from "../types";
  import Button from "./ui/Button.svelte";
  import Kbd from "./ui/Kbd.svelte";

  export let result: TriageResult;

  const dispatch = createEventDispatcher<{
    startOver: void;
  }>();

  function handleStartOver() {
    dispatch("startOver");
  }

  function handleKeydown(event: KeyboardEvent) {
    if (event.key === "Enter") {
      event.preventDefault();
      handleStartOver();
    }
  }

  onMount(() => {
    window.addEventListener("keydown", handleKeydown);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeydown);
  });
</script>

<div class="summary">
  <div class="content">
    <!-- <div class="icon">
      <svg
        width="40"
        height="40"
        viewBox="0 0 24 24"
        fill="none"
        stroke="currentColor"
        stroke-width="1.5"
        stroke-linecap="round"
        stroke-linejoin="round"
      >
        <polyline points="20 6 9 17 4 12" />
      </svg>
    </div> -->

    <h2 class="heading">Done</h2>
    <p class="description">{result.total} files reviewed</p>

    <div class="stats">
      <div class="stat">
        <span class="stat-value">{result.kept}</span>
        <span class="stat-label">kept</span>
      </div>
      <div class="stat-divider"></div>
      <div class="stat">
        <span class="stat-value">{result.deleted}</span>
        <span class="stat-label">deleted</span>
      </div>
    </div>

    <p class="note">Files moved to trash</p>

    <Button variant="primary" on:click={handleStartOver}>
      Start Over
      <Kbd>⏎</Kbd>
    </Button>
  </div>
</div>

<style>
  .summary {
    flex: 1;
    display: flex;
    align-items: center;
    justify-content: center;
    padding: 32px;
  }

  .content {
    text-align: center;
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .icon {
    color: var(--accent);
    margin-bottom: 20px;
  }

  .heading {
    font-size: 20px;
    font-weight: 600;
    color: var(--text-primary);
    margin-bottom: 4px;
  }

  .description {
    color: var(--text-muted);
    font-size: 13px;
    margin-bottom: 24px;
    font-family: var(--font-mono);
  }

  .stats {
    display: flex;
    align-items: center;
    gap: 24px;
    margin-bottom: 24px;
  }

  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .stat-value {
    font-size: 28px;
    font-weight: 600;
    font-family: var(--font-mono);
    color: var(--text-primary);
  }

  .stat-label {
    font-size: 12px;
    color: var(--text-muted);
  }

  .stat-divider {
    width: 1px;
    height: 40px;
    background: var(--border-color);
  }

  .note {
    font-size: 12px;
    color: var(--text-muted);
    margin-bottom: 24px;
  }
</style>
