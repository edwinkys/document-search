<script lang="ts">
  import { supabase } from "$lib"
  import { alerts } from "$lib/stores"
  import { validEmail } from "$lib/utils/validations"
  import Input from "$lib/components/fields/input.svelte"
  import Button from "$lib/components/utils/button.svelte"
  import {
    AssemblyCluster,
    TextMining,
    VisualRecognition,
    Chat,
    CicsSystemGroup,
    Locked
  } from "carbon-icons-svelte"

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

  const features = [
    {
      icon: AssemblyCluster,
      title: "Multi-modal processing",
      description: `Understand the document's content and structure as a whole
      using Visual Language Models.`
    },
    {
      icon: TextMining,
      title: "Semantic full-text search",
      description: `Search through PDF documents based on the content semantic
      with weighted keyword matching.`
    },
    {
      icon: VisualRecognition,
      title: "Entity recognition",
      description: `Using LLM to extract important entities like dates and
      names to improve search result relevancy.`
    },
    {
      icon: Chat,
      title: "Natural language queries",
      description: `Search through PDFs with natural language just like you
      would with a search engine.`
    },
    {
      icon: CicsSystemGroup,
      title: "Multi-tenant support",
      description: `Create a single project to handle PDF documents for you,
      your users, and your user's users.`
    },
    {
      icon: Locked,
      title: "Data privacy & isolation",
      description: `Your data is isolated from other projects in a dedicated
      storage and processing environment.`
    }
  ]
</script>

<section class="section-container large">
  <div class="max-w-screen-sm mx-auto">
    <div class="flex flex-col space-y-9 justify-center">
      <div class="flex flex-col space-y-6 md:text-center">
        <h1>10x your multi-modal PDF search with AI</h1>
        <p class="text-gray-500">
          An API platform to help you process and search through multi-modal
          PDFs containing images and tables with natural language queries.
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
      Create a project and integrate the SDK in your application in less than 5
      minutes with a single API to start processing PDF documents.
    </p>
  </div>
  <div class="flex flex-col items-center justify-center space-y-3">
    <div class="code-container">
      doculens.upload(<span class="text-green-500">
        "./chase_statement_122024.pdf"
      </span>)
    </div>
    <div class="code-container">
      doculens.search(<span class="text-green-500">
        "When did I make a purchase at Walmart?"
      </span>)
    </div>
  </div>
</section>

<section id="features" class="section-container large space-y-9">
  <div class="flex flex-col space-y-4 md:text-center">
    <small class="text-gray-500">Here's why you need to use DocuLens</small>
    <h2>Features</h2>
  </div>
  <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
    {#each features as feature}
      <div class="flex flex-col bg-white rounded-lg p-4 space-y-6">
        <div class="p-3 w-fit rounded-full border border-indigo-600">
          <feature.icon size={24} class="text-indigo-600" />
        </div>
        <div class="flex flex-col space-y-3">
          <h3>{feature.title}</h3>
          <p class="text-gray-500">{feature.description}</p>
        </div>
      </div>
    {/each}
  </div>
</section>

<style lang="postcss">
  .section-container {
    @apply container mx-auto px-6 py-12;
    @apply flex flex-col;
  }

  .section-container.large {
    @apply py-24;
  }

  .code-container {
    @apply relative w-full max-w-screen-md p-6 bg-gray-900 rounded-lg;
    @apply font-mono text-gray-100 text-lg whitespace-nowrap overflow-y-auto;
    -ms-overflow-style: none;
    scrollbar-width: none;
  }

  .code-container::-webkit-scrollbar {
    display: none;
  }
</style>
