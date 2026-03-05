<script lang="ts">
  import { goto } from "$app/navigation";
  import * as api from "$lib/api";
  import { toaster } from "$lib/components/Toast.svelte";
  import { store } from "$lib/services.svelte";
  import { ArrowLeft, Plus, Trash2 } from "lucide-svelte";

  let name = $state("");
  let autoRestart = $state(true);
  let restartLimit = $state<string>("3");
  let capInput = $state("");
  let capabilities = $state<string[]>([]);
  let loading = $state(false);

  function addCapability() {
    const cap = capInput.trim();
    if (cap && !capabilities.includes(cap)) {
      capabilities = [...capabilities, cap];
    }
    capInput = "";
  }

  function removeCapability(i: number) {
    capabilities = capabilities.filter((_, idx) => idx !== i);
  }

  async function handleCreate() {
    if (!name.trim()) return;

    loading = true;
    try {
      const limit =
        restartLimit.trim() !== "" ? parseInt(restartLimit, 10) : null;
      const result = await api.initService(name.trim(), undefined, {
        auto_restart: autoRestart,
        restart_limit: limit !== null && !isNaN(limit) ? limit : null,
        linux_capabilities: capabilities,
      });

      toaster.create({
        title: result.message,
        type: result.status ? "success" : "error",
      });

      if (result.status && result.id) {
        await store.refresh();
        goto(`/service/${result.id}`);
      }
    } catch (e: unknown) {
      const msg = e instanceof Error ? e.message : "Failed to create service";
      toaster.create({ title: msg, type: "error" });
    } finally {
      loading = false;
    }
  }
</script>

<main class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-8">
  <button
    class="inline-flex items-center gap-2 text-sm mb-6 transition-colors btn"
    onclick={() => goto("/")}
  >
    <ArrowLeft class="w-4 h-4" />
    Back to Services
  </button>

  <div class="max-w-xl mx-auto">
    <div class="card bg-surface-50-950/60 p-8">
      <div class="mb-6">
        <h2 class="text-2xl font-semibold mb-2">Create Service</h2>
        <p class="text-sm opacity-70">
          Create a new service. You'll be able to configure it with environment
          variables, upload binaries, and manage versions after creation.
        </p>
      </div>

      <div class="space-y-6">
        <div>
          <label class="label">
            <span class="label-text">Service Name</span>
            <input
              id="svc-name"
              class="input"
              type="text"
              bind:value={name}
              placeholder="my-service"
              onkeydown={(e) => {
                if (e.key === "Enter" && name.trim() && !loading) {
                  handleCreate();
                }
              }}
            />
          </label>
          <p class="text-xs opacity-60 mt-1.5">
            A service ID will be automatically generated from the name
          </p>
        </div>

        <div class="space-y-3">
          <label class="flex items-center gap-3 cursor-pointer">
            <input
              type="checkbox"
              bind:checked={autoRestart}
              class="w-4 h-4 rounded"
            />
            <div>
              <p class="text-sm font-medium">Auto Restart</p>
              <p class="text-xs opacity-60">Restart automatically on crash</p>
            </div>
          </label>

          <div>
            <label for="restart-limit" class="block text-xs opacity-60 mb-1">
              Restart Limit (leave empty for unlimited)
            </label>
            <input
              id="restart-limit"
              type="number"
              bind:value={restartLimit}
              placeholder="Unlimited"
              min="0"
              step="1"
              class="input w-full max-w-xs"
            />
          </div>
        </div>

        <div>
          <p class="text-sm font-medium mb-2">Linux Capabilities</p>
          <p class="text-xs opacity-60 mb-3">
            Grant capabilities to the binary via <code class="font-mono"
              >setcap</code
            >, e.g. <code class="font-mono">cap_net_raw+eip</code>
          </p>
          <div class="space-y-2">
            {#each capabilities as cap, i}
              <div class="flex items-center gap-2">
                <span
                  class="font-mono text-xs px-2 py-1 rounded bg-surface-100-800 flex-1"
                  >{cap}</span
                >
                <button
                  class="btn preset-outlined-error w-8 h-8 p-0 flex items-center justify-center"
                  onclick={() => removeCapability(i)}
                  title="Remove"
                >
                  <Trash2 class="w-3 h-3" />
                </button>
              </div>
            {/each}
            <div class="flex items-center gap-2">
              <input
                type="text"
                bind:value={capInput}
                placeholder="e.g. cap_net_raw+eip"
                class="input flex-1 font-mono text-sm"
                onkeydown={(e) => {
                  if (e.key === "Enter") {
                    e.preventDefault();
                    addCapability();
                  }
                }}
              />
              <button
                class="btn preset-outlined w-8 h-8 p-0 flex items-center justify-center"
                onclick={addCapability}
                title="Add capability"
              >
                <Plus class="w-4 h-4" />
              </button>
            </div>
          </div>
        </div>

        <div class="flex items-center gap-3 pt-4">
          <button
            type="button"
            class="btn preset-outlined flex-1"
            disabled={!name.trim() || loading}
            onclick={handleCreate}
          >
            {#if loading}
              <span
                class="w-4 h-4 border-2 border-current border-t-transparent rounded-full animate-spin shrink-0"
              ></span>
            {/if}
            Create & Configure
          </button>
          <button
            type="button"
            class="btn preset-tonal"
            onclick={() => goto("/")}
          >
            Cancel
          </button>
        </div>
      </div>
    </div>
  </div>
</main>
