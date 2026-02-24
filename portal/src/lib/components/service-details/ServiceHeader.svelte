<script lang="ts">
  import type {
    ServiceView,
    ServiceDefinition,
    ServiceStats,
  } from "$lib/types";
  import {
    Play,
    Square,
    RotateCw,
    Trash2,
    Cpu,
    HardDrive,
  } from "lucide-svelte";

  let {
    service,
    serviceDetail,
    stats,
    activeAction,
    onAction,
  }: {
    service: ServiceView;
    serviceDetail: ServiceDefinition | null;
    stats: ServiceStats | null;
    activeAction: string;
    onAction: (action: string) => void;
  } = $props();
</script>

<div class="card bg-surface-50-950/60 p-6 mb-6">
  <div class="flex items-start justify-between mb-4">
    <div>
      <h1 class="text-2xl font-semibold mb-1">
        {service.name}
      </h1>
      <p class="text-sm font-mono opacity-60">{service.id}</p>
      {#if serviceDetail?.port}
        <p class="text-sm font-mono opacity-60 mt-1">
          Port: <span class="text-primary-500">{serviceDetail.port}</span>
        </p>
      {/if}
    </div>
    <div class="flex items-center gap-3">
      {#if serviceDetail && !serviceDetail.ready}
        <div
          class="flex items-center gap-1.5 px-3 py-1.5 rounded-full bg-amber-500/10"
        >
          <div class="h-2 w-2 rounded-full bg-amber-500"></div>
          <span class="text-sm font-medium text-amber-600">Not Ready</span>
        </div>
      {/if}
      <div
        class="flex items-center gap-1.5 px-3 py-1.5 rounded-full {service.state ===
        'Running'
          ? 'bg-green-500/10'
          : service.state === 'Stopped'
            ? 'bg-gray-500/10'
            : 'bg-yellow-500/10'}"
      >
        <div
          class="h-2 w-2 rounded-full {service.state === 'Running'
            ? 'bg-green-500'
            : service.state === 'Stopped'
              ? 'bg-gray-400'
              : 'bg-yellow-500'}"
        ></div>
        <span
          class="text-sm font-medium {service.state === 'Running'
            ? 'text-green-600'
            : service.state === 'Stopped'
              ? 'opacity-60'
              : 'text-yellow-600'}">{service.state}</span
        >
      </div>
    </div>
  </div>

  <!-- Actions -->
  <div class="flex items-center gap-2 flex-wrap">
    <button
      class="btn preset-outlined inline-flex items-center gap-2"
      disabled={!!activeAction ||
        service.state === "Running" ||
        (serviceDetail && !serviceDetail.ready)}
      onclick={() => onAction("start")}
    >
      {#if activeAction === "start"}
        <span
          class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin shrink-0"
        ></span>
      {:else}
        <Play class="w-4 h-4" />
      {/if}
      Start
    </button>

    <button
      class="btn preset-tonal inline-flex items-center gap-2"
      disabled={!!activeAction || service.state === "Stopped"}
      onclick={() => onAction("stop")}
    >
      {#if activeAction === "stop"}
        <span
          class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin shrink-0"
        ></span>
      {:else}
        <Square class="w-4 h-4" />
      {/if}
      Stop
    </button>

    <button
      class="btn preset-tonal inline-flex items-center gap-2"
      disabled={!!activeAction || (serviceDetail && !serviceDetail.ready)}
      onclick={() => onAction("restart")}
    >
      {#if activeAction === "restart"}
        <span
          class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin shrink-0"
        ></span>
      {:else}
        <RotateCw class="w-4 h-4" />
      {/if}
      Restart
    </button>

    <button
      class="btn preset-tonal inline-flex items-center gap-2 hover:text-red-600"
      disabled={!!activeAction}
      onclick={() => onAction("delete")}
    >
      {#if activeAction === "delete"}
        <span
          class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin shrink-0"
        ></span>
      {:else}
        <Trash2 class="w-4 h-4" />
      {/if}
      Delete
    </button>
  </div>

  {#if stats}
    <div class="mt-4 grid grid-cols-3 gap-3">
      <div
        class="flex items-center gap-3 px-4 py-3 rounded-lg bg-surface-100-800"
      >
        <div
          class="flex items-center justify-center w-10 h-10 rounded-full bg-primary-500/10"
        >
          <Cpu class="w-5 h-5 text-primary-500" />
        </div>
        <div>
          <p class="text-xs opacity-60">CPU Usage</p>
          <p class="text-lg font-semibold">{stats.cpu_usage.toFixed(1)}%</p>
        </div>
      </div>
      <div
        class="flex items-center gap-3 px-4 py-3 rounded-lg bg-surface-100-800"
      >
        <div
          class="flex items-center justify-center w-10 h-10 rounded-full bg-secondary-500/10"
        >
          <HardDrive class="w-5 h-5 text-secondary-500" />
        </div>
        <div>
          <p class="text-xs opacity-60">Memory</p>
          <p class="text-lg font-semibold">{stats.memory_mb.toFixed(0)} MB</p>
        </div>
      </div>
      {#if stats.pid}
        <div
          class="flex items-center gap-3 px-4 py-3 rounded-lg bg-surface-100-800"
        >
          <div class="w-10"></div>
          <div>
            <p class="text-xs opacity-60 font-mono">PID</p>
            <p class="text-lg font-semibold font-mono">{stats.pid}</p>
          </div>
        </div>
      {/if}
    </div>
  {/if}
</div>
