<script lang="ts">
  import { AppBar } from "@skeletonlabs/skeleton-svelte";
  import { store } from "$lib/services.svelte";
  import { formatUptime } from "$lib/utils";
  import { toaster } from "./Toast.svelte";
  import {
    ServerIcon,
    CopyIcon,
    CircleDotIcon,
    ClockIcon,
  } from "lucide-svelte";

  const health = $derived(store.health);
  const loading = $derived(store.loading);
</script>

<AppBar class="bg-surface-50-950/60! shadow-xl backdrop-blur-sm">
  <AppBar.Toolbar class="grid-cols-[auto_1fr_auto]">
    <AppBar.Lead>
      <div class="flex items-center gap-2 px-2">
        <ServerIcon class="w-5 h-5 md:w-6 md:h-6 text-primary-500" />
      </div>
    </AppBar.Lead>
    <AppBar.Headline>
      <div>
        <h1 class="text-lg md:text-xl font-semibold">Dockless</h1>
        <p class="text-xs opacity-60 -mt-0.5 hidden sm:block">
          Edge Runtime Platform
        </p>
      </div>
    </AppBar.Headline>
    <AppBar.Trail>
      <div class="flex items-center gap-1.5 md:gap-3">
        {#if loading}
          <div class="flex items-center gap-1.5">
            <div
              class="placeholder h-6 w-16 md:w-24 animate-pulse rounded-md"
            ></div>
            <div
              class="placeholder h-6 w-20 md:w-32 animate-pulse rounded-full"
            ></div>
          </div>
        {:else if health}
          <!-- Node ID -->
          <button
            class="hidden sm:flex items-center gap-1.5 md:gap-2 px-2 md:px-3 py-1.5 rounded-lg bg-surface-100-800 hover:bg-surface-200-700 transition-colors"
            title="Click to copy Node ID: {health.node_id}"
            onclick={() => {
              navigator.clipboard.writeText(health.node_id);
              toaster.info({ title: "Node ID copied to clipboard" });
            }}
          >
            <span class="text-xs font-mono opacity-60 hidden md:inline"
              >Node ID</span
            >
            <span class="text-xs md:text-sm font-mono font-medium"
              >{health.node_id.slice(0, 8)}</span
            >
            <CopyIcon class="w-3 h-3 opacity-40" />
          </button>

          <!-- Status Badge -->
          <div
            class="flex items-center gap-1.5 md:gap-2 px-2 md:px-3 py-1.5 rounded-full {health.status ===
            'alive'
              ? 'bg-green-500/10'
              : 'bg-red-500/10'}"
          >
            <CircleDotIcon
              class="w-3.5 h-3.5 md:w-4 md:h-4 {health.status === 'alive'
                ? 'text-green-500'
                : 'text-red-500'} animate-pulse"
            />
            <span
              class="text-xs md:text-sm font-medium {health.status === 'alive'
                ? 'text-green-600'
                : 'text-red-600'}"
            >
              {health.status === "alive" ? "Online" : health.status}
            </span>
          </div>

          <!-- Uptime -->
          <div
            class="hidden lg:flex items-center gap-2 px-3 py-1.5 rounded-lg bg-surface-100-800"
            title="Node uptime"
          >
            <ClockIcon class="w-4 h-4 opacity-60" />
            <span class="text-sm font-medium">
              {formatUptime(health.stats.uptime)}
            </span>
          </div>
        {/if}
      </div>
    </AppBar.Trail>
  </AppBar.Toolbar>
</AppBar>
