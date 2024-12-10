<script lang="ts">
  import Title from "$lib/components/utils/title.svelte"
  import Input from "$lib/components/fields/input.svelte"
  import Button from "$lib/components/utils/button.svelte"

  let email: string = ""

  let disabled: boolean = true
  $: disabled = email.length < 3

  async function resetPassword() {
    if (!email) {
      return
    }

    disabled = true
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
