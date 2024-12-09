<script lang="ts">
  import { supabase } from "$lib"
  import { alerts } from "$lib/stores"
  import type { Alert } from "$lib/types"
  import Input from "$lib/components/utils/input.svelte"
  import Button from "$lib/components/utils/button.svelte"

  let email: string = ""

  async function joinWaitlist() {
    if (!email) return
    if (!email.includes("@")) return

    const { error } = await supabase.from("waitlist").insert([{ email }])
    if (!error) {
      alerts.update((alerts: Alert[]) => [
        ...alerts,
        {
          id: crypto.randomUUID(),
          message: "You've been added to the waitlist!",
          type: "success"
        }
      ])
    }

    email = ""
  }
</script>

<section class="hero-section">
  <div class="hero-container">
    <div class="flex flex-col space-y-9 justify-center">
      <div class="flex flex-col space-y-2">
        <h1>Collaborate on your Bluesky account</h1>
        <p class="text-gray-600">
          We're building a platform to help you collaborate with your team on
          growing your Bluesky business account. Join the waitlist to get early
          access!
        </p>
      </div>
      <div class="flex flex-col space-y-4">
        <Input
          id="email"
          label="Email"
          bind:value={email}
          placeholder="name@domain.com"
          type="email"
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
    @apply px-6 py-24 bg-gradient-to-r from-blue-100 to-violet-300;
  }

  @screen md {
    .hero-container {
      @apply grid-cols-2;
    }
  }

  @screen lg {
    .hero-section {
      @apply container mx-auto p-6;
    }

    .hero-container {
      @apply px-12 py-6 rounded-3xl;
    }
  }
</style>
