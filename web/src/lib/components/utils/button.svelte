<script lang="ts">
  export let content: string
  export let type: "primary" | "secondary" = "primary"
  export let action: () => void = () => {}

  const buttonType = {
    primary: "bg-blue-500 text-white hover:bg-blue-600",
    secondary: "text-blue-500 border border-blue-500"
  }
</script>

<button class="button-container space-x-2 {buttonType[type]}" on:click={action}>
  {content}
</button>

<style lang="postcss">
  .button-container {
    @apply flex flex-row items-center justify-center;
    @apply px-6 py-3 rounded w-full;
  }
</style>
