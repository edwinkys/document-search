<script lang="ts">
  import { onMount } from "svelte"
  import { goto } from "$app/navigation"
  import { supabase } from "$lib"
  import { alerts } from "$lib/stores"
  import { validEmail } from "$lib/utils/validations"
  import Title from "$lib/components/utils/title.svelte"
  import Input from "$lib/components/fields/input.svelte"
  import NewPassword from "$lib/components/fields/new-password.svelte"
  import Button from "$lib/components/utils/button.svelte"

  let email: string = ""
  let password: string = ""

  let disabled: boolean = true
  $: disabled = !validEmail(email) || password.length < 8

  let origin: string = ""
  onMount(() => {
    origin = window?.location.origin
  })

  async function createAccount() {
    disabled = true

    const { data, error } = await supabase.auth.signUp({
      email,
      password,
      options: { emailRedirectTo: `${origin}/projects` }
    })

    if (error) {
      disabled = false
      console.error(error)
      alerts.update((alerts) => [
        ...alerts,
        {
          id: crypto.randomUUID(),
          type: "error",
          message: "Failed to create an account."
        }
      ])

      return
    }

    if (data) goto("/projects")
  }
</script>

<Title title="Create Account" />
<div class="flex flex-col items-center text-center space-y-2">
  <h2>Welcome to DocuLens</h2>
  <p class="text-gray-500">A robust search API platform for complex PDFs.</p>
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
<Button content="Create account" action={createAccount} {disabled} />
<p class="text-center text-gray-500">
  Have an account?
  <a href="/sign-in">Sign in</a>
</p>
