<script lang="ts">
    import type { PageData } from "./$types";

    export let data: PageData;
</script>

<h1>Ping pong</h1>
{#if !data.slug}
    <p>You didn't pass a word; the server responded...</p>
{:else}
    <p>You passed <code>'{data.slug}'</code>; the server responded...</p>
{/if}

<p><code>{data.answer}</code></p>

<style></style>