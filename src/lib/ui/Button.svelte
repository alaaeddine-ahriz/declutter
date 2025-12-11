<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let variant: "primary" | "secondary" | "outline" | "ghost" | "danger" = "primary";
    export let size: "sm" | "md" | "lg" = "md";
    export let disabled = false;
    export let type: "button" | "submit" | "reset" = "button";
    export let fullWidth = false;

    const dispatch = createEventDispatcher<{ click: MouseEvent }>();

    function handleClick(event: MouseEvent) {
        if (!disabled) {
            dispatch("click", event);
        }
    }
</script>

<button
    {type}
    class="btn variant-{variant} size-{size}"
    class:full-width={fullWidth}
    {disabled}
    on:click={handleClick}
    {...$$restProps}
>
    <slot />
</button>

<style>
    .btn {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        font-family: var(--font-sans);
        font-weight: 500;
        border-radius: var(--radius-md);
        transition: background-color var(--transition-fast), 
                    color var(--transition-fast),
                    border-color var(--transition-fast);
        cursor: pointer;
        border: 1px solid transparent;
        gap: 6px;
        white-space: nowrap;
    }

    .btn:disabled {
        opacity: 0.4;
        cursor: not-allowed;
    }

    .full-width {
        width: 100%;
    }

    /* Sizes */
    .size-sm {
        font-size: 12px;
        padding: 6px 10px;
        border-radius: var(--radius-sm);
    }

    .size-md {
        font-size: 13px;
        padding: 8px 14px;
    }

    .size-lg {
        font-size: 14px;
        padding: 10px 20px;
    }

    /* Primary - Purple accent */
    .variant-primary {
        background-color: var(--accent);
        color: white;
    }

    .variant-primary:hover:not(:disabled) {
        background-color: var(--accent-hover);
    }

    .variant-primary:active:not(:disabled) {
        background-color: var(--accent-muted);
    }

    /* Secondary - Subtle background */
    .variant-secondary {
        background-color: var(--bg-tertiary);
        color: var(--text-primary);
        border-color: var(--border-color);
    }

    .variant-secondary:hover:not(:disabled) {
        background-color: var(--bg-hover);
        border-color: var(--text-muted);
    }

    /* Outline - Border only, transparent bg */
    .variant-outline {
        background-color: transparent;
        color: var(--text-primary);
        border-color: var(--border-color);
    }

    .variant-outline:hover:not(:disabled) {
        background-color: var(--bg-tertiary);
        border-color: var(--text-muted);
    }

    /* Ghost - Transparent */
    .variant-ghost {
        background-color: transparent;
        color: var(--text-secondary);
    }

    .variant-ghost:hover:not(:disabled) {
        color: var(--text-primary);
        background-color: var(--bg-hover);
    }

    /* Danger - Muted red for destructive actions */
    .variant-danger {
        background-color: var(--bg-tertiary);
        color: var(--text-primary);
        border-color: var(--border-color);
    }

    .variant-danger:hover:not(:disabled) {
        background-color: #371520;
        border-color: #5c2333;
        color: #fca5a5;
    }
</style>
