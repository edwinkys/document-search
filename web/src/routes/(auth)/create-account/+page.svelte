<script lang="ts">
  import Title from "$lib/components/utils/title.svelte"
  import Input from "$lib/components/utils/input.svelte"
  import Button from "$lib/components/utils/button.svelte"

  let email: string = ""
  let password: string = ""

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

  function createAccount() {}
</script>

<Title title="Create Account" />
<div class="flex flex-col items-center text-center space-y-2">
  <h2>Welcome to Skylens</h2>
  <p class="text-gray-500">Bluesky account manager designed for teams.</p>
</div>
<div class="flex flex-col space-y-4">
  <Input
    type="email"
    label="Email"
    placeholder="name@domain.com"
    bind:value={email}
  />
  <Input
    type="password"
    label="Password"
    placeholder="••••••••"
    bind:value={password}
    autocomplete="new-password"
  />
  <ul class="bullet-list space-y-1">
    {#each Object.keys(criteria) as key}
      <li class={criteria[key].met ? "text-green-500" : ""}>
        {criteria[key].label}
      </li>
    {/each}
  </ul>
</div>
<Button content="Create account" action={createAccount} />
<p class="text-center text-gray-500">
  Have an account?
  <a href="/sign-in">Sign in</a>
</p>

<style lang="postcss">
  .bullet-list {
    @apply text-sm text-gray-500;
  }

  .bullet-list li {
    @apply list-disc list-inside ml-2;
  }
</style>
