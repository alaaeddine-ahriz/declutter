<script lang="ts">
    export let className: string = "";
</script>

<kbd class="kbd {className}">
    <slot />
</kbd>

<style>
    .kbd {
        display: inline-flex;
        align-items: center;
        justify-content: center;
        min-width: 18px;
        height: 18px;
        padding: 0 5px;
        font-family: var(--font-mono);
        font-size: 10px;
        font-weight: 500;
        line-height: 1;
        background-color: var(--bg-elevated);
        color: var(--text-muted);
        border: 1px solid var(--border-color);
        border-radius: 3px;
        flex-shrink: 0;
    }

    /* When inside primary button */
    :global(.variant-primary) .kbd {
        background-color: rgba(255, 255, 255, 0.15);
        border-color: rgba(255, 255, 255, 0.2);
        color: rgba(255, 255, 255, 0.9);
    }

    /* When inside danger button on hover */
    :global(.variant-danger:hover) .kbd {
        background-color: rgba(252, 165, 165, 0.15);
        border-color: rgba(252, 165, 165, 0.25);
        color: rgba(252, 165, 165, 0.9);
    }
</style>
