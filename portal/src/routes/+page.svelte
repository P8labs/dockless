<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { store } from "$lib/services.svelte";
  import TopBar from "$lib/components/TopBar.svelte";
  import NodeStats from "$lib/components/NodeStats.svelte";
  import ServiceCard from "$lib/components/ServiceCard.svelte";
  import CreateServiceModal from "$lib/components/CreateServiceModal.svelte";
  import ToastContainer from "$lib/components/ToastContainer.svelte";

  let showCreateModal = $state(false);

  const services = $derived(store.merged);
  const health = $derived(store.health);

  // Mock stats if not provided by backend
  const mockStats = {
    cpu_usage: 23.5,
    memory_used: 3 * 1024 * 1024 * 1024, // 3GB
    memory_total: 4 * 1024 * 1024 * 1024, // 4GB
    disk_used: 45 * 1024 * 1024 * 1024, // 45GB
    disk_total: 100 * 1024 * 1024 * 1024, // 100GB
    uptime: 345600, // 4 days
  };

  const displayStats = $derived(health?.stats || mockStats);

  onMount(() => {
    store.startPolling(3000);
  });

  onDestroy(() => {
    store.stopPolling();
  });
</script>

<div class="min-h-screen bg-gray-50">
  <TopBar />

  <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    {#if health}
      <div class="mb-8">
        <NodeStats stats={displayStats} />
      </div>
    {/if}

    <div class="flex items-center justify-between mb-6">
      <div>
        <h2 class="text-xl font-semibold text-gray-900">Services</h2>
        <p class="text-sm text-gray-500 mt-1">
          {services.length}
          {services.length === 1 ? "service" : "services"} registered
        </p>
      </div>
      <button
        class="rounded-lg px-4 py-2.5 text-sm font-medium text-white bg-black hover:bg-gray-800 active:scale-95 transition-all duration-150 inline-flex items-center gap-2"
        onclick={() => {
          showCreateModal = true;
        }}
      >
        <svg
          class="w-4 h-4"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M12 4v16m8-8H4"
          />
        </svg>
        Create Service
      </button>
    </div>

    {#if services.length === 0}
      <div class="bg-white border border-gray-200 rounded-xl p-12 text-center">
        <div
          class="w-16 h-16 mx-auto mb-4 rounded-full bg-gray-100 flex items-center justify-center"
        >
          <svg
            class="w-8 h-8 text-gray-400"
            fill="none"
            stroke="currentColor"
            viewBox="0 0 24 24"
          >
            <path
              stroke-linecap="round"
              stroke-linejoin="round"
              stroke-width="2"
              d="M20 13V6a2 2 0 00-2-2H6a2 2 0 00-2 2v7m16 0v5a2 2 0 01-2 2H6a2 2 0 01-2-2v-5m16 0h-2.586a1 1 0 00-.707.293l-2.414 2.414a1 1 0 01-.707.293h-3.172a1 1 0 01-.707-.293l-2.414-2.414A1 1 0 006.586 13H4"
            />
          </svg>
        </div>
        <h3 class="text-lg font-semibold text-gray-900 mb-2">
          No services yet
        </h3>
        <p class="text-sm text-gray-500 mb-6">
          Get started by creating your first service
        </p>
        <button
          class="rounded-lg px-4 py-2 text-sm font-medium text-white bg-black hover:bg-gray-800 transition-colors"
          onclick={() => {
            showCreateModal = true;
          }}
        >
          Create Service
        </button>
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
  </main>

  <CreateServiceModal
    open={showCreateModal}
    onclose={() => {
      showCreateModal = false;
    }}
  />
  <ToastContainer />
</div>
