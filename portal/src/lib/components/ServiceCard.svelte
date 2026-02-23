<script lang="ts">
  import { goto } from "$app/navigation";
  import type { ServiceView } from "$lib/types";
  import * as api from "$lib/api";
  import { store } from "$lib/services.svelte";
  import { toasts } from "$lib/toasts.svelte";

  let { service }: { service: ServiceView } = $props();

  let activeAction = $state("");

  function getStateStyles(state: string): {
    bg: string;
    text: string;
    dot: string;
  } {
    switch (state) {
      case "Running":
        return {
          bg: "bg-green-50",
          text: "text-green-700",
          dot: "bg-green-500",
        };
      case "Starting":
        return {
          bg: "bg-blue-50",
          text: "text-blue-700",
          dot: "bg-blue-500",
        };
      case "Stopping":
        return {
          bg: "bg-yellow-50",
          text: "text-yellow-700",
          dot: "bg-yellow-500",
        };
      case "Stopped":
        return {
          bg: "bg-gray-50",
          text: "text-gray-600",
          dot: "bg-gray-400",
        };
      case "Crashed":
      case "Failed":
        return {
          bg: "bg-red-50",
          text: "text-red-700",
          dot: "bg-red-500",
        };
      default:
        return {
          bg: "bg-gray-50",
          text: "text-gray-600",
          dot: "bg-gray-400",
        };
    }
  }

  const stateStyles = $derived(getStateStyles(service.state));

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
        toasts.add(result.message, result.status ? "success" : "error");
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

<div
  class="group bg-white border border-gray-200 rounded-lg p-5 hover:border-gray-400 transition-all duration-200 cursor-pointer"
  role="button"
  tabindex="0"
  onclick={() => goto(`/service/${service.id}`)}
  onkeydown={(e) => {
    if (e.key === "Enter" || e.key === " ") {
      e.preventDefault();
      goto(`/service/${service.id}`);
    }
  }}
>
  <div class="flex items-start justify-between mb-4">
    <div class="min-w-0 flex-1 mr-3">
      <h3 class="text-base font-semibold text-gray-900 truncate mb-1">
        {service.name}
      </h3>
      <p class="text-xs font-mono text-gray-400 truncate">{service.id}</p>
    </div>
    <div
      class="flex items-center gap-1.5 px-2.5 py-1 rounded-full {stateStyles.bg} shrink-0"
    >
      <div class="h-1.5 w-1.5 rounded-full {stateStyles.dot}"></div>
      <span class="text-xs font-medium {stateStyles.text}">{service.state}</span
      >
    </div>
  </div>

  <div class="grid grid-cols-1 gap-2 mb-4 text-sm">
    {#if service.current_version}
      <div class="flex items-center justify-between">
        <span class="text-gray-500 text-xs">Version</span>
        <span class="font-mono text-gray-900 text-xs"
          >{service.current_version}</span
        >
      </div>
    {/if}
    {#if service.restart_limit != null}
      <div class="flex items-center justify-between">
        <span class="text-gray-500 text-xs">Restart Limit</span>
        <span class="text-gray-900 text-xs">{service.restart_limit}</span>
      </div>
    {/if}
    <div class="flex items-center justify-between">
      <span class="text-gray-500 text-xs">Auto Restart</span>
      <span class="text-gray-900 text-xs"
        >{service.auto_restart ? "Yes" : "No"}</span
      >
    </div>
  </div>

  <div class="mb-4">
    <p
      class="text-xs text-gray-400 truncate font-mono bg-gray-50 px-2 py-1.5 rounded"
      title={service.binary_path}
    >
      {service.binary_path}
    </p>
  </div>

  <div class="flex items-center gap-2 flex-wrap">
    <button
      class="flex-1 min-w-20 rounded-lg px-3 py-2 text-xs font-medium transition-all duration-150 bg-black text-white hover:bg-gray-800 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center justify-center gap-1.5"
      disabled={!!activeAction || service.state === "Running"}
      onclick={(e) => handleAction("start", e)}
      aria-label="Start service"
    >
      {#if activeAction === "start"}
        <span
          class="w-3 h-3 border-2 border-white border-t-transparent rounded-full animate-spin shrink-0"
        ></span>
      {:else}
        <svg
          class="w-3 h-3"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M14.752 11.168l-3.197-2.132A1 1 0 0010 9.87v4.263a1 1 0 001.555.832l3.197-2.132a1 1 0 000-1.664z"
          />
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
        </svg>
      {/if}
      Start
    </button>

    <button
      class="flex-1 min-w-20 rounded-lg px-3 py-2 text-xs font-medium transition-all duration-150 bg-gray-100 text-gray-700 hover:bg-gray-200 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center justify-center gap-1.5"
      disabled={!!activeAction || service.state === "Stopped"}
      onclick={(e) => handleAction("stop", e)}
      aria-label="Stop service"
    >
      {#if activeAction === "stop"}
        <span
          class="w-3 h-3 border-2 border-gray-700 border-t-transparent rounded-full animate-spin shrink-0"
        ></span>
      {:else}
        <svg
          class="w-3 h-3"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
          />
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M9 10a1 1 0 011-1h4a1 1 0 011 1v4a1 1 0 01-1 1h-4a1 1 0 01-1-1v-4z"
          />
        </svg>
      {/if}
      Stop
    </button>

    <button
      class="rounded-lg px-3 py-2 text-xs font-medium transition-all duration-150 bg-gray-100 text-gray-700 hover:bg-gray-200 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center gap-1.5"
      disabled={!!activeAction}
      onclick={(e) => handleAction("restart", e)}
      aria-label="Restart service"
    >
      {#if activeAction === "restart"}
        <span
          class="w-3 h-3 border-2 border-gray-700 border-t-transparent rounded-full animate-spin shrink-0"
        ></span>
      {:else}
        <svg
          class="w-3 h-3"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M4 4v5h.582m15.356 2A8.001 8.001 0 004.582 9m0 0H9m11 11v-5h-.581m0 0a8.003 8.003 0 01-15.357-2m15.357 2H15"
          />
        </svg>
      {/if}
    </button>

    <button
      class="rounded-lg px-3 py-2 text-xs font-medium transition-all duration-150 border border-gray-200 text-gray-600 hover:bg-red-50 hover:text-red-600 hover:border-red-200 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center gap-1.5"
      disabled={!!activeAction}
      onclick={(e) => handleAction("delete", e)}
      aria-label="Delete service"
    >
      {#if activeAction === "delete"}
        <span
          class="w-3 h-3 border-2 border-red-600 border-t-transparent rounded-full animate-spin shrink-0"
        ></span>
      {:else}
        <svg
          class="w-3 h-3"
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path
            stroke-linecap="round"
            stroke-linejoin="round"
            stroke-width="2"
            d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"
          />
        </svg>
      {/if}
    </button>
  </div>
</div>
