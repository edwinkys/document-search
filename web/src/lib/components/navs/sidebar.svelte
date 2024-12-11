<script lang="ts">
  import { fade, fly } from "svelte/transition"
  import Wordmark from "$lib/components/utils/wordmark.svelte"

  export let show: boolean = false
  export let close: () => void = () => {}
</script>

{#if show}
  <div
    class="overlay"
    transition:fade={{ duration: 100 }}
    on:click={close}
    role="none"
  ></div>
  <nav class="sidebar" transition:fly={{ x: -320, opacity: 1 }}>
    <div class="sidebar-control">
      <Wordmark />
    </div>
    <div class="sidebar-content space-y-6">
      <slot />
    </div>
  </nav>
{/if}

<style lang="postcss">
  .overlay {
    @apply fixed top-0 w-dvw h-dvh;
    @apply bg-black bg-opacity-50;
  }

  .sidebar {
    @apply fixed top-0 h-dvh;
    @apply flex flex-col flex-none bg-white;
    width: 320px;
  }

  .sidebar-control {
    @apply flex flex-row flex-none justify-between items-center;
    @apply py-4 px-6;
    height: 80px;
  }

  .sidebar-content {
    @apply flex flex-col p-4;
    @apply overflow-y-auto overflow-x-hidden;
  }

  @screen lg {
    .overlay {
      @apply hidden;
    }

    .sidebar {
      @apply static;
    }
  }
</style>
