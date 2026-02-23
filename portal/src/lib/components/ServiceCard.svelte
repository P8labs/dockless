<script lang="ts">
  import type { ServiceView } from "$lib/types";
  import * as api from "$lib/api";
  import { store } from "$lib/services.svelte";
  import { toasts } from "$lib/toasts.svelte";
  import { toaster } from "./Toast.svelte";
  import { PauseIcon, PlayIcon, RefreshCcwIcon } from "lucide-svelte";

  let { service }: { service: ServiceView } = $props();

  let activeAction = $state("");

  async function handleAction(action: string, e: Event) {
    e.stopPropagation();
    activeAction = action;
    try {
      let result: { status: boolean; message: string } | undefined;
      switch (action) {
        case "start":
          result = await api.startService(service.id);
          break;
        case "stop":
          result = await api.stopService(service.id);
          break;
        case "restart":
          result = await api.restartService(service.id);
          break;
        case "delete":
          result = await api.deleteService(service.id);
          break;
      }
      if (result) {
        toaster.create({
          title: result.message,
          type: result.status ? "success" : "error",
        });
      }
      await store.refresh();
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Action failed";
      toasts.add(msg, "error");
    } finally {
      activeAction = "";
    }
  }
</script>

<div class="card bg-surface-50-950/60 rounded-lg p-5">
  <a href={`/service/${service.id}`}>
    <div class="flex items-start justify-between mb-4 cursor-pointer">
      <div class="min-w-0 flex-1 mr-3">
        <h3 class="text-base font-semibold truncate mb-1">
          {service.name}
        </h3>
        <p class="text-xs font-mono opacity-40 truncate">{service.id}</p>
      </div>
      <span class="badge preset-filled-tertiary-50-950 font-bold">
        {service.state}
      </span>
    </div>
  </a>

  <div class="grid grid-cols-1 gap-2 mb-4 text-sm">
    {#if service.current_version}
      <div class="flex items-center justify-between">
        <span class="text-xs">Version</span>
        <span class="font-mono opacity-40 text-xs"
          >{service.current_version}</span
        >
      </div>
    {/if}
  </div>

  <div class="flex items-center gap-2 flex-wrap">
    <button
      class="flex-1 min-w-20 btn btn-icon-sm preset-filled-primary-500 h-full"
      disabled={!!activeAction || service.state === "Running"}
      onclick={(e) => handleAction("start", e)}
      aria-label="Start service"
    >
      {#if activeAction === "start"}
        <span
          class="w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin shrink-0"
        ></span>
      {:else}
        <PlayIcon class="w-3 h-3" />
      {/if}
      Start
    </button>

    <button
      class="flex-1 min-w-20 h-full btn btn-icon-sm preset-filled"
      disabled={!!activeAction || service.state === "Stopped"}
      onclick={(e) => handleAction("stop", e)}
      aria-label="Stop service"
    >
      {#if activeAction === "stop"}
        <span
          class="w-3 h-3 border-2 border-gray-700 border-t-transparent rounded-full animate-spin shrink-0"
        ></span>
      {:else}
        <PauseIcon class="w-3 h-3" />
      {/if}
      Stop
    </button>

    <button
      class="btn-icon preset-filled"
      disabled={!!activeAction}
      onclick={(e) => handleAction("restart", e)}
      aria-label="Restart service"
    >
      {#if activeAction === "restart"}
        <span
          class="w-3 h-3 border-2 border-gray-700 border-t-transparent rounded-full animate-spin shrink-0"
        ></span>
      {:else}
        <RefreshCcwIcon class="w-3 h-3" />
      {/if}
    </button>
  </div>
</div>
