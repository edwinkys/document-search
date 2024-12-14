<script lang="ts">
  import { goto } from "$app/navigation"
  import { alerts } from "$lib/stores"
  import { supabase } from "$lib"
  import { validEmail } from "$lib/utils/validations"
  import Title from "$lib/components/utils/title.svelte"
  import Input from "$lib/components/fields/input.svelte"
  import Button from "$lib/components/utils/button.svelte"

  let email: string = ""
  let password: string = ""

  let disabled: boolean = true
  $: disabled = !validEmail(email) || password.length < 8

  async function signIn() {
    disabled = true

    const { data, error } = await supabase.auth.signInWithPassword({
      email,
      password
    })

    if (error) {
      disabled = false
      console.error(error)
      alerts.update((alerts) => [
        ...alerts,
        {
          id: crypto.randomUUID(),
          type: "error",
          message: "Failed to sign in. Please check your credentials."
        }
      ])

      return
    }

    if (data) goto("/projects")
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
  New to DocuLens?
  <a href="/create-account">Create an account</a>
</p>
