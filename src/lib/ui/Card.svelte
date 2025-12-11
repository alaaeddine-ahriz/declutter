<script lang="ts">
    export let padding: "none" | "sm" | "md" | "lg" = "md";
</script>

<div class="card padding-{padding}">
    <slot />
</div>

<style>
    .card {
        background-color: var(--bg-secondary);
        border: 1px solid var(--border-color);
        border-radius: var(--radius-lg);
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .padding-none {
        padding: 0;
    }

    .padding-sm {
        padding: 12px;
    }

    .padding-md {
        padding: 16px;
    }

    .padding-lg {
        padding: 24px;
    }
</style>
