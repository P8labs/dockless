<script lang="ts">
  import { page } from "$app/state";
  import { onMount, onDestroy } from "svelte";
  import { goto } from "$app/navigation";
  import { store } from "$lib/services.svelte";
  import * as api from "$lib/api";
  import { toaster } from "$lib/components/Toast.svelte";
  import { ArrowLeft } from "lucide-svelte";
  import type {
    ArtifactInfo,
    ServiceConfig,
    ServiceDefinition,
    LogEntry,
    ServiceStats,
  } from "$lib/types";

  import ServiceHeader from "$lib/components/service-details/ServiceHeader.svelte";
  import SetupTab from "$lib/components/service-details/SetupTab.svelte";
  import DetailsTab from "$lib/components/service-details/DetailsTab.svelte";
  import EnvironmentTab from "$lib/components/service-details/EnvironmentTab.svelte";
  import ConfigTab from "$lib/components/service-details/ConfigTab.svelte";
  import VersionsTab from "$lib/components/service-details/VersionsTab.svelte";
  import LogsTab from "$lib/components/service-details/LogsTab.svelte";
  import SettingsTab from "$lib/components/service-details/SettingsTab.svelte";

  const serviceId = $derived(page.params.id);
  const service = $derived(store.merged.find((s) => s.id === serviceId));

  let activeTab = $state<
    "details" | "env" | "config" | "versions" | "logs" | "setup" | "settings"
  >("details");
  let activeAction = $state("");

  let serviceDetail = $state<ServiceDefinition | null>(null);
  let detailLoading = $state(false);

  let artifactInfo = $state<ArtifactInfo | null>(null);
  let artifactLoading = $state(false);
  let uploadVersion = $state("");
  let uploadFile = $state<File | null>(null);
  let ghRepo = $state("");
  let ghVersion = $state("");
  let ghAsset = $state("");

  let serviceConfig = $state<ServiceConfig | null>(null);
  let configLoading = $state(false);
  let configValues = $state<Record<string, string | boolean>>({});
  let configSaving = $state(false);

  let logs = $state<LogEntry[]>([]);
  let logEventSource: EventSource | null = null;
  let logsRefreshing = $state(false);

  let stats = $state<ServiceStats | null>(null);
  let statsInterval: ReturnType<typeof setInterval> | null = null;

  let settingsSaving = $state(false);

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
            return;
          }
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
      toaster.create({ title: msg, type: "error" });
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
      toaster.create({ title: msg, type: "error" });
    } finally {
      artifactLoading = false;
    }
  }

  async function loadServiceConfig() {
    if (!service) return;
    configLoading = true;
    try {
      serviceConfig = await api.getServiceConfig(service.id);
      const initialValues: Record<string, string | boolean> = {};
      for (const field of serviceConfig.fields) {
        if (field.field_type === "boolean") {
          initialValues[field.key] = field.value === "true";
        } else {
          initialValues[field.key] = field.value;
        }
      }
      configValues = initialValues;
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Failed to load config";
      toaster.create({ title: msg, type: "error" });
    } finally {
      configLoading = false;
    }
  }

  async function saveServiceConfig() {
    if (!service) return;
    configSaving = true;
    try {
      const valuesToSave: Record<string, string> = {};
      for (const [key, value] of Object.entries(configValues)) {
        valuesToSave[key] = String(value);
      }
      const result = await api.updateServiceConfig(service.id, valuesToSave);
      toaster.create({
        title: result.message,
        type: result.status ? "success" : "error",
      });
      await loadServiceConfig();
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Failed to save config";
      toaster.create({ title: msg, type: "error" });
    } finally {
      configSaving = false;
    }
  }

  async function createTemplateConfig(
    fields: Record<
      string,
      { value: string; field_type: string; description: string }
    >,
  ) {
    if (!service) return;
    try {
      const result = await api.createConfigTemplate(service.id, fields);
      toaster.create({
        title: result.message,
        type: result.status ? "success" : "error",
      });
      if (result.status) {
        await loadServiceConfig();
      }
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Failed to create template";
      toaster.create({ title: msg, type: "error" });
    }
  }

  async function deleteTemplate() {
    if (!service) return;
    try {
      const result = await api.deleteConfigTemplate(service.id);
      toaster.create({
        title: result.message,
        type: result.status ? "success" : "error",
      });
      if (result.status) {
        await loadServiceConfig();
      }
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Failed to delete template";
      toaster.create({ title: msg, type: "error" });
    }
  }

  async function saveServiceSettings(config: {
    auto_restart: boolean;
    restart_limit: number | null;
    env: Record<string, string>;
    args: string[];
  }) {
    if (!service) return;
    settingsSaving = true;
    try {
      const result = await api.configureService(service.id, config);
      toaster.create({
        title: result.message,
        type: result.status ? "success" : "error",
      });
      if (result.status) {
        await loadServiceDetail();
        await store.refresh();
      }
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Failed to save settings";
      toaster.create({ title: msg, type: "error" });
    } finally {
      settingsSaving = false;
    }
  }

  async function loadServiceDetail() {
    if (!serviceId) return;
    detailLoading = true;
    try {
      serviceDetail = await api.getService(serviceId);
      if (serviceDetail && !serviceDetail.ready) {
        activeTab = "setup";
      }
    } catch (e: unknown) {
      const msg =
        e instanceof Error ? e.message : "Failed to load service details";
      toaster.create({ title: msg, type: "error" });
    } finally {
      detailLoading = false;
    }
  }

  async function handleUpload() {
    if (!service || !uploadFile || !uploadVersion) return;
    activeAction = "upload";
    try {
      const result = await api.uploadArtifact(
        service.id,
        uploadFile,
        uploadVersion.trim(),
      );
      toaster.create({
        title: result.message,
        type: result.status ? "success" : "error",
      });
      if (result.status) {
        await loadArtifactInfo();
        await loadServiceDetail();
        await store.refresh();
        uploadVersion = "";
        uploadFile = null;
        activeTab = "details";
      }
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Upload failed";
      toaster.create({ title: msg, type: "error" });
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
        ghRepo.trim(),
        ghVersion.trim(),
        ghAsset.trim(),
      );
      toaster.create({
        title: result.message,
        type: result.status ? "success" : "error",
      });
      if (result.status) {
        await loadArtifactInfo();
        await loadServiceDetail();
        await store.refresh();
        ghRepo = "";
        ghVersion = "";
        ghAsset = "";
        activeTab = "details";
      }
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Install failed";
      toaster.create({ title: msg, type: "error" });
    } finally {
      activeAction = "";
    }
  }

  async function loadLogs() {
    if (!serviceId) return;
    logsRefreshing = true;
    try {
      const data = await api.getLogs(serviceId);
      logs = data.logs;
    } catch (e: unknown) {
      console.error("Failed to load logs:", e);
    } finally {
      logsRefreshing = false;
    }
  }

  function startLogStreaming() {
    if (!serviceId || logEventSource) return;

    logEventSource = api.streamLogs(serviceId);

    logEventSource.onmessage = (event) => {
      try {
        const logEntry: LogEntry = JSON.parse(event.data);
        logs = [...logs, logEntry];
      } catch (e) {
        console.error("Failed to parse log entry:", e);
      }
    };

    logEventSource.onerror = (error) => {
      console.error("Log stream error:", error);
      stopLogStreaming();
    };
  }

  function stopLogStreaming() {
    if (logEventSource) {
      logEventSource.close();
      logEventSource = null;
    }
  }

  async function loadStats() {
    if (!serviceId) return;
    try {
      stats = await api.getServiceStats(serviceId);
    } catch (e: unknown) {
      console.error("Failed to load stats:", e);
    }
  }

  function startStatsPolling() {
    if (statsInterval) return;
    loadStats();
    statsInterval = setInterval(() => {
      loadStats();
    }, 2000);
  }

  function stopStatsPolling() {
    if (statsInterval) {
      clearInterval(statsInterval);
      statsInterval = null;
    }
  }

  async function handleClearLogsPermanent() {
    if (!serviceId) return;
    try {
      const result = await api.clearLogs(serviceId);
      toaster.create({
        title: result.message,
        type: result.status ? "success" : "error",
      });
      if (result.status) {
        logs = [];
      }
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Failed to clear logs";
      toaster.create({ title: msg, type: "error" });
    }
  }

  onMount(() => {
    store.startPolling(3000);
    loadServiceDetail();
    startStatsPolling();
  });

  onDestroy(() => {
    store.stopPolling();
    stopLogStreaming();
    stopStatsPolling();
  });

  // Watch activeTab and manage log streaming
  $effect(() => {
    if (activeTab === "logs") {
      loadLogs();
      startLogStreaming();
    } else {
      stopLogStreaming();
    }
  });
</script>

<main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
  <button
    class="inline-flex items-center gap-2 text-sm mb-6 transition-colors btn"
    onclick={() => goto("/")}
  >
    <ArrowLeft class="w-4 h-4" />
    Back to Services
  </button>

  {#if detailLoading && !service}
    <!-- Loading skeleton -->
    <div class="w-full space-y-6">
      <!-- ServiceHeader skeleton -->
      <div class="card bg-surface-50-950/60 p-6">
        <div class="flex items-start justify-between mb-4">
          <div class="flex-1 space-y-3">
            <div class="placeholder h-8 w-48 animate-pulse"></div>
            <div class="placeholder h-4 w-64 animate-pulse"></div>
            <div class="placeholder h-4 w-32 animate-pulse"></div>
          </div>
          <div class="flex items-center gap-3">
            <div class="placeholder h-8 w-24 animate-pulse rounded-full"></div>
            <div class="placeholder h-8 w-24 animate-pulse rounded-full"></div>
          </div>
        </div>
        <div class="flex items-center gap-2 flex-wrap">
          <div class="placeholder h-10 w-24 animate-pulse rounded-md"></div>
          <div class="placeholder h-10 w-24 animate-pulse rounded-md"></div>
          <div class="placeholder h-10 w-28 animate-pulse rounded-md"></div>
          <div class="placeholder h-10 w-24 animate-pulse rounded-md"></div>
        </div>
      </div>

      <!-- Tabs skeleton -->
      <div class="card bg-surface-50-950/60 overflow-hidden">
        <div class="border-b border-surface-200/10">
          <div class="flex gap-4 px-6 py-3">
            <div class="placeholder h-6 w-20 animate-pulse"></div>
            <div class="placeholder h-6 w-24 animate-pulse"></div>
            <div class="placeholder h-6 w-20 animate-pulse"></div>
            <div class="placeholder h-6 w-24 animate-pulse"></div>
            <div class="placeholder h-6 w-16 animate-pulse"></div>
          </div>
        </div>
        <div class="p-6 space-y-4">
          <div class="placeholder h-4 w-full animate-pulse"></div>
          <div class="grid grid-cols-2 gap-4">
            <div class="placeholder h-20 animate-pulse"></div>
            <div class="placeholder h-20 animate-pulse"></div>
          </div>
          <div class="placeholder h-4 w-3/4 animate-pulse"></div>
          <div class="placeholder h-4 w-full animate-pulse"></div>
        </div>
      </div>
    </div>
  {:else if !service}
    <div class="card bg-surface-50-950/60 p-12 text-center">
      <p class="opacity-60">Service not found</p>
    </div>
  {:else}
    <ServiceHeader
      {service}
      {serviceDetail}
      {stats}
      {activeAction}
      onAction={handleAction}
    />

    <div class="card bg-surface-50-950/60 overflow-hidden">
      <div class="border-b border-surface-200/10">
        <div class="flex overflow-x-auto">
          {#if serviceDetail && !serviceDetail.ready}
            <button
              class="px-6 py-3 text-sm font-medium transition-colors border-b-2 {activeTab ===
              'setup'
                ? 'border-current'
                : 'border-transparent opacity-60 hover:opacity-100'}"
              onclick={() => (activeTab = "setup")}
            >
              Setup
            </button>
          {/if}
          <button
            class="px-6 py-3 text-sm font-medium transition-colors border-b-2 {activeTab ===
            'details'
              ? 'border-current'
              : 'border-transparent opacity-60 hover:opacity-100'}"
            onclick={() => (activeTab = "details")}
          >
            Details
          </button>
          <button
            class="px-6 py-3 text-sm font-medium transition-colors border-b-2 {activeTab ===
            'env'
              ? 'border-current'
              : 'border-transparent opacity-60 hover:opacity-100'}"
            onclick={() => (activeTab = "env")}
          >
            Environment
          </button>
          <button
            class="px-6 py-3 text-sm font-medium transition-colors border-b-2 {activeTab ===
            'config'
              ? 'border-current'
              : 'border-transparent opacity-60 hover:opacity-100'}"
            onclick={() => {
              activeTab = "config";
              if (!serviceConfig) loadServiceConfig();
            }}
          >
            Config
          </button>
          <button
            class="px-6 py-3 text-sm font-medium transition-colors border-b-2 {activeTab ===
            'versions'
              ? 'border-current'
              : 'border-transparent opacity-60 hover:opacity-100'}"
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
              ? 'border-current'
              : 'border-transparent opacity-60 hover:opacity-100'}"
            onclick={() => (activeTab = "logs")}
          >
            Logs
          </button>
          <button
            class="px-6 py-3 text-sm font-medium transition-colors border-b-2 {activeTab ===
            'settings'
              ? 'border-current'
              : 'border-transparent opacity-60 hover:opacity-100'}"
            onclick={() => (activeTab = "settings")}
          >
            Settings
          </button>
        </div>
      </div>

      <div class="p-6">
        {#if activeTab === "setup"}
          <SetupTab
            {activeAction}
            bind:uploadVersion
            bind:uploadFile
            bind:ghRepo
            bind:ghVersion
            bind:ghAsset
            onUpload={handleUpload}
            onGithubInstall={handleGithubInstall}
          />
        {:else if activeTab === "details"}
          <DetailsTab {serviceDetail} />
        {:else if activeTab === "env"}
          <EnvironmentTab {serviceDetail} />
        {:else if activeTab === "config"}
          <ConfigTab
            {serviceConfig}
            {configLoading}
            bind:configValues
            {configSaving}
            onSave={saveServiceConfig}
            onCreateTemplate={createTemplateConfig}
            onDeleteTemplate={deleteTemplate}
          />
        {:else if activeTab === "versions"}
          <VersionsTab
            {artifactInfo}
            {artifactLoading}
            {activeAction}
            bind:uploadVersion
            bind:uploadFile
            bind:ghRepo
            bind:ghVersion
            bind:ghAsset
            onUpload={handleUpload}
            onGithubInstall={handleGithubInstall}
          />
        {:else if activeTab === "logs"}
          <LogsTab
            {logs}
            onClear={() => (logs = [])}
            onClearPermanent={handleClearLogsPermanent}
            onRefresh={loadLogs}
            refreshing={logsRefreshing}
          />
        {:else if activeTab === "settings"}
          <SettingsTab
            {serviceDetail}
            saving={settingsSaving}
            onSave={saveServiceSettings}
          />
        {/if}
      </div>
    </div>
  {/if}
</main>
