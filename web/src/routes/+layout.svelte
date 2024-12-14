<script lang="ts">
  import "../app.css"
  import "@fontsource/dm-mono"
  import "@fontsource/outfit/400.css"
  import "@fontsource/outfit/700.css"
  import { flip } from "svelte/animate"
  import { alerts } from "$lib/stores"
  import AlertCard from "$lib/components/cards/alert.svelte"
</script>

<ul class="alert-container space-y-1">
  {#each $alerts as alert (alert.id)}
    <li animate:flip={{ duration: 200 }}>
      <AlertCard {alert} />
    </li>
  {/each}
</ul>

<slot />
