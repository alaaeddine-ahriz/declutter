<script lang="ts">
    import { createEventDispatcher, onMount, onDestroy } from "svelte";
    import { settings, formatKeyCombo } from "./stores/settings";
    import type { KeyCombo, Settings } from "../types";
    import Button from "./ui/Button.svelte";
    import Kbd from "./ui/Kbd.svelte";

    const dispatch = createEventDispatcher();
    let recordingKey: keyof Settings["keybindings"] | null = null;

    function startRecording(key: keyof Settings["keybindings"]) {
        recordingKey = key;
    }

    function stopRecording() {
        recordingKey = null;
    }

    function startRecordingAction(action: string) {
        // Safe cast since we know the keys come from the settings object
        recordingKey = action as keyof Settings["keybindings"];
    }

    function handleKeydown(event: KeyboardEvent) {
        if (!recordingKey) return;

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

    function handleBack() {
        dispatch("back");
    }

    function handleReset() {
        settings.reset();
    }

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
            <h2>Keybindings</h2>
            <p class="description">
                Click on a keybinding to record a new one. Press <Kbd>Esc</Kbd> to
                cancel recording.
            </p>

            <div class="key-list">
                {#each Object.entries($settings.keybindings) as [action, combo]}
                    <button
                        class="key-item"
                        class:recording={recordingKey === action}
                        on:click={() => startRecordingAction(action)}
                    >
                        <span class="action-name">{action}</span>
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
                    >Reset to Defaults</Button
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

    .actions {
        margin-top: 24px;
        display: flex;
        justify-content: flex-end;
    }
</style>
