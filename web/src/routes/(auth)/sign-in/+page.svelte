<script lang="ts">
  import Title from "$lib/components/utils/title.svelte"
  import Input from "$lib/components/fields/input.svelte"
  import Button from "$lib/components/utils/button.svelte"

  let email: string = ""
  let password: string = ""

  let disabled: boolean = true
  $: disabled = email.length < 3 || password.length < 8

  async function signIn() {
    if (!email || !password) {
      return
    }

    disabled = true
  }
</script>

<Title title="Sign In" />
<div class="flex flex-col items-center text-center space-y-2">
  <h2>Welcome back!</h2>
  <p class="text-gray-500">Sign in to access your account dashboard.</p>
</div>
<div class="flex flex-col space-y-4">
  <Input
    type="email"
    label="Email"
    placeholder="name@domain.com"
    bind:value={email}
  />
  <div class="flex flex-col space-y-1">
    <Input
      type="password"
      label="Password"
      placeholder="••••••••"
      bind:value={password}
      autocomplete="current-password"
    />
    <p class="text-right text-sm text-gray-500">
      <a href="/reset-password">Forgot password?</a>
    </p>
  </div>
</div>
<Button content="Sign in" action={signIn} {disabled} />
<p class="text-center text-gray-500">
  New to Skylens?
  <a href="/create-account">Create an account</a>
</p>
