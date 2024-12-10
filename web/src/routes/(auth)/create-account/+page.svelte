<script lang="ts">
  import Title from "$lib/components/utils/title.svelte"
  import Input from "$lib/components/fields/input.svelte"
  import NewPassword from "$lib/components/fields/new-password.svelte"
  import Button from "$lib/components/utils/button.svelte"

  let email: string = ""
  let password: string = ""

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
  <NewPassword bind:password />
</div>
<Button content="Create account" action={createAccount} />
<p class="text-center text-gray-500">
  Have an account?
  <a href="/sign-in">Sign in</a>
</p>
