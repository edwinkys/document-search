<script lang="ts">
  import { goto } from "$app/navigation"
  import Wordmark from "$lib/components/utils/wordmark.svelte"
  import Sidebar from "$lib/components/navs/sidebar.svelte"
  import Button from "$lib/components/utils/button.svelte"
  import { SidePanelOpenFilled } from "carbon-icons-svelte"

  const headerLinks = [
    { name: "Features", href: "#features" },
    { name: "Pricing", href: "#pricing" }
  ]

  const headerButtons = [
    {
      content: "Join waitlist",
      type: "primary",
      action: () => goto("/")
    }
  ]

  const footerLinks = [
    { name: "Documentation", href: "https://docs.doculens.dev" },
    { name: "Pricing", href: "#pricing" },
    { name: "Support", href: "mailto: edwin@doculens.dev" }
  ]

  let sidebarShow = false
  const openSidebar = () => (sidebarShow = true)
  const closeSidebar = () => (sidebarShow = false)
</script>

<div class="lg:hidden">
  <Sidebar show={sidebarShow} close={closeSidebar}>
    <div class="flex flex-col space-y-1">
      {#each headerLinks as link}
        <a href={link.href} class="p-3 rounded hover:bg-gray-100">
          {link.name}
        </a>
      {/each}
    </div>
    <div class="flex flex-col flex-none space-y-2">
      {#each headerButtons as button}
        <Button
          content={button.content}
          type={button?.type as "primary" | "secondary"}
          action={button.action}
        />
      {/each}
    </div>
  </Sidebar>
</div>

<div class="w-full h-[80px]"></div>
<header class="fixed top-0">
  <div class="header-container">
    <Wordmark />
    <div class="desktop-menu space-x-2">
      {#each headerLinks as link}
        <a href={link.href} class="py-2 px-3 rounded">{link.name}</a>
      {/each}
    </div>
    <div class="desktop-menu space-x-4">
      {#each headerButtons as button}
        <Button
          content={button.content}
          type={button.type as "primary" | "secondary"}
          action={button.action}
        />
      {/each}
    </div>
    <button class="sidebar-toggle-button lg:hidden" on:click={openSidebar}>
      <SidePanelOpenFilled size={20} />
    </button>
  </div>
</header>

<slot />

<footer class="container mx-auto px-6 py-24">
  <div class="flex flex-col items-center space-y-9">
    <Wordmark />
    <div class="flex flex-col md:flex-row text-center">
      {#each footerLinks as link}
        <a
          href={link.href}
          target={link.href.startsWith("http") ? "_blank" : "_self"}
          class="py-3 px-9"
        >
          {link.name}
        </a>
      {/each}
    </div>
  </div>
</footer>

<style lang="postcss">
  .header-container {
    @apply flex flex-row items-center justify-between;
    @apply container mx-auto px-6;
  }

  .desktop-menu {
    @apply hidden;
  }

  @screen lg {
    .desktop-menu {
      @apply flex items-center;
    }
  }
</style>
