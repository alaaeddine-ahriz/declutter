<script lang="ts">
    export let padding: "none" | "sm" | "md" | "lg" = "md";
</script>

<div class="card padding-{padding}">
    <slot />
</div>

<style>
    .card {
        background-color: white;
        border: 1px solid var(--border-color);
        border-radius: 12px;
        box-shadow: 0 1px 3px 0 rgba(0, 0, 0, 0.05); /* Subtle shadow */
        overflow: hidden;
        display: flex;
        flex-direction: column;
    }

    .padding-none {
        padding: 0;
    }

    .padding-sm {
        padding: 0.75rem;
    }

    .padding-md {
        padding: 1.5rem;
    }

    .padding-lg {
        padding: 2rem;
    }
</style>
