<script lang="ts">
    import { onMount, onDestroy } from "svelte";
    import { settings, formatKeyCombo } from "./stores/settings";
    import type { KeyCombo, Settings, TriageMode } from "../types";
    import Button from "./ui/Button.svelte";
    import Kbd from "./ui/Kbd.svelte";
    import Badge from "./ui/Badge.svelte";

    let recordingKey: keyof Settings["keybindings"] | null = null;

    function stopRecording() {
        recordingKey = null;
    }

    function startRecordingAction(action: string) {
        // Safe cast since we know the keys come from the settings object
        recordingKey = action as keyof Settings["keybindings"];
    }

    function handleKeydown(event: KeyboardEvent) {
        if (!recordingKey) {
            if (event.key === "R" && event.shiftKey) {
                event.preventDefault();
                handleReset();
            }
            return;
        }

        event.preventDefault();
        event.stopPropagation();

        // Ignore modifier-only key presses
        if (["Control", "Alt", "Meta", "Shift"].includes(event.key)) return;

        if (event.key === "Escape") {
            stopRecording();
            return;
        }

        const combo: KeyCombo = {
            key: event.key,
            modifiers: {
                ctrl: event.ctrlKey,
                alt: event.altKey,
                meta: event.metaKey,
                shift: event.shiftKey,
            },
        };

        settings.updateKeybinding(recordingKey, combo);
        stopRecording();
    }

    function handleReset() {
        settings.reset();
    }

    function handleModeChange(mode: TriageMode) {
        settings.setMode(mode);
    }

    $: displayedKeybindings =
        $settings.mode === "classic"
            ? [
                  {
                      action: "keep",
                      label: "Keep",
                      combo: $settings.keybindings.keep,
                  },
                  {
                      action: "delete",
                      label: "Delete",
                      combo: $settings.keybindings.delete,
                  },
                  {
                      action: "undo",
                      label: "Undo",
                      combo: $settings.keybindings.undo,
                  },
                  {
                      action: "preview",
                      label: "Preview",
                      combo: $settings.keybindings.preview,
                  },
              ]
            : [
                  {
                      action: "exploreNext",
                      label: "Next",
                      combo: $settings.keybindings.exploreNext,
                  },
                  {
                      action: "explorePrevious",
                      label: "Previous",
                      combo: $settings.keybindings.explorePrevious,
                  },
                  {
                      action: "exploreDelete",
                      label: "Delete",
                      combo: $settings.keybindings.exploreDelete,
                  },
                  {
                      action: "preview",
                      label: "Preview",
                      combo: $settings.keybindings.preview,
                  },
              ];

    const modeDescription = {
        classic:
            "Quickly sort files by keeping or deleting. Ideal for fast cleanup.",
        explore:
            "Navigate freely and explicitly mark delete. Good for reviewing content.",
    };

    onMount(() => {
        window.addEventListener("keydown", handleKeydown);
    });

    onDestroy(() => {
        window.removeEventListener("keydown", handleKeydown);
    });
</script>

<div class="settings-page">
    <div class="content">
        <div class="section">
            <h1>Settings</h1>
            <h2>Mode</h2>
            <div class="mode-selector">
                <button
                    class="mode-option"
                    class:selected={$settings.mode === "classic"}
                    on:click={() => handleModeChange("classic")}
                >
                    <div class="mode-header">
                        <span class="mode-title">Classic Triage</span>
                        {#if $settings.mode === "classic"}<Badge
                                variant="accent">Active</Badge
                            >{/if}
                    </div>
                    <p class="mode-desc">{modeDescription.classic}</p>
                </button>

                <button
                    class="mode-option"
                    class:selected={$settings.mode === "explore"}
                    on:click={() => handleModeChange("explore")}
                >
                    <div class="mode-header">
                        <span class="mode-title">Explore Mode</span>
                        {#if $settings.mode === "explore"}<Badge
                                variant="accent">Active</Badge
                            >{/if}
                    </div>
                    <p class="mode-desc">{modeDescription.explore}</p>
                </button>
            </div>
        </div>

        <div class="section keybindings-section">
            <h2>Keybindings</h2>
            <p class="description">
                Click on a keybinding to record a new one. Press <Kbd>Esc</Kbd> to
                cancel recording. Bindings are auto-saved.
            </p>

            <div class="key-list">
                {#each displayedKeybindings as { action, label, combo }}
                    <button
                        class="key-item"
                        class:recording={recordingKey === action}
                        on:click={() => startRecordingAction(action)}
                    >
                        <span class="action-name">{label}</span>
                        <div class="key-display">
                            {#if recordingKey === action}
                                <span class="recording-text">Press keys...</span
                                >
                            {:else}
                                <Kbd>{formatKeyCombo(combo)}</Kbd>
                            {/if}
                        </div>
                    </button>
                {/each}
            </div>

            <div class="actions">
                <Button variant="outline" on:click={handleReset}
                    >Reset to Defaults <Kbd>⇧ + R</Kbd></Button
                >
            </div>
        </div>
    </div>
</div>

<style>
    .settings-page {
        display: flex;
        flex-direction: column;
        height: 100%;
        color: var(--text-primary);
    }

    .header {
        padding: 0 16px;
        display: flex;
        align-items: center;
        justify-content: center;
        height: 48px;
        position: relative;
        border-bottom: 1px solid var(--border-subtle);
    }

    .header h1 {
        font-size: 14px;
        font-weight: 500;
    }

    .back-btn {
        position: absolute;
        left: 12px;
        top: 50%;
        transform: translateY(-50%);
    }

    .content {
        flex: 1;
        padding: 32px;
        max-width: 600px;
        margin: 0 auto;
        width: 100%;
    }

    .section {
        display: flex;
        flex-direction: column;
        gap: 16px;
    }

    .keybindings-section {
        gap: 8px; /* Reduce gap between title and description */
        margin-top: 16px; /* Add extra spacing before this section */
    }

    h2 {
        font-size: 18px;
        font-weight: 600;
        margin: 0;
    }

    .description {
        font-size: 14px;
        color: var(--text-muted);
        margin: 0;
    }

    .key-list {
        display: flex;
        flex-direction: column;

        background: var(--bg-secondary);
        padding: 4px;
        border-radius: var(--radius-md);
        border: 1px solid var(--border-subtle);
    }

    .key-item {
        display: flex;
        justify-content: space-between;
        align-items: center;
        padding: 12px 16px;
        width: 100%;
        background: none;
        border: none;
        cursor: pointer;
        text-align: left;
        border-radius: var(--radius-sm);
        transition: background-color var(--transition-fast);
        color: inherit;
    }

    .key-item:hover {
        background-color: var(--bg-tertiary);
    }

    .key-item.recording {
        background-color: var(--bg-accent);
        color: var(--text-on-accent);
    }

    .key-item:not(:last-child) {
        border-bottom: 1px solid var(--border-subtle);
    }

    .action-name {
        font-size: 14px;
        text-transform: capitalize;
        font-weight: 500;
    }

    .key-display {
        display: flex;
        align-items: center;
    }

    .recording-text {
        font-size: 13px;
        font-weight: 500;
    }

    .mode-selector {
        display: flex;
        flex-direction: column;
        gap: 12px;
    }

    .mode-option {
        display: flex;
        flex-direction: column;
        gap: 4px;
        padding: 16px;
        background: var(--bg-secondary);
        border: 1px solid var(--border-subtle);
        border-radius: var(--radius-md);
        text-align: left;
        cursor: pointer;
        transition: all var(--transition-fast);
    }

    .mode-option:hover {
        border-color: var(--border-hover);
        background: var(--bg-tertiary);
    }

    .mode-option.selected {
        border-color: var(--primary);
        box-shadow: 0 0 0 1px var(--primary);
    }

    .mode-header {
        display: flex;
        justify-content: space-between;
        align-items: center;
    }

    .mode-title {
        font-weight: 500;
        font-size: 15px;
    }

    .mode-desc {
        font-size: 13px;
        color: var(--text-muted);
        margin: 0;
        line-height: 1.4;
    }

    .group-header {
        font-size: 12px;
        font-weight: 600;
        text-transform: uppercase;
        letter-spacing: 0.05em;
        color: var(--text-muted);
        margin-top: 16px;
        margin-bottom: 8px;
    }

    .actions {
        margin-top: 24px;
        display: flex;
        justify-content: flex-end;
    }
</style>
