<script lang="ts">
  import Wordmark from "$lib/components/utils/wordmark.svelte"
</script>

<div class="box-container space-y-12">
  <Wordmark />
  <div class="flex flex-col w-full space-y-8">
    <slot />
  </div>
</div>

<style lang="postcss">
  .box-container {
    @apply flex flex-col items-center;
    @apply w-full mx-auto px-6 py-24;
    max-width: 480px;
  }
</style>
