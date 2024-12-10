<script lang="ts">
  export let content: string
  export let type: "primary" | "secondary" = "primary"
  export let action: () => void = () => {}
  export let disabled: boolean = false
</script>

<button class="button-container {type} space-x-2" on:click={action} {disabled}>
  {content}
</button>

<style lang="postcss">
  .button-container {
    @apply flex flex-row items-center justify-center;
    @apply px-6 py-3 rounded w-full;
  }

  .button-container.primary {
    @apply bg-blue-500 text-white;
  }

  .button-container.primary:hover {
    @apply bg-blue-600;
  }

  .button-container.primary:disabled {
    @apply bg-blue-400;
  }

  .button-container.secondary {
    @apply text-blue-500 border border-blue-500;
  }
</style>
