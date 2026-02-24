<script lang="ts">
  import type { SystemStats } from "$lib/types";
  import { formatBytes, formatMB, formatUptime } from "$lib/utils";
  import { Progress } from "@skeletonlabs/skeleton-svelte";
  import { CpuIcon, HardDriveIcon, MemoryStickIcon } from "lucide-svelte";

  let { stats }: { stats: SystemStats } = $props();

  const displayStats = $derived(stats);
  const memoryPercent = $derived(
    (displayStats.memory_used / displayStats.memory_total) * 100,
  );
  const diskPercent = $derived(
    (displayStats.disk_used / displayStats.disk_total) * 100,
  );
</script>

<div class="card w-full bg-surface-50-950/60 p-4 text-center">
  <div class="grid md:grid-cols-3 gap-3">
    <div
      class="flex items-center gap-3 px-4 py-3 rounded-lg bg-surface-100-800"
    >
      <div
        class="flex items-center justify-center w-10 h-10 rounded-full bg-primary-500/10"
      >
        <CpuIcon class="w-5 h-5 text-primary-500" />
      </div>
      <div class="text-start">
        <p class="text-xs opacity-60">CPU Usage</p>
        <p class="text-md font-semibold">
          {displayStats.cpu_usage.toFixed(1)}%
        </p>
      </div>
    </div>

    <div
      class="flex items-start gap-3 px-4 py-3 rounded-lg bg-surface-100-800"
      title="{formatMB(displayStats.memory_used)} / {formatMB(
        displayStats.memory_total,
      )}"
    >
      <div
        class="flex items-center justify-center w-10 h-10 rounded-full bg-secondary-500/10"
      >
        <MemoryStickIcon class="w-5 h-5 text-secondary-500" />
      </div>
      <div class="text-start">
        <p class="text-xs opacity-60">Memory {memoryPercent.toFixed(1)}%</p>
        <p class="text-md font-semibold">
          {formatBytes(displayStats.memory_used)}/{formatBytes(
            displayStats.memory_total,
          )}
        </p>
      </div>
    </div>

    <div class="flex items-start gap-3 px-4 py-3 rounded-lg bg-surface-100-800">
      <div
        class="flex items-center justify-center w-10 h-10 rounded-full bg-tertiary-500/10"
      >
        <HardDriveIcon class="w-5 h-5 text-tertiary-500" />
      </div>
      <div class="text-start">
        <p class="text-xs opacity-60">Disk {diskPercent.toFixed(1)}%</p>
        <p class="text-md font-semibold">
          {formatBytes(displayStats.disk_used)}/{formatBytes(
            displayStats.disk_total,
          )}
        </p>
      </div>
    </div>
  </div>
</div>
