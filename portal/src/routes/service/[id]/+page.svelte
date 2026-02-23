<script lang="ts">
  import { page } from "$app/stores";
  import { onMount, onDestroy } from "svelte";
  import { goto } from "$app/navigation";
  import { store } from "$lib/services.svelte";
  import * as api from "$lib/api";
  import { toasts } from "$lib/toasts.svelte";
  import TopBar from "$lib/components/TopBar.svelte";
  import ToastContainer from "$lib/components/ToastContainer.svelte";
  import { get } from "svelte/store";
  import type { ArtifactInfo } from "$lib/types";

  const serviceId = $derived(get(page).params.id);
  const service = $derived(store.merged.find((s) => s.id === serviceId));

  let activeTab = $state<"details" | "env" | "versions" | "logs">("details");
  let activeAction = $state("");

  let artifactInfo = $state<ArtifactInfo | null>(null);
  let artifactLoading = $state(false);
  let uploadVersion = $state("");
  let uploadFile = $state<File | null>(null);
  let ghRepo = $state("");
  let ghVersion = $state("");
  let ghAsset = $state("");

  let logs = $state<{ timestamp: string; message: string; level: string }[]>([
    {
      timestamp: new Date().toISOString(),
      message: "Service started successfully",
      level: "info",
    },
    {
      timestamp: new Date(Date.now() - 5000).toISOString(),
      message: "Listening on port 3000",
      level: "info",
    },
    {
      timestamp: new Date(Date.now() - 10000).toISOString(),
      message: "Database connection established",
      level: "info",
    },
    {
      timestamp: new Date(Date.now() - 15000).toISOString(),
      message: "Environment loaded",
      level: "info",
    },
  ]);

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

  async function handleAction(action: string) {
    if (!service) return;
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
          if (result.status) {
            await goto("/");
          }
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

  async function loadArtifactInfo() {
    if (!service) return;
    artifactLoading = true;
    try {
      artifactInfo = await api.getArtifactInfo(service.id);
    } catch (e: unknown) {
      const msg =
        e instanceof Error ? e.message : "Failed to load artifact info";
      toasts.add(msg, "error");
    } finally {
      artifactLoading = false;
    }
  }

  async function handleUpload() {
    if (!service || !uploadFile || !uploadVersion) return;
    activeAction = "upload";
    try {
      const result = await api.uploadArtifact(
        service.id,
        uploadFile,
        uploadVersion,
      );
      toasts.add(result.message, result.status ? "success" : "error");
      await loadArtifactInfo();
      await store.refresh();
      uploadVersion = "";
      uploadFile = null;
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Upload failed";
      toasts.add(msg, "error");
    } finally {
      activeAction = "";
    }
  }

  async function handleGithubInstall() {
    if (!service || !ghRepo || !ghVersion || !ghAsset) return;
    activeAction = "install";
    try {
      const result = await api.installGithubArtifact(
        service.id,
        ghRepo,
        ghVersion,
        ghAsset,
      );
      toasts.add(result.message, result.status ? "success" : "error");
      await loadArtifactInfo();
      await store.refresh();
      ghRepo = "";
      ghVersion = "";
      ghAsset = "";
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Install failed";
      toasts.add(msg, "error");
    } finally {
      activeAction = "";
    }
  }

  onMount(() => {
    store.startPolling(3000);
  });

  onDestroy(() => {
    store.stopPolling();
  });
</script>

<div class="min-h-screen bg-gray-50">
  <TopBar />

  <main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
    <!-- Back Button -->
    <button
      class="inline-flex items-center gap-2 text-sm text-gray-600 hover:text-gray-900 mb-6 transition-colors"
      onclick={() => goto("/")}
    >
      <svg
        class="w-4 h-4"
        fill="none"
        stroke="currentColor"
        viewBox="0 0 24 24"
      >
        <path
          stroke-linecap="round"
          stroke-linejoin="round"
          stroke-width="2"
          d="M15 19l-7-7 7-7"
        />
      </svg>
      Back to Services
    </button>

    {#if !service}
      <div class="bg-white border border-gray-200 rounded-lg p-12 text-center">
        <p class="text-gray-500">Service not found</p>
      </div>
    {:else}
      <!-- Service Header -->
      <div class="bg-white border border-gray-200 rounded-lg p-6 mb-6">
        <div class="flex items-start justify-between mb-4">
          <div>
            <h1 class="text-2xl font-semibold text-gray-900 mb-1">
              {service.name}
            </h1>
            <p class="text-sm font-mono text-gray-500">{service.id}</p>
          </div>
          <div
            class="flex items-center gap-1.5 px-3 py-1.5 rounded-full {service.state ===
            'Running'
              ? 'bg-green-50'
              : service.state === 'Stopped'
                ? 'bg-gray-50'
                : 'bg-yellow-50'}"
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
                ? 'text-green-700'
                : service.state === 'Stopped'
                  ? 'text-gray-600'
                  : 'text-yellow-700'}">{service.state}</span
            >
          </div>
        </div>

        <!-- Actions -->
        <div class="flex items-center gap-2 flex-wrap">
          <button
            class="rounded-lg px-4 py-2 text-sm font-medium transition-all duration-150 bg-black text-white hover:bg-gray-800 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center gap-2"
            disabled={!!activeAction || service.state === "Running"}
            onclick={() => handleAction("start")}
          >
            {#if activeAction === "start"}
              <span
                class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin"
              ></span>
            {:else}
              <svg
                class="w-4 h-4"
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
              </svg>
            {/if}
            Start
          </button>

          <button
            class="rounded-lg px-4 py-2 text-sm font-medium transition-all duration-150 bg-gray-100 text-gray-700 hover:bg-gray-200 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center gap-2"
            disabled={!!activeAction || service.state === "Stopped"}
            onclick={() => handleAction("stop")}
          >
            {#if activeAction === "stop"}
              <span
                class="w-4 h-4 border-2 border-gray-700 border-t-transparent rounded-full animate-spin"
              ></span>
            {:else}
              <svg
                class="w-4 h-4"
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
            class="rounded-lg px-4 py-2 text-sm font-medium transition-all duration-150 bg-gray-100 text-gray-700 hover:bg-gray-200 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center gap-2"
            disabled={!!activeAction}
            onclick={() => handleAction("restart")}
          >
            {#if activeAction === "restart"}
              <span
                class="w-4 h-4 border-2 border-gray-700 border-t-transparent rounded-full animate-spin"
              ></span>
            {:else}
              <svg
                class="w-4 h-4"
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
            Restart
          </button>

          <button
            class="rounded-lg px-4 py-2 text-sm font-medium transition-all duration-150 border border-gray-200 text-gray-600 hover:bg-red-50 hover:text-red-600 hover:border-red-200 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center gap-2"
            disabled={!!activeAction}
            onclick={() => handleAction("delete")}
          >
            {#if activeAction === "delete"}
              <span
                class="w-4 h-4 border-2 border-red-600 border-t-transparent rounded-full animate-spin"
              ></span>
            {:else}
              <svg
                class="w-4 h-4"
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
            Delete
          </button>
        </div>
      </div>

      <!-- Tabs Navigation -->
      <div class="bg-white border border-gray-200 rounded-lg overflow-hidden">
        <div class="border-b border-gray-200">
          <div class="flex">
            <button
              class="px-6 py-3 text-sm font-medium transition-colors border-b-2 {activeTab ===
              'details'
                ? 'border-black text-gray-900'
                : 'border-transparent text-gray-500 hover:text-gray-700'}"
              onclick={() => (activeTab = "details")}
            >
              Details
            </button>
            <button
              class="px-6 py-3 text-sm font-medium transition-colors border-b-2 {activeTab ===
              'env'
                ? 'border-black text-gray-900'
                : 'border-transparent text-gray-500 hover:text-gray-700'}"
              onclick={() => (activeTab = "env")}
            >
              Environment
            </button>
            <button
              class="px-6 py-3 text-sm font-medium transition-colors border-b-2 {activeTab ===
              'versions'
                ? 'border-black text-gray-900'
                : 'border-transparent text-gray-500 hover:text-gray-700'}"
              onclick={() => {
                activeTab = "versions";
                if (!artifactInfo) loadArtifactInfo();
              }}
            >
              Versions
            </button>
            <button
              class="px-6 py-3 text-sm font-medium transition-colors border-b-2 {activeTab ===
              'logs'
                ? 'border-black text-gray-900'
                : 'border-transparent text-gray-500 hover:text-gray-700'}"
              onclick={() => (activeTab = "logs")}
            >
              Logs
            </button>
          </div>
        </div>

        <!-- Tab Content -->
        <div class="p-6">
          {#if activeTab === "details"}
            <div class="space-y-4 max-w-2xl">
              <div>
                <p class="text-xs text-gray-500 mb-1">Binary Path</p>
                <p class="text-sm font-mono text-gray-900 break-all">
                  {service.binary_path}
                </p>
              </div>
              {#if service.current_version}
                <div>
                  <p class="text-xs text-gray-500 mb-1">Version</p>
                  <p class="text-sm font-mono text-gray-900">
                    {service.current_version}
                  </p>
                </div>
              {/if}
              <div>
                <p class="text-xs text-gray-500 mb-1">Auto Restart</p>
                <p class="text-sm text-gray-900">
                  {service.auto_restart ? "Enabled" : "Disabled"}
                </p>
              </div>
              {#if service.restart_limit !== null}
                <div>
                  <p class="text-xs text-gray-500 mb-1">Restart Limit</p>
                  <p class="text-sm text-gray-900">{service.restart_limit}</p>
                </div>
              {/if}
              {#if service.args.length > 0}
                <div>
                  <p class="text-xs text-gray-500 mb-1">Arguments</p>
                  <div class="flex flex-wrap gap-1.5">
                    {#each service.args as arg}
                      <span
                        class="text-xs font-mono bg-gray-100 px-2 py-1 rounded"
                      >
                        {arg}
                      </span>
                    {/each}
                  </div>
                </div>
              {/if}
            </div>
          {:else if activeTab === "env"}
            {#if Object.keys(service.env).length > 0}
              <div class="space-y-2 max-w-2xl">
                {#each Object.entries(service.env) as [key, value]}
                  <div
                    class="flex items-center gap-3 py-2 px-3 bg-gray-50 rounded-lg"
                  >
                    <span class="font-mono text-sm text-gray-900 font-medium"
                      >{key}</span
                    >
                    <span class="text-gray-400">=</span>
                    <span class="font-mono text-sm text-gray-600 flex-1"
                      >{value}</span
                    >
                  </div>
                {/each}
              </div>
            {:else}
              <div class="text-center py-12">
                <div class="text-gray-400 mb-2">
                  <svg
                    class="w-12 h-12 mx-auto"
                    fill="none"
                    stroke="currentColor"
                    viewBox="0 0 24 24"
                  >
                    <path
                      stroke-linecap="round"
                      stroke-linejoin="round"
                      stroke-width="2"
                      d="M9 12h6m-6 4h6m2 5H7a2 2 0 01-2-2V5a2 2 0 012-2h5.586a1 1 0 01.707.293l5.414 5.414a1 1 0 01.293.707V19a2 2 0 01-2 2z"
                    />
                  </svg>
                </div>
                <p class="text-sm text-gray-500">
                  No environment variables configured
                </p>
              </div>
            {/if}
          {:else if activeTab === "versions"}
            <div class="space-y-6 max-w-3xl">
              {#if artifactLoading}
                <div class="flex items-center justify-center py-12">
                  <div
                    class="w-8 h-8 border-2 border-gray-300 border-t-gray-900 rounded-full animate-spin"
                  ></div>
                </div>
              {:else}
                {#if artifactInfo}
                  <div class="bg-gray-50 rounded-lg p-4 space-y-3">
                    <div class="flex items-center justify-between">
                      <span class="text-sm font-medium text-gray-600"
                        >Current Version</span
                      >
                      <span class="font-mono text-sm text-gray-900 font-medium"
                        >{artifactInfo.current_version ?? "None"}</span
                      >
                    </div>
                    {#if artifactInfo.available_versions.length > 0}
                      <div>
                        <span
                          class="text-sm font-medium text-gray-600 block mb-2"
                          >Available Versions</span
                        >
                        <div class="flex flex-wrap gap-2">
                          {#each artifactInfo.available_versions as ver}
                            <span
                              class="border border-gray-300 bg-white rounded-md px-3 py-1.5 text-sm font-mono hover:bg-gray-100 transition-colors cursor-default"
                            >
                              {ver}
                            </span>
                          {/each}
                        </div>
                      </div>
                    {/if}
                  </div>
                {/if}

                <div class="border-t border-gray-200 pt-6">
                  <h3
                    class="text-sm font-semibold text-gray-900 mb-4 uppercase tracking-wide"
                  >
                    Manual Upload
                  </h3>
                  <div class="flex items-end gap-3 flex-wrap">
                    <div class="flex-1 min-w-40">
                      <label
                        for="upload-version"
                        class="text-xs text-gray-600 mb-1.5 block"
                        >Version</label
                      >
                      <input
                        id="upload-version"
                        type="text"
                        placeholder="v1.0.0"
                        bind:value={uploadVersion}
                        class="w-full border border-gray-200 rounded-lg px-3 py-2 text-sm focus:ring-2 focus:ring-black focus:border-black focus:outline-none transition-all"
                      />
                    </div>
                    <div class="flex-1 min-w-50">
                      <label
                        for="upload-file"
                        class="text-xs text-gray-600 mb-1.5 block"
                        >Binary File</label
                      >
                      <input
                        id="upload-file"
                        type="file"
                        onchange={(e) => {
                          const target = e.currentTarget as HTMLInputElement;
                          uploadFile = target.files?.[0] ?? null;
                        }}
                        class="w-full text-sm text-gray-500 file:mr-2 file:rounded-lg file:border-0 file:bg-gray-100 file:px-3 file:py-2 file:text-sm file:font-medium file:text-gray-700 hover:file:bg-gray-200 file:transition-colors"
                      />
                    </div>
                    <button
                      class="rounded-lg px-4 py-2 text-sm font-medium transition-all duration-150 bg-black text-white hover:bg-gray-800 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center gap-2"
                      disabled={!uploadFile || !uploadVersion || !!activeAction}
                      onclick={handleUpload}
                    >
                      {#if activeAction === "upload"}
                        <span
                          class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin shrink-0"
                        ></span>
                      {/if}
                      Upload
                    </button>
                  </div>
                </div>

                <div class="border-t border-gray-200 pt-6">
                  <h3
                    class="text-sm font-semibold text-gray-900 mb-4 uppercase tracking-wide"
                  >
                    Install from GitHub
                  </h3>
                  <div class="flex flex-wrap items-end gap-3">
                    <div class="flex-1 min-w-50">
                      <label
                        for="gh-repo"
                        class="text-xs text-gray-600 mb-1.5 block"
                        >Repository</label
                      >
                      <input
                        id="gh-repo"
                        type="text"
                        placeholder="owner/repo"
                        bind:value={ghRepo}
                        class="w-full border border-gray-200 rounded-lg px-3 py-2 text-sm focus:ring-2 focus:ring-black focus:border-black focus:outline-none transition-all"
                      />
                    </div>
                    <div class="flex-1 min-w-30">
                      <label
                        for="gh-version"
                        class="text-xs text-gray-600 mb-1.5 block"
                        >Version</label
                      >
                      <input
                        id="gh-version"
                        type="text"
                        placeholder="v1.0.0"
                        bind:value={ghVersion}
                        class="w-full border border-gray-200 rounded-lg px-3 py-2 text-sm focus:ring-2 focus:ring-black focus:border-black focus:outline-none transition-all"
                      />
                    </div>
                    <div class="flex-1 min-w-40">
                      <label
                        for="gh-asset"
                        class="text-xs text-gray-600 mb-1.5 block"
                        >Asset Name</label
                      >
                      <input
                        id="gh-asset"
                        type="text"
                        placeholder="binary-linux-amd64"
                        bind:value={ghAsset}
                        class="w-full border border-gray-200 rounded-lg px-3 py-2 text-sm focus:ring-2 focus:ring-black focus:border-black focus:outline-none transition-all"
                      />
                    </div>
                    <button
                      class="rounded-lg px-4 py-2 text-sm font-medium transition-all duration-150 bg-black text-white hover:bg-gray-800 active:scale-95 disabled:opacity-50 disabled:cursor-not-allowed inline-flex items-center gap-2"
                      disabled={!ghRepo ||
                        !ghVersion ||
                        !ghAsset ||
                        !!activeAction}
                      onclick={handleGithubInstall}
                    >
                      {#if activeAction === "install"}
                        <span
                          class="w-4 h-4 border-2 border-white border-t-transparent rounded-full animate-spin shrink-0"
                        ></span>
                      {/if}
                      Install
                    </button>
                  </div>
                </div>
              {/if}
            </div>
          {:else if activeTab === "logs"}
            <div>
              <div class="flex items-center justify-between mb-4">
                <p class="text-sm text-gray-600">Real-time service logs</p>
                <button
                  class="text-sm text-gray-600 hover:text-gray-900 transition-colors font-medium"
                >
                  Clear
                </button>
              </div>
              <div
                class="bg-gray-950 rounded-lg p-4 font-mono text-sm h-128 overflow-y-auto"
              >
                {#each logs as log}
                  <div class="flex gap-3 mb-1">
                    <span class="text-gray-500"
                      >{formatTimestamp(log.timestamp)}</span
                    >
                    <span class={getLogColor(log.level)}
                      >[{log.level.toUpperCase()}]</span
                    >
                    <span class="text-gray-300">{log.message}</span>
                  </div>
                {/each}
              </div>
            </div>
          {/if}
        </div>
      </div>
    {/if}
  </main>

  <ToastContainer />
</div>
