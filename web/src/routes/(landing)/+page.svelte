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

<section class="hero-section">
  <div class="hero-container">
    <div class="flex flex-col space-y-9 justify-center">
      <div class="flex flex-col space-y-6">
        <h1>10x your PDF search with AI</h1>
        <p class="text-gray-500">
          We're building an API platform to help you process and search complex
          PDFs containing images and tables with natural language queries with
          ease.
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
      </div>
    </div>
    <div class="hidden md:flex">
      <img
        src="/imgs/hero-illustration.png"
        alt="Illustration"
        class="object-contain"
      />
    </div>
  </div>
</section>

<style lang="postcss">
  .hero-container {
    @apply grid grid-cols-1 gap-6;
    @apply px-6 py-24 bg-indigo-100;
  }

  @screen md {
    .hero-container {
      @apply grid-cols-2;
    }
  }

  @screen lg {
    .hero-section {
      @apply container mx-auto px-6 py-12;
    }

    .hero-container {
      @apply px-12 py-6 rounded-3xl;
    }
  }
</style>
