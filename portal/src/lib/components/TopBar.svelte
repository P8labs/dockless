<script lang="ts">
  import { AppBar } from "@skeletonlabs/skeleton-svelte";
  import { store } from "$lib/services.svelte";
  import { formatUptime } from "$lib/utils";
  import { toaster } from "./Toast.svelte";

  const health = $derived(store.health);
</script>

<AppBar class="bg-surface-50-950/60! shadow-xl">
  <AppBar.Toolbar class="grid-cols-[auto_1fr_auto]">
    <AppBar.Lead></AppBar.Lead>
    <AppBar.Headline>
      <div>
        <h1 class="text-xl">Dockless</h1>
        <p class="text-xs -mt-0.5">Edge Runtime Platform</p>
      </div>
    </AppBar.Headline>
    <AppBar.Trail>
      {#if health}
        <button
          title="Click to copy Node Id"
          onclick={() => {
            navigator.clipboard.writeText(health.node_id);
            toaster.info({ title: "Node ID copied to clipboard" });
          }}
          >{health.node_id.slice(0, 8)}...
        </button>

        <span
          class="badge {health.status === 'alive'
            ? 'preset-tonal-success'
            : 'preset-tonal-error'}"
        >
          <span
            >{health.status === "alive" ? "online" : health.status}
            {formatUptime(health.stats.uptime)}</span
          >
        </span>
      {/if}
    </AppBar.Trail>
  </AppBar.Toolbar>
</AppBar>
