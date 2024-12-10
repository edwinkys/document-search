<script lang="ts">
  import { goto } from "$app/navigation"
  import Button from "$lib/components/utils/button.svelte"
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
      <Button content="Get started" action={() => goto("/create-account")} />
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
