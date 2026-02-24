<script lang="ts">
  import type { LogEntry } from "$lib/types";
  import { Trash2, RefreshCw } from "lucide-svelte";

  let {
    logs,
    onClear,
    onClearPermanent,
    onRefresh,
    refreshing = false,
  }: {
    logs: LogEntry[];
    onClear: () => void;
    onClearPermanent: () => void;
    onRefresh: () => void;
    refreshing?: boolean;
  } = $props();

  function formatTimestamp(iso: string): string {
    const date = new Date(iso);
    return date.toLocaleTimeString("en-US", { hour12: false });
  }

  function getLogColor(level: string): string {
    switch (level) {
      case "error":
        return "text-red-600";
      case "warn":
        return "text-yellow-600";
      case "info":
        return "text-blue-600";
      default:
        return "text-gray-600";
    }
  }
</script>

<div>
  <div class="flex items-center justify-between mb-4">
    <p class="text-sm opacity-70">
      Real-time service logs
      {#if logs.length > 0}
        <span class="opacity-40">({logs.length} entries)</span>
      {/if}
    </p>
    <div class="flex items-center gap-2">
      <button
        class="text-sm font-medium btn preset-tonal inline-flex items-center gap-2"
        onclick={onRefresh}
        disabled={refreshing}
        title="Fetch latest logs from file"
      >
        <RefreshCw class="w-4 h-4 {refreshing ? 'animate-spin' : ''}" />
        Refresh
      </button>
      <button
        class="text-sm font-medium btn preset-tonal"
        onclick={onClear}
        disabled={logs.length === 0}
        title="Clear logs from view (temporary)"
      >
        Clear View
      </button>
      <button
        class="text-sm font-medium btn preset-outlined-error inline-flex items-center gap-2"
        onclick={onClearPermanent}
        disabled={logs.length === 0}
        title="Permanently clear logs from service file"
      >
        <Trash2 class="w-4 h-4" />
        Clear Permanently
      </button>
    </div>
  </div>
  <div
    class="bg-black/50 rounded-lg p-4 font-mono text-sm h-128 overflow-y-auto"
  >
    {#if logs.length === 0}
      <p class="text-center opacity-40 py-12">No logs yet</p>
    {:else}
      {#each logs as log}
        <div class="flex gap-3 mb-1">
          <span class="opacity-50">{formatTimestamp(log.timestamp)}</span>
          <span class={getLogColor(log.level)}>[{log.level.toUpperCase()}]</span
          >
          <span class="opacity-70">{log.message}</span>
        </div>
      {/each}
    {/if}
  </div>
</div>
