<script lang="ts">
  import { goto } from "$app/navigation"
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
    Locked,
    CheckmarkOutline,
    Db2Database,
    Api_1,
    FileStorage,
    Process
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

  const pricing = [
    {
      name: "Founding",
      price: "$500/mo",
      description: "Exclusive pricing for early adopters",
      features: [
        "Unlimited documents",
        "Unlimited searches",
        "Bring your own LLM",
        "Deploy to your own cloud",
        "Project consultation",
        "24/7 technical support"
      ],
      button: {
        content: "Join waitlist",
        type: "primary",
        action: () => goto("/")
      }
    }
  ]

  const components = [
    { icon: Db2Database, title: "Postgres instance" },
    { icon: Api_1, title: "API server" },
    { icon: Process, title: "Extractor worker" },
    { icon: FileStorage, title: "Storage bucket" }
  ]
</script>

<section class="section-container">
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

<section id="features" class="section-container space-y-9">
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

<section id="pricing" class="section-container">
  <div class="grid grid-cols-1 md:grid-cols-3 gap-9">
    <h2>Pricing</h2>
    <div class="flex flex-col space-y-4 md:col-span-2">
      {#each pricing as plan}
        <div class="pricing-card space-y-6">
          <div class="flex flex-col space-y-1">
            <div class="flex flex-row justify-between">
              <h3>{plan.name}</h3>
              <h3>{plan.price}</h3>
            </div>
            <small class="text-gray-500">{plan.description} </small>
          </div>
          <div class="grid grid-cols-1 lg:grid-cols-2 gap-3">
            {#each plan.features as feature}
              <div class="flex flex-row space-x-2">
                <span class="flex-none py-[2px] text-green-500">
                  <CheckmarkOutline size={20} />
                </span>
                <p>{feature}</p>
              </div>
            {/each}
          </div>
          <Button
            content={plan.button.content}
            type={plan.button.type as "primary" | "secondary"}
            action={plan.button.action}
          />
        </div>
      {/each}
    </div>
  </div>
</section>

<section class="section-container space-y-12">
  <h2 class="md:text-center">How we extract PDFs</h2>
  <div class="flex items-center justify-center">
    <img
      src="/images/pdf-extraction.png"
      alt="Extraction Diagram"
      class="lg:max-w-screen-md"
    />
  </div>
  <div class="flex flex-col items-center">
    <p class="max-w-screen-sm text-gray-500 md:text-center md:text-lg">
      We employ multiple layers of extractions to ensure that we capture as much
      information as possible from a PDF. This process includes extracting
      features from the document as an image and as a text. We then store the
      extracted embeddings in a vector index for quick retrieval.
    </p>
  </div>
</section>

<section class="section-container space-y-9">
  <h2 class="md:text-center">Each project includes</h2>
  <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
    {#each components as component}
      <div class="flex items-center p-4 space-x-4 bg-white rounded-lg">
        <div class="flex p-2 bg-indigo-100 rounded-lg">
          <component.icon size={24} class="text-indigo-600" />
        </div>
        <p class="text-xl">{component.title}</p>
      </div>
    {/each}
  </div>
  <p class="text-lg text-gray-500 md:text-center">
    All dedicated only for your project!
  </p>
</section>

<style lang="postcss">
  .section-container {
    @apply container mx-auto px-6 py-24;
    @apply flex flex-col;
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

  .pricing-card {
    @apply flex flex-col p-6 rounded-lg bg-white;
  }
</style>
