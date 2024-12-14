<script lang="ts">
  import { fly } from "svelte/transition"
  import { alerts } from "$lib/stores"
  import type { Alert } from "$lib/types"

  export let alert: Alert

  function dismiss() {
    alerts.update((state) => state.filter((a) => a.id !== alert.id))
  }
</script>

<button
  class="alert-card {alert.type}"
  on:click={dismiss}
  in:fly={{ x: -200 }}
  out:fly={{ x: 200 }}
>
  {alert.message}
</button>

<style lang="postcss">
  .alert-card {
    @apply flex p-4 text-sm rounded w-full;
    @apply border border-l-4;
  }

  .alert-card.success {
    @apply bg-green-50 border-green-500;
  }

  .alert-card.error {
    @apply bg-red-50 border-red-500;
  }
</style>
