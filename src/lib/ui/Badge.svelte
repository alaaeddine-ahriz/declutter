<script lang="ts">
    export let variant: "neutral" | "success" | "danger" | "warning" | "info" =
        "neutral";
</script>

<span class="badge variant-{variant}">
    <slot />
</span>

<style>
    .badge {
        display: inline-flex;
        align-items: center;
        padding: 0.25rem 0.625rem;
        font-size: 0.75rem;
        font-weight: 500;
        line-height: 1;
        border-radius: 9999px;
        font-family: var(--font-mono);
    }

    .variant-neutral {
        background-color: var(--bg-secondary);
        color: var(--text-secondary);
        border: 1px solid var(--border-color);
    }

    .variant-success {
        background-color: #dcfce7; /* green-100 */
        color: #166534; /* green-800 */
        border: 1px solid #bbf7d0;
    }

    .variant-danger {
        background-color: #fee2e2; /* red-100 */
        color: #991b1b; /* red-800 */
        border: 1px solid #fecaca;
    }

    .variant-warning {
        background-color: #fef3c7; /* amber-100 */
        color: #92400e; /* amber-800 */
        border: 1px solid #fde68a;
    }

    .variant-info {
        background-color: #e0f2fe; /* sky-100 */
        color: #075985; /* sky-800 */
        border: 1px solid #bae6fd;
    }
</style>
