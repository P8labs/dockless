<script lang="ts">
  import type { ServiceDefinition } from "$lib/types";

  let { serviceDetail }: { serviceDetail: ServiceDefinition | null } = $props();
</script>

{#if serviceDetail && Object.keys(serviceDetail.env).length > 0}
  <div class="space-y-2 max-w-2xl">
    {#each Object.entries(serviceDetail.env) as [key, value]}
      <div
        class="flex items-center gap-3 py-2 px-3 rounded-lg bg-surface-100-800"
      >
        <span class="font-mono text-sm font-medium">{key}</span>
        <span class="opacity-40">=</span>
        <span class="font-mono text-sm opacity-70 flex-1">{value}</span>
      </div>
    {/each}
  </div>
{:else}
  <div class="text-center py-12">
    <p class="text-sm opacity-60">No environment variables configured</p>
  </div>
{/if}
