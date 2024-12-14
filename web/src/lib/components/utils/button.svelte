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
    @apply flex flex-row flex-none items-center justify-center;
    @apply px-6 py-3 rounded truncate;
  }

  .button-container.primary {
    @apply bg-indigo-700 text-white;
  }

  .button-container.primary:hover {
    @apply bg-indigo-600;
  }

  .button-container.primary:disabled {
    @apply bg-indigo-700 bg-opacity-50;
  }

  .button-container.secondary {
    @apply text-indigo-700 border border-indigo-700;
  }
</style>
