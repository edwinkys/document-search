<script lang="ts">
  import { flip } from "svelte/animate"
  import { alerts } from "$lib/stores"
  import "../app.css"

  // Typography.
  import "@fontsource/dm-mono"
  import "@fontsource/outfit/400.css"
  import "@fontsource/outfit/700.css"

  // Components.
  import Wordmark from "$lib/components/utils/wordmark.svelte"
  import AlertCard from "$lib/components/cards/alert.svelte"
</script>

<ul class="alert-container">
  {#each $alerts as alert (alert.id)}
    <li animate:flip={{ duration: 200 }}>
      <AlertCard {alert} />
    </li>
  {/each}
</ul>

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
