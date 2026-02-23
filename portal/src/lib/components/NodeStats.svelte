<script lang="ts">
  import type { SystemStats } from "$lib/types";

  let { stats }: { stats: SystemStats } = $props();

  function formatBytes(bytes: number): string {
    if (bytes === 0) return "0 B";

    const units = ["B", "KB", "MB", "GB", "TB"];
    const k = 1024;
    const i = Math.floor(Math.log(bytes) / Math.log(k));

    const value = bytes / Math.pow(k, i);
    return `${value.toFixed(i >= 3 ? 1 : 0)} ${units[i]}`;
  }

  function formatMB(bytes: number): string {
    return `${(bytes / (1024 * 1024)).toFixed(1)} MB`;
  }

  function formatUptime(seconds: number): string {
    const days = Math.floor(seconds / 86400);
    const hours = Math.floor((seconds % 86400) / 3600);
    const minutes = Math.floor((seconds % 3600) / 60);

    if (days > 0) {
      return `${days}d ${hours}h ${minutes}m`;
    }
    if (hours > 0) {
      return `${hours}h ${minutes}m`;
    }
    return `${minutes}m`;
  }

  const displayStats = $derived(stats);
  const memoryPercent = $derived(
    (displayStats.memory_used / displayStats.memory_total) * 100,
  );
  const diskPercent = $derived(
    (displayStats.disk_used / displayStats.disk_total) * 100,
  );
</script>

<div class="bg-white border border-gray-200 rounded-lg p-4">
  <div class="grid grid-cols-2 lg:grid-cols-4 gap-4">
    <div class="space-y-1">
      <div class="flex items-center justify-between">
        <span class="text-xs text-gray-500">CPU</span>
        <span class="text-xs font-mono text-gray-900"
          >{displayStats.cpu_usage.toFixed(1)}%</span
        >
      </div>
      <div class="w-full bg-gray-100 rounded-full h-1.5 overflow-hidden">
        <div
          class="h-full rounded-full transition-all duration-500 {displayStats.cpu_usage >
          80
            ? 'bg-red-500'
            : displayStats.cpu_usage > 60
              ? 'bg-yellow-500'
              : 'bg-green-500'}"
          style="width: {displayStats.cpu_usage}%"
        ></div>
      </div>
    </div>

    <div
      class="space-y-1"
      title="{formatMB(displayStats.memory_used)} / {formatMB(
        displayStats.memory_total,
      )}"
    >
      <div class="flex items-center justify-between">
        <span class="text-xs text-gray-500">Memory</span>
        <span class="text-xs font-mono text-gray-900"
          >{formatBytes(displayStats.memory_used)}/{formatBytes(
            displayStats.memory_total,
          )}</span
        >
      </div>
      <div class="w-full bg-gray-100 rounded-full h-1.5 overflow-hidden">
        <div
          class="h-full rounded-full transition-all duration-500 {memoryPercent >
          80
            ? 'bg-red-500'
            : memoryPercent > 60
              ? 'bg-yellow-500'
              : 'bg-blue-500'}"
          style="width: {memoryPercent}%"
        ></div>
      </div>
    </div>

    <div class="space-y-1">
      <div class="flex items-center justify-between">
        <span class="text-xs text-gray-500">Disk</span>
        <span class="text-xs font-mono text-gray-900"
          >{formatBytes(displayStats.disk_used)}/{formatBytes(
            displayStats.disk_total,
          )}</span
        >
      </div>
      <div class="w-full bg-gray-100 rounded-full h-1.5 overflow-hidden">
        <div
          class="h-full rounded-full transition-all duration-500 {diskPercent >
          80
            ? 'bg-red-500'
            : diskPercent > 60
              ? 'bg-yellow-500'
              : 'bg-purple-500'}"
          style="width: {diskPercent}%"
        ></div>
      </div>
    </div>

    <div class="space-y-1">
      <div class="flex items-center justify-between">
        <span class="text-xs text-gray-500">Uptime</span>
        <span class="text-xs font-mono text-gray-900"
          >{formatUptime(displayStats.uptime)}</span
        >
      </div>
      <div class="flex items-center gap-2 mt-1">
        <div class="h-1.5 w-1.5 bg-green-500 rounded-full animate-pulse"></div>
        <span class="text-xs text-gray-500">Online</span>
      </div>
    </div>
  </div>
</div>
