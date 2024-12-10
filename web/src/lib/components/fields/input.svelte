<script lang="ts">
  import type { FullAutoFill } from "svelte/elements"

  export let label: string
  export let placeholder: string
  export let value: string
  export let type: "text" | "email" | "password" = "text"
  export let autocomplete: FullAutoFill = "on"

  // Generate an ID based on the label.
  const id = label.toLowerCase().replaceAll(" ", "-")
</script>

<label for={id} class="input-container border-gray-500">
  <span class="text-black text-opacity-50 text-sm">{label}</span>
  <input
    class="bg-transparent outline-none"
    bind:value
    name={id}
    {id}
    {placeholder}
    {type}
    {autocomplete}
  />
</label>

<style lang="postcss">
  .input-container {
    @apply flex flex-col px-3 py-2;
    @apply border border-black border-opacity-25 rounded;
  }

  .input-container:focus-within {
    @apply outline outline-blue-500;
  }
</style>
