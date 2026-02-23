<script lang="ts">
  import type { SystemStats } from "$lib/types";
  import { formatBytes, formatMB, formatUptime } from "$lib/utils";
  import { Progress } from "@skeletonlabs/skeleton-svelte";

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
  <div
    class="flex justify-between items-center flex-col space-y-3 md:space-x-3 md:flex-row"
  >
    <div class="flex justify-between items-center w-full">
      <div class="flex flex-col items-start">
        <span>Total CPU</span>
      </div>
      <Progress value={displayStats.cpu_usage} class="w-fit relative">
        <div class="absolute inset-0 flex items-center justify-center">
          <Progress.ValueText class="text-xs">
            <Progress.Context>
              {#snippet children(progress)}
                {progress().value?.toFixed(1)}%
              {/snippet}
            </Progress.Context>
          </Progress.ValueText>
        </div>
        <Progress.Circle class="[--size:--spacing(12)]">
          <Progress.CircleTrack />
          <Progress.CircleRange />
        </Progress.Circle>
      </Progress>
    </div>

    <div
      class="flex justify-between items-center w-full"
      title="{formatMB(displayStats.memory_used)} / {formatMB(
        displayStats.memory_total,
      )}"
    >
      <div class="flex items-start flex-col">
        <span class="text-xs">Memory</span>
        <span class="text-xs font-mono"
          >{formatBytes(displayStats.memory_used)}/{formatBytes(
            displayStats.memory_total,
          )}
        </span>
      </div>

      <Progress value={memoryPercent} class="w-fit relative">
        <div class="absolute inset-0 flex items-center justify-center">
          <Progress.ValueText class="text-xs">
            <Progress.Context>
              {#snippet children(progress)}
                {progress().value?.toFixed(1)}%
              {/snippet}
            </Progress.Context>
          </Progress.ValueText>
        </div>
        <Progress.Circle class="[--size:--spacing(12)]">
          <Progress.CircleTrack
            class={memoryPercent > 80
              ? "stroke-tertiary-50-950"
              : memoryPercent > 60
                ? "stroke-secondary-50-950"
                : ""}
          />
          <Progress.CircleRange
            class={memoryPercent > 80
              ? "stroke-tertiary-500"
              : memoryPercent > 60
                ? "stroke-secondary-500"
                : ""}
          />
        </Progress.Circle>
      </Progress>
    </div>

    <div class="flex justify-between items-center w-full">
      <div class="flex items-start flex-col">
        <span class="text-xs">Disk</span>
        <span class="text-xs font-mono"
          >{formatBytes(displayStats.disk_used)}/{formatBytes(
            displayStats.disk_total,
          )}</span
        >
      </div>

      <Progress value={diskPercent} class="w-fit relative">
        <div class="absolute inset-0 flex items-center justify-center">
          <Progress.ValueText class="text-xs">
            <Progress.Context>
              {#snippet children(progress)}
                {progress().value?.toFixed(1)}%
              {/snippet}
            </Progress.Context>
          </Progress.ValueText>
        </div>
        <Progress.Circle class="[--size:--spacing(12)]">
          <Progress.CircleTrack
            class={diskPercent > 80
              ? "stroke-tertiary-50-950"
              : diskPercent > 60
                ? "stroke-secondary-50-950"
                : ""}
          />
          <Progress.CircleRange
            class={diskPercent > 80
              ? "stroke-tertiary-500"
              : diskPercent > 60
                ? "stroke-secondary-500"
                : ""}
          />
        </Progress.Circle>
      </Progress>
    </div>
  </div>
</div>
