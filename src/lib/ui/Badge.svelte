<script lang="ts">
    export let variant: "neutral" | "accent" | "muted" = "neutral";
</script>

<span class="badge variant-{variant}">
    <slot />
</span>

<style>
    .badge {
        display: inline-flex;
        align-items: center;
        padding: 3px 8px;
        font-size: 11px;
        font-weight: 500;
        line-height: 1;
        border-radius: var(--radius-sm);
        font-family: var(--font-mono);
    }

    .variant-neutral {
        background-color: var(--bg-tertiary);
        color: var(--text-secondary);
        border: 1px solid var(--border-color);
    }

    .variant-accent {
        background-color: var(--accent-subtle);
        color: var(--accent-text);
        border: 1px solid transparent;
    }

    .variant-muted {
        background-color: transparent;
        color: var(--text-muted);
        border: 1px solid var(--border-color);
    }
</style>
