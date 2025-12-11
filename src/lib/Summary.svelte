<script lang="ts">
  import { createEventDispatcher } from "svelte";
  import type { TriageResult } from "../types";

  export let result: TriageResult;

  const dispatch = createEventDispatcher<{
    startOver: void;
  }>();

  function handleStartOver() {
    dispatch("startOver");
  }
</script>

<div class="summary">
  <div class="content">
    <h2 class="heading">Done</h2>
    <p class="description">{result.total} files reviewed</p>

    <div class="stats">
      <div class="stat">
        <span class="stat-value kept">{result.kept}</span>
        <span class="stat-label">kept</span>
      </div>
      <div class="stat">
        <span class="stat-value deleted">{result.deleted}</span>
        <span class="stat-label">deleted</span>
      </div>
    </div>

    <p class="note">Files have been permanently deleted</p>

    <button class="btn btn-primary" on:click={handleStartOver}>
      Start Over
    </button>
  </div>
</div>

<style>
  .summary {
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

  .stats {
    display: flex;
    gap: 2rem;
    justify-content: center;
    margin-bottom: 1.5rem;
  }

  .stat {
    display: flex;
    flex-direction: column;
    align-items: center;
  }

  .stat-value {
    font-size: 1.5rem;
    font-weight: 600;
    font-family: var(--font-mono);
  }

  .stat-value.kept {
    color: var(--success);
  }

  .stat-value.deleted {
    color: var(--danger);
  }

  .stat-label {
    font-size: 0.75rem;
    color: var(--text-muted);
  }

  .note {
    font-size: 0.75rem;
    color: var(--text-muted);
    margin-bottom: 1.5rem;
  }

</style>

