<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { store } from "$lib/services.svelte";
  import NodeStats from "$lib/components/NodeStats.svelte";
  import ServiceCard from "$lib/components/ServiceCard.svelte";
  import { goto } from "$app/navigation";
  import TopBar from "$lib/components/TopBar.svelte";
  import { InboxIcon } from "lucide-svelte";

  const services = $derived(store.merged);
  const health = $derived(store.health);
  const loading = $derived(store.loading);

  onMount(() => {
    store.startPolling(3000);
  });

  onDestroy(() => {
    store.stopPolling();
  });
</script>

<TopBar />
<main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
  {#if loading}
    <div class="w-full space-y-6">
      <div class="card w-full bg-surface-50-950/60 p-4">
        <div class="grid md:grid-cols-3 gap-3">
          <div
            class="flex items-center gap-3 px-4 py-3 rounded-lg bg-surface-100-800"
          >
            <div class="placeholder-circle size-10 animate-pulse"></div>
            <div class="flex-1 space-y-2">
              <div class="placeholder h-3 w-16 animate-pulse"></div>
              <div class="placeholder h-4 w-12 animate-pulse"></div>
            </div>
          </div>
          <div
            class="flex items-center gap-3 px-4 py-3 rounded-lg bg-surface-100-800"
          >
            <div class="placeholder-circle size-10 animate-pulse"></div>
            <div class="flex-1 space-y-2">
              <div class="placeholder h-3 w-20 animate-pulse"></div>
              <div class="placeholder h-4 w-24 animate-pulse"></div>
            </div>
          </div>
          <div
            class="flex items-center gap-3 px-4 py-3 rounded-lg bg-surface-100-800"
          >
            <div class="placeholder-circle size-10 animate-pulse"></div>
            <div class="flex-1 space-y-2">
              <div class="placeholder h-3 w-16 animate-pulse"></div>
              <div class="placeholder h-4 w-20 animate-pulse"></div>
            </div>
          </div>
        </div>
      </div>

      <div class="flex items-center justify-between mb-6">
        <div class="space-y-2">
          <div class="placeholder h-6 w-24 animate-pulse"></div>
          <div class="placeholder h-4 w-32 animate-pulse"></div>
        </div>
        <div class="placeholder h-10 w-36 animate-pulse rounded-md"></div>
      </div>

      <div
        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 lg:gap-6"
      >
        {#each Array(3) as _}
          <div class="card bg-surface-50-950/60 p-5">
            <div class="space-y-4">
              <div class="flex items-start justify-between">
                <div class="flex-1 space-y-2">
                  <div class="placeholder h-5 w-32 animate-pulse"></div>
                  <div class="placeholder h-3 w-48 animate-pulse"></div>
                </div>
                <div
                  class="placeholder h-6 w-20 animate-pulse rounded-full"
                ></div>
              </div>
              <div class="placeholder h-4 w-16 animate-pulse"></div>
              <div class="flex gap-2">
                <div
                  class="placeholder h-8 flex-1 animate-pulse rounded-md"
                ></div>
                <div
                  class="placeholder h-8 flex-1 animate-pulse rounded-md"
                ></div>
              </div>
            </div>
          </div>
        {/each}
      </div>
    </div>
  {:else if health}
    <div class="mb-8">
      <NodeStats stats={health.stats} />
    </div>
  {/if}

  {#if !loading}
    <div class="flex items-center justify-between mb-6">
      <div>
        <h2 class="text-xl font-semibold">Services</h2>
        <p class="text-sm mt-1">
          {services.length}
          {services.length === 1 ? "service" : "services"} registered
        </p>
      </div>
      <button onclick={() => goto("/service/create")} class="btn preset-filled">
        Create Service
      </button>
    </div>

    {#if services.length === 0}
      <div class="card bg-surface-50-950/60 rounded-xl p-12 text-center">
        <div
          class="w-16 h-16 mx-auto mb-4 rounded-full flex items-center justify-center"
        >
          <InboxIcon />
        </div>
        <h3 class="text-lg font-semibold mb-2">No services yet</h3>
        <p class="text-sm opacity-60 mb-6">
          Get started by creating your first service
        </p>
      </div>
    {:else}
      <div
        class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 lg:gap-6"
      >
        {#each services as service (service.id)}
          <ServiceCard {service} />
        {/each}
      </div>
    {/if}
  {/if}
</main>
