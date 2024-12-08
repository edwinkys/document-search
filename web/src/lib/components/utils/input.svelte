<script lang="ts">
  export let id: string
  export let label: string
  export let placeholder: string
  export let value: string
  export let type: string = "text"
</script>

<label for={id} class="input-container border-gray-500">
  <span class="text-black text-opacity-50 text-sm">{label}</span>
  <input
    class="bg-transparent outline-none"
    bind:value
    name={id}
    {placeholder}
    {type}
  />
</label>

<style lang="postcss">
  .input-container {
    @apply flex flex-col px-3 py-2;
    @apply border border-black border-opacity-25 rounded-lg;
  }

  .input-container:focus-within {
    @apply outline outline-indigo-700;
  }
</style>
