<script lang="ts">
  import Sidebar from "$lib/components/navs/sidebar.svelte"
  import { SidePanelOpenFilled } from "carbon-icons-svelte"

  let sidebarShow: boolean = true
  const openSidebar = () => (sidebarShow = true)
  const closeSidebar = () => (sidebarShow = false)
</script>

<div class="flex flex-row h-dvh">
  <Sidebar show={sidebarShow} close={closeSidebar} />
  <div class="flex flex-col w-full h-dvh overflow-y-auto">
    <header class="flex flex-none bg-gray-50">
      <div class="box-container flex flex-row items-center justify-between">
        <div class="flex flex-row items-center space-x-3">
          <button
            class="sidebar-toggle-button lg:hidden"
            on:click={openSidebar}
          >
            <svelte:component this={SidePanelOpenFilled} size={20} />
          </button>
          <h3>Dashboard</h3>
        </div>
      </div>
    </header>
    <div class="flex flex-col h-full overflow-y-auto bg-gray-100">
      <slot />
    </div>
  </div>
</div>

<style lang="postcss">
  .box-container {
    @apply container mx-auto px-6;
  }
</style>
