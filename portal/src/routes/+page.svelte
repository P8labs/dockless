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

  onMount(() => {
    store.startPolling(3000);
  });

  onDestroy(() => {
    store.stopPolling();
  });
</script>

<TopBar />
<main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
  {#if health}
    <div class="mb-8">
      <NodeStats stats={health.stats} />
    </div>
  {/if}

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
    <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4 lg:gap-6">
      {#each services as service (service.id)}
        <ServiceCard {service} />
      {/each}
    </div>
  {/if}
</main>
