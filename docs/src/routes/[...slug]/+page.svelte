<script lang="ts">
  import { page } from "$app/stores";
  import { slide, fade } from "svelte/transition";

  let { data } = $props();
  let mobileOpen = $state(false);

  const nav = [
    { title: "Introduction", href: "/docs" },
    { title: "Get Started", href: "/docs/get-started" },
    { title: "Configuration", href: "/docs/configuration" },
    { title: "Runtime Model", href: "/docs/runtime-model" },
    { title: "Supported Runtimes", href: "/docs/supported-runtimes" },
    { title: "Web Dashboard", href: "/docs/web-dashboard" },
  ];

  function closeMenu() {
    mobileOpen = false;
  }
</script>

<svelte:head>
  <title>{data.meta.title} | Dockless Docs</title>
  {#if data.meta.description}
    <meta name="description" content={data.meta.description} />
  {/if}
  <meta property="og:type" content="article" />
  <meta property="og:title" content={data.meta.title} />
</svelte:head>

<div class="min-h-screen bg-bg font-sans relative">
  <div
    class="max-w-7xl mx-auto px-6 py-10 grid grid-cols-1 md:grid-cols-[260px_1fr] gap-12"
  >
    <aside class="hidden md:block">
      <nav class="space-y-1">
        {#each nav as item}
          <a
            href={item.href}
            class="block px-4 py-2 rounded-md text-sm transition-colors"
            class:bg-[#f3f4f6]={$page.url.pathname === item.href}
            class:text-accent={$page.url.pathname === item.href}
            class:text-[#6b7280]={$page.url.pathname !== item.href}
            class:hover:bg-[#f3f4f6]={$page.url.pathname !== item.href}
          >
            {item.title}
          </a>
        {/each}
      </nav>
    </aside>

    <main class="max-w-3xl">
      <article class="prose prose-neutral max-w-none">
        <data.content />
      </article>
    </main>
  </div>

  <button
    class="md:hidden fixed bottom-6 right-6 z-40
           w-12 h-12 rounded-full
           bg-accent text-white
           flex items-center justify-center
           shadow-lg hover:scale-105 transition"
    onclick={() => (mobileOpen = true)}
  >
    ☰
  </button>

  {#if mobileOpen}
    <button
      class="fixed inset-0 bg-black/40 z-40 md:hidden"
      title="Toggle Sidebar"
      onclick={closeMenu}
      in:fade={{ duration: 150 }}
      out:fade={{ duration: 150 }}
    ></button>
  {/if}

  {#if mobileOpen}
    <div
      class="fixed top-0 left-0 h-full w-64 bg-white z-50 md:hidden
             shadow-xl p-6"
      in:slide={{ duration: 200, axis: "x" }}
      out:slide={{ duration: 200, axis: "x" }}
    >
      <div class="flex items-center justify-between mb-6">
        <span class="font-semibold">Docs</span>
        <button onclick={closeMenu} class="text-sm text-ink-mute">
          Close
        </button>
      </div>

      <nav class="space-y-2">
        {#each nav as item}
          <a
            href={item.href}
            onclick={closeMenu}
            class="block px-3 py-2 rounded-md text-sm transition-colors"
            class:bg-[#f3f4f6]={$page.url.pathname === item.href}
            class:text-accent={$page.url.pathname === item.href}
            class:text-[#6b7280]={$page.url.pathname !== item.href}
          >
            {item.title}
          </a>
        {/each}
      </nav>
    </div>
  {/if}
</div>
