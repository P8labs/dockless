<script lang="ts">
  import { store } from "$lib/services.svelte";
  import { toasts } from "$lib/toasts.svelte";

  const health = $derived(store.health);
  const serviceCount = $derived(store.merged.length);
  const runningCount = $derived(
    store.merged.filter((s) => s.state === "Running").length,
  );
</script>

<header
  class="sticky top-0 z-50 bg-white/80 backdrop-blur-md border-b border-gray-200"
>
  <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
    <div class="flex items-center justify-between h-16">
      <!-- Logo & Title -->
      <div class="flex items-center gap-4">
        <div>
          <h1 class="text-xl font-semibold text-gray-900">Dockless</h1>
          <p class="text-xs text-gray-500 -mt-0.5">Edge Runtime Platform</p>
        </div>
      </div>

      <!-- Stats & Info -->
      <div class="flex items-center gap-6">
        <!-- Service Count -->
        <div class="hidden md:flex items-center gap-2">
          <div class="text-right">
            <p class="text-xs text-gray-500">Services</p>
            <p class="text-sm font-semibold text-gray-900">
              {runningCount}/{serviceCount}
            </p>
          </div>
        </div>

        <!-- Node ID -->
        {#if health}
          <button
            class="hidden lg:flex items-center gap-2 hover:bg-gray-50 px-2 py-1 rounded transition-colors cursor-pointer"
            title="Click to copy: {health.node_id}"
            onclick={() => {
              navigator.clipboard.writeText(health.node_id);
              toasts.add("Node ID copied to clipboard", "success");
            }}
          >
            <div class="text-right">
              <p class="text-xs text-gray-500">Node ID</p>
              <p class="text-xs font-mono text-gray-900">
                {health.node_id.slice(0, 8)}...
              </p>
            </div>
          </button>
        {/if}

        <!-- Status Indicator -->
        {#if health}
          <div
            class="flex items-center gap-2 px-3 py-1.5 bg-gray-50 rounded-full"
          >
            <div
              class="h-2 w-2 rounded-full {health.status === 'ok' ||
              health.status === 'alive'
                ? 'bg-green-500'
                : 'bg-red-500'} {health.status === 'ok' ||
              health.status === 'alive'
                ? 'animate-pulse'
                : ''}"
            ></div>
            <span class="text-xs font-medium text-gray-700 capitalize"
              >{health.status === "alive" ? "online" : health.status}</span
            >
          </div>
        {/if}
      </div>
    </div>
  </div>
</header>
