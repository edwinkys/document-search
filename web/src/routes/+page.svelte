<section class="hero-section">
  <div class="hero-container">
    <div class="flex flex-col space-y-4 justify-center">
      <h1>Find videos your users are looking for</h1>
      <p class="text-gray-600">10x your video search experience with AI</p>
    </div>
    <div class="hidden md:flex">
      <img src="/imgs/hero-illustration.png" alt="Illustration" />
    </div>
  </div>
</section>

<style lang="postcss">
  .hero-section {
    @apply container mx-auto;
  }

  .hero-container {
    @apply grid grid-cols-1 gap-6;
    @apply px-6 py-32 bg-gradient-to-r from-indigo-100 to-violet-300;
  }

  @screen md {
    .hero-section {
      @apply p-6;
    }

    .hero-container {
      @apply py-6 rounded-xl grid-cols-2;
    }
  }
</style>
