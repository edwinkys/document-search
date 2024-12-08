<script lang="ts">
  export let content: string
  export let type: "primary" | "secondary" = "primary"
  export let action: () => void = () => {}

  const buttonType = {
    primary: "bg-indigo-700 text-white hover:bg-indigo-600",
    secondary: "text-indigo-700 border border-indigo-700"
  }
</script>

<button class="button-container {buttonType[type]}" on:click={action}>
  {content}
</button>

<style lang="postcss">
  .button-container {
    @apply flex flex-row items-center justify-center space-x-2;
    @apply px-6 py-3 rounded-lg;
  }
</style>
