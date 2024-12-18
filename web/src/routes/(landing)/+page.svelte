<script lang="ts">
  import { supabase } from "$lib"
  import { alerts } from "$lib/stores"
  import { validEmail } from "$lib/utils/validations"
  import Input from "$lib/components/fields/input.svelte"
  import Button from "$lib/components/utils/button.svelte"

  let email: string = ""

  async function joinWaitlist() {
    if (!validEmail(email)) return

    email = email.trim().toLowerCase()

    const { error } = await supabase.from("waitlist").insert({ email })
    if (error) console.error(error)

    email = ""

    alerts.update((alerts) => [
      ...alerts,
      {
        id: crypto.randomUUID(),
        type: "success",
        message: "Thank you for joining the waitlist!"
      }
    ])
  }
</script>

<section class="section-container">
  <div class="max-w-screen-sm mx-auto py-12">
    <div class="flex flex-col space-y-9 justify-center">
      <div class="flex flex-col space-y-6 md:text-center">
        <h1>Extract 10x more insight from PDFs</h1>
        <p class="text-gray-500">
          We're building an API platform to help you process complex multi-modal
          PDFs containing images and tables into structured data that your AI
          agents can use.
        </p>
      </div>
      <div class="flex flex-col space-y-4">
        <Input
          label="Email"
          placeholder="name@domain.com"
          type="email"
          bind:value={email}
        />
        <Button content="Join waitlist" action={joinWaitlist} />
        <small class="text-center text-gray-500">
          We'll never share your data with anyone!
        </small>
      </div>
    </div>
  </div>
</section>

<section class="section-container space-y-9">
  <div class="flex flex-col space-y-3 items-center md:text-center">
    <h2>Get started in minutes</h2>
    <p class="max-w-screen-sm text-gray-500">
      Create a project and integrate the API or SDK in your application in less
      than 5 minutes with a single API to start processing PDF documents.
    </p>
  </div>
  <div class="flex items-center justify-center">
    <div class="code-container max-w-screen-md">
      doculens.upload(<span class="text-green-500">
        "./chase_statement_122024.pdf"
      </span>)
    </div>
  </div>
</section>

<style lang="postcss">
  .section-container {
    @apply container mx-auto px-6 py-12;
    @apply flex flex-col;
  }

  .code-container {
    @apply relative w-full p-6 bg-gray-900 text-gray-100 rounded-lg;
    @apply font-mono text-lg whitespace-nowrap overflow-y-auto;
    -ms-overflow-style: none;
    scrollbar-width: none;
  }

  .code-container::-webkit-scrollbar {
    display: none;
  }
</style>
