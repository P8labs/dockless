<script lang="ts">
  import { page } from "$app/stores";
  import { onMount, onDestroy } from "svelte";
  import { goto } from "$app/navigation";
  import { store } from "$lib/services.svelte";
  import * as api from "$lib/api";
  import { toaster } from "$lib/components/Toast.svelte";
  import { get } from "svelte/store";
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

  const serviceId = $derived(get(page).params.id);
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
        uploadVersion,
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
        ghRepo,
        ghVersion,
        ghAsset,
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
    try {
      const data = await api.getLogs(serviceId);
      logs = data.logs;
    } catch (e: unknown) {
      console.error("Failed to load logs:", e);
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
    loadLogs();
    startLogStreaming();
    startStatsPolling();
  });

  onDestroy(() => {
    store.stopPolling();
    stopLogStreaming();
    stopStatsPolling();
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

  {#if !service}
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
