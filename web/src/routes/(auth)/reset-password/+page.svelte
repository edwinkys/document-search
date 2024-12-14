<script lang="ts">
  import { onMount } from "svelte"
  import { supabase } from "$lib"
  import { alerts } from "$lib/stores"
  import { validEmail } from "$lib/utils/validations"
  import Title from "$lib/components/utils/title.svelte"
  import Input from "$lib/components/fields/input.svelte"
  import Button from "$lib/components/utils/button.svelte"

  let email: string = ""
  let disabled: boolean = true
  $: disabled = !validEmail(email)

  let origin: string = ""
  onMount(() => {
    origin = window?.location.origin
  })

  async function resetPassword() {
    disabled = true
    let { error } = await supabase.auth.resetPasswordForEmail(email, {
      redirectTo: `${origin}/settings`
    })

    if (error) {
      disabled = false
      console.error(error)
      alerts.update((alerts) => [
        ...alerts,
        {
          id: crypto.randomUUID(),
          type: "error",
          message: "Failed to reset your password."
        }
      ])

      return
    }

    alerts.update((alerts) => [
      ...alerts,
      {
        id: crypto.randomUUID(),
        type: "success",
        message: "Please check your email to continue."
      }
    ])
  }
</script>

<Title title="Reset Password" />
<div class="flex flex-col items-center text-center space-y-2">
  <h2>Forgot password?</h2>
  <p class="text-gray-500">We'll send a link to reset your password.</p>
</div>
<div class="flex flex-col space-y-4">
  <Input
    type="email"
    label="Email"
    placeholder="name@domain.com"
    bind:value={email}
  />
  <Button content="Reset password" action={resetPassword} {disabled} />
</div>
