<script lang="ts">
  import "../app.css"

  // Typography.
  import "@fontsource/dm-mono"
  import "@fontsource/outfit/400.css"
  import "@fontsource/outfit/700.css"

  // Components.
  import Wordmark from "$lib/components/utils/wordmark.svelte"
</script>

<header>
  <div class="header-container">
    <Wordmark />
  </div>
</header>

<slot />

<style lang="postcss">
  .header-container {
    @apply flex flex-row items-center justify-between;
    @apply container mx-auto px-6;
  }
</style>
