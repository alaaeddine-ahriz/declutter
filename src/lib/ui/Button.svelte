<script lang="ts">
    import { createEventDispatcher } from "svelte";

    export let variant:
        | "primary"
        | "secondary"
        | "danger"
        | "ghost"
        | "success" = "primary";
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
    class="btn variant-{variant} size-{size} {fullWidth ? 'full-width' : ''}"
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
        border-radius: 8px;
        transition: all 0.2s ease;
        cursor: pointer;
        border: 1px solid transparent;
        gap: 0.5rem;
        white-space: nowrap;
    }

    .btn:disabled {
        opacity: 0.6;
        cursor: not-allowed;
    }

    .full-width {
        width: 100%;
        display: flex;
    }

    /* Sizes */
    .size-sm {
        font-size: 0.75rem;
        padding: 0.375rem 0.75rem;
    }

    .size-md {
        font-size: 0.875rem;
        padding: 0.625rem 1.25rem;
    }

    .size-lg {
        font-size: 1rem;
        padding: 0.875rem 2rem;
    }

    /* Variants */
    .variant-primary {
        background-color: var(--primary);
        color: white;
        box-shadow: 0 4px 6px -1px var(--primary-shadow, rgba(0, 0, 0, 0.1));
    }
    .variant-primary:hover:not(:disabled) {
        background-color: var(--primary-hover);
        transform: translateY(-1px);
        box-shadow: 0 10px 15px -3px var(--primary-shadow, rgba(0, 0, 0, 0.2));
    }
    .variant-primary:active:not(:disabled) {
        transform: translateY(0);
    }

    .variant-secondary {
        background-color: white;
        color: var(--text-primary);
        border-color: var(--border-color);
    }
    .variant-secondary:hover:not(:disabled) {
        background-color: var(--bg-hover);
        border-color: var(--text-muted);
    }

    .variant-success {
        background-color: #dcfce7; /* green-100 */
        color: #166534; /* green-800 */
        border: 1px solid #bbf7d0;
    }
    .variant-success:hover:not(:disabled) {
        background-color: #bbf7d0;
    }

    .variant-danger {
        background-color: #fee2e2; /* red-100 */
        color: #991b1b; /* red-800 */
        border: 1px solid #fecaca;
    }
    .variant-danger:hover:not(:disabled) {
        background-color: #fecaca;
    }

    .variant-ghost {
        background-color: transparent;
        color: var(--text-muted);
    }
    .variant-ghost:hover:not(:disabled) {
        color: var(--text-primary);
        background-color: var(--bg-hover);
    }
</style>
