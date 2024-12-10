<script lang="ts">
  import Input from "$lib/components/fields/input.svelte"

  export let password: string = ""

  type PasswordRequirement = {
    label: string
    met: boolean
  }

  let criteria: { [name: string]: PasswordRequirement } = {
    length: { label: "At least 8 characters", met: false },
    uppercase: { label: "Uppercase letters", met: false },
    lowercase: { label: "Lowercase letters", met: false },
    numbers: { label: "Numbers", met: false }
  }

  $: {
    criteria.length.met = password.length >= 8
    criteria.uppercase.met = /[A-Z]/.test(password)
    criteria.lowercase.met = /[a-z]/.test(password)
    criteria.numbers.met = /[0-9]/.test(password)
  }
</script>

<div class="flex flex-col space-y-4">
  <Input
    type="password"
    label="Password"
    placeholder="••••••••"
    bind:value={password}
    autocomplete="new-password"
  />
  <ul class="bullet-list space-y-1 px-2">
    {#each Object.keys(criteria) as key}
      <li class={criteria[key].met ? "text-green-500" : ""}>
        {criteria[key].label}
      </li>
    {/each}
  </ul>
</div>

<style lang="postcss">
  .bullet-list {
    @apply text-sm text-gray-500;
  }

  .bullet-list li {
    @apply list-disc list-inside;
  }
</style>
