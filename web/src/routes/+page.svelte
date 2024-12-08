<script lang="ts">
  import Input from "$lib/components/utils/input.svelte"
  import Button from "$lib/components/utils/button.svelte"

  let email: string = ""

  function joinWaitlist() {}
</script>

<section class="hero-section">
  <div class="hero-container">
    <div class="flex flex-col space-y-9 justify-center">
      <div class="flex flex-col space-y-2">
        <h1>10x your video search with AI</h1>
        <p class="text-gray-600">
          We're creating an API platform to build a powerful video search system
          by understanding the meaning behind their frames and dialogues.
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
    @apply px-6 py-24 bg-gradient-to-r from-indigo-100 to-violet-300;
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
