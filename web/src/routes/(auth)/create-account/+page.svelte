<script lang="ts">
  import Title from "$lib/components/utils/title.svelte"
  import Input from "$lib/components/fields/input.svelte"
  import NewPassword from "$lib/components/fields/new-password.svelte"
  import Button from "$lib/components/utils/button.svelte"

  let email: string = ""
  let password: string = ""

  let disabled: boolean = true
  $: disabled = email.length < 3 || !email.includes("@") || password.length < 8

  async function createAccount() {
    if (!email || !password) {
      return
    }

    disabled = true
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
